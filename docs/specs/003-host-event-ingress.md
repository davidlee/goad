# SPEC-003: Host event ingress

**Status:** active
**Kind:** technical
**Owns:** the local socket a user-owned watcher writes an event to — its
lifecycle, the envelope it accepts, the reply it returns, and the closed set of
reasons an envelope may be refused.

<!-- A spec is evergreen and normative: it describes what is true now, not how
     it came to be true. No changelog, no revision history, no "we used to".
     Amending it requires explicit user endorsement.
     Requirement ids (R-N) are immutable — append, never renumber. Cite from
     elsewhere as SPEC-003/R-N. SPEC-002 also numbers a requirement 12; both
     ids are always written qualified, per §2 Boundaries. -->

## 1. Intent

goad evaluates when it starts, when a person asks, and when a check comes due.
Nothing outside the process can make it ask its backend anything, so a
user-owned watcher that has decided *something interesting just happened* has
nowhere to say so. Brief §7 puts that watcher outside the host deliberately: the
host must not subscribe to window-manager, browser or filesystem event streams,
because filtering, classification and debouncing are the user's own code's job.

This document is the door between the two. It says what a watcher writes, what
it gets back, and what the host does with what it was given. The host reads the
envelope's *shape* — four fields, their presence and their form — and nothing
else. `data` is opaque, and `kind` is the watcher's own vocabulary carried
verbatim.

Once this exists, a watcher author can write to a published contract in any
language that can open a Unix socket, and know from the reply whether the host
took the event and, if not, which of a closed set of reasons applied. A second
host implementation can be held to the same contract.

## 2. Scope

**In scope:** the socket's path, mode and lifecycle, including what the host
does when the path is already occupied; the event envelope's wire form and the
rules that admit or refuse it; the reply's wire form; the closed set of refusal
reasons; the bounds on a single connection; and the minimum spacing's
*observable consequence* at the socket.

**Out of scope:** what a backend does with the resulting `evaluate` — that is
SPEC-001's; how the host bounds its own evaluation rate, which is SPEC-002's
subject and only *observed* here; how a next check is resolved; persistence of
an event across host restarts, of which there is none; any command-line client;
peer-credential checks and multi-user hardening; and any interpretation of an
event.

**Boundaries:** this spec abuts SPEC-001 at exactly two points. It **produces**
an `evaluate` whose event SPEC-001/R-7 shapes and whose `data` SPEC-001/R-9
protects, and it **enforces** at ingress what SPEC-001/R-56 reserves at
emission. It abuts SPEC-002 at one point: the spacing **SPEC-002/R-12** requires
of an ingested evaluation is what **this spec's own R-12** (below) makes visible
at the socket. The two are different requirements in different documents that
happen to share a number; every mention of either across a document boundary is
qualified, here and everywhere else. It reads no backend response and writes no
schedule.

The **backend** transport (brief §6.1) is a different socket, in the other
direction, under a different contract. The two share a word and nothing else.

## 3. Principles

**P-A — The envelope is shape, never meaning.** The host reads four fields'
presence and form. It compares `source` against exactly one reserved string and
otherwise interprets nothing: not `kind`, not `data`, and not how far
`timestamp` is from now. A host that branches on any of them has learned the
user's domain, which is the one thing this product refuses to do.

**P-B — Every envelope is answered, and the answer is true.** There is no
silent acceptance and no silent refusal. A reply that says *accepted* means the
host acted on it; a reply that says *refused* names which of a closed set of
reasons applied. The host holds no queue, so it never answers *accepted* for
something it has merely remembered.

**P-C — The writer decides what to do about a refusal.** Filtering, debouncing
and retry belong to the watcher (brief §7). The host's obligation ends at
telling the truth promptly, and it never delays an envelope in order to avoid
refusing it — a bound expressed as a silent delay is a bound the writer cannot
see.

**P-D — An absolute clause names its own exception.** A sentence here that says
a mechanism *always* holds or *never* fails is making a claim about the
mechanism, not about how much it was wanted to be true. Where the mechanism has
an exception, the clause names it and bounds it; a clause that cannot say where
its exception would be is a clause that has not been checked, and this document
treats that as a defect in the clause rather than a matter of emphasis.

**The criterion, stated so it can be applied rather than agreed with.** An
absolute clause here is read by asking *which side decides the thing it is
absolute about* — and a clause that cannot answer has not been checked, whatever
else is true of it. §6.3 applies this to which refusals reach a person: a clause
saying a refusal *always* reaches someone is making a claim about **who decided
it**, not about how important it is, and §6.3's own list is grouped by deciding
side for that reason rather than by reason token. §6.1's fork window is the same
criterion applied to the lock. Amending this spec means meeting the criterion
for any absolute clause added, and a clause that cannot meet it is not ready to
be written down (`docs/slices/004/review-code.md` F-3, F-15, F-19).

## 4. Requirements

| id | requirement | verified by |
|----|-------------|-------------|
| R-1 | The host MUST bind a listening Unix domain socket if, and only if, its configuration names a path for one. With no path configured the host MUST bind nothing and MUST behave exactly as a host without this capability. | §7 |
| R-2 | The host MUST set the socket's mode to owner-only itself, and MUST NOT rely on the umask it was started under. | §7 |
| R-3 | **Liveness is an exclusive advisory lock, and never a connection to the socket.** The host MUST decide whether a live host holds the path by taking an exclusive lock on a sidecar file beside it, and MUST hold that lock for as long as the process runs. Taking the lock means no live host holds the path: a socket at the path MUST be reclaimed — the host unlinks it and binds. Being unable to take it because another process holds it means a live host does hold the path: that MUST be a startup failure naming the path, whatever is or is not at the path itself, and the running host MUST keep its socket. Liveness MUST NOT be assumed either way when it cannot be determined — a lock the host holds the file for but cannot ask is a startup failure naming the path, and says that is what happened. A lock file that cannot be **opened** is a different failure and is R-4's: the location is unusable, and the message must name that rather than a liveness question. The path is inspected **without following symbolic links**, so this requirement is about a socket *at* the path and never about one a link at the path points to (R-4). **The lock file's own path is inspected the same way, and for the same reason**: anything there that is not a regular file — a symbolic link above all — MUST NOT be opened, and MUST be a startup failure naming the socket's path and saying the lock beside it is what was wrong. Following a link would put the owner-only mode R-2 requires on a file the configuration did not name. | §7 |
| R-4 | A path occupied by anything that is not a socket, and any other failure to bind or to set the mode, MUST be a startup failure naming the path and what was found. A **symbolic link** at the path is one such thing: it MUST NOT be followed, and MUST be a startup failure naming it as a symlink, whatever it points at — including a socket a live host holds, which is a startup failure as a symlink rather than as R-3's in-use case. Following a link would put the owner-only mode R-2 requires on a file the configuration did not name. The host MUST NOT start without the listener its configuration asked for. | §7 |
| R-5 | The host MUST NOT unlink the socket on exit, and MUST NOT unlink the lock file beside it either. Reclamation (R-3) is the one mechanism, so it is exercised on every ordinary restart rather than only after a crash. A lock file nobody holds is not a claim: the **lock** is the signal, not the file. | §7 |
| R-6 | One connection carries exactly one envelope. The host MUST read until the first newline or until end of input, whichever comes first, and MUST NOT read further on that connection. | §7 |
| R-7 | The host MUST bound every read from a connection in both bytes and time, and both bounds MUST be stated rather than implied. Exceeding either is a refusal, reported before the connection is closed. | §7 |
| R-8 | Every envelope MUST receive exactly one reply on the same connection, after which the host closes it. The only case in which a connection may close unanswered is one in which the host process itself is gone. | §7 |
| R-9 | An envelope MUST be one JSON object carrying exactly four keys: `source`, `kind`, `timestamp` and `data`. A top-level value that is well-formed JSON but is **not an object** MUST be refused, naming the type found. A missing key, a key of the wrong type, an empty `source` or `kind`, a key the object repeats at any depth, and any key beside the four MUST each be refused, and the refusal MUST name the key. | §7 |
| R-10 | `timestamp` MUST be an RFC 3339 instant carrying an explicit UTC offset. One without an offset MUST be refused with a reason distinct from a general parse failure, exactly as SPEC-001/R-22 requires of a backend's instant. | §7 |
| R-11 | The host MUST NOT interpret `data`, MUST NOT interpret `kind`, and MUST NOT judge `timestamp` beyond its form. All four fields reach the backend as the event of an `evaluate` (SPEC-001/R-7): `source` and `kind` as the strings sent, `data` as the **value** sent — carried whole and read into nowhere — and `timestamp` as the instant sent. As with `timestamp` in §6.2, the value is preserved and its *spelling* is not: `data` round-trips through the host's JSON parser, so a key order or a numeric precision the host cannot represent is not a promise this requirement makes. The request's own `now` is the host's instant, not the envelope's. | §7 |
| R-12 | An envelope arriving while an exchange is in flight, or within the minimum spacing **SPEC-002/R-12** sets after the previous ingested evaluation the host attempted, MUST be refused naming which, and MUST NOT be queued, delayed or coalesced. The host holds no pending event. | §7 |
| R-13 | An envelope naming `source` of `"host"` MUST be refused. That value is reserved to evaluations the host originates (SPEC-001/R-56), and a backend's right to read it as such depends on this refusal. | §7 |
| R-14 | Every refusal MUST carry a machine-readable reason drawn from the closed set in §6.3, and MAY carry human-readable detail. A reader MUST NOT branch on the detail. A `too_soon` refusal MUST additionally carry `retry_after_ms`: a whole number of milliseconds, measured at the moment of refusal and **rounded up**, after which the spacing will have elapsed. Rounding up is required rather than incidental — a truncated remainder leaves a writer that waits exactly that long still inside the spacing, which would make this requirement's own sentence false of the host's own field. No other reason carries that field, and a reader MAY act on it. | §7 |
| R-15 | A refusal the host decides **while no exchange is in flight** MUST also be reported on the host's own diagnostics surface, so that it is visible to a person who is not the writer. A refusal decided while an exchange *is* in flight, and one decided after the host's loop has ended, are reported to the writer only. This is a bound on what the surface can hold, not a licence to be silent: **every envelope's** refusal reaches its writer in the reply R-8 requires. The one refusal that reaches no writer is the one that answers no envelope — the ingress-stopped `unavailable` of §6.3, for which this surface is the only report there is. | §7 |
| R-16 | No envelope, however malformed, may terminate the host, cause it to panic, or leave it unable to invoke its backend again. A malformed envelope MUST NOT reach the backend. | §7 |

## 5. Behaviour

**The ordinary case.** A watcher connects, writes one envelope terminated by a
newline or by closing its write side, and reads one line. The host normalizes
the envelope, finds itself idle and outside the spacing, answers `accepted`, and
begins an `evaluate` carrying the four fields and its own instant. Whether
anything appears on screen is the backend's decision and no part of this
contract.

```mermaid
sequenceDiagram
  participant W as watcher
  participant H as host
  participant B as backend
  W->>H: {"source":…,"kind":…,"timestamp":…,"data":…}\n
  H->>H: read (bounded), normalize
  alt idle and outside the spacing
    H-->>W: {"protocol":1,"accepted":true}
    H->>B: evaluate { now: host's instant, event }
  else
    H-->>W: {"protocol":1,"accepted":false,"reason":…,"detail":…}
  end
  H->>W: close
```

**Order of judgement.** A refusal about the envelope's *shape* takes precedence
over one about the host's *state*: a malformed envelope is malformed whatever
the host was doing, and it is the more actionable thing for its writer to be
told. This holds **while an exchange is in flight as much as while the host is
idle** — a shape refusal is never reported as `engaged` because of timing.

**What the spacing looks like from outside.** The host bounds how often it
begins an ingested evaluation, and the bound is SPEC-002/R-12's. Here it is visible
as a refusal rather than as a delay (P-C): a writer emitting flat out receives
one `accepted` per spacing interval and `too_soon` for everything in between.
**Events are lost under load, by design and visibly.** The watcher decides
whether to retry, because only the watcher knows whether the event still means
anything.

An evaluation the host *attempted* and could not complete — its clock was
unreadable, say — still counts against the spacing, so a broken clock produces
one refusal per interval rather than a spin.

**When the host is stopping.** An envelope in flight when the loop ends is
refused as `unavailable`. Once the process is gone, a connection closes with no
reply; that is the single case R-8 admits.

**When ingress stops but the host does not.** Whatever accepts connections may
end while the host keeps running, and nothing restarts it. The host reports that
once, as `unavailable`, on the surface R-15 names — the only place it can, since
no envelope reaches it afterwards to be refused. This one is unconditional on
the host's state: it is reported whether the loop was idle or mid-exchange when
ingress died, which is the one exception to §6.3's rule that a refusal reaching
a person depends on which side decided it. The host itself is unaffected
and keeps evaluating (R-16).

**What the host never does.** It holds no queue and no pending event; it keeps
no record of what any source has sent; it does not deduplicate; it does not rate
a source differently from any other; and it never asks whether an event is
interesting. Each of those is the watcher's, and a host that took one on would
have to understand the domain to do it.

## 6. Interfaces & contracts

### 6.1 Configuration and the socket

The socket's path comes from the host's configuration and has no default. The
mode is owner-only. **The containing directory is the user's responsibility**: a
directory another user can write lets that user replace the socket, and the mode
does not reach that. This is a stated limit, not a defect.

**The lock beside the socket is what says a host is live** (R-3). It is a
regular file at the socket's own path with `.lock` appended, opened owner-only
and locked exclusively; the host holds that lock from before it binds until the
process exits, and reads it only by trying to take it. A **connection to the
socket cannot answer the same question**: a successful `connect` says a
listening socket is bound at the path, not that a host holds it, and a `fork`
duplicates a listening descriptor into the child — so the socket stays bound and
connectable after its owner has closed its own descriptor, for as long as any
child holds the copy. A host that forks reads its own stale sockets as live and
refuses to start against a path nobody holds.

**Inheritance is not what makes the lock safe.** A `flock` belongs to the open
file description, so a `fork` duplicates it exactly as it duplicates a listening
descriptor. What closes the gap is narrower: **the lock is never released while
a host lives.** The failure the probe had needed a release followed by a probe
of the same path, with an unrelated concurrent `fork` supplying the gap between
them; under this rule a path that reads as reclaimable has never been locked by
anyone at the moment the bind reaches it, so there is no release to race.
(Measured, three implementations: `docs/slices/004/review-code.md` F-18.)

**The filesystem holding the path must support advisory locking**, which most
do and some — various FUSE mounts, and NFS depending on version and mount
options — do not. On one that does not, liveness cannot be determined and the
host does not start (R-3); the remedy is to put the socket somewhere that can
lock. This is an environmental requirement of the location, like the containing
directory's permissions above.

Two consequences follow, and both are properties of this contract rather than
limits on it. **Two hosts starting in the same instant cannot both bind**:
the lock is exclusive, so exactly one takes it and the other meets R-3's
startup failure — the check for a live holder and the bind are not one atomic
step, but the lock spans both. And **one live host per socket path** is
enforced for as long as that host runs, which is single-instance enforcement
for this path and no more: nothing here bounds how many hosts run against
*different* paths, or none.

The exclusion is only as durable as the lock file's inode. Removing the lock
file while a host runs lets the next one create a fresh inode and take a lock
on that, which is the same exposure the socket file already has — **the
containing directory is the user's responsibility**, as above.

**Non-normative limit — the fork window at a host's death.** Because the lock
belongs to the open file description, an un-exec'd child of the holder holds it
too: `CLOEXEC` ends that at `exec` and not before. A host that dies while a
child of its own is inside that window leaves the lock held until that child
execs or exits, and the next host to start reads that as a live holder and
refuses. The exception is bounded to the instant of a host's death and clears
itself — the next attempt succeeds — where the probe it replaced had a window
that recurred on every fork, for the whole of a process's life. It is stated rather than closed: closing it
would mean resting on how a spawn is implemented, which is the kind of unstated
accident F-18 was raised about.

**Non-normative limit — upgrade skew.** A socket left by a host that predates
this rule has no lock beside it, so it reads as stale and is unlinked. That is
right for a dead old host and wrong for a live one, which would go on serving a
socket the path no longer reaches (*the path after the bind*, below). One
restart of the old host closes it; the window is the single upgrade that
crosses this change.

**Non-normative limit — the path after the bind.** Nothing re-probes the path
once the socket is bound, and R-5 means the host never unlinks it either. A path
unlinked or replaced underneath a live listener — by a `rm`, or by a second host
reclaiming it under R-3 — leaves that listener holding a bound descriptor no
`connect` can reach. Later writers get a connection error, and the host reports
nothing, because from its side nothing arrives. This is a stated residue and not
a defect R-1..R-16 close; what would close it is the host re-probing its own
path after the bind, which this contract does not require.

### 6.2 The envelope

```json
{ "source": "reddit-watcher",
  "kind": "reddit-opened",
  "timestamp": "2026-08-22T17:10:00+10:00",
  "data": { "count_last_hour": 4 } }
```

| field | admits | notes |
|---|---|---|
| `source` | a non-empty string, not `"host"` | the only value the host compares against (R-13) |
| `kind` | a non-empty string | the watcher's vocabulary, carried |
| `timestamp` | RFC 3339 with an explicit offset | the instant is preserved; its **spelling** is not — the host emits the canonical UTC form |
| `data` | any JSON value | opaque (SPEC-001/R-9), and the envelope's extension point; like `timestamp` above, the **value** is preserved and its spelling is not — object key order is normalized and numbers past an IEEE 754 double lose precision |

**There is no version field and none is required.** The envelope's four fields
are fixed by SPEC-001/R-7, and anything a watcher wants to add goes inside
`data`. A key beside the four is therefore a mistake rather than a newer
watcher, and R-9 refuses it by name.

### 6.3 The reply

One JSON object, newline-terminated, then the host closes.

```json
{"protocol":1,"accepted":true}
{"protocol":1,"accepted":false,"reason":"too_soon","retry_after_ms":1800,"detail":"an event-triggered evaluation began 1.2s ago; the minimum spacing is 3s"}
```

`protocol` is this contract's version, not SPEC-001's; no client speaks both.
`accepted` is present on every reply. `reason` is present exactly when
`accepted` is `false`, and is drawn from this closed set:

| reason | means | the writer's fix |
|---|---|---|
| `malformed` | the bytes were not one JSON document | the serializer |
| `invalid_envelope` | the top-level value was not a JSON object; or a missing, wrong-typed, empty, unknown or duplicated key; or a timestamp the host could not read | the envelope; `detail` names the key, or the type found |
| `reserved_source` | `source` was `"host"` (R-13) | choose another source |
| `too_large` | the envelope exceeded the byte bound (R-7) | send less, or move bulk elsewhere |
| `timed_out` | nothing complete arrived within the time bound (R-7) | terminate the envelope with a newline, or close the write side |
| `engaged` | an exchange was already in flight and the envelope's shape was good (SPEC-002/R-9) | retry, or do not |
| `too_soon` | inside the minimum spacing (R-12) | coalesce in the watcher (brief §7), or wait `retry_after_ms` |
| `unavailable` | the host cannot act on this envelope — it is stopping, its clock is unreadable, the connection faulted before the envelope could be read, or its ingress has stopped for the life of the process | `detail` says which; wait, except for the last, where nothing will change |

`detail` is prose for a person. **Nothing may branch on it**, and its wording is
not part of this contract.

**`unavailable` covers four causes, and one of them does not pass.** The host
is stopping, its clock is unreadable, or the connection faulted while the
envelope was being read — all conditions of the moment, and *wait* (or
reconnect) is sound advice for each. A transport fault is deliberately **not**
`malformed`: that reason names the writer's serializer as the thing to fix, and
a writer whose bytes never arrived did not send bad ones. The fourth cause is
that the host's ingress has stopped: whatever accepts connections has ended, and nothing restarts it, so the
condition is **permanent for the life of the process**. A host in that state
takes no further envelope, so that cause never reaches a writer as a reply; the
diagnostics surface is the only report it has, and it is reported there
whichever state the loop was in when ingress died (§5). The reason set remains closed
at the eight above — what this admits is a fourth cause of one of them, not a
ninth token.

`retry_after_ms` is the one structured thing a writer may act on beyond
`reason`. It is present exactly when `reason` is `too_soon`, it is **rounded up**
to the millisecond so that waiting exactly that long is outside the spacing
rather than one truncated remainder short of it (R-14), and it is what
keeps R-14's prohibition on branching on `detail` from leaving a writer with
only two strategies — drop, or retry blind. It is **advice, not a reservation**:
the host holds nothing on the writer's behalf, an envelope sent after it may
still be refused for another reason, and a writer that ignores it is conforming.

**Which refusals a person sees.** Every envelope's refusal reaches its writer,
always (R-8). Only those the host decides while no exchange is in flight also
reach the diagnostics surface a person reads (R-15). **Which of the eight that
is turns on which side decided the refusal**, and it is worth reading that way
rather than reason by reason, because the reason is not what settles it:

- **Decided by whatever accepts connections**, before the host's loop has seen
  the envelope at all — `malformed`, `invalid_envelope`, `reserved_source`,
  `too_large`, `timed_out`, and the `unavailable` of a connection that faulted
  mid-read. These travel to the loop *with* the envelope, so the loop's state on
  arrival is what decides their fate: they reach the surface when it happened to
  be idle, and **not otherwise**. That the negative case exists is what makes
  shape-before-state (§5) a claim rather than a convenience — the host does not
  relabel a bad envelope as `engaged`, and it does not owe a person a report of
  one it was too busy to be idle for.
- **Decided by the host's loop, and only while nothing is in flight** —
  `too_soon`, and the `unavailable` of an unreadable clock. Both are judgements
  the loop makes about its own state (§5.4's steps 3 and 4), reachable only
  when it is idle, so for these two *always* is exact.
- **Decided during an exchange, by definition** — `engaged`. It never reaches
  the surface, because the state that produces it is the state that withholds
  it.
- **Decided with no loop left to present it** — the stopping `unavailable`.
  It never reaches the surface either.
- **Answering no envelope at all** — the ingress-stopped `unavailable`, which
  travels the other way: it reaches a person here or nowhere, because there is
  no envelope left for it to be the reply to. It is reported whichever state
  the loop was in when ingress died.

A clause of this paragraph that says a refusal *always* reaches a person is
therefore making a claim about **who decides it**, not about how important it
is. One that cannot say which side decides is a clause that has not been
checked.

### 6.4 The bounds

| bound | value | why it is stated |
|---|---|---|
| bytes per **read** | 64 KiB | an unbounded read from an untrusted writer is the defect SPEC-001/R-43 names on the other socket |
| time per **read** | 500 ms | a writer that connects and never completes an envelope must not hold ingress; the value is the transport's cleanup budget's sibling |

Both are host constants. Neither is configurable, and neither is readable by a
backend or by a watcher.

**What is not bounded, and why.** These bound the *read*, not the connection. A
connection's life is `accept → read (bounded) → hand the arrival to the host →
await its judgement → reply → close`, and **the wait for judgement has no
bound**: it ends when the host judges, and the host judges on the one thread it
does everything else on. A host that is not making progress therefore delays an
arrival exactly as it delays a due check, a person's click and a backend's
answer — ingress inherits that dependency rather than adding one, and no number
here would change it, because a timeout on the wait would mean answering an
envelope the host had not judged. The consequence is that a host holds **one**
arrival at a time (§5, no queue), so an unjudged arrival stops all ingress for
as long as it lasts, and the writer waits with it rather than being told
something untrue.

**One connection at a time, so the read bound is also a denial bound.** The
host accepts, serves and closes one connection before it accepts the next, so
the per-read bound above is *also* the longest a single writer can hold off
every other. A connection that opens and writes nothing holds ingress for the
full 500 ms; two such connections a second, from any process that can open the
socket, make ingress effectively unavailable to every legitimate watcher. This
is stated because a watcher author cannot derive it from the two numbers above,
and because it is the one way an envelope fails to arrive **without a
refusal**: the stalled connections are answered `timed_out` to *their own*
writers, and a watcher whose envelopes are simply never accepted sees latency
and nothing else. The socket's owner-only mode (§6.1) bounds who can do it to
the user's own uid, which on a desktop is every application the person runs.

## 7. Verification

Each row names the kind of verification and the test that discharges it, so the
claim is checkable rather than asserted. Paths are relative to the repository
root. A row naming no test is a row this spec may not be amended holding: where
a clause cannot be reached by a test, the row says so in terms and says what
review holds instead, rather than passing over it.

| requirement | verified by |
|---|---|
| R-1 | renderer, both arms of the **decision**, and integration for its consequence. `listener::some_path_binds` (`crates/goad/tests/renderer/startup.rs`) — a configuration naming a path is bound, and a real socket is at it; `listener::none_binds_nothing` (same file) — no path in the configuration, no file created. The two are written as a pair, each one's doc comment naming the other, so the negative is not vacuous; and it is the **pair** that holds R-1's *if and only if*, because both drive `listener(…)` over an `IngressConfig`, which is the value R-1 quantifies over. `ingress::a_well_formed_envelope_reaches_the_judge_as_the_event_it_wrote` (`crates/goad-shell/tests/integration/ingress.rs`) holds R-1's **consequence** — a bound path serves — and not its decision: it calls `bind` directly, and no configuration type appears in that file. Behaviour with the key absent is unchanged. Stated as an invariant that stays checkable rather than a snapshot that goes stale, against the close of slice 003 (`29e6d9a`): **no case in the `renderer`, `event_loop` or `event_loop_schedule` targets was removed or renamed** — all three grew by addition alone — and **no assertion inside them changed**. Both halves are one command each. The first: list the cases at either revision and difference the names. The second: `git diff 29e6d9a HEAD` over the **fourteen** files those three targets held at the base deletes 27 lines and no others — twenty-one `serve(…)` calls that gained `Ingress::none()`, and six lines rewritten in place in `renderer/main.rs` and `renderer/startup.rs`, being one `use` line and five lines of module or item documentation the slice made false or incomplete (*no socket opened*, *Eight modules*). Not one is a case, an assertion or a fixture; everything else in those files is addition. The scope is the fourteen files and not only the four that carry the `serve(…)` calls, because the sentence quantifies over the files: a command narrower than its own sentence proves less than it appears to. `crates/goad/src/main.rs` is deliberately outside the claim — it is production code and it did change, gaining the `startup::listener` call (`docs/slices/004/review-code.md` F-25, F-28) |
| R-2 | integration: `ingress::the_socket_is_owner_only_after_bind` (`crates/goad-shell/tests/integration/ingress.rs`) — the bound socket's mode is `0600` after `bind`. The host sets it **itself**, with `std::os::unix::fs::set_permissions`; no case sets a umask, because this workspace has no safe umask API and `umask(2)` is process-global while cases run in parallel |
| R-3 | integration, four arms (`crates/goad-shell/tests/integration/ingress.rs`): `ingress::a_stale_socket_with_no_listener_is_reclaimed_and_the_new_one_serves` and `ingress::a_live_socket_refuses_a_second_bind_and_keeps_serving` for the two outcomes; `ingress::a_socket_a_forked_child_still_holds_is_reclaimed_and_the_new_listener_serves` for **the case a `connect` gets wrong** — the child holds the listening descriptor as its stdin, so the socket stays connectable with no host behind it, and the case is deterministic rather than the race it was found as; and `ingress::a_live_host_keeps_its_path_after_the_socket_file_is_removed` for the lock being the signal rather than the file, which is also the arm no `connect` probe could reach at all. The lock's own name is `ingress::tests::the_lock_is_the_socket_s_own_path_with_a_suffix` (`crates/goad-shell/src/ingress/mod.rs`). The lock path's own symlink rule is `ingress::a_symlink_at_the_lock_path_is_refused_and_nothing_is_created_through_it`, and it asserts **two** things because the fault alone would not catch the damage: the refusal, and that nothing was created at the dangling link's target — the arm where following the link is what creates the owner-only file (`docs/slices/004/review-code.md` F-22). **R-3's third clause is review, not a test**: *liveness MUST NOT be assumed either way when it cannot be determined*. `BindFault::LivenessUnknown` (`crates/goad-shell/src/ingress/mod.rs`) is raised from the `TryLockError::Error` arm of `hold` in the same module — what a filesystem with no advisory locking returns — and no cooperating test in this workspace can produce one, the same position as R-4's exit code below and R-5's process exit. What review holds: the arm exists, it is the `Err` that is **not** `WouldBlock` so nothing can fall into it by default, and it carries the path into a startup failure like its siblings. It is declared rather than passed over because a row silent about a clause cannot be told from a row whose author overlooked one (`docs/slices/004/review-code.md` F-26) |
| R-4 | integration: `ingress::a_regular_file_at_the_path_is_refused_naming_what_was_found`, `ingress::a_directory_with_no_write_permission_is_refused_naming_the_path`, `ingress::a_symlink_to_a_live_socket_is_refused_unfollowed_and_the_target_keeps_serving` — the symlink rule, on the one case where it and R-3's letter disagree (all `crates/goad-shell/tests/integration/ingress.rs`); rendered beside every other `StartupError` variant by `display_text::ingress_is_unwrapped_and_unprefixed_and_names_the_path` and `stderr_outlets::report_startup_line_renders_ingress_like_its_siblings` (`crates/goad/tests/renderer/startup.rs`). **The non-zero exit is review, not a test**: no test target links the binary, and `main`'s single `match run()` (`crates/goad/src/main.rs`) maps every `Err` to exit 2. This requirement has two halves — *names the path* and *names what was found* — and they are verified by **two cases rather than one asserting both**, because the history here is one half moving while the other stayed true (`docs/slices/004/review-code.md` F-21): `a_directory_with_no_write_permission_is_refused_naming_the_path` holds the path, and `a_missing_directory_is_refused_as_an_unusable_location_not_as_an_unknown_liveness` holds the variant. A location the host cannot use is met at the **lock file** now rather than at the bind, and it is still R-4's fault and not R-3's last clause: the two are the same startup **failure** and not the same **message**, and R-4's requirement is about the message |
| R-5 | integration, on the drop path: `ingress::dropping_the_ingress_unlinks_neither_the_socket_nor_the_lock` (`crates/goad-shell/tests/integration/ingress.rs`) — bind a fresh path, drop the `Ingress`, and both the socket and its lock file are still there. The absence of an unlink is **behavioural** and so is assertable without asserting the absence of code, and it is what makes R-3's reclaim path the one every ordinary restart takes (`docs/slices/004/review-code.md` F-24). **What the case holds is narrower than this requirement's sentence, and the difference is the honest part**: process exit is not `Drop`, so nothing here speaks for a killed host. The drop path is the only path a regression could reach — `main` drops `Served.ingress` — which is what makes it the right bound to claim rather than a convenient one. Both halves were falsified by injection: an unlink of the lock after `try_lock`, and an `impl Drop for Ingress` removing the socket, each turn this case red. The *presence* half is asserted separately — `a_socket_a_forked_child_still_holds_…` checks the lock file is beside the socket after a successful bind |
| R-6 | integration: `ingress::an_envelope_terminated_by_a_newline_is_accepted`, `ingress::an_envelope_terminated_by_closing_the_write_side_is_accepted`, `ingress::a_second_envelope_on_the_same_connection_is_never_read` (`crates/goad-shell/tests/integration/ingress.rs`) |
| R-7 | integration: `ingress::more_than_the_byte_limit_is_refused_too_large_and_the_limit_itself_is_accepted`, `ingress::a_connection_that_writes_nothing_times_out_and_the_listener_serves_next` (`crates/goad-shell/tests/integration/ingress.rs`) |
| R-8 | integration: every case above (R-6) reads exactly one reply line, through the shared `send_with_newline`/`send_half_closed` fixtures, and never a second; `ingress::a_dropped_answer_yields_unavailable_then_a_close` and `ingress::a_connection_accepted_after_the_judge_is_gone_is_answered_unavailable` (`crates/goad-shell/tests/integration/ingress.rs`) — a judge that goes away yields `unavailable` before the close, whether it went before the arrival was sent or after; `ingress::every_reply_is_newline_terminated_before_the_close` (same file) — §6.3's framing, asserted on the raw bytes, which is the one thing a reader that parses cannot see |
| R-9 | unit, one case per clause, all `crates/goad-shell/src/ingress/envelope.rs`: `envelope::tests::a_non_object_top_level_is_refused_naming_the_type_found` (the type found); `each_of_the_four_keys_missing_is_refused_naming_it` (missing); `each_typed_key_wrong_typed_is_refused_naming_it` (wrong-typed); `an_empty_source_or_kind_is_refused_naming_it` (empty); `a_fifth_key_beside_the_four_is_refused_naming_it` (unknown); `a_top_level_duplicate_key_is_refused_naming_it` and `a_duplicate_key_nested_inside_data_is_refused_naming_it` (duplicated, at both depths) |
| R-10 | unit: `envelope::tests::an_offsetless_instant_is_refused_distinctly_from_an_unparseable_one` (`crates/goad-shell/src/ingress/envelope.rs`) |
| R-11 | integration end to end: `ingress::a_well_formed_envelope_produces_one_evaluation_carrying_all_four_fields` (`crates/goad/tests/renderer/ingress.rs`) — the backend's recorded request carries `source` and `kind` as sent, `data` as the same JSON **value** (compared as a value, which is what this requirement claims) and `timestamp` as the same instant, with `now` the host's own |
| R-12 | integration and renderer, all `crates/goad/tests/renderer/ingress.rs` unless noted: `ingress::an_envelope_arriving_during_an_exchange_is_refused_engaged_before_it_completes`; `ingress::a_second_envelope_inside_the_spacing_is_refused_too_soon_and_says_how_long`; `ingress::a_flat_out_writer_raises_no_evaluation_rate_and_costs_one_presentation_per_refusal` — a writer emitting flat out produces a bounded number of evaluations over a window far shorter than the spacing, the excess replies name the bound, and the same test **records** the number of presentations the host makes over that window (measured 845/845, 1.000 per refusal, ~1690/s — F-15's settlement) rather than merely detecting a rate; `tests::a_writer_arriving_exactly_at_the_anchor_is_outside_the_spacing` (`crates/goad/src/controller.rs`) — the spacing's own boundary, refused on `<` and not `<=`. The anchor's independence in **three** directions: `ingress::an_ingested_firing_never_writes_the_scheduled_floor`; `ingress::an_ingested_firing_does_not_advance_the_scheduled_floor` — the case ADR-004 says no standing test could reach, CD-3's discharge — a scheduled firing at T₀, an ingested firing at T₀+ε **whose own exchange resolves to a deadline no later than T₀+1 s**, and a `next_check` due at T₀+1 s do not put a scheduled evaluation at the backend before T₀+3 s; `ingress::a_scheduled_firing_does_not_clear_the_event_floor` |
| R-13 | unit and integration, in two halves: `envelope::tests::a_reserved_source_is_refused_with_every_other_field_valid` (`crates/goad-shell/src/ingress/envelope.rs`) — the `EnvelopeFault`, unit-level; `ingress::the_three_shape_reasons_this_phase_owns_are_read_off_the_wire` (`crates/goad-shell/tests/integration/ingress.rs`) — `reserved_source` as its own wire reason, read off the wire |
| R-14 | integration: `ingress::the_reason_token_set_is_closed_at_eight` — the eight-way mapping, and a token **renamed** or **removed** from `Refusal::reason()`'s match, both held by assertion. A token **added** is held differently and this row says so rather than overclaiming it: the case's own `match` has no `_` arm over `Refusal`'s variants or `UnavailableCause`'s and is the source of the set it compares, so a ninth reason fails to **compile** in that file — the suite is red until someone edits it — but whether the assertion then also fails depends on that author adding a witness beside the arm the compiler made them write. Rust cannot force that without a derive macro or an enumeration crate, neither of which is on this manifest. **A second implementation should read the added direction as compile gate plus review, not as an assertion.** The three measured results are in the case's doc comment; `ingress::a_too_soon_reply_carries_retry_after_ms_rounded_up` and `ingress::retry_after_ms_is_absent_from_every_reason_but_too_soon` — the field, present on `too_soon` alone, and the rounding (both `crates/goad-shell/tests/integration/ingress.rs`); `tests::a_writer_arriving_exactly_at_the_anchor_is_outside_the_spacing` (`crates/goad/src/controller.rs`) — the other side of the same equation: rounding up is only true *of the host* if the spacing check refuses on `<`, so a writer that waits exactly `retry_after_ms` is accepted |
| R-15 | renderer, both `crates/goad/tests/renderer/ingress.rs`: `ingress::a_too_soon_refusal_decided_while_idle_reaches_the_diagnostics_surface` (positive) and `ingress::a_shape_refusal_decided_during_an_exchange_does_not_reach_the_diagnostics_surface` (negative — what makes the bound a claim rather than an excuse), which reads the **live** window while the exchange is still running: the retained `Diagnostics` are replaced by `absorb` on the way out, so a read at the end cannot tell a branch that presents nothing from one that presents the refusal and is wiped afterwards (`docs/slices/004/review-code.md` F-23); `ingress::a_dead_accept_task_is_folded_once_parks_the_arm_and_leaves_the_host_evaluating` and `ingress::ingress_stopping_during_an_exchange_still_reaches_the_diagnostics_surface` (same file) — the last clause from both sides: the ingress-stopped `unavailable` is the one refusal that answers no envelope, and it reaches the surface whether the loop was idle or mid-exchange when ingress died |
| R-16 | integration and renderer: `ingress::a_malformed_envelope_reaches_no_event_and_the_listener_stays_up` (`crates/goad-shell/tests/integration/ingress.rs`); `ingress::after_a_flood_of_malformed_envelopes_the_host_still_evaluates`, `ingress::a_dead_accept_task_is_folded_once_parks_the_arm_and_leaves_the_host_evaluating` (`crates/goad/tests/renderer/ingress.rs`) |

## 8. Open questions

- **OQ-1. Answered, and no longer open.** The reply says when a `too_soon`
  writer may retry, as `retry_after_ms` (R-14, §6.3). The host computes the
  value at the moment of refusal whatever it does with it, and putting it on the
  wire later would be a change to a versioned contract whose first real client
  is one slice away. What stays out is any reservation or fairness guarantee
  attached to it — see P-C.
- **OQ-2.** Whether a watcher should be able to learn the spacing without
  provoking a refusal. Nothing in this contract carries host policy toward a
  writer, and inventing a channel for it here would be the wrong place — the
  same reason SPEC-002/OQ-2 gives for the backend side.
- **OQ-3.** Whether an ingested evaluation should be distinguishable on the
  host's diagnostics surface when it is *accepted*, not only when refused. The
  writer already has its own answer; this is about the person who is not the
  writer.
- **OQ-4.** SPEC-002/OQ-4, carried and not answered here: an ingested evaluation
  can supersede a view a person is mid-answering. A second stimulus raises the
  rate at which that is reached and changes nothing else about it.

## 9. References

- SPEC-001 (the host/backend interaction protocol) — R-7 and R-9, the event this
  contract produces and the opacity it preserves; R-22, the offset rule R-10
  mirrors; R-45, R-46 and R-47, which R-16 restates for a second untrusted
  input; R-56, whose reservation R-13 enforces.
- SPEC-002 (the host's scheduling behaviour) — R-5, which obliged slice 004 to
  bound this stimulus separately; R-9, the one-exchange rule `engaged` reports;
  and **SPEC-002/R-12**, the requirement that states the ingested spacing
  itself. Qualified deliberately: this spec has an R-12 of its own, and the two
  are the two sides of one seam rather than one requirement cited twice.
- ADR-001 (one-way strata) — event ingress is stratum 2's by name. It names
  wire-to-canonical normalization as stratum 1's, and the envelope is both, so
  the placement is a decision ADR-001 §Consequences requires be made
  deliberately: the envelope normalizes beside the listener because that is
  where *this* contract lives, while stratum 1's normalization holds SPEC-001's.
  ADR-005 is the record of that decision.
- ADR-004 (the scheduled anchor) — why an ingested evaluation neither clears the
  scheduled anchor nor is bounded by it.
- `docs/brief.md` §7 (external event ingress and what stays in the watcher),
  §19 (the representative externally-triggered scenario).
- ADR-005 (the event envelope normalizes in stratum 2) — why this contract's
  normalization sits beside the listener rather than in stratum 1 beside
  SPEC-001's.
- `docs/slices/004/design.md` — the design this contract was written from.
- `docs/slices/004/review-code.md` — every `F-N` in this document names a
  finding there; several are the reason a requirement or a Verification row is
  stated the way it is.

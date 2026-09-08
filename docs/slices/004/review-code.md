# Review — implementation — Slice 004

**Subject:** implementation — `9cfb679^..93abab3` on `main`, every code phase of
slice 004 (26 non-documentation files; the ingress module, the loop's two new
arms, the config key, the startup bind, and both test tiers)
**Reviewer:** fresh agent, Claude Opus 5 (1M context)
**Opened:** 2026-09-09
**State:** open

Structured, append-only findings ledger for one adversarial review. Everything
needed to drive it is in this file. Narrative history — what was decided and
why, round by round — stays in the matching `-log.md`; this file holds findings
and their fate.

## Protocol

**Roles.** The **raiser** finds and states; the **responder** disposes. One agent
may hold both roles, but must switch deliberately and say which it is acting as —
disposing a finding while still wearing the raiser's hat is how a review talks
itself into `aligned`.

**Append-only.** Findings are never edited or deleted once raised, and ids
(`F-1`, `F-2`, …) are immutable across rounds. A finding raised in error is
**withdrawn**, not removed. A second round appends `F-4` onward to this same
file; it does not start a new ledger.

**Severity** — set by the raiser at raise time, not negotiated afterwards:

| | |
|---|---|
| `blocker` | Must not proceed. The only severity that gates acceptance. |
| `major` | Real defect, unsound design, or breach of canon. Recorded, does not gate. |
| `minor` | Worth fixing, survivable. |
| `nit` | Style or taste. Costs nothing to note, nothing to ignore. |

**Disposition** — set by the responder, one per finding:

| | |
|---|---|
| `aligned` | The observation is correct but nothing needs to change. Say why. |
| `fix-now` | Fix inside the current unit of work, before it closes. |
| `doc-wrong` | The artefact under review is the defect, not the thing it describes. Amend the design / plan / spec. |
| `follow-up` | Owned future work. Must land in `slice-nnn.md` Follow-ups — a disposition is not a place to put things down. |
| `tolerated` | Knowingly accepted, with a written rationale. |
| `settle-in-code` | Real, unsettled, and cheaper to answer in code than in prose. Names the phase that settles it and the test that will. Design and plan reviews only. |

**Outcome** — set by the raiser, terminal:

| | |
|---|---|
| `verified` | Disposition accepted. Done. |
| `contested` | Disagree; hands back to the responder for re-disposition. Not terminal — the finding returns to open. |
| `withdrawn` | The finding was wrong. Terminal. |

**Done** = every finding `verified` or `withdrawn`, and no `blocker` outstanding.
A ledger with no findings at all is **not** done — it means the review has not
run yet.

**Guardrails.** Do not reach for `follow-up` because the fix is large. Do not
normalise `tolerated` without a real reason. Do not downgrade a `blocker` to get
past the gate. `settle-in-code` is not a way to end an argument you are losing:
it needs a named phase and a named test, it is unavailable to a `blocker`, and a
finding that survives its phase returns to the ledger `contested`. Reject a
finding on **evidence**, never on assertion. Confirm each disposition with the
user before acting on it. Fix the class, not the instance, and do not introduce
new defects repairing old ones.

## Brief

Written before reading the diff. `audit.md` established that the slice did what
it said; this review asks whether what it did is sound. It does not re-run the
gate, re-walk the acceptance criteria, or re-check the surface delta — that
evidence stands.

**The invariants this review holds the code to** (`CLAUDE.md`), in the order it
attacked them:

1. **Permissive wire, canonical internals.** `envelope::normalize` is the only
   door. Is anything unvalidated past it? Is anything *ambiguous* guessed at
   rather than refused?
2. **Wire compatibility is not narrowed to fit a consumer.** The published
   contract is `draft-spec.md`, and the host is one implementation of it. Every
   place the code and §6.2/§6.3/§6.4 could disagree, checked in both
   directions — including where the code is right and the *contract about to be
   promoted to canon* is wrong.
3. **The host does not understand the domain.** No branch on `kind`, no read
   into `data`, no comparison of `timestamp` to now.
4. **A failure never takes the host down, and every refusal says which side was
   wrong.** Partial reads, timeouts, dropped connections, a writer that never
   writes, concurrent connections, a `oneshot` whose receiver is gone, and the
   accept task's own death.
5. **Strata run one way.** Stratum 1 unchanged but for one visibility widening.

**Where the bodies were expected.** The seams: the listener/loop split (two
authors of one reason vocabulary), the reply's bytes (the one thing a second
implementation would be held to), the accept task's error path (no test drives
it), and anything the *diagnostics surface* is the only report of — the places
where the spec makes a promise that only prose keeps.

**Two leads inherited undispositioned, both re-derived from the code rather
than accepted:** the `ingress_stopped()` reason literal (F-4 below, **confirmed
as stated**), and the `accept_loop` retry (F-2, **confirmed, and it is worse
than the audit's framing**).

**Round 1** — 2026-09-09 — the whole code diff: `crates/goad-shell/src/ingress/`
(both files), `crates/goad/src/controller.rs`'s new arms and helpers,
`config.rs`, `startup.rs`, `main.rs`, `diagnostics.rs`, both ingress test
modules, and the demo's config and example backend. Read against
`draft-spec.md` R-1..R-16 and §5/§6 clause by clause, `design.md` §5.1-§5.5,
and SPEC-001/R-7, R-9, R-45..R-47, R-56.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | major | | |
| F-2 | major | | |
| F-3 | major | | |
| F-4 | minor | | |
| F-5 | minor | | |
| F-6 | minor | | |
| F-7 | minor | | |
| F-8 | minor | | |
| F-9 | minor | | |
| F-10 | minor | | |
| F-11 | minor | | |
| F-12 | minor | | |
| F-13 | nit | | |

### F-1 — the reply is not newline-terminated, and §6.3 says it is

**Severity:** major
**Location:** `crates/goad-shell/src/ingress/mod.rs:419-444` (`reply`),
`:496` (`write_all`); `draft-spec.md` §6.3

**Expected:** `draft-spec.md` §6.3 opens the reply's normative description with
*"One JSON object, **newline-terminated**, then the host closes."* That sentence
is the wire form a second implementation would be held to, and the framing a
watcher author writing to the published contract in any language would code
against.

**Observed:** the host writes no newline. `reply()` returns
`serde_json::to_string(&wire)` and `handle` writes exactly those bytes, then
shuts down the write side. The module's own doc comment states the choice
outright: *"No trailing newline: the connection's close is the line's
terminator."* `notes.md:657` and `:703` record it as a deliberate PHASE-03
decision, matched to the A-1 probe's harvested byte count. **The spec was never
brought into line with it.**

The contract and the host therefore disagree about the reply's framing, and the
document is the one about to be promoted to canon. A writer that reads until
`\n` — the obvious reading of "newline-terminated", and what `nc -U | head -1`
does — gets its line only because EOF happens to terminate a read too; a client
that requires the terminator, or a second host implementation that emits it, is
now on the wrong side of whichever document it read.

**Evidence:** `draft-spec.md:218`; `crates/goad-shell/src/ingress/mod.rs:419-421`
(the doc comment), `:443` (`to_string`, no terminator appended), `:496`. **No
test discriminates the two**: every reader in both tiers is
`read_to_string`-to-EOF (`crates/goad-shell/tests/integration/ingress.rs:130-137`,
`crates/goad/tests/renderer/ingress.rs:132-135`) and every assertion goes
through `serde_json::from_str`, which accepts the JSON with or without a
trailing newline. `design.md:213` says only "One line, then the host closes",
which is compatible with either — so the disagreement is spec-versus-code, with
nothing between them holding it.

**Disposition:**
**Response:**

**Outcome:**

### F-2 — `accept_loop` spins without bound or backoff on a persistent `accept()` error

**Severity:** major
**Location:** `crates/goad-shell/src/ingress/mod.rs:452-461`

**Expected:** every other loop in this workspace that could be driven by
something outside the host's control is bounded. `serve`'s scheduled arm re-arms
at the floor after a refusal precisely so a failing firing cannot spin
(`controller.rs:636` and `:673-677`, `refusal_re_arms`); `Ingress::arrival` drops the
receiver as it yields `None` so a closed channel parks instead of spinning
(`ingress/mod.rs:220-229`), and `PHASE-04/VT-7` exists to assert exactly that.
`drain` is bounded in bytes with the reason written down. The accept path is the
one loop with no bound at all.

**Observed:**

```rust
loop {
  let Ok((stream, _addr)) = listener.accept().await else {
    continue;
  };
  ...
}
```

Every `accept()` error is discarded, unlogged and uncounted, and the loop
retries immediately. `accept(2)` errors that persist rather than clearing on the
next call are ordinary: `EMFILE`/`ENFILE` (the process or the system is out of
descriptors — reachable here, since the host spawns a subprocess with three
pipes per exchange *and* holds a descriptor per connection), `ENOBUFS`/`ENOMEM`,
and `EBADF`/`EINVAL` if the listener is ever invalidated. Under any of them the
task busies a tokio worker at 100% until the condition clears by itself, and
`EMFILE` is a condition a spin actively prolongs.

The audit's framing — *"it is on a spawned task, so it costs no presentation"* —
is correct and incomplete. The runtime is `new_multi_thread`
(`crates/goad/src/main.rs:59`), so the Slint thread is not blocked and no
presentation is charged; what is charged is one worker pegged for the life of
the fault, on a desktop application that is expected to sit idle. Nothing
reports it, so the first symptom a person gets is a hot machine.

**Evidence:** `ingress/mod.rs:452-461`; the docstring at `:449-451` states *"An
`accept()` error does not end the task"* and says nothing about how often it
retries. `crates/goad/src/main.rs:59` for the runtime kind. **No test drives an
`accept()` error at all** — grep both ingress test modules: every case exercises
a connection that was accepted. The nearest neighbour,
`a_dead_accept_task_is_folded_once_parks_the_arm_and_leaves_the_host_evaluating`
(`renderer/ingress.rs:664`), asserts the no-spin property on the *receiver* side
and has no analogue on the accept side.

**Disposition:**
**Response:**

**Outcome:**

### F-3 — the ingress-stopped `unavailable` is discarded, not reported, when ingress dies during an exchange

**Severity:** major
**Location:** `crates/goad/src/controller.rs:722` (the inner arm),
`:446-455` (`ingress_stopped`); `draft-spec.md` R-15, §5

**Expected:** `draft-spec.md` R-15 closes with *"The one refusal that reaches no
writer is the one that answers no envelope — the ingress-stopped `unavailable` of
§6.3, **for which this surface is the only report there is**."* §5 states it as
behaviour: *"The host reports that once, as `unavailable`, on the surface R-15
names — the only place it can, since no envelope reaches it afterwards to be
refused."* Neither sentence is conditional on what the host happened to be doing
at the moment ingress died.

**Observed:** the inner (during-exchange) arm folds it onto the controller and
nothing ever presents it:

```rust
arrival = ingress.arrival() => match arrival {
  None => controller.refuse(&ingress_stopped()),
  Some(arrival) => refuse_during_exchange(&mut controller, arrival),
},
```

The inner loop calls no `glass.present`, and its only non-cancelling exit is the
`outcome` arm, which calls `controller.absorb(..)` — and `absorb` assigns
`self.diagnostics = diagnostics` wholesale (`controller.rs:178`). So when the
accept task dies while an exchange is in flight, the fold is **guaranteed** to be
overwritten before any presentation. Not a race: there is no path from that arm
to a frame. The permanent, unrecoverable condition R-15 says the surface is the
only report of is reported nowhere at all, and `Ingress::arrival` has already
parked the arm, so it never recurs.

The design already knew: `design.md:229`'s reply table qualifies this row *"the
ingress-stopped case, **when the loop was idle**"*. The draft spec does not carry
that qualification. One of the two is wrong, and the one being promoted to canon
is the unqualified one.

**Evidence:** `controller.rs:720-724` (the inner arm; no `present`, no `break`),
`:710-717` (the `outcome` arm, `absorb` then `break`), `:178` (`absorb` replaces
`self.diagnostics`); `draft-spec.md:107` (R-15) and `:157-161` (§5);
`design.md:229`. The negative is already proven for the sibling case by a test:
`a_shape_refusal_decided_during_an_exchange_does_not_reach_the_diagnostics_surface`
(`renderer/ingress.rs:1131`) asserts that a refusal folded in this same arm is
gone by the time `serve` returns. **No case drives the `None` arm during an
exchange** — `a_dead_accept_task_is_folded_once_…` (`:664`) kills the accept task
before `serve` has anything in flight, so it only ever exercises the outer arm at
`:626-629`.

**Disposition:**
**Response:**

**Outcome:**

### F-4 — the ingress-stopped refusal's wire token is a string literal, and nothing reads it

**Severity:** minor
**Location:** `crates/goad/src/controller.rs:446-455`

**Expected:** the reason vocabulary is closed at eight tokens and `Refusal` owns
it. `Refusal::reason()` is written specifically so that *"the compiler — not a
reviewer — is what notices the day a ninth reason is needed"*
(`ingress/mod.rs:288-290`), and `refuse_arrival` — the only other site that puts
a reason on the diagnostics surface — reads it off the value
(`controller.rs:421`, `reason: refusal.reason().to_owned()`).

**Observed:** `ingress_stopped()` writes the token by hand:

```rust
Refused::Ingress {
  reason: "unavailable".to_owned(),
  detail: "ingress has stopped; no further events will be accepted".to_owned(),
}
```

The comment above it argues correctly that no `Refusal` value describes this
cause — `UnavailableCause` has two variants and neither is it. That justifies not
constructing a `Refusal`; it does not justify duplicating the *token*, which is
the one part of the value that is shared vocabulary rather than a description of
the cause. The token now has two authors, and the second is a literal.

**Nothing would fail if it were misspelled.** `PHASE-08/VT-8`
(`the_reason_token_set_is_closed_at_eight`, integration) reaches only
`Refusal::reason()`, and `PHASE-07/VT-7`
(`a_dead_accept_task_is_folded_once_…`, `renderer/ingress.rs:705-709`) asserts
`line.contains("ingress has stopped")` — the *detail*, never the token. Compare
`a_too_soon_refusal_decided_while_idle_reaches_the_diagnostics_surface`
(`:1102-1107`), which does assert `line.contains("too_soon")`: the token is
checked on the surface for every reason but this one.

Raised `minor` rather than `major` deliberately: the wire's own reason set is
untouched, and the consequence of a misspelling is a wrong token in one
person-facing line. The defect is the class — an eight-token closed set with a
second, unchecked author — not the current spelling, which is right.

**Evidence:** `controller.rs:446-455`; `controller.rs:418-424` (`refuse_arrival`,
the site that does it correctly); `ingress/mod.rs:318-335` (`reason()`);
`renderer/ingress.rs:705-709` versus `:1102-1107`. PHASE-07 reported and did not
fix it, on purpose, so this review would meet it cold — confirmed independently
here from the code, and the audit's description of it at `audit.md:264-267` is
accurate as far as it goes.

**Disposition:**
**Response:**

**Outcome:**

### F-5 — `the_reason_token_set_is_closed_at_eight` does not close the set against an added reason

**Severity:** minor
**Location:** `crates/goad-shell/tests/integration/ingress.rs:720-761`

**Expected:** `draft-spec.md` R-14's Verification row claims the test holds *"the
**exact token set**, so a reason added or renamed fails here rather than at a
client"*, and `audit.md:111` repeats it: *"so a token added, removed or renamed
fails there."*

**Observed:** the test builds its input from a **hand-written array of eight
`Refusal` values** and compares `reason()` over that array against a
hand-written array of eight strings. A ninth `Refusal` variant that is not added
to the array leaves both sets unchanged and the test green. What the test
actually holds is: (a) a *renamed* token, (b) a token *removed* from `reason()`'s
match, and (c) the eight-way mapping being correct. It does not hold (d), a
token *added* — which is the direction the requirement is about, since the
closure claim is what a client's parser depends on.

The compiler does force a change to `reason()` when a variant is added
(exhaustive match, no `_` arm) — that is real and worth keeping — but it forces
an edit, not a *review of the wire contract*, and the test is what R-14 names as
the instrument for the latter.

**Evidence:** `crates/goad-shell/tests/integration/ingress.rs:722-737` (the
literal array of eight constructions), `:739-748` (the literal expected set);
`draft-spec.md:318` (R-14's verification claim); `audit.md:111` (AC-4's, the same
claim). The `#[test]` is a plain unit test with no listener, so nothing else in
the case reaches `Refusal`'s variant list either.

**Disposition:**
**Response:**

**Outcome:**

### F-6 — R-11's "byte-for-byte" is false of `data`, and the test cannot see it

**Severity:** minor
**Location:** `crates/goad-shell/src/ingress/envelope.rs:140-142`;
`draft-spec.md` R-11; `crates/goad/tests/renderer/ingress.rs:271-275`

**Expected:** R-11 — *"All four fields reach the backend as the event of an
`evaluate` (SPEC-001/R-7): `source`, `kind` and `data` **byte-for-byte as
sent**."*

**Observed:** `data` is parsed into a `serde_json::Value` and re-serialized on
the way out. Two byte-level changes survive that round trip:

- **Object key order is normalized.** `serde_json` is on the manifest without
  the `preserve_order` feature (`Cargo.lock:4605-4616` lists `itoa`, `memchr`,
  `serde`, `serde_core`, `zmij` — no `indexmap`), so `serde_json::Map` is a
  `BTreeMap` and keys reach the backend sorted, not as written. `{"b":1,"a":2}`
  in, `{"a":2,"b":1}` out.
- **Numbers outside `f64`'s exact range lose precision.** Without
  `arbitrary_precision`, an integer literal larger than `u64::MAX` is parsed to
  `f64` and re-emitted in exponential form.

Neither breaks SPEC-001/R-9, which says *"carries both verbatim"* and is about
the host not *interpreting* the payload — that invariant holds, and this module
genuinely reads nothing (`envelope.rs:140-142` is a `remove` and nothing else).
The defect is R-11's stronger sentence, which is newly written by this slice and
is about to be promoted as canon, on the strength of an implementation that
cannot keep it.

**Evidence:** `envelope.rs:140-142`; `canonical.rs:490-496` (`data:
serde_json::Value`); `Cargo.lock:4605-4616`. **The test is insensitive to it by
construction**: `renderer/ingress.rs:271-275` asserts
`request["event"]["data"] == serde_json::json!({"count_last_hour": 4})`, an
equality between two already-parsed `Value`s, which compares maps as maps —
order-blind, and it would pass under any of the transformations above. The
integration tier's `a_well_formed_envelope_reaches_the_judge_as_the_event_it_wrote`
(`integration/ingress.rs:571`) asserts the same way, one key deep.

**Disposition:**
**Response:**

**Outcome:**

### F-7 — a read error on the connection is reported to the writer, and to a person, as `malformed`

**Severity:** minor
**Location:** `crates/goad-shell/src/ingress/mod.rs:539`

**Expected:** `CLAUDE.md` — *"Every refusal is reported and says which side was
wrong."* `malformed` means, per §6.3's own table, *"the bytes were not one JSON
document"*, and its stated *writer's fix* is *"the serializer"*.

**Observed:**

```rust
Ok(Err(_io)) => Err(Refusal::Malformed),
```

An `io::Error` from `read_until` — a connection reset mid-envelope, `EIO`, any
transport fault — is folded into the one reason that names the writer's
serializer as the thing at fault. The writer is told it sent bad bytes when it
sent none the host could read; more to the point, `refuse_arrival` puts the same
verdict on the diagnostics surface, where a person reads *"an event was refused
(malformed): the bytes are not one JSON document"* about a connection that
failed underneath. `unavailable` is the reason set's own token for *the host
cannot act on this envelope*, and it is the honest one; the error is discarded
rather than carried into `detail`.

**Evidence:** `ingress/mod.rs:534-540`; `draft-spec.md:231-232` (the
`malformed` row and its writer's fix); `controller.rs:418-424` (`refuse_arrival`
folds the same refusal onto the surface). Reachability is narrow — a peer that
sent an RST usually cannot read the reply either — but the diagnostics half is
not narrow: the fold happens whether or not the reply lands. No test drives an
I/O error on the read.

**Disposition:**
**Response:**

**Outcome:**

### F-8 — a connection accepted after the judge is gone closes with no reply, while the process is still running

**Severity:** minor
**Location:** `crates/goad-shell/src/ingress/mod.rs:487-489`

**Expected:** R-8 — *"Every envelope MUST receive exactly one reply on the same
connection… The only case in which a connection may close unanswered is one in
which **the host process itself is gone**."*

**Observed:** when `arrivals.send(arrival)` fails — the receiver has been
dropped — `handle` returns `false` immediately. The `Arrival` comes back inside
the `SendError` and is dropped with it, taking the `Answer` and its `oneshot`
sender; nothing is written to `stream`, which is then dropped and closed. The
writer gets EOF with no reply.

The process is not gone at that moment. `main.rs:110-127` binds `_served` — and
with it the `Ingress` receiver — inside the `spawn_local` future, so the
receiver is dropped when that block ends, and only *then* does the Slint loop
unwind, `start` return, the runtime guard drop, and the runtime (with the accept
task) drop. The accept task keeps accepting for the whole of that unwind.

The window is short — milliseconds — and this is the same shape as the defined
`Unavailable(Stopping)` path, which is what the *dropped-`Answer`* case already
does correctly (`:490-495`). The difference is only which of two adjacent
failures happened first, and one of them is answered and the other is not.
`audit.md:110` asserts the exception is admitted because *"the judge is gone,
which is the host process going away"*; that is an assumption about timing, not
a property the code holds.

**Evidence:** `ingress/mod.rs:482-495` (the `send` failure returns before any
write; the `rx.await` failure two lines below writes `Unavailable(Stopping)`);
`crates/goad/src/main.rs:110-131`. No test covers a connection arriving in that
window; `a_dropped_answer_yields_unavailable_then_a_close`
(`integration/ingress.rs:424-435`) covers only the second, answered case.

**Disposition:**
**Response:**

**Outcome:**

### F-9 — any ingress refusal replaces the whole diagnostics surface, so an outside writer can erase the host's own fault reports

**Severity:** minor
**Location:** `crates/goad/src/controller.rs:418-424` (`refuse_arrival`),
`:188-190` (`Controller::refuse`), `crates/goad/src/diagnostics.rs:146-166`

**Expected:** the diagnostics surface is *"everything a person reads"* and is
where a backend failure, a cleanup failure and every discarded field are
reported. Slice 004 adds an input to it that is reached from **outside the
process**, at a rate the host does not bound.

**Observed:** `Diagnostics::refused` builds `lines: vec![one line]` and
`Controller::refuse` assigns it over the whole retained value. Every ingress
refusal the loop decides while idle therefore wipes whatever was on the surface.
That was already true of a superseded click, but a click is the person's own act;
an envelope is not. The measured cost is one full replacement per refusal at
~1690/s (`audit.md:112`, PHASE-07's own measurement), so any local process that
can open the socket can hold the surface at *"an event was refused (too_soon)"*
indefinitely, and a backend failure a person needs to see is gone within a
millisecond of appearing.

A concrete, non-adversarial instance already exists in the design: a second
host's startup probes the path with a bare `connect`, which the live host reads
as an empty envelope and refuses `malformed`
(`ingress/mod.rs:152-158`). So a failed `goad` start on a machine that is already
running one wipes the running host's surface and leaves a spurious *"an event
was refused (malformed)"* on it — a message about the operator's own second
process, phrased as if a watcher sent bad bytes.

The repair is probably design-level (the surface holds one thing; ingress now
has an unbounded, external author of it), which is why this is raised rather
than costed here.

**Evidence:** `diagnostics.rs:146-166` (`lines: vec![…]`, replacing);
`controller.rs:188-190`; `ingress/mod.rs:147-158` (the reclaim probe's stated
side effect); `audit.md:112` for the rate. `review-design.md` F-15 settled the
*presentation cost per refusal*; it did not consider what the surface's single
slot is worth when an outside party writes to it.

**Disposition:**
**Response:**

**Outcome:**

### F-10 — a writer that connects and never writes starves ingress at 500 ms per connection, and the spec does not say so

**Severity:** minor
**Location:** `crates/goad-shell/src/ingress/mod.rs:452-461`;
`draft-spec.md` §6.4

**Expected:** §6.4's *"What is not bounded, and why"* names exactly one
unbounded wait — the wait for judgement — and argues it is inherited from the
main thread rather than added. Both read bounds are stated so a writer knows
what it can hold.

**Observed:** the accept loop is strictly sequential: `accept` → `handle`
(read, hand over, await judgement, reply, close) → `accept`. A connection that
writes nothing holds it for the full `ENVELOPE_DEADLINE` — 500 ms — during which
no other envelope is accepted at all. Two such connections per second, from any
process on the machine that can open the socket, make ingress permanently
unavailable to every legitimate watcher, and nothing on the diagnostics surface
says why: the stalled connections are answered `timed_out` to *their own*
writers, and a watcher whose envelopes are simply never accepted sees only
latency.

The socket is `0600`, so this is confined to the user's own uid — which on a
desktop includes every application the person runs, not only code they wrote.
`draft-spec.md` §6.4 states the *per-read* bound and the *judgement* wait, and
says the host holds one arrival at a time; it never states the consequence that
the read bound is also a bound on how long one connection can deny all others.
That is the property a watcher author needs and cannot derive from the two
numbers given.

**Evidence:** `ingress/mod.rs:452-461` (sequential; `handle` is awaited before
the next `accept`), `:40` (`ENVELOPE_DEADLINE`), `:518-527` (the read is per
connection); `draft-spec.md:283-294` (§6.4's own account of what is not
bounded); `design.md:523` (I-2, which states the one-at-a-time property for the
judgement wait only). `a_connection_that_writes_nothing_times_out_and_the_listener_serves_next`
(`integration/ingress.rs:617`) drives one such connection and asserts the
listener recovers; nothing drives two.

**Disposition:**
**Response:**

**Outcome:**

### F-11 — the contract records no rule for a symlink at the socket path, and the implementation has one

**Severity:** minor
**Location:** `crates/goad-shell/src/ingress/mod.rs:159-196`;
`draft-spec.md` R-3, R-4

**Expected:** R-3 and R-4 divide the world in two: *"a socket no live host
holds"* is reclaimed, *"a socket a live host holds"* is a startup failure, and
*"anything that is not a socket"* is a startup failure naming what was found.
The spec is what a person configuring the host reads.

**Observed:** `reclaim` uses `symlink_metadata`, so it never follows the link,
and `describe` reports `"a symlink"`. A symlink pointing at a socket a live host
holds is therefore refused as `NotASocket { found: "a symlink" }` — with the
message *"not a socket — found a symlink"* — rather than as `InUse`. The
behaviour is deliberate and defensible (`design.md:581`: *"A symlink to a socket
is ambiguous and is refused rather than followed"*), and it is the right call:
following a link at a path the host is about to `chmod` would be worse.

What is missing is that the *spec* says none of it. R-3/R-4 admit only "socket"
and "not a socket", and by the letter of R-3 a symlink to a live socket is a
path occupied by a socket a live host holds. A person who symlinks the
configured path — a normal thing to do with a socket under `/run` — gets a
startup failure whose message asserts something that is not true of the target,
and no document to check it against. The design records the rule; the design is
not canon, and `docs/AGENTS.md` §*Canon that does not exist yet* makes the draft
spec, not `design.md`, the slice's working authority.

**Evidence:** `ingress/mod.rs:160-172` (`symlink_metadata`, then the file-type
gate), `:180-196` (`describe`'s `"a symlink"` arm); `draft-spec.md:95-96` (R-3,
R-4); `design.md:581`. The path is untested in both tiers — the integration
module covers a regular file and a directory, not a link.

**Disposition:**
**Response:**

**Outcome:**

### F-12 — the example backend interpolates envelope values into JSON without escaping

**Severity:** minor
**Location:** `examples/shell/backend.sh:39-55`

**Expected:** `examples/` is the worked example a watcher author copies, and
`examples/shell/backend.sh` is the backend the AC-13 human observation actually
ran. Its existing comment already warns that substring matching on JSON *"is not
something to imitate"*; the new code goes further than matching.

**Observed:** the new arm extracts `source` and `kind` with shell parameter
expansion and interpolates them straight into a JSON string literal:

```sh
source=${request#*'"source":"'}
source=${source%%'"'*}
…
"title": "An event arrived: '"$source"' / '"$kind"'",
```

`source` and `kind` are watcher-chosen and admit any non-empty string. A `source`
of `he said "hi"` closes the JSON string early and the backend emits a document
the host refuses; a `source` containing a backslash produces an invalid escape.
The host handles it correctly — this is a backend failure, reported, and the
host stays up — but the example demonstrates a pattern that breaks on the first
quote, in a file whose whole job is to show a watcher author what a backend
looks like.

Second, smaller point in the same file: `*'"source":"host"'*` matches the
substring anywhere in the request, so an ingested envelope whose `data` contains
the literal `"source":"host"` takes the host branch. The comment above it argues
correctly that field order makes the *extraction* read `event.source`; the
`case` pattern above it has no such protection.

**Evidence:** `examples/shell/backend.sh:39-55` (the new arm), and `:25` for the branch
pattern above it; `draft-spec.md:206` (`source` admits *"a non-empty string, not
`\"host\"`"* — nothing about quoting). `envelope.rs`'s own tests confirm the host
carries such a `source` through unexamined, which is correct and is what makes
this reachable.

**Disposition:**
**Response:**

**Outcome:**

### F-13 — `ingest` writes the event anchor with a panicking `Instant` addition

**Severity:** nit
**Location:** `crates/goad/src/controller.rs:492`

**Expected:** `deadline_after` (`controller.rs:361-379`) exists because
`Instant + Duration` panics on overflow and *"a panic here would be a backend
failure taking the host down (SPEC-001/R-45)"*; it uses `checked_add` with a
clamp, and has a test named for exactly that.

**Observed:** the event anchor's one write site is
`*event_floor_until = arrived + MINIMUM_SPACING;` — the plain, panicking `+`.

Unreachable in practice: `arrived` is `Instant::now()` and `MINIMUM_SPACING` is
a 3-second constant, so unlike `deadline_after`'s input this addition is not
driven by anything a backend or a writer chose. Raised only because the module
states the hazard in a doc comment thirty lines above and then does not follow
its own rule at the one new site, and because `ingress/mod.rs` carries
`#![deny(clippy::arithmetic_side_effects)]` while `controller.rs` does not — so
the two halves of one slice are held to different standards by the lint, not by
a decision anyone recorded.

**Evidence:** `controller.rs:492`; `controller.rs:361-379` and `:762-777` of the
inline test module for the rule it departs from; `ingress/mod.rs:14` versus
`controller.rs:1-6` for the lint asymmetry.

**Disposition:**
**Response:**

**Outcome:**

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

**Round 1 is complete and the ledger is open.** Thirteen findings: three
`major`, nine `minor`, one `nit`. **No `blocker`** — nothing found makes the
slice unsafe to run or breaks the invariant that a failure never takes the host
down, and the ingress path's core is sound: the envelope's normalization is a
genuine single door, nothing past it is unvalidated, no host code reads `kind`
or `data` or judges `timestamp`, the two anchors are independent in all three
directions with falsifying tests behind them, and the eight-token reason
vocabulary is one exhaustive match with the compiler behind it.

What the review did not find is worth stating, because these were the lines of
attack most likely to produce a `blocker`. **Nothing narrowed the wire contract
to fit the renderer**: the envelope admits any JSON `data`, and no field is
constrained by what the loop happens to consume. **No domain vocabulary entered
host types or module names**, and the paraphrase check comes back clean too —
`reddit-watcher` appears only as a fixture value drawn from brief §19, never as
a type, a module, or a branch. **Strata still run one way**: stratum 1's only
change is one `pub(crate)` → `pub`, and `cargo tree -p goad-semantics -i tokio` errors
*"did not match any packages"* — run here, not taken from the audit. **The malformed-input paths are safe**: `serde_json`'s own
recursion limit bounds the duplicate-key walk, attacker-chosen key names reach
the diagnostics surface only through `Escaped` and `bound`, and the reply is
built with `serde_json` rather than interpolated so a watcher-chosen key cannot
break the reply's own syntax.

The three `major`s are all one shape — **the contract about to become canon says
something the code does not do**, in a place no test discriminates: the reply's
framing (F-1), the ingress-stopped report's reach (F-3), and, at `minor`, the
opacity claim for `data` (F-6). In two of the three the *code* is defensible and
the *spec sentence* is the likely defect; F-2 is the one that is a plain code
defect with no document on either side of it.

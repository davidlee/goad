# Review — design + plan — Slice 005

**Subject:** design + plan — `docs/slices/005/design.md` and
`docs/slices/005/plan.md` at `03b0286`, together with the `slice-005.md` they
must be consistent with. One ledger for both, per `docs/AGENTS.md` §Tiers
(tier 1), at most two rounds.
**Reviewer:** fresh agent
**Opened:** 2026-09-11
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

Written before the review, and deliberately naming the places the author is
least confident, so that the ledger is not shaped by what turned out to be easy
to find.

**What this review holds the subject to**

1. **CLAUDE.md invariant 3** — the protocol is the contract; a renderer, or here
   a *client*, is one consumer of it. Anything in this design that narrows what
   a conforming host may send, or that bakes the current host's behaviour into
   the CLI's expectations, is the failure this project exists to avoid.
2. **CLAUDE.md invariant 2** — permissive wire, canonical inside; an *ambiguous*
   message fails rather than being guessed at.
3. **ADR-001** — one-way strata, and whether the new member and the moved code
   land where this design says they do.
4. **SPEC-003** — the client half must be conformant, not merely compatible with
   the host that exists.
5. **`docs/AGENTS.md` §Tiers** — tier 1 is a claim that this slice writes no
   canon. A design that needs a normative sentence is a mis-tiered slice.

**Where the bodies are likely buried** — the author's own list, not exhaustive,
and the reviewer is not bound by it:

- **OQ-3's stratum-3 argument** (`slice-005.md`, and D-8). It concludes *no
  allowlist row* by reading POL-001 §Verification and `allowlist.rs`'s module
  doc. If that reading is wrong, the slice is mis-tiered and the finding is a
  `blocker`.
- **D-1 — the client living in `goad-shell`.** It rests on ADR-005's reasoning
  by analogy. Does stratum 2 owning a *client* of its own listener create a
  cycle, a confusion of roles, or a surface that ADR-001 did not contemplate?
- **D-2 — one `wire::Reply` with both directions.** Making the host's private
  reply type public and deserializable widens stratum 2's API. Does the absence
  of `deny_unknown_fields` on the *read* side do what §5.5 claims, and does the
  *write* side change at all in the process?
- **`design.md` §5.5's `Event` assumption.** The claim is that `Event`'s
  `Serialize` output is exactly SPEC-003 §6.2's four keys, and that
  `Timestamp`'s serialization satisfies R-10's *explicit offset*. Check it
  against the type, not against this sentence.
- **`design.md` §5.4 — emit has no read timeout of its own.** The design says
  the host bounds its side and a caller can use `timeout(1)`. A host that
  accepts a connection and then never replies leaves a cron job hung forever.
  Is that acceptable, and does SPEC-003 actually promise what §5.4 assumes?
- **AC-6 and AC-8's division of labour.** AC-6 was rephrased away from "reaches
  the backend" because no test target links both the CLI and a running host.
  Is the remaining AC-6 still worth asserting, or is it now a tautology — and is
  the backend leg genuinely unreachable by any instrument?
- **Coverage.** `plan.md`'s table maps every AC to a phase. Look for an AC whose
  named test cannot actually see what the AC claims, which is the failure
  `docs/memory/a-green-test-can-assert-a-proxy.md` records from slice 004.
- **Phase sizing and STOP conditions.** Four phases, one agent-session each.
  S-2 says an edited 004 test means a lift was a rewrite — is that reachable,
  or will PHASE-01 breach it by construction?

**Round 1** — 2026-09-11 — design + plan, entire, against the five invariants
above.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | blocker | fix-now | verified |
| F-2 | blocker | fix-now | verified |
| F-3 | major | fix-now | verified |
| F-4 | major | fix-now | verified |
| F-5 | major | fix-now (re-disposed r2) | contested |
| F-6 | major | fix-now | verified |
| F-7 | minor | fix-now | verified |
| F-8 | minor | fix-now | verified |
| F-9 | minor | fix-now | verified |
| F-10 | minor | fix-now | verified |
| F-11 | nit | fix-now | verified |
| F-12 | nit | fix-now | verified |
| F-13 | major | fix-now | |
| F-14 | blocker | fix-now | |
| F-15 | minor | fix-now | |
| F-16 | minor | fix-now | |
| F-17 | nit | fix-now | |

### F-1 — `jiff::Timestamp::now()` does not exist in this workspace, and the reason it does not is a decision already on the page

**Severity:** blocker
**Location:** `slice-005.md` OQ-6; `design.md` §5.2 (`main.rs` — "the only impure file: env, clock, …"), §5.3; `plan.md` PHASE-03/EX-1, EX-4

**Expected:** OQ-6 answers *no `--timestamp` flag* on the ground that
"`jiff::Timestamp::now()` through `goad_semantics::protocol::canonical::Timestamp`
gives R-10's explicit offset for free", and PHASE-03/EX-1 fixes emit's manifest at
`goad-shell`, `goad-semantics`, `serde_json`, `jiff` "and nothing else". For that
to hold, `Timestamp::now()` must be callable from a member carrying the
workspace's `jiff` entry.

**Observed:** it is not. `jiff` is declared workspace-wide as
`jiff = { version = "0.2", default-features = false }` (`Cargo.toml:32`), no member
adds a feature to it, and `Timestamp::now` is `#[cfg(feature = "std")]`. A
`goad-emit` built to EX-1's manifest fails to compile on that call. The three
repairs are each closed or unplanned:

- **Enable `jiff/std` in `crates/goad-emit`.** This is the case
  `crates/goad/src/clock.rs:43-47` already refuses, in terms: *"**Not**
  `jiff::Timestamp::now()`, which needs jiff's `std` feature — enabling it in
  stratum 3 unifies it into stratum 1's build, weakening the purity claim in a way
  the manifest test cannot see (`Cargo.toml:23`, D25)."* It is also POL-001
  §Verification's named **residue**: *"a feature switched on in a shared dependency
  by stratum 2 or 3 unifies into stratum 1's build under `--workspace`, and no
  command in this gate rejects it. Adding a feature to a dependency shared with
  stratum 1 is therefore a design decision, and is argued in the slice that takes
  it."* This design argues it nowhere, and it would reverse a prior slice's
  decision.
- **Use `goad::clock::wall_clock`.** Unreachable: it lives in `crates/goad`, which
  links Slint and which ADR-001 §Decision puts in the same stratum as `goad-emit`;
  naming it would breach AC-7 and S-3 as well.
- **Copy `wall_clock` into `goad-emit`.** Parallel implementation of a judgement
  (`SystemTime` → nanos → `Timestamp::from_nanosecond`, with `BeforeEpoch` and
  `OutOfRange` arms) that OQ-4 already refused to duplicate for a four-line
  `line_to`.

**Evidence:** `Cargo.toml:32`; `jiff-0.2.35/src/timestamp.rs:388` —
`#[cfg(feature = "std")]` immediately above `pub fn now()`;
`crates/goad/src/clock.rs:43-47`; `docs/policy/001-the-phase-gate.md` §Verification,
"The residue"; `docs/adr/001-one-way-strata.md` §Decision.

What makes it go away: a fourth lift in PHASE-01 — `wall_clock` and `ClockError`
move to stratum 2 beside `config::default_path` and `report::line_to`, on the same
argument, with `crates/goad` calling it — recorded as an OQ-6 correction and added
to PHASE-01's surfaces, exit criteria and coverage. If that lands, emit may no
longer need `jiff` on its own manifest at all, and EX-1 should say so.

**Disposition:** fix-now
**Response:** Confirmed at source — `crates/goad/src/clock.rs:43-47` refuses
`jiff::Timestamp::now()` in exactly these terms, and `Cargo.toml:32` carries
`default-features = false`. The repair is the one the finding names: **`clock`
becomes a fourth lift**, `wall_clock` and `ClockError` moving whole to
`goad_shell::clock` beside `config::default_path` and `report::line_to`, on the
same argument. `crates/goad/src/clock.rs` is deleted and its three call sites
(`main.rs:53`, `main.rs:116`, `controller.rs:769`) name the stratum-2 path.
Consequences applied: OQ-6's reasoning is corrected in `slice-005.md` rather than
quietly repaired; `design.md` §2, §5.1, §5.2, D-9 and §9 carry the lift;
PHASE-01 gains EX-3, VT-1's neighbours and the `crates/goad` surfaces; PHASE-03/EX-1
drops `jiff` from emit's manifest entirely, as the finding predicted; and
**S-5** is added — a phase that finds itself enabling a feature on a dependency
shared with stratum 1 stops, because that is POL-001's residue and reverses a
prior slice's decision.

**Outcome:** verified

### F-2 — PHASE-04's own tests cannot be written under PHASE-03's manifest, and AC-6 has no other discharge

**Severity:** blocker
**Location:** `plan.md` PHASE-04/EX-1, VT-1..VT-4; PHASE-03/EX-1; S-3; Coverage (AC-6)

**Expected:** PHASE-04/EX-1 — "the integration target spawns the binary via
`env!("CARGO_BIN_EXE_goad-emit")` **against a listener bound in-process**" — and
VT-4, the sole discharge of AC-6, reads "the `Event` the listener normalizes".
S-3 STOPs if "`goad-emit` needs any dependency beyond `goad-shell`,
`goad-semantics`, `serde_json`, `jiff`"; PHASE-03/EX-1 repeats "and nothing else".

**Observed:** `ingress::bind` is unusable without a Tokio runtime — it calls
`tokio::net::UnixListener::bind` and `tokio::spawn(accept_loop(…))`
(`crates/goad-shell/src/ingress/mod.rs:155-166`), and its own doc comment says
"called under the runtime guard `main.rs` already holds"; `Ingress::arrival()` is
`async`. A test target under `crates/goad-emit/tests/` that binds a real listener
therefore needs `tokio` as a **dev-dependency**, which S-3 forbids by name and
which this workspace elsewhere treats as a real runtime entry
(`crates/goad-boundary/tests/checks/allowlist.rs:49-53`,
`tokio_in_dev_dependencies_is_refused` — "a runtime in dev-dependencies is still a
runtime in the test build"). PHASE-04 fires its own STOP condition by
construction.

An escape exists, but it is a different test from the one written down:
`goad_shell::ingress::envelope::normalize(bytes) -> Result<Event, EnvelopeFault>`
is public and pure (`crates/goad-shell/src/ingress/envelope.rs:95`), so a fake
listener on blocking `std::os::unix::net::UnixListener` can read the line the
binary sent, normalize it, and assert the `Event`, with a canned reply line
serving VT-1..VT-3. That is sound — but it is not "a listener bound in-process"
and not "the `Event` **the listener** normalizes": nothing in `ingress/mod.rs`
runs, so R-6's framing and R-7's read bounds are not on the path the case
exercises.

**Evidence:** `crates/goad-shell/src/ingress/mod.rs:147-166`;
`crates/goad-shell/src/ingress/envelope.rs:95`;
`crates/goad-boundary/tests/checks/allowlist.rs:49-53`; `plan.md` S-3, PHASE-03/EX-1.

What makes it go away: PHASE-04/EX-1 and VT-1..VT-4 are rewritten to name the fake
listener's actual shape (a blocking `std` `UnixListener` in the test target,
`envelope::normalize` as the assertion's subject) and AC-6's sentence adjusted to
what that case can see — or S-3 and EX-1 are amended to admit a `tokio`
dev-dependency for `goad-emit`, argued. Either is a decision; neither is the plan
as written.

**Disposition:** fix-now
**Response:** Confirmed — `bind` calls `tokio::spawn` (`mod.rs:155-166`) and
`allowlist.rs:49-53` is explicit that a runtime in `dev-dependencies` is still a
runtime. The escape the finding identifies is taken, and its cost is stated
rather than hidden: PHASE-04/EX-1 now names a **blocking `std`
`UnixListener` fake on its own thread**, and says in terms that R-6's framing and
R-7's read bounds are *not* on that path — they are held one tier up, by
PHASE-02's cases against the real listener. VT-4 keeps its teeth because
`envelope::normalize` is public and pure, so the assertion's subject is a real
normalized `Event` rather than emit's raw bytes. S-3 is unchanged and now says
"including a **dev-dependency**" out loud. AC-6's sentence in `slice-005.md`
already said "the bytes a real invocation of the built binary puts on a real
socket normalize to an `Event`", which is exactly what the case does; it needed
no weakening.

**Outcome:** verified

### F-3 — the tier-1 argument never engages the one canon sentence about adding a member

**Severity:** major
**Location:** `slice-005.md` OQ-3 and Follow-ups; `design.md` §7 D-8, §10

**Expected:** OQ-3 is the slice's own declared tier hinge ("**The one thing that
could have raised it is settled**"), and `slice-005.md` §Governing canon lists
ADR-003 as binding precisely because it "governs what a member is and **how one is
added**".

**Observed:** OQ-3 rests on two citations — POL-001 §Verification's "in a stratum 1
or 2 manifest" and `allowlist.rs`'s module doc — and both are quoted accurately
(verified at `docs/policy/001-the-phase-gate.md:126` and
`crates/goad-boundary/tests/checks/allowlist.rs:8-11`). But ADR-003
§Consequences/Negative says the opposite in terms, and is cited nowhere in the
slice, the design or the plan: *"`goad-boundary` is a fifth thing to keep in view:
**a new workspace member needs its own entry in the manifest allowlist and its own
reach in the vocabulary scan's walk**, and nothing but review catches a member
added without either."* An accepted ADR is governing canon; a tier argument that
turns on "no canon requires a row" must answer the canon that says one is
required.

Half of that sentence is already stale, and the slice is the right place to say
so: the vocabulary scan reads `workspace.members` for itself
(`crates/goad-boundary/tests/checks/vocabulary.rs:45-58` — "so a new member arrives
already covered"), so `goad-emit` needs no reach added there. The allowlist half
was arguably never true of `goad` either, which is the exemption `allowlist.rs`
records — an argument that ADR-003's sentence is over-broad, not an argument that
it may be passed over.

**Evidence:** `docs/adr/003-the-host-splits-into-a-workspace-of-strata.md`
§Consequences → Negative, second bullet;
`crates/goad-boundary/tests/checks/vocabulary.rs:45-58`;
`crates/goad-boundary/tests/checks/allowlist.rs:8-11`;
`docs/policy/001-the-phase-gate.md:126`.

What makes it go away: OQ-3 and D-8 cite the sentence and dispose of it
explicitly. The consequence for the tier is the responder's to take to the user:
reading it as binding means a row plus a POL-001 amendment (tier 2); reading it as
over-broad and correcting it means a `canon-delta.md` against ADR-003 (tier 2 by
`docs/AGENTS.md` §Tiers); reading it as a **review obligation this slice has now
discharged** keeps tier 1, and is the only route that does.

**Disposition:** fix-now
**Response:** The finding is right that the argument stopped one document short,
and the missed sentence is governing canon. Put to the user 2026-09-11 with three
readings and their tier consequences; **the user ruled for the third** — the
sentence states a review obligation over-broadly, and this slice discharges it by
deciding in the open. The evidence that it is over-broad rather than binding is
that it is **already untrue of `crates/goad`**, which has carried no allowlist row
since 002, and that its other half is stale (the vocabulary scan reads
`workspace.members` itself, `vocabulary.rs:45-58`). Applied: OQ-3 in
`slice-005.md` gains the sentence, the quotation, the staleness of its first half
and the ruling; `design.md` §3 lists ADR-003 among the forces and D-8 disposes of
it in the decision row itself, where a later reader looking to reverse the choice
will actually be standing. Tier stays 1.

**Outcome:** verified

### F-4 — §5.4 rests "no timeout" on R-7, and SPEC-003 §6.4 says R-7 is not the bound that would carry it

**Severity:** major
**Location:** `design.md` §5.4 (closing paragraph) and §5.2 (`SendFault` has no timeout variant); `slice-005.md` AC-3

**Expected:** "Connect, write, read, exit. No retry, no timeout of emit's own —
the host bounds its side (R-7), and a caller that wants a deadline has
`timeout(1)`."

**Observed:** R-7 bounds *reads from a connection*, and SPEC-003 §6.4 states in
terms that this is not the bound the design needs: *"These bound the **read**, not
the connection. A connection's life is `accept → read (bounded) → hand the arrival
to the host → await its judgement → reply → close`, and **the wait for judgement
has no bound** … a host that is not making progress therefore delays an arrival
exactly as it delays a due check … the writer waits with it rather than being told
something untrue."* The host is also single-connection — "an unjudged arrival
stops all ingress for as long as it lasts". So a backend hung inside its own
timeout, or an ingress already occupied, leaves `emit` blocked in `read` with no
bound: the cron-job hang, reached through the mechanism the contract documents
rather than through host misbehaviour.

The decision may still be right — `timeout(1)` is a real answer and P-C puts
policy in the writer. The *justification* is not, and no consequence is recorded
anywhere: `SendFault` has no timeout variant, AC-3's exit-2 list does not mention
a deadline, and §8's risk table does not carry it.

**Evidence:** `docs/specs/003-host-event-ingress.md` §6.4 ("What is not bounded,
and why"; "One connection at a time…"); R-7's own text — "MUST bound every **read**
from a connection"; `design.md` §5.2, §5.4.

What makes it go away: §5.4 cites §6.4 instead of R-7 and states the consequence
§6.4 names — a host not making progress holds the connection indefinitely, by
contract, and `emit` waits with it — with the choice not to add a deadline made
against that sentence. If the answer stays "no deadline", §8's risk table is where
it is carried, and AC-3 should say a hung host is deliberately not an exit-2 case.

**Disposition:** fix-now
**Response:** Confirmed — §6.4 says the opposite of what §5.4 assumed, in terms,
and R-7 was the wrong citation. The decision survives its justification: put to
the user 2026-09-11 against `--timeout SECS` and a default deadline, and **the
user ruled for no deadline**, on §6.4's own ground — a deadline makes emit report
*no answer* about an envelope the host may be mid-judging, which is the untruth
§6.4 exists to prevent. Applied: §5.4's closing paragraph now cites §6.4, quotes
it, and states the consequence (one connection at a time, so an unjudged arrival
stops all ingress, and emit blocks indefinitely **by design**); D-10 records the
decision and its rejected alternatives; §8 gains **R-4**, the wedged-host risk,
with `timeout(1)` as mitigation and "a cron job that never returns" as its signal;
AC-3 states it; PHASE-03/EX-6 requires `--help` to say it; and `--timeout` is a
named Follow-up for 007, on evidence rather than speculation.

**Outcome:** verified

### F-5 — the `Ambiguous` rule cannot fire for the case its own test names

**Severity:** major
**Location:** `design.md` §5.5 (first bullet) and §5.2 (`wire::Reply`); `plan.md` PHASE-01/EX-3, PHASE-02/EX-3, VT-5

**Expected:** §5.5 — "A reply that is valid JSON but carries no `accepted`, or
carries `accepted: false` with no `reason`, is `SendFault::Ambiguous` and exit 2";
PHASE-02/VT-5 — "`Ambiguous` for `{}` and for `{"protocol":1,"accepted":false}`";
EX-3 makes normalization "a **named, pure** function over the parsed
`wire::Reply`".

**Observed:** §5.2 gives `wire::Reply` a non-optional `protocol: u8` and
`accepted: bool`, and PHASE-01/EX-3 adds `#[serde(default)]` to "the **three**
optional fields" only. A missing non-`Option`, non-`default` field is a serde
deserialization error, so `{}` never reaches the normalization function at all —
it returns `SendFault::Unreadable(serde_json::Error)`. VT-5's first case asserts an
outcome the type makes unreachable. (Its second case,
`{"protocol":1,"accepted":false}`, does parse — the three `Option` fields are
implicitly optional — and is `Ambiguous` as written.)

Underneath the test are two things the design should settle rather than leave to
an implementer:

1. `accepted: false` with no `reason` is not *ambiguous about whether the host
   accepted the envelope* — it says refused, unambiguously. Routing it to exit 2
   contradicts §4 principle 3 ("2 means emit could not get an answer at all") and
   reports a refusal to a wrapper as a transport fault. Whether a non-conforming
   reply should exit 2 is a real decision; `Ambiguous` is the wrong name for the
   reason it is taken.
2. Requiring `protocol` on the read side is the one place this design narrows what
   it accepts (CLAUDE.md invariant 2). It is defensible — §6.3 says `protocol` is
   on every reply — but it is decided silently by a struct field, and it turns an
   otherwise readable `{"accepted":true}` into exit 2.

**Evidence:** `design.md` §5.2's struct and §5.5's first bullet; `plan.md`
PHASE-01/EX-3 and PHASE-02/VT-5; serde's derive emits `missing_field` for a
non-`Option` field carrying no `default`.

What makes it go away: either `accepted` (and `protocol`, if the choice is to be
permissive) becomes `Option` on the read side with normalization mapping absence to
`Ambiguous` — noting the collision with D-2's one-struct decision, since the write
side must keep emitting `accepted` unconditionally per §6.3 — or §5.5 and VT-5 are
restated so `{}` is `Unreadable`, and the missing-`reason` case gets a variant whose
name says what it is.

**Disposition:** fix-now
**Response:** Confirmed on both counts, and both underlying questions are
decided rather than left to an implementer. (1) `{}` is a serde `missing_field`
error and never reaches normalization: VT-5 is restated so `{}` is **`Unreadable`**,
and the case that *does* exercise the rule is `{"protocol":1,"accepted":false}`.
(2) The variant is renamed **`NonConforming(&'static str)`** — it names the
*host's* breach of §6.3 rather than emit's confusion, which is what "ambiguous"
wrongly implied. It stays exit 2, and §4 principle 3 is widened to say why: exit 1
promises a reason token a wrapper can branch on, and a reply without one cannot
carry that promise. (3) The narrowing the finding spotted underneath is removed:
`protocol` **and** `accepted` become `Option` on the read side (D-11,
PHASE-01/EX-4), so `{"accepted":true}` is accepted rather than refused for a field
emit does not use — invariant 2 — while the host keeps writing both
unconditionally, so D-2's one struct survives and the host's bytes do not change
(PHASE-01/VT-3 pins that).

**Outcome:** contested — the observation this finding made still stands against the
repaired artefact, for a new reason. F-5 said the design asserts an outcome its own
type makes unreachable; D-11 changed the type and left the assertion, so `{}` is now
claimed `Unreadable` by a type that parses it. Returns to open with **F-13**, which
carries the measurement and the full sweep D-11 needs.

### F-6 — VH-1 discharges AC-8, and as written it cannot be run

**Severity:** major
**Location:** `plan.md` PHASE-04/VH-1; `slice-005.md` AC-8, AC-4

**Expected:** VH-1 — "**a person**: `just demo`, then `goad-emit --source hand
--kind poke` from another terminal, and the view changes. AC-8." This is the one
criterion `docs/AGENTS.md` §Tiers makes mandatory at both tiers, and the Coverage
table gives it as AC-8's sole discharge.

**Observed:** `just demo` is `demo: (run "examples/demo.toml")` (`justfile:67`) —
the host starts on an **explicit configuration path**, and that file's socket is
`path = "./goad-demo.sock"`, relative to the repository root
(`examples/demo.toml:25-26`). `goad-emit` without `--socket` consults
`$XDG_CONFIG_HOME/goad/config.toml`, else `$HOME/.config/goad/config.toml` (AC-4) —
a file the demo neither creates nor points at. VH-1 as written reaches an absent
configuration or an unrelated one, never the demo's socket.

This is more than a wording fix, because it shows AC-4's rule is narrower than "the
same rule the host uses": the host also accepts a configuration path as its single
positional argument (`crates/goad/src/startup.rs:96-99`, the *exactly one* row),
and emit has no equivalent. For any host not started on the default path — which is
every host anyone has run so far — `--socket` is the only route, and `--socket`
"consults no configuration at all".

**Evidence:** `justfile:67`; `examples/demo.toml:13-26`;
`crates/goad/src/startup.rs:96-99, 110-129`; `slice-005.md` AC-4, AC-8.

What makes it go away: VH-1 names an invocation that works
(`goad-emit --socket ./goad-demo.sock --source hand --kind poke`), and the design
says once — §5.3 or §5.5 — that emit's discovery covers the host's *default* path
only, and that a host started on an explicit configuration is reached with
`--socket`. A `--config PATH` flag instead is a scope change and is the user's
call, not PHASE-03's.

**Disposition:** fix-now
**Response:** Confirmed by running the citations: `justfile:67` starts the demo on
`examples/demo.toml`, whose socket is `./goad-demo.sock` (`demo.toml:25-26`), and
emit's discovery never looks there. VH-1 now reads
`cargo run -p goad-emit -- --socket ./goad-demo.sock --source hand --kind poke`
and says why the flag is not optional. The deeper half is taken as the finding
frames it — a scope question, so the user's: `--config PATH` is a **Follow-up**,
not part of this slice, and `design.md` §5.5 now states once that emit's discovery
covers the host's *default* path only and that a host started on an explicit
configuration is reached with `--socket`. AC-4 says the same and lists the four
exit-2 endings of that road (see F-9).

**Outcome:** verified

### F-7 — `Verdict` already means something else in the file PHASE-02 extends

**Severity:** minor
**Location:** `design.md` §5.2 (`pub enum Verdict`); `plan.md` PHASE-02 surfaces and Notes

**Expected:** PHASE-02 adds its cases to
`crates/goad-shell/tests/integration/ingress.rs` and is told to extend 004's
`judge` fixture "rather than minting a second fixture".

**Observed:** that file already defines
`enum Verdict { Accept, Drop, Refuse(Refusal) }`
(`crates/goad-shell/tests/integration/ingress.rs:53-64`) — *what the fake judge does
with one arrival*. The design's `client::Verdict` is *what the client concluded
about a reply*. Both would live in one file, meaning opposite ends of the exchange,
with new cases reading `Verdict::Refuse(…)` and `Verdict::Refused { … }` a few lines
apart.

**Evidence:** `crates/goad-shell/tests/integration/ingress.rs:53-64`; `design.md` §5.2.

What makes it go away: rename the client's type — `Answered`, `Outcome`,
`ReplyVerdict` — or rename the fixture's. Decide it in the design, not in the first
`use … as …` an implementer reaches for.

**Disposition:** fix-now
**Response:** Confirmed — `tests/integration/ingress.rs:53-64` already owns
`Verdict` for the judge's side of the same exchange, and PHASE-02 adds its cases
to that very file. The client's type is renamed **`Answered`** in `design.md` §5.2,
§5.1's diagram, §9 and every PHASE-02 criterion, with the reason recorded beside
the definition so it is not renamed back. The fixture keeps its name: it was there
first, and its `Verdict` is the better fit for what a judge does.

**Outcome:** verified

### F-8 — EX-3's derive list does not survive the workspace lint table, and EX-4's "otherwise unchanged" is not available

**Severity:** minor
**Location:** `plan.md` PHASE-01/EX-3, EX-4, VT-2

**Expected:** EX-3 — "`goad_shell::ingress::wire::Reply` is public, derives
`Serialize` and `Deserialize`, drops the `'a` lifetime (`reason: Option<String>`),
keeps every `skip_serializing_if`…"; EX-4 — "`ingress::mod::reply` builds a
`wire::Reply` and is **otherwise unchanged**, terminator included"; VT-2 — the round
trip "parses back to an **equal** value".

**Observed:** three gaps, all mechanical, all caught by the gate rather than by
review — which is why they belong in the exit criterion rather than in the
implementer's first red build:

- `missing_debug_implementations = "deny"` (`Cargo.toml:80`) fires on a public type
  with no `Debug`. `Wire` escapes it today only by being private.
- VT-2's equality needs `PartialEq`, which is on neither list.
- Dropping the `'a` **does** change `reply`'s body:
  `reason: refusal.map(Refusal::reason)` yields `Option<&'static str>` and must
  become an owning form (`crates/goad-shell/src/ingress/mod.rs:600`). "Otherwise
  unchanged" is then false of the one line D-2's decision actually touches, which is
  the line a reviewer should be pointed at rather than shielded from.

**Evidence:** `Cargo.toml:80`; `crates/goad-shell/src/ingress/mod.rs:573-601`;
`plan.md` PHASE-01/EX-3, EX-4, VT-2.

What makes it go away: EX-3 names `Debug` and `PartialEq`; EX-4 says which one line
of `reply` changes and that nothing else does.

**Disposition:** fix-now
**Response:** All three confirmed against `Cargo.toml:80` and
`ingress/mod.rs:573-601`. EX-4 now names `Debug` and `PartialEq` alongside the two
serde derives. EX-5 replaces "otherwise unchanged" with the truth: **one line
changes** — `reason: refusal.map(Refusal::reason)` becomes an owning form now the
field is `String` — and nothing else in `reply` moves, terminator included. That
line is exactly where D-2's decision touches the host, so pointing a reviewer at
it is better than shielding them from it.

**Outcome:** verified

### F-9 — a configuration that is absent or will not parse is emit's most likely first failure, and no AC or test names it

**Severity:** minor
**Location:** `slice-005.md` AC-3, AC-4; `plan.md` PHASE-03/EX-5, VT-3

**Expected:** AC-3 enumerates what exits 2 — "no socket at the path, a connection
that fails or faults, a reply that is not readable as one JSON object, and a usage
error" — "with a message naming which of those happened and the path involved".
AC-4 adds one more: a configuration with no `[ingress]` section.

**Observed:** the configuration *file itself* is missing from both. No file at the
discovered path, a file that cannot be read, and a file `Config::load` rejects
(`crates/goad-shell/src/config.rs:142`) are three distinct exit-2 causes with three
different remedies, and none is an AC clause or a named test. VT-3 tests `--socket`
"honoured when no configuration file exists at all", which is the arm where the file
is deliberately **not** read. Given F-6, "no configuration at the default path" is
the case a first user meets first.

**Evidence:** `slice-005.md` AC-3, AC-4; `plan.md` PHASE-03/VT-3;
`crates/goad-shell/src/config.rs:142` — `pub fn load(path: &Path) -> Result<Self, ConfigError>`.

What makes it go away: AC-4 gains the clause — an absent, unreadable or unparseable
configuration at the discovered path is exit 2, naming the path and the fault — and
PHASE-03/VT-3 gains the case.

**Disposition:** fix-now
**Response:** Agreed, and it is the failure a person meets first on a fresh
machine. AC-4 now names **four** exit-2 endings of the discovery road, each
required to name the path and the fault: no path discoverable at all; the file
absent or unreadable; the file unparseable; and a configuration with no
`[ingress]`. PHASE-03/EX-5 gives them one type, `StartupFault`, and **VT-6** is
added to assert one rendered line per ending. The Coverage table's AC-4 row cites
it.

**Outcome:** verified

### F-10 — the Coverage table presents a clause as tested that no test can see

**Severity:** minor
**Location:** `plan.md` Coverage (AC-2 row), PHASE-03/VT-4; `slice-005.md` AC-2

**Expected:** AC-2 ends "Nothing branches on `detail` (R-14)". The Coverage table
maps AC-2 to four verifications without qualification, and VT-4 reads "`detail` is
appended but never parsed".

**Observed:** "never parsed" is an absence of code, not a behaviour: a test that
renders a line containing `detail` cannot distinguish appending it from having
branched on it. This is the class `docs/memory/a-green-test-can-assert-a-proxy.md`
records, and SPEC-003 §7 already sets the house convention for it — *"A row naming
no test is a row this spec may not be amended holding: where a clause cannot be
reached by a test, the row says so in terms and says what review holds instead"* —
which three rows of SPEC-003's own table then do (R-3's third clause, R-4's exit
code, R-5's process exit).

**Evidence:** `docs/specs/003-host-event-ingress.md` §7 preamble and the R-3/R-4
rows; `plan.md` Coverage and PHASE-03/VT-4;
`docs/memory/a-green-test-can-assert-a-proxy.md`.

What makes it go away: the AC-2 row says which part is review-held and what review
looks at — one call site, `detail` reaching `refused_line` as a string and nothing
else reading it — in the form SPEC-003 §7 uses.

**Disposition:** fix-now
**Response:** Agreed, and SPEC-003 §7's convention is the right form to copy — a
row that cannot be reached by a test says so and says what review holds instead.
The Coverage table's AC-2 row now carries that sentence explicitly: "nothing
branches on `detail`" is **review-held**, because the absence of a branch is not
observable from a rendered line, and what review looks at is named —
`refused_line` is the one function reading `detail`, it interpolates the string,
and no arm matches on it. PHASE-03/VT-4 points at the same note rather than
implying it tests the clause.

**Outcome:** verified

### F-11 — the Scope's "by re-export" is not available and is not what AC-7 describes

**Severity:** nit
**Location:** `slice-005.md` §Scope, first bullet

**Expected:** "**`crates/goad-emit/`** — … It depends on `goad-shell` (config, and
the envelope's canonical types **by re-export**)".

**Observed:** two things contradict it. AC-7 and PHASE-03/EX-1 both put
`goad-semantics` on emit's manifest directly, so `Event` and `Timestamp` arrive by
their own crate edge and no re-export is involved; and `clippy::pub_use = "deny"`
(workspace clippy table, root `Cargo.toml`) would refuse the re-export if one were
written. Nothing in `goad-shell` re-exports stratum 1 today
(`crates/goad-shell/src/lib.rs`).

**Evidence:** `slice-005.md` §Scope vs AC-7; root `Cargo.toml`, workspace clippy
table, `pub_use = "deny"`; `crates/goad-shell/src/lib.rs`.

What makes it go away: strike "by re-export" — the parenthesis is "(config, and
`report::line_to`)", with `goad-semantics` named separately.

**Disposition:** fix-now
**Response:** Confirmed — `clippy::pub_use = "deny"` would refuse the re-export,
nothing in `goad-shell/src/lib.rs` does it today, and AC-7 already put
`goad-semantics` on emit's manifest directly. The Scope bullet is rewritten: emit
depends on `goad-shell` for configuration, the client, the clock and the output
sink, and on `goad-semantics` **directly** for `Event` and `Timestamp`.

**Outcome:** verified

### F-12 — two declared surfaces contain nothing to change

**Severity:** nit
**Location:** `slice-005.md` §Scope, fourth bullet; `plan.md` PHASE-04 surfaces

**Expected:** "**`justfile`, `examples/demo.toml`, `examples/shell/backend.sh`** —
the demo's documented one-liner becomes `goad-emit`", and PHASE-04's surfaces add
"`README.md` if it names the one-liner".

**Observed:** the one-liner exists in exactly one place, `examples/demo.toml:17-21`.
`examples/shell/backend.sh` carries no `socat`, no `nc` and no envelope; `README.md`
names neither; the `justfile` holds only `demo: (run "examples/demo.toml")`
(`justfile:67`) and no emit line to convert. A declared surface with no work in it
is an invitation to find some.

**Evidence:** `grep -rn "socat|nc -U" README.md justfile examples/` returns three
lines, all in `examples/demo.toml`.

What makes it go away: the scope line names `examples/demo.toml` and the `justfile`
(where a new recipe or comment may be wanted), and drops `examples/shell/backend.sh`
and `README.md`.

**Disposition:** fix-now
**Response:** Confirmed by the same grep — the one-liner lives only in
`examples/demo.toml:17-21`. `examples/shell/backend.sh` and `README.md` are struck
from the Scope bullet and from PHASE-04's surfaces, and the bullet says why they
are not surfaces, so the next reader does not re-add them. `examples/demo.toml`
and the `justfile` stay; PHASE-04/EX-2 says the `justfile` recipe is added only if
it earns its place.

**Outcome:** verified

### F-13 — D-11 changed the type and left four statements about it standing; `{}` now parses

**Severity:** major
**Location:** `design.md` §5.2 (the `Unreadable` comment) and §5.5 (first bullet); `plan.md` PHASE-02/VT-5 and PHASE-01/EX-5
**Round:** 2 — raised against the F-5 repair

**Expected:** F-5's repair makes `protocol` and `accepted` `Option` on the read side
(D-11, PHASE-01/EX-4) *and* keeps the ruling that `{}` is `Unreadable`: §5.2
annotates that variant "`{}` lands here"; §5.5 says "`{}` never reaches that rule —
it is `Unreadable`, a serde error, and the design says so rather than letting a test
assert an unreachable outcome (F-5)"; PHASE-02/VT-5 asserts "`{}` … is a serde error
at the parse step above it and is **`Unreadable`**".

**Observed:** the two halves contradict each other. Once both fields are `Option`,
**every field of `Reply` is `Option`**, and serde's implicit-optional rule — the same
one that made `{"protocol":1,"accepted":false}` parse in round 1 — makes `{}` parse
too. Measured on the exact struct §5.2 specifies:

```
empty  -> Ok(Reply { protocol: None, accepted: None, reason: None, retry_after_ms: None, detail: None })
acc    -> Ok(Reply { protocol: None, accepted: Some(true), ... })
arr    -> Err(Error("invalid type: integer `2`, expected a boolean", line: 1, column: 4))
```

So `{}` reaches `read_reply` with `accepted: None` and is **`NonConforming`**, not
`Unreadable`. VT-5's first case asserts an unreachable outcome — which is F-5's own
sentence, now true of the repair. This is the class rather than the instance: the
design states what a byte string does without checking it against the type it has
just specified.

A fourth statement fell to the same change. PHASE-01/EX-5 says "**One line changes**:
`reason: refusal.map(Refusal::reason)` becomes an owning form". Three lines change:
`protocol: 1` becomes `protocol: Some(1)` and `accepted` becomes `Some(accepted)`,
because the host must keep writing both unconditionally. That is not cosmetic — it is
the pair of lines on which "the host's bytes do not change" rests.

**Evidence:** measured, `serde 1` / `serde_json 1`, on §5.2's struct verbatim (output
above). The write side was measured on the same struct and **is** byte-identical to
today's `Wire`: `{"protocol":1,"accepted":true}` and
`{"protocol":1,"accepted":false,"reason":"too_soon","retry_after_ms":1800,"detail":"d"}`
— so D-11 is sound and PHASE-01/VT-3 does hold it, provided `reply()` writes `Some`.
A `Reply` built with `protocol: None` serializes as `{"protocol":null,"accepted":null}`,
which is what VT-3's exact-string comparison exists to catch.

What makes it go away: §5.2's comment, §5.5's first bullet and PHASE-02/VT-5 all say
`{}` is **`NonConforming`** — it is a JSON object that breaches §6.3 by carrying no
`accepted`, which is exactly what that variant was renamed to mean. `Unreadable` keeps
a case of its own; `[1,2]` and `not json` are both real ones (the array is measured
above). PHASE-01/EX-5 says **three** lines change and names them.

**Disposition:** fix-now
**Response:** The measurement is right and the finding is the round-1 finding
inverted, which is the fairest possible thing for a reviewer to catch: the repair
changed the type and left four statements about it standing. All four are
corrected to what the type now does. `design.md` §5.2's comment gives
`Unreadable` its real cases (`[1,2]`, `not json`); §5.5's first bullet says `{}`
**parses** and is `NonConforming`, a JSON object breaching §6.3 by carrying no
`accepted`; `plan.md` PHASE-02/VT-5 asserts that, with `[1,2]`/`not json` as
`Unreadable`'s own cases; and PHASE-01/EX-5 now says **three** lines change and
names them — `protocol: Some(1)`, `Some(accepted)`, and `reason`'s owning form —
recording that the first two are what keep the host's bytes identical. The class
behind it is noted where it can act: the design stated what a byte string does
without checking it against the type it had just specified, and PHASE-01/VT-2 and
PHASE-02/VT-5 are now the places that check rather than assert.

**Outcome:**

### F-14 — the clock lift breaks five test files, so PHASE-01 fires S-2 by construction

**Severity:** blocker
**Location:** `plan.md` PHASE-01/EX-6, S-2, PHASE-01 surfaces and Notes for the implementer
**Round:** 2 — raised against the F-1 repair

**Expected:** EX-6 — "every existing test in the workspace passes **unedited** (S-2)".
S-2 — "an existing test anywhere must be **edited** to stay green in PHASE-01. That
means a lift was a rewrite; **stop and re-plan** rather than adjust the assertion."
The Notes locate the lift's reach: "`wall_clock` is called from `main.rs:53` and
`main.rs:116`, and `controller.rs` imports `ClockError` in its test module
(`controller.rs:769`) — that import changes, the assertions do not."

**Observed:** the reach is wider than the Notes, and it lands in test targets that
EX-6 forbids touching. Deleting `crates/goad/src/clock.rs` breaks **five test files**
at compile time, each of which names the module by path:

- `crates/goad/tests/event_loop/closing.rs:16` — `use goad::clock::wall_clock;`
- `crates/goad/tests/event_loop_schedule/scheduling.rs:21` — `use goad::clock::wall_clock;`
- `crates/goad/tests/renderer/harness.rs:13` — `use goad::clock::ClockError;`
- `crates/goad/tests/renderer/scheduling.rs:13` — `use goad::clock::ClockError;`
- `crates/goad/tests/renderer/startup.rs:17` — `use goad::clock::ClockError;`

Each must be edited to stay green. Under S-2 as written, the implementer must **stop
and re-plan** on the first one, and EX-6 cannot be discharged at all. A re-export
would avoid the edits and is unavailable: `clippy::pub_use = "deny"`.

Two production sites are missing from the same list. `crates/goad/src/lib.rs:3`
(`pub mod clock;`) must go, and `crates/goad/src/controller.rs:18`
(`use crate::clock::Clock;`) is the **type alias** `Clock`, which appears in four
production signatures (`controller.rs:274, 477, 532, 581`) — a different symbol from
the `ClockError` the Notes name at `:769`, which is the *test* module's import.
Neither `lib.rs` nor the five test files is in PHASE-01's declared surfaces.

This is the shape `docs/memory/a-repair-sweep-misses-the-binding-site.md` records: the
repair swept the call sites it could see from the design and stopped there. S-5 does
not catch it — S-5 is about dependency features, and this failure has nothing to do
with features.

**Evidence:** `grep -rn "clock\|Clock" crates/goad/src crates/goad/tests` — the five
`use goad::clock::…` lines above, plus `crates/goad/src/lib.rs:3` and
`crates/goad/src/controller.rs:18`; `Cargo.toml` workspace clippy table,
`pub_use = "deny"`; `plan.md` PHASE-01/EX-6, S-2.

What makes it go away: S-2 and EX-6 distinguish the two things they currently
conflate — **an assertion or a fixture changing** (a lift was a rewrite; stop) from
**an import path changing** (a lift's necessary consequence; proceed). State the
second as a bounded, enumerated allowance: these five files, `use` lines only, no
assertion and no fixture touched, and `git diff` over them shows nothing but the
import. Add `crates/goad/src/lib.rs` and the five test files to PHASE-01's surfaces,
and correct the Notes to name `controller.rs:18`'s `Clock` alias beside the test
module's `ClockError`. (`crates/goad/tests/renderer/startup.rs:179` mentions `line_to`
in a doc comment only — stale prose, not a compile break, and worth the same
allowance.)

**Disposition:** fix-now
**Response:** Confirmed by grep before disposing: five test files name
`goad::clock::…`, plus `src/lib.rs:3`, `src/main.rs:7`, `src/startup.rs:34`, and
`src/controller.rs:18` — which is the `Clock` **type alias** in four production
signatures (`:274, :477, :532, :581`), a different symbol from the test module's
`ClockError` at `:769`. The Notes named the wrong one of those two. As written the
plan would have halted PHASE-01 on its first import edit.

The repair is the two-sentence one the finding specifies, taken as a class fix.
**S-2 now separates what it conflated**: an *assertion or fixture* changing means
a lift was a rewrite and stops the phase; an *import path* changing is a lift's
necessary consequence and proceeds — bounded and enumerated, `use` lines only,
with a `git diff` over those files showing nothing else, and anything outside the
list is S-2 again. **EX-7** is added as that enumeration, citing
`docs/memory/a-repair-sweep-misses-the-binding-site.md` by name so the next
sweeper works from a list rather than a grep they trust; EX-6 is rewritten to
"no assertion and no fixture changed"; the surfaces gain `crates/goad/src/lib.rs`
and the five test files, marked *import lines only*; EX-3 carries the `Clock`
alias; and the Notes point at the two sites that are easy to miss, including the
stale `line_to` doc-comment mention at `renderer/startup.rs:179`.

**Outcome:**

### F-15 — PHASE-04/EX-1 attributes R-7's bounds to a tier that holds no case for them

**Severity:** minor
**Location:** `plan.md` PHASE-04/EX-1; `design.md` §9 (third bullet) and §5.5
**Round:** 2 — raised against the F-2 repair

**Expected:** EX-1 — "R-6's framing and R-7's read bounds are *not* on this path, and
the module doc says so" — with §9 and the F-2 response both placing them "one tier up,
by PHASE-02's cases against the real listener".

**Observed:** true of R-6, not of R-7. **R-6 is genuinely held**: PHASE-02/VT-1 drives
`send` into a real `bind`ed listener and requires `Answered::Accepted`, so a client
that framed wrongly — no newline, no `shutdown(Write)` — would draw `timed_out` and
turn the case red. That is a real instrument and it is worth the plan saying which
case it is.

**R-7's bounds are held by nothing in this slice.** No PHASE-02 case sends an
over-large envelope or a stalled connection; VT-2's "one scripted refusal per
remaining shape the fixture can produce" cannot reach `too_large` or `timed_out`,
because those are decided by the listener's own read before the judge sees anything.
The cases that hold R-7 are 004's
(`ingress::more_than_the_byte_limit_is_refused_too_large_and_the_limit_itself_is_accepted`,
`ingress::a_connection_that_writes_nothing_times_out_and_the_listener_serves_next`),
and they are about the **listener**, not about emit.

The design also lost the sentence that made emit's position on the byte bound legible.
The round-0 §5.5 carried *"**Edge:** a `--data` value that is valid JSON but enormous
is the host's to refuse (`too_large`, R-7). Emit does not second-guess the byte
bound"*, and the rewrite deleted it. It is the R-13 argument applied to R-7 — emit
does not pre-empt a host rule — and it is the only statement that said so.

**Evidence:** `plan.md` PHASE-02/VT-1..VT-5 (no byte- or time-bound case);
`docs/specs/003-host-event-ingress.md` §7, rows R-6 and R-7, naming 004's cases;
`git diff 03b0286 55e0adf -- docs/slices/005/design.md` — the deleted `too_large`
edge bullet.

What makes it go away: EX-1 splits the claim. R-6's framing is held by **PHASE-02/VT-1**,
named. R-7's bounds are held by **004's listener cases** and are not emit's to hold —
emit does not second-guess them, exactly as it does not pre-empt R-13 — and §5.5
restores the one-line edge bullet that says so.

**Disposition:** fix-now
**Response:** Accepted in full, including the part that makes the slice's coverage
look thinner: R-6 is genuinely held and R-7 is held by nothing here. PHASE-04/EX-1
now splits the claim rather than making one blanket statement — **R-6's framing by
PHASE-02/VT-1**, named, where a mis-framed write draws `timed_out` from the real
listener; **R-7's bounds by 004's listener cases**, which are about the listener
and not about emit. `design.md` §9's third tier says the same, and the deleted
edge bullet is **restored** to §5.5 with its reason made explicit: emit does not
second-guess the byte bound for the same reason it does not pre-empt R-13 — a
client that duplicated a host rule would leave that rule untested from the only
side that exercises it. That sentence was load-bearing and its loss in the rewrite
was an accident, not a decision.

**Outcome:**

### F-16 — three statements in `slice-005.md` were not swept by the repairs that falsified them

**Severity:** minor
**Location:** `slice-005.md` AC-7 (`:120`), OQ-7 (`:248`), §Scope → Tests (`:56`)
**Round:** 2 — raised against the F-1 and F-2 repairs

**Expected:** `docs/AGENTS.md` §Where it goes puts **current truth** in the artefact
files, and the Design step requires `slice-nnn.md` be revised for consistency with the
design. `design.md` D-9 now says "after the lift, emit needs no `jiff` at all";
PHASE-03/EX-1 says "**no `jiff`**"; PHASE-03/VA-1 asserts the manifest "names no
`slint`, no `tokio`, no `jiff`"; and PHASE-04 reaches no host at all.

**Observed:** `slice-005.md` still says the opposite in three places. The repairs
reached `design.md` and `plan.md` thoroughly — `Verdict` and `Ambiguous` are swept
from both, checked — and stopped at the slice file:

- **AC-7 (`:120`)** — "its manifest names `goad-shell`, `goad-semantics`, `serde_json`
  and `jiff`". It is the criterion the audit walks, and its own verification (VA-1)
  now contradicts it.
- **OQ-7 (`:248`)** — "the manifest is `goad-shell`, `goad-semantics`, `serde_json`,
  `jiff` and nothing else", in the same answer whose neighbour OQ-6 was corrected to
  say emit carries no `jiff`.
- **§Scope → Tests (`:56`)** — "an integration tier in the new crate against a fake
  listener, **and the end-to-end case that goes through a real host**". After F-2
  there is no such case: the only thing that goes through a real host is VH-1, which
  is a person, not a test.

**Evidence:** `slice-005.md:56, 120, 248` against `design.md` D-9 and `plan.md`
PHASE-03/EX-1, PHASE-03/VA-1, PHASE-04/EX-1.

What makes it go away: AC-7 and OQ-7 drop `jiff` and AC-7 says what the manifest is
after PHASE-01 (`goad-shell`, `goad-semantics`, `serde_json`); the Scope bullet says
the real-host leg is AC-8's, observed by a person, not a test.

**Disposition:** fix-now
**Response:** Confirmed — the repairs swept `design.md` and `plan.md` and stopped
at the artefact file, which is the file `docs/AGENTS.md` §Where it goes makes
*current truth* and the one the audit walks. All three corrected: **AC-7** drops
`jiff` and adds "and no dev-dependency", so it agrees with PHASE-03/VA-1 that
verifies it; **OQ-7** records the manifest as `goad-shell`, `goad-semantics`,
`serde_json` after OQ-6's correction, rather than contradicting its own neighbour;
and **§Scope → Tests** now says what the two test tiers actually are and states
that **nothing in this slice drives a real host end to end** — the only thing that
does is AC-8, and a person does it.

**Outcome:**

### F-17 — the same set is three in one place and four in two others

**Severity:** nit
**Location:** `plan.md` PHASE-03/EX-5 against `slice-005.md` AC-4 and `plan.md` PHASE-03/VT-6
**Round:** 2 — raised against the F-9 repair

**Expected:** one count of the ways configuration discovery ends at exit 2, since
`StartupFault`'s variants are what EX-5 is fixing.

**Observed:** EX-5 says "`StartupFault` names the **three** ways that road ends at
exit 2 — no path discoverable, the file absent or unreadable or unparseable, and a
config with `ingress: None`", grouping absent/unreadable/unparseable into one. AC-4
says **four**, separating "the file absent or unreadable" from "the file unparseable",
and VT-6 asserts **four** rendered lines on the same split. A reader implementing
`StartupFault` from EX-5 writes three variants and then cannot satisfy VT-6.

**Evidence:** `plan.md` PHASE-03/EX-5 and VT-6; `slice-005.md` AC-4.

What makes it go away: EX-5 says four and splits them as AC-4 does. The distinction
earns its place — *no file* and *a file that will not parse* have different remedies,
which is F-9's whole argument.

**Disposition:** fix-now
**Response:** Agreed, and the count that survives is four, because the split is
F-9's own argument: *no file* and *a file that will not parse* have different
remedies and deserve different messages. PHASE-03/EX-5 now says four and splits
them exactly as AC-4 and VT-6 do, so an implementer writing `StartupFault` from
the exit criterion produces the four variants VT-6 asserts.

**Outcome:**

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

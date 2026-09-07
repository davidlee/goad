# Review — implementation — Slice 003

**Subject:** implementation — the whole of slice 003, commit range
`0b2e50f..3777c22` on branch `slice-002` (six phase commits, PHASE-01 to
PHASE-06), and the working tree at `3777c22`
**Reviewer:** fresh adversarial agent (Claude Opus 5), no authorship in this slice
**Opened:** 2026-09-07
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
past the gate. Reject a finding on **evidence**, never on assertion. Confirm each
disposition with the user before acting on it. Fix the class, not the instance,
and do not introduce new defects repairing old ones.

## Brief

<!-- Written BEFORE the review, so it is not shaped by what turned out to be easy
     to find. What this review is probing, and the invariants it holds the
     subject to. Where the bodies are likely buried. -->

**Round 1** — 2026-09-07 — the whole implementation, read as a diff against the
slice's own design, its draft spec, and canon.

**What is being held to what.** The subject is code, so canon binds directly,
not by analogy. Five invariants from `CLAUDE.md`: no domain vocabulary in host
types; permissive wire parsing with canonical internals; **no narrowing of wire
compatibility to match what the renderer happens to implement**; a backend
failure never takes the host down and never leaves it uninvocable (SPEC-001
R-45); and one-way strata (ADR-001) with `cargo test --no-default-features` as
the compiler, not a convention. Above those, `draft-spec.md` R-1..R-11 —
this slice's working authority — and `design.md`'s D-1..D-18 and E-1..E-6.
POL-001 forbids weakening the gate, which includes a test whose passing depends
on machine load.

**Where the bodies are likely buried, stated before looking.**

1. **The `select!` and the pinned `Sleep`.** This is the only genuinely
   concurrent code in the slice and the design describes it in prose. A `biased`
   ordering that starves the timer arm; a `reset` racing a stop; an elapsed
   deadline that is silently dropped rather than superseded on the two
   `continue` paths the design claims preserve it (§5.4 property 1); a floor
   write that happens on a path other than the one the design names as its
   single write site (§5.3). The design asserts *"the sleep is always armed"*
   (I-2) as an invariant over every exit from every iteration — that is a claim
   about a control-flow graph, and control-flow graphs grow arms during
   implementation.

2. **The floor's arithmetic on a monotonic clock.** `Instant + Duration` panics
   on overflow in std and tokio alike. `MINIMUM_SPACING` is small, so the
   plausible overflow is not the floor but `Instant::now() + wait_for(...)`
   where `wait_for` may legitimately return ~8000 years (design E-6 concedes
   this and hand-waves it onto tokio's clamp — which clamps *deadlines it is
   given*, not the `Add` that produces one). That is a panic reachable from
   backend input, which is invariant 4.

3. **The three `refusal_re_arms` sites.** The task brief says two are never
   driven. Untested branches in a refusal path are where R-45 dies. Either they
   are dead — in which case the code says something the design does not — or
   they are wrong, and the floor re-arm fires on a refusal the design says
   leaves the deadline untouched.

4. **D-11's lateness.** The wait is computed from the request's `now`, not a
   fresh read. The design bounds the error at *one exchange duration* and cites
   2.9 ms. The unexamined case is an exchange **longer than `default_poll`**,
   where `wait_for` returns zero on every cycle and the loop's only bound is the
   floor — a hot-ish loop the design attributes solely to a misbehaving backend.

5. **R-9 — no scheduled evaluation while an exchange is in flight.** The plan
   discharges this "by construction, witnessed by six unchanged tests"
   (PHASE-02/EX-6). No test asserts it. A structural claim with no instrument is
   exactly what an adversarial reader should try to break, and the second
   `select!` is where to try.

6. **AC-6's two instruments.** An instrument that can be defeated is worse than
   no instrument, because it is believed. Probe: `use ... as` renaming, raw
   identifiers (`r#resolve`), macro-generated paths, `mod` re-export, and the
   `#[cfg(test)]` cutoff applied to an *inner item* rather than a module. The
   design already concedes two residues (I-1a); the question is whether there
   are others it did not concede, and whether the vacuity guards are real
   rather than decorative.

7. **The tests, read for what they actually assert.** Every timed assertion is
   suspect twice over: does it pass with the feature removed (vacuity), and does
   it fail under load (POL-001)? The design's margin table claims a smallest
   margin of 19x with one 3x outlier; PHASE-06 re-measured. Any wall-clock sleep
   used as *synchronisation* rather than as a *bound* is a finding regardless of
   its margin. The shell backend script is the least-reviewed artefact in any
   slice and gets read line by line.

8. **Design conformance and silent departures.** The design's `serve` sketch is
   concrete enough to diff against. Anywhere the code is not the sketch, the
   question is whether the departure is recorded in `notes.md` as a phase
   decision or simply happened.

9. **The renderer surface.** One line, one property, one setter — and the
   explicit prohibition (D-17) that it must not touch `diagnostic-lines`, on
   pain of retiring slice 002's DT-1 by side effect. Also: is the line ever a
   *prediction*? Draft SPEC-002 §6 forbids presenting the instruction as one.

10. **The cheap stuff that is still real.** `expect`/`unwrap`/`allow` outside
    tests; a clock read added to stratum 3's frame path (AC-6's neighbour);
    dead code; duplication with helpers that already exist (`CLAUDE.md`: DRY,
    no parallel implementation); naming.

**Round 2** — 2026-09-07 — the sixteen dispositions and the uncommitted repairs
in the working tree, verified on the ground rather than from the responses.

Every `fix-now` is checked against the code it claims to change, not against its
own description of it. Four break-and-reverts are run rather than trusted: the
brace-grouped `use` that defeated the old instrument (b); the participle rule
the old control could not see; an unconditional refusal re-arm; and an
unsatisfiable watcher predicate in the event-loop tier. The three attack
surfaces the round opens on its own account are whether instrument (b)'s new
count still holds AC-6's property or has swapped it for a number, whether the
item-scoped `#[cfg(test)]` cut is correct for every item shape, and whether
widening the liveness bound moved risk into the assertions the design says
cannot fail under load. Every document the repairs make false is a finding, on
the same footing as code.

**Round 3** — 2026-09-08 — the five round-2 repairs, verified only.

Each is broken deliberately and watched go red, or — for F-17 — the round-2
probe that reded is re-run and watched stay green. The round also asks the
question a verification round exists to ask and round 2 did not press hard
enough: does the whole gate still pass, repeatedly, under the parallelism the
gate actually uses. One target, three consecutive full runs, not one.

**Round 4** — 2026-09-08 — F-22's repair alone, verified only.

The question is whether the class was swept or the instance patched, so every
test in the module is sorted into those that assert a post-absorb fact and those
that do not, and each is checked against what it waits on before it stops the
loop. The two stated exceptions are checked against what they need rather than
against the rule. The gate is then run repeatedly, because one green run is what
let this defect through in the first place.

**What this review does not do.** It does not disposition. The standing grant in
`design-log.md`'s head lets the owner disposition without the user; the raiser
states and the responder disposes, and this agent is only the raiser.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | major | `fix-now` | `verified` |
| F-2 | major | `fix-now` | `verified` |
| F-3 | minor | `fix-now` | `verified` |
| F-4 | minor | `fix-now` | `verified` |
| F-5 | minor | `fix-now` | `verified` |
| F-6 | minor | `doc-wrong` | `verified` |
| F-7 | minor | `fix-now` | `verified` |
| F-8 | minor | `fix-now` | `verified` |
| F-9 | minor | `fix-now` | `verified` |
| F-10 | minor | `fix-now` | `verified` |
| F-11 | nit | `fix-now` | `verified` |
| F-12 | nit | `fix-now` | `verified` |
| F-13 | nit | `tolerated` | `verified` |
| F-14 | nit | `fix-now` | `verified` |
| F-15 | nit | `fix-now` | `verified` |
| F-16 | minor | `fix-now` | `verified` |
| F-17 | minor | `fix-now` | `verified` |
| F-18 | minor | `fix-now` | `verified` |
| F-19 | minor | `fix-now` | `verified` |
| F-20 | minor | `fix-now` | `verified` |
| F-21 | minor | `fix-now` | `verified` |
| F-22 | blocker | `fix-now` | `verified` |

### F-1 — AC-6's stratum 2 instrument is defeated by the very import shape the design built the stratum 3 instrument to survive, and is blind to half of two files

**Severity:** major
**Location:** `crates/goad-boundary/tests/checks/structure.rs:218` (`schedule_resolve_is_called_only_from_host`), matching on `"schedule::resolve"` through `structure.rs:70-81` (`production_lines`) and `goad_boundary::scan::mentions` (`scan.rs:225-234`)

**Expected:** `slice-003.md` AC-6 (b) and `draft-spec.md` §7's R-2 row both cite
this test as the evidence that `schedule::resolve` has no third call site. R-2
is *"The host MUST NOT resolve a next check anywhere but the one resolution
SPEC-001/R-26 describes."* Design D-16 and I-1a state the reasoning that
produced instrument (a): *"an item cannot be called without being named"*, and
*"a path grep is defeated by the brace-grouped import this slice makes
natural"* (F-3).

**Observed:** instrument (b) is that path grep. Two independent holes.

1. **The brace group.** `mentions` branches on `::` (`scan.rs:227-229`): a
   token containing `::` is matched as a plain substring. A third call site
   spelled `use goad_semantics::schedule::{resolve, parse};` followed by bare
   `resolve(…)` contains the substring `schedule::resolve` on no line, so the
   count stays at 2 and the instrument passes green. That is exactly the shape
   design F-3 names, applied to the one stratum where resolution actually
   happens. `use goad_semantics::schedule::resolve;` *is* caught, so the hole
   is not total — but the shape the design predicted an author would reach for
   is the one that escapes.
2. **The `#[cfg(test)]` cut is per file and takes the whole tail.**
   `production_lines` breaks at the first line whose trim is `#[cfg(test)]`
   and returns nothing after it, whatever that attribute was on. Over
   `crates/goad-shell/src` today that leaves **101 of 217 lines of `state.rs`**
   and **48 of 342 of `process.rs`** unread — measured — and `state.rs:41` is
   already a doc comment naming `schedule::resolve`, so it is a plausible
   place for a future call. An attribute placed early in `host.rs` would blind
   the instrument's own subject file.

**Evidence:** `scan.rs:227-229` (`if token.contains("::") { return
code_of(line).contains(token) }`); `structure.rs:74-76` (`if line.trim() ==
"#[cfg(test)]" { break; }`); first `#[cfg(test)]` line per file, measured:
`state.rs:116` of 217, `process.rs:294` of 342, `config.rs:185` of 360,
`host.rs` none. The vacuity guard
(`the_subject_directories_are_found_and_are_not_empty`, `:154-163`, and the
in-test repeat at `:219-222`) asserts a non-zero **file** count, never a
non-zero **line** count, so a blinded file is indistinguishable from a clean
one. `design.md` §9's *"Each instrument asserts its walk inspected a non-zero
file count, so neither can pass vacuously"* is therefore true as written and
weaker than it reads.

Instrument (a) (`:203`) has neither hole: it matches the bare identifier as a
word, and stratum 3's only `#[cfg(test)]` lines sit at the ends of their files.
The finding is about (b) alone.

**Disposition:** `fix-now`
**Response:**

Accepted in full; both holes are real and both are closed.

**The brace group.** Instrument (b) now passes the bare token `resolve` to the
same identifier matcher instrument (a) uses, and is renamed
`the_identifier_resolve_is_confined_to_the_hosts_resolution_path` — the old
name would have been a lie about the new file set. What it asserts, measured:
**9 production lines naming the identifier `resolve` in
`crates/goad-shell/src`, across exactly three files** — `host.rs` ×7 (the two
`schedule::resolve` calls, the two `resolve_from` sites and their two
`resolve_to` writes, plus `resolve_from`'s own definition), `state.rs:60`
(`resolve_to`), and `error.rs:163` (a message string, since `code_of` leaves
string literals intact). It is therefore a **change detector over the word**,
not a count of call sites, and the two residues are stated at the instrument
rather than filtered out: filtering them would need a second matcher, and a new
occurrence anywhere is a line worth reading before it ships, which is the whole
job. Break-and-revert: adding `use goad_semantics::schedule::{resolve, parse};`
to `state.rs` — the exact defeat this finding names — takes the count to 10 and
the test red; reverting it green.

**The `#[cfg(test)]` cut.** `production_lines` now skips the **item** the
attribute introduces (brace-tracked, with a one-line fallback for an unbraced
item) and resumes after it, rather than breaking at the first such line. A
fixture with production code after an inline test module is on disk at
`crates/goad-boundary/tests/fixtures/structure/production_after_tests.rs`, and
the control over it failed before the change and passes after. No subject file
has that shape today, which is why the fixture rather than the tree holds it.

**Vacuity.** `the_subject_directories_are_found_and_are_not_empty` now asserts,
per directory, that every file yields at least one production line and that the
directory clears a line-count floor — measured 1907 lines for
`crates/goad/src`, 1224 for `crates/goad-shell/src`. §9's *"non-zero file
count"* sentence is restated to say what each guard actually holds.

Instrument (a) was broken and reverted too: a bare `resolve` planted in
`crates/goad/src/wire.rs` turns it red. Recorded as D-23; `design.md` §9's AC-6
row and `draft-spec.md` §7's R-2 row both restated.

**Outcome:** `verified` — round 2. Both holes are closed and both closures are run, not read.
**The brace group:** instrument (b) is now
`the_identifier_resolve_is_confined_to_the_hosts_resolution_path`, matching the
bare identifier. Break-and-revert: inserting `use
goad_semantics::schedule::{resolve, parse};` at the top of
`crates/goad-shell/src/state.rs` takes the count 9 → 10 and the test red
(`left: 10, right: 9`); reverted, green. That is the exact defeat this finding
named, now caught. **The cut:** `production_lines` skips the attributed item and
resumes; the on-disk fixture and
`production_after_an_inline_test_module_is_still_read` hold it. Whole boundary
suite: 38 passed. `design.md` §9's AC-6 row and `draft-spec.md` §7's R-2 row
both follow the rename and the new claim.

Two consequences of the repair are raised separately rather than contested here:
the item skip is brace-counted over text `code_of` leaves string literals in
(F-17), and the count of 9 is coupled to things AC-6 does not care about
(F-18). Neither reopens this finding — the tree is strictly better defended than
it was.
### F-2 — canon-delta CD-1 closes `event.kind` to three values for every host, which is the shape `CLAUDE.md`'s third invariant forbids

**Severity:** major
**Location:** `docs/slices/003/canon-delta.md` CD-1, the drafted SPEC-001 R-56

**Expected:** *"Do not narrow wire compatibility merely because the current
renderer implements only a subset of admitted protocol capabilities. The
protocol is the contract; a renderer is one consumer of it."* (`CLAUDE.md`,
invariant 3 — named there as *the failure the project exists to avoid*.)

**Observed:** R-56 as drafted reads *"Its `event.kind` is one of exactly three
values … the host MUST NOT add a fourth without amending this requirement."*
The set is exactly what this build emits, and it is closed for **every**
conforming host, not just this one. The requirement CD-1 actually needs — so
that a backend can branch — is that these three kinds mean these three things.
Closing the set is a separate, stronger claim, and it is the one that binds a
second implementation with a stimulus this renderer does not have.

Two supporting observations rather than one:

- **The slice's own roadmap collides with it inside one slice.** `slice-003.md`
  Non-goals says slice 004 adds event ingress, *"a second stimulus into one
  path"*. Whether an evaluation driven by an ingested event is *"an `evaluate`
  the host originates"* is not settled by R-56's own text, so the next slice
  either amends a just-promoted requirement or argues its way around it. A
  requirement drafted in the knowledge that it must be amended immediately is
  a sign the closure is doing work the contract did not ask for.
- **Nothing obliges a backend to tolerate an unrecognised kind.** SPEC-001 R-7
  requires the field and says nothing about unknown values. The closed set is
  standing in for a tolerance requirement that was never written. The permissive
  half of `CLAUDE.md`'s second invariant points the other way.

CD-1 is `proposed`, not applied, which is why this is raised now: the choice is
still open at reconciliation.

**Evidence:** `canon-delta.md` CD-1's *"The change, as it will be stated"* table;
`slice-003.md` Non-goals, *"Event ingress (slice 004)"*; `CLAUDE.md` invariant 3;
SPEC-001 R-7.

**Disposition:** `fix-now`
**Response:**

Accepted. The closure is the narrowing `CLAUDE.md`'s third invariant names, and
the finding is right that the cost of deciding now is nil because CD-1 is
`proposed`.

**No canon applied** — that is reserved to the user. `canon-delta.md` CD-1 is
**redrafted**, and the user endorses the wording at reconciliation as with every
canon change. R-56 as it now reads: the host's `evaluate` carries `event.source`
of `"host"` and an `event.kind` naming why it is asking; three kinds are named
and fixed in meaning (`"startup"`, `"requested"`, `"scheduled"`) and a host must
not reuse one for anything else; **the set is open** — a host may originate a
further kind, and a backend **must tolerate** a kind it does not recognise,
treating it as an evaluation whose reason it does not know rather than as a
protocol error. CD-1's *Why* now states both the invariant and the slice-004
collision, and notes that the tolerance clause is a backend obligation verified
by inspection.

`draft-spec.md` carried the same narrowing by reference in two places and is
redrafted with it: §2's Boundaries now says R-56 *names* the kind rather than
*fixes* it, and §6 adds *"R-56 leaves the set of kinds open; this spec adds no
kind of its own and closes nothing."*

Recorded as D-19 in `design.md` §7, with the decision in `design-log.md`.

**Outcome:** `verified` — round 2. R-56 is redrafted in `canon-delta.md` and the set is open:
three kinds named and fixed in *meaning*, a host MAY originate a further kind,
and a backend MUST tolerate one it does not recognise rather than treating it as
a protocol error. The *Why* now quotes `CLAUDE.md`'s third invariant and names
the slice-004 collision, and states that the tolerance clause is a backend
obligation verified by inspection — which is honest about what a host-side test
can hold. The two places `draft-spec.md` carried the narrowing by reference are
corrected: §2 says R-56 *names* rather than *fixes*, and §6 adds that R-56
leaves the set open and this spec closes nothing. D-19 records it. Nothing is
applied to `docs/specs/`, which is right — that is the user's at
reconciliation.
### F-3 — two of the three `refusal_re_arms` sites are unreachable; the flag is right and two of its three copies are dead

**Severity:** minor
**Location:** `crates/goad/src/controller.rs:415` and `:428` (the two inside `Command::Choose`); the live one is `:406`

**Expected:** `design.md` §5.4 describes one conditional: *"re-arm iff this
iteration came from the timer arm."* `CLAUDE.md`: *write less code*, and
*small, composable, single responsibility*.

**Observed:** `refusal_re_arms` is `matches!(fired, Fired::Scheduled)`
(`:378`), and `Fired::Scheduled` unconditionally becomes
`Command::Evaluate(Stimulus::Scheduled)` (`:382`). So under
`Command::Choose { .. }` the flag is `false` by construction, and both `if
refusal_re_arms { sleep.as_mut().reset(floor_until); }` blocks inside that arm
are dead code no input can reach. Only `:406`, in the `Command::Evaluate` arm,
is ever true — and `renderer/scheduling.rs`'s VT-3 is the one test that drives
it.

Not a behaviour defect: the live site does the right thing and the dead ones
would too. It is a correctness hazard of the second kind — three copies of the
same two lines, two of which imply the flag is orthogonal to the command when
it is fully determined by it, so a future reader may preserve the wrong
invariant. The three refusal `continue`s were already a triplication before
this slice; this slice makes each of them longer rather than folding them.

**Evidence:** `controller.rs:378-383`, `:404-410`, `:413-419`, `:424-432`.

**Disposition:** `fix-now`
**Response:**

Accepted, and repaired as a class rather than by deleting two blocks. The three
refusal `continue`s are folded into **one**: the four command arms now produce a
`Result<Pending, Refused>` — `Command::Evaluate` maps `stamp`, `Command::Choose`
chains `answer` into `stamp` with `and_then`, and the two diagnostics commands
still `continue` where they stand — and a single `match` reports the refusal and
applies the conditional re-arm. Identity is still checked before the clock,
because `answer` runs before `stamp` inside the chain.

`refusal_re_arms` now has exactly one reader, on the only path that can make it
true. The triplication it removes predates this slice; this slice had made each
copy longer.

The behaviour was uncovered, so a test was added first:
`a_refusal_that_did_not_come_from_the_timer_leaves_the_deadline_standing` — one
exchange instructs 60 s, a `Choose` naming a token nothing ever minted is
refused as `SupersededView` (read off the tray the production glass wrote), and
no invocation lands across a 500 ms window. It discriminates: forcing the
`Choose` arm to re-arm unconditionally — the shape the dead code invited — resets
the sleep to a `floor_until` already in the past and fires within a millisecond,
and the test goes red. An anti-fire window, so load can only cause a false pass,
never a false failure. Recorded as D-21.

**Outcome:** `verified` — round 2. `serve` now has one refusal site. The four command arms
produce `Result<Pending, Refused>`; the two diagnostics arms still `continue`
where they stand; one `match` reports and applies the conditional re-arm.
Identity is still checked before the clock, because `answer` runs before `stamp`
inside the `and_then`. Every refusal path reaches the one site — checked arm by
arm, not assumed.

The new test discriminates. Break-and-revert: forcing the re-arm unconditional
(`if refusal_re_arms || true`) turns
`a_refusal_that_did_not_come_from_the_timer_leaves_the_deadline_standing` red;
reverted, green. Worth recording is *how* it goes red — the unwanted scheduled
firing overwrites the refusal before the tray poll sees it, so the failure lands
on the `until` for the tray text rather than on the invocation count the doc
comment nominates. It detects the defect either way. Also worth recording:
plainly removing the condition does not even compile, because `refusal_re_arms`
then has no reader — the single site made the flag's deadness a compiler
concern, which is the strongest form this repair could have taken.

One deviation noted, not raised: this test opens its channel at capacity 2 where
production and every sibling test use 1 (`main.rs:72`). Nothing in the test needs
it.
### F-4 — the participle control test is vacuous: its fixture is a comment, which the matcher strips before it can reach the word

**Severity:** minor
**Location:** `crates/goad-boundary/tests/checks/structure.rs:284`, `the_participle_resolved_is_not_counted_by_the_word_matcher`

**Expected:** the test's own doc comment says it exists so the participle
decision is *"decided and asserted, per EX-2/VT-5, rather than left for a reader
to work out from the matcher's source."* Its name asserts that `resolved` is not
counted **as a word**.

**Observed:** the fixture is `"// the outcome resolved cleanly on the first
try"`. `mentions` calls `code_of` first (`scan.rs:226`), and `code_of` returns
the empty string for a line that begins with `//`. The assertion therefore holds
for the same reason the test two above it holds — comment stripping — and says
nothing whatever about the participle. Remove the participle from the fixture
and the test still passes; that is the definition of vacuous.

The underlying claim is in fact true (`is_singular_or_plural_of("resolved",
"resolve")` is false at `scan.rs:402-407`: `resolved` neither equals the token
nor strips an `s`). So the fix is one line — a fixture that is code, e.g.
`"let resolved = outcome;"` — and it is worth taking, because a green vacuous
control is the thing the slice's own vacuity discipline exists to prevent.

**Evidence:** `structure.rs:279-287`; `scan.rs:225-234`; `scan.rs:402-413`.

**Disposition:** `fix-now`
**Response:**

Accepted; the control was vacuous exactly as described. The fixture is now code
(`"    let resolved = outcome;"`), and the test asserts first that the fixture
survives `code_of` — otherwise the repair could rot back into a comment without
anyone noticing.

Verified non-vacuous by breaking the rule it controls: teaching
`is_singular_or_plural_of` to strip a trailing `d` makes it red; reverting makes
it green. The old fixture would have stayed green through both.

**Outcome:** `verified` — round 2. The fixture is `"    let resolved = outcome;"`, and the
test asserts it survives `code_of` before asserting the matcher does not fire —
so the repair cannot rot back into a comment silently.

Break-and-revert: teaching `is_singular_or_plural_of` to strip a trailing `d`
turns `the_participle_resolved_is_not_counted_by_the_word_matcher` red
(`assertion failed: !mentions(line, "resolve")`); reverted, green. The old
fixture would have stayed green through both, which is what made it vacuous.
### F-5 — `Instant::now() + wait` is a panicking add reachable from backend input, and E-6's justification is about a different operation

**Severity:** minor
**Location:** `crates/goad/src/controller.rs:462`

**Expected:** `CLAUDE.md`: *a backend failure never takes the host down.*
`design.md` §5.5 E-6 addresses an instruction further out than a timer can
hold: *"`wait_for` is total across jiff's range … tokio clamps a deadline at
`MAX_SAFE_MILLIS_DURATION` … so such a sleep fires early rather than
overflowing or hanging."*

**Observed:** E-6's clamp is applied by tokio to a deadline it is **given**. It
is not applied to the `Add` that produces the deadline. The code performs
`tokio::time::Instant::now() + wait` first, and `Add<Duration> for Instant`
panics on overflow rather than returning an `Option`. `wait` is
backend-controlled up to the jiff `Timestamp` range — a `next_check` in the
year 9999 yields roughly 2.5×10^11 seconds.

The add does not in fact overflow on Linux, where `std::time::Instant` is a
`timespec` with an `i64` seconds field and 2.5×10^11 is nowhere near its
bound. So this is a latent hazard, not a live one. But the safety comes from a
platform representation detail that no document states and no test covers:
`wait_for_is_total_at_the_future_edge_of_representable_time`
(`schedule.rs:384`) tests the pure function only, and no loop-level test drives
a far-future instruction through `serve`. A `checked_add` with a saturating
fallback would cost one line and retire the whole class.

**Evidence:** `controller.rs:462`; `design.md` §5.5 E-6; `schedule.rs:382-388`;
absence of any `serve` test with a far-future absolute instruction in
`renderer/scheduling.rs`.

**Disposition:** `fix-now`
**Response:**

Accepted. The finding is right that E-6's clamp is tokio's, applied to a
deadline it is *given*, and says nothing about the `Add` that produces one, and
right that the safety on Linux rests on a platform representation no document
states.

The arithmetic is now a named total function, `controller::deadline_after(now,
wait, floor)`: `now.checked_add(wait)`, falling back to `now.checked_add(
LONGEST_WAIT)` and finally to `now` itself, then `max`ed against the floor
exactly as before. `LONGEST_WAIT` is 365 days. A clamped firing is
self-correcting — the exchange it produces re-resolves from the instruction the
host still holds — and nothing stored or reported moves, so SPEC-001/R-28 is
untouched.

Red first: the helper was introduced carrying the original `now + wait`, and
`a_wait_that_would_overflow_the_clock_is_clamped_rather_than_panicking` panicked
inside tokio's `Instant` add. Two more unit tests pin the ordinary sum and the
floor winning. The finding's other half — *no loop-level test drives a
far-future instruction through `serve`* — is closed by
`an_instruction_at_the_far_edge_of_time_arms_the_sleep_without_panicking`, which
drives `9999-12-01T00:00:00Z` (the far edge of what jiff's range admits) through
a real backend and the production `serve`, and asserts the instruction is stored
and reported as given. Recorded as D-20.

**Outcome:** `verified` — round 2. `deadline_after(now, wait, floor)` is total:
`checked_add(wait)`, else `checked_add(LONGEST_WAIT)`, else `now`, then `max`ed
against the floor. Three unit tests beside it, including
`Duration::MAX` forcing the clamp, and one loop-level test driving
`9999-12-01T00:00:00Z` through a real backend and the production `serve`, which
is the path the finding said no test reached.

**The fallback to `now` is not a busy loop, and the floor is why.** A deadline of
`now` is already elapsed, so the sleep fires at once; the timer arm's first act
is `floor_until = now + MINIMUM_SPACING`; the next re-arm takes
`max(now, floor_until)` and lands three seconds out. It degrades to exactly the
past-instant case R-4 already bounds — one unfloored firing, then one per
spacing. Traced through the code, not taken from the doc comment.

On Linux the clamp branch is unreachable for any jiff-representable instruction,
because `Instant` is a `timespec` with an `i64` seconds field and ~2.5×10^11
seconds fits; the clamp is defence against a platform where it does not, which
is what the finding asked for. The residue is that `design.md` §5.5 E-6 was not
updated to say so — raised as F-20.
### F-6 — an unprompted scheduled evaluation can supersede a view a person is mid-answering, and no document says so

**Severity:** minor
**Location:** behaviour of `crates/goad/src/controller.rs:363-383` combined with `Controller::answer` (`:193-211`); `design.md` §5.4, D-3; `draft-spec.md` R-5

**Expected:** design D-3 and `draft-spec.md` R-5 both frame the floor as the
thing that keeps a person unaffected: *"a person's own action must never be
delayed"*, *"It MUST NOT delay an evaluation a person asked for."* Design §5.4
property 2 considers what a scheduled instant does to an exchange in flight and
concludes it *"does not preempt"* it.

**Observed:** neither considers a scheduled evaluation arriving while a person
is *looking at a prompt*. Before this slice a view could only be replaced by an
evaluation the person themselves asked for. Now the loop dispatches one
unprompted, and if the backend answers it with a view, `absorb` replaces the
retained presentation and mints a new `ViewId`. The click that person then
makes names the old token and is refused `Refused::SupersededView`
(`controller.rs:193-196`) — a refusal they did nothing to cause and that the
diagnostic surface reports without a reason they can act on. With
`MINIMUM_SPACING` at 3 s this is bounded at twenty replacements a minute, which
is D-2's own stated budget read from the other side.

The behaviour follows from the feature and is arguably correct; the finding is
that it is undesigned. D-3's *"a person's own actions are never delayed"* is
true and incomplete: a person's own actions can now be **refused** because of a
scheduled firing, which is a different and worse outcome than being delayed.
The floor bounds the rate but the design never weighs it against this.

**Evidence:** `controller.rs:363-383` and `:457-466`; `controller.rs:193-196`;
`design.md` §7 D-3; `draft-spec.md` R-5; no acceptance criterion, test or
SPEC-002 open question covers it.

**Disposition:** `doc-wrong`
**Response:**

Accepted as stated: the behaviour follows from the feature and is arguably
correct, and the defect is that nothing weighs it. D-3's *"a person's own
actions are never delayed"* is true and incomplete — an action can now be
**refused** because of a firing the person did not cause, which is a different
and worse outcome than a delay.

Documented rather than changed, in three places: `design.md` D-22 states the
decision and why both candidate repairs were rejected; `draft-spec.md` §5 gains
*"What a scheduled evaluation may replace"*, naming SPEC-001/R-33 and saying
plainly that R-4 bounds the rate and nothing else does; and §8 gains OQ-4.

**Not repaired in this slice**, and carried as a Follow-up in `slice-003.md`.
Both repairs — suppressing a firing while a presentation is outstanding, or
deferring it — ask the host to decide that a view is worth protecting, which is
domain meaning the host does not hold (`CLAUDE.md`, invariant 1). The likeliest
answer is a backend affordance, which makes it a protocol question rather than a
loop one. This is not a fix deferred for being large; it is one deferred because
taking it here would put domain judgement in the host.

**Outcome:** `verified` — round 2, and the deferral is a scope boundary rather than a
downgrade. Stated exactly, because the question was asked exactly.

**It is not the guardrail case.** The protocol's guardrail is *"do not reach for
`follow-up` because the fix is large."* The stated reason is not size: both
candidate repairs — suppress a firing while a presentation is outstanding, or
defer it — require the host to hold that *this view is worth protecting*, which
is a judgement about what the view is for. `CLAUDE.md`'s first invariant puts
that in the backend. That is a boundary argument, and it is the same argument
the project makes everywhere else.

**It could not have gated anything.** The finding was raised `minor`; only a
`blocker` gates acceptance. There was no gate to clear.

**The documentation is real, not a gesture.** `draft-spec.md` §5 gains *"What a
scheduled evaluation may replace"*, which says plainly that R-4 bounds the rate
and nothing else does and that R-5's *"MUST NOT delay"* is about delay only;
§8 gains OQ-4 with both repairs and why each is refused here; `design.md` D-22
records the decision; and `slice-003.md` Follow-ups carries it with the
mechanism named. A reader meeting the refusal now has somewhere to go.

Two editorial residues, noted rather than raised: the new §5 block is inserted
between *"begins an `evaluate` with the `"scheduled"` kind."* and *"That
exchange reports a new resolved next check"*, orphaning the second sentence from
the paragraph it continued; and the Follow-up calls the disposition
`accept-in-part`, which is not one of the five this ledger's Protocol defines —
the table says `doc-wrong`.
### F-7 — `design.md` §9's margin table is now false about the tree, and the slice ships on margins near its own STOP floor

**Severity:** minor
**Location:** `design.md` §9's margin table (`:700-713`) against `notes.md`'s PHASE-06 measured table

**Expected:** the design states one number per timed assertion and says *"the
plan re-measures the gate before and after, and a phase that finds the estimate
badly wrong is a finding, not a rounding error."* Four rows claim `~105 ms`
expected against `until(2 s)` and a **19x** margin; `design.md` §9's prose calls
19x *"the smallest"*.

**Observed:** PHASE-06 measured those four rows at ~250–290 ms, giving
**6.9x–8x**. The design's table was not corrected — correctly, since PHASE-06's
Surfaces line forbids touching `design.md` — and the drift is recorded honestly
under *Design drift not reconciled*. But the document is the one a later reader
consults, and it now overstates the slice's real margin by roughly 2.7x on its
four most load-sensitive assertions. The closest measured row, AC-2 from a
`respond` at ~6.9x, sits materially nearer the plan's own 5x STOP floor (S-19)
than §9's confidence implies, and `just check` is a gate POL-001 says must not
be load-dependent.

Two further mismatches in the same table, both from PHASE-06's own list: the
AC-9 row predicts one liveness figure for a test that produces no comparable
number, and the AC-7 row predicts ~2000x against a measured ~20 000x. The
table's shape, not just its numbers, does not correspond to the tests.

**Evidence:** `design.md:700-713`; `notes.md` PHASE-06 *The measured margin
table* and *Design drift not reconciled*; `plan.md` PHASE-06 S-19.

**Disposition:** `fix-now`
**Response:**

Accepted, and the repair is the bound rather than the number.

**The table.** `design.md` §9's margin table is rewritten with measured values
in place of estimates, and a paragraph above it says plainly that the estimate
was wrong by ~2.7x and why (a per-test figure pays for real `bash` subprocess
spawns the idealised wait does not). The AC-9 and AC-7 rows the finding names as
mismatched in *shape* are restated to describe what those tests actually
produce, and two rows are added for the assertions this round introduced. R1's
mitigation prose no longer claims 19x.

**The margin.** The liveness bound is raised from two seconds to five, as
`waiting::LIVENESS_BOUND`, and every `until` call site in the renderer tier now
names it rather than a literal. Re-measured, three runs each, per-test with
`--exact`: AC-1 0.27/0.25/0.27 s, AC-2 from an `evaluate` 0.26/0.26/0.28 s, AC-2
from a `respond` 0.29/0.28/0.28 s, AC-3 earlier 0.27/0.26/0.28 s, AC-10
0.27/0.26/0.28 s. Against five seconds that is ~18x throughout, against the 5x
STOP floor with room. A wider bound costs nothing in the passing case, because
the poll returns as soon as its predicate holds; the whole cost is paid by a
test that was going to fail anyway.

**Outcome:** `verified` — round 2, on both halves. `design.md` §9's table now carries
measured values, the AC-9 and AC-7 rows are restated to describe what those
tests actually produce, two rows are added for the assertions this round
introduced, and R1's mitigation no longer claims 19x — it states the ~7x
measurement and why the bound moved.

The bound is `waiting::LIVENESS_BOUND`, five seconds, named at every renderer
call site; no literal `Duration::from_secs(2)` remains in `scheduling.rs`. The
reasoning that a wider bound costs nothing in the passing case is correct: the
poll returns as soon as its predicate holds.

**It does not weaken any anti-spin or anti-fire assertion directly** — those are
fixed 300 ms and 500 ms `sleep`s, independent of the poll bound — and it cannot
turn a failing test green, because every window is anchored to the moment its
`until` returns. One test's *stated* load-safety no longer holds, which is
raised as F-21 rather than contested here.
### F-8 — `until` is implemented twice

**Severity:** minor
**Location:** `crates/goad/tests/renderer/harness.rs:76` and `crates/goad/tests/event_loop_schedule/scheduling.rs:54`

**Expected:** `CLAUDE.md`: *"DRY — find out whether existing code can be
adapted. No parallel implementation! Find potential duplication before writing
new code."* The slice already took the trouble of splitting
`tests/support/driving.rs` into `scripting.rs` (D-18) precisely so a shared
helper could be shared without a lint suppression.

**Observed:** the same poll-until-true loop is written twice, five lines apart
in shape, differing only in whether the bound is a parameter or a module
constant. The second copy's doc comment states the reason — *"restated here
rather than shared, because this target includes only `scripting.rs`"* — which
is a true statement about the file layout, not an argument that a third support
file was unavailable. D-18's own mechanism (a `tests/support/*.rs` file
included by `#[path]`, containing only what its includers use) is exactly what
would have carried it.

The cost is small and the precedent is not: the next target that needs to
observe over time will make a third copy for the same reason.

**Evidence:** `harness.rs:71-88`; `event_loop_schedule/scheduling.rs:50-64`;
`design.md` §7 D-18.

**Disposition:** `fix-now`
**Response:**

Accepted. The poll loop now exists once, in `tests/support/waiting.rs`, included
by both targets through the `#[path]` mechanism D-18 established. The doc
comment the finding quotes is gone with the copy it justified.

The split is not quite the obvious one, because F-10 wants the other half:
`waiting::within(bound, predicate) -> bool` holds the loop and never panics;
`renderer/harness.rs::until` is now three lines asserting over it, for the
twenty-odd call sites that want the panic where they stand; and the event-loop
tier calls `within` directly, so nothing is dead in either target and no lint
suppression is needed. `LIVENESS_BOUND` and the 5 ms poll interval live there
too, which is what F-7's repair then had one place to change.

**Outcome:** `verified` — round 2. One poll loop, in `tests/support/waiting.rs`, included by
both targets through the `#[path]` mechanism. The split is better than the one
this finding asked for: `within` returns a bool and never panics, `until` is
the three-line asserting wrapper in `renderer/harness.rs`, and the event-loop
tier calls `within` directly — which is what F-10's repair needed. `LIVENESS_BOUND`
and the poll interval live there too, so F-7's change had one site. Both targets
build clean, so no `pub(crate)` symbol is unreachable from either includer.
### F-9 — new code and markup cite slice-local `D-N`/`F-N` ids, which a durable project decision forbids; two of the citations point at the wrong thing

**Severity:** minor
**Location:** `crates/goad/src/controller.rs:50`, `:109`, `:301`, `:313`, `:319`, `:354`, `:359`, `:371`; `crates/goad/src/diagnostics.rs:265`, `:269`; `crates/goad/ui/app.slint:22`; and throughout `crates/goad/tests/renderer/scheduling.rs`

**Expected:** `docs/memory/cite-requirements-not-finding-ids.md`: *"A comment in
`src/`, `tests/` or `examples/` … cites a spec requirement … It does not cite a
slice-local review finding (`F-N`) or design decision (`D-N`)."* The memory is
explicit that slice 001's existing citations are tolerated and *"Do not extend
the practice."* PHASE-01..06's sheets each record reading all sixteen memory
files.

**Observed:** the practice is extended, not retired. `MINIMUM_SPACING`'s doc
comment is the one that does it right — it leads with `SPEC-002/R-4` — and the
rest of the slice's new comments lead with `D-N` or `F-N`. Two are also simply
wrong, which is the failure mode the memory predicts:

- `controller.rs:316-319`: the `Fired` enum's rationale cites *"design.md §5.4,
  F-12"*. F-12 in this slice's ledgers is the finding behind D-14, the placement
  of `MINIMUM_SPACING` in stratum 3 (`design.md:174`, `:649`). It has nothing to
  do with a provenance discriminant.
- `diagnostics.rs:265-272`: *"never a prediction of when the host will actually
  fire (draft-spec.md R-2's closing clause)"*. R-2's closing clause is *"its
  whole input is the resolved instant the last exchange reported"*. The rule
  actually being cited is `draft-spec.md` §6: *"A host that surfaces its next
  check to a person MUST NOT present the instruction as a prediction of when it
  will fire."*

Raised once as a class, not per instance. The class fix is either a sweep to
requirement ids or an explicit amendment retiring the memory — both are the
owner's call, and the two miscitations are worth correcting either way.

**Evidence:** `docs/memory/cite-requirements-not-finding-ids.md`;
`controller.rs:316-319`; `design.md:174` and `:649` for what F-12 is;
`diagnostics.rs:265-272` against `draft-spec.md` R-2 and §6.

**Disposition:** `fix-now`
**Response:**

Accepted, and swept as a class together with F-16 rather than instance by
instance. Every `D-N` and `F-N` citation **this slice added to production
source** is replaced by the requirement it stands for or by the reason in words:
eight `D-N` and one `F-N`, across `crates/goad/src/controller.rs`,
`crates/goad/src/diagnostics.rs` and `crates/goad/ui/app.slint`. Nothing this
slice added remains.

Both miscitations are corrected at the same time. The `Fired` enum no longer
cites F-12 — which is D-14's finding and has nothing to do with a provenance
discriminant — but states what the discriminant is for and cites SPEC-002/R-4
and R-8. `next_check_line` no longer cites *"R-2's closing clause"* but
`draft-spec.md` §6, which is the rule it meant.

**Boundary of the sweep, stated.** Slice 001's and slice 002's existing
citations are untouched: the memory tolerates slice 001's by user decision, and
retiring slice 002's would rewrite files this slice never opened. The comments
written *during this repair round* carry no slice-local ids either — the reason
is stated in words instead, which is what the memory asks for. See F-13 for the
one pre-existing citation raised separately.

**Outcome:** `verified` — round 2, measured. Zero `D-N` citations remain anywhere in
production source or markup, and every `F-N` still present is one this slice
did not write: `controller.rs` and `diagnostics.rs` carry the same counts at
`0b2e50f` as they do now (3 and 4), and the rest are slice 001's and 002's in
crates this slice never opened. Both miscitations are corrected — the `Fired`
enum now cites SPEC-002/R-4 and R-8 and says what the discriminant is *for*, and
`next_check_line` cites `draft-spec.md` §6, which is the rule it meant. The
comments written during the repair round carry no slice-local ids either, which
is the part that would have been easiest to get wrong.
### F-10 — the AC-10 test's only liveness bound sits inside the event loop it is bounding, so its failure mode may be a hang rather than a failure

**Severity:** minor
**Location:** `crates/goad/tests/event_loop_schedule/scheduling.rs:146-152`, with `until` at `:54` and `slint::run_event_loop_until_quit()` at `:155`

**Expected:** `slice-003.md` AC-12 and POL-001 forbid a test whose passing
depends on machine load; the corollary a gate depends on is that a *failing*
test fails, promptly and legibly. Every renderer-tier test in this slice keeps
its bound in the driving task, where a blown `until` panics the test directly.

**Observed:** here the bound is inside the watcher `spawn_local` task. If the
scheduled firing never lands, `until` panics **inside** the Slint event loop's
poll, so `stopper.stop()` is never reached, `serve` never returns, its
`quit_event_loop()` is never called, and `run_event_loop_until_quit()` on the
test thread has nothing to end it. Whether the process then fails or hangs
depends entirely on whether that panic unwinds cleanly back out through the
testing backend's loop — which nothing in the slice establishes, and which is
the one component `design.md` A-1 already declares unproven.

There is no outer timeout: `cargo test` imposes none, so a hang is an
indefinitely wedged gate rather than a red one. A `tokio::time::timeout` around
the watcher's `until`, or a `slint::Timer`-driven backstop that quits the loop,
would make the failure mode a failure.

**Evidence:** `event_loop_schedule/scheduling.rs:50-64`, `:146-156`;
`renderer/scheduling.rs` for the contrasting shape (bound in the driving task,
e.g. `:119`); `design.md` §5.5 A-1.

**Disposition:** `fix-now`
**Response:**

Accepted; the failure mode was a hang, and a wedged gate is worse than a red
one because nothing ends it.

The watcher no longer panics on the wait. It calls `waiting::within`, which
returns a bool, records it in an `Rc<RefCell<bool>>`, and stops the loop either
way; the assertion is made on the test thread after
`run_event_loop_until_quit()` returns. So `serve` always returns, its
`quit_event_loop` always runs, and a missed firing is a legible failure.

Proven by breaking it: changing the watcher's predicate to `>= 99` makes the
test **fail in 5.16 s** with *"the scheduled evaluation never landed within
5s"*, where the previous shape would have hung. Reverted.

**Outcome:** `verified` — round 2, by breaking it. The watcher calls `within`, records the
bool in an `Rc<RefCell<bool>>`, and stops the loop either way; the assertion is
made on the test thread after `run_event_loop_until_quit()` returns.

Break-and-revert: changing the watcher's predicate to `>= 99` makes the test
**fail in 5.17 s** with *"the scheduled evaluation never landed within 5s"*.
Under the old shape that same break would have panicked inside the event loop's
own poll, leaving nothing to call `quit_event_loop`. Reverted, green in 0.26 s.
The one test that runs the production topology now fails legibly.
### F-11 — the new backend script's header claims parity with `answers-as-instructed.sh` that its past-the-end behaviour does not have

**Severity:** nit
**Location:** `tests/backends/logs-the-request-then-answers.sh:1-5` and `:22`

**Expected:** the header says *"Like `answers-as-instructed.sh` — one
instruction per invocation, past the end of the list it behaves — except the
invocation log holds each raw request."* The stated difference is the log
only.

**Observed:** there is a second difference. `answers-as-instructed.sh` answers
past the end of its list with `{"view":null,"next_check":"45 minutes"}`; the
new script answers `{"view":null}`, which carries no instruction and so hands
the schedule back to the caller's `default_poll`. For the two cases that use it
today the difference is invisible, because neither runs past its list far
enough to matter. It is a trap for the next author, who will read the header
and not the body.

Neither script sets `set -eu`; that is the existing convention and is not
raised here.

**Evidence:** `tests/backends/logs-the-request-then-answers.sh:22-26` against
`tests/backends/answers-as-instructed.sh:30-33`.

**Disposition:** `fix-now`
**Response:**

Accepted. The header now states both differences from
`answers-as-instructed.sh`, numbered: the raw-request log, and that past the end
of its list this script answers `{"view":null}` — no instruction, so the
caller's own `default_poll` governs — where `answers-as-instructed.sh` answers
with a standing `45 minutes`. The behaviour is left as it is; it is the right
behaviour for the cases that use it, and it was only ever the header that was
wrong.

**Outcome:** `verified` — round 2. The header numbers both differences, and the second one
says what it costs: *"A case that runs past its list will see the two scripts
schedule differently."* The behaviour is unchanged, which is right — it was only
ever the header that was wrong.
### F-12 — `next_check_line` rounds half-up, so it can display an instant the host does not hold

**Severity:** nit
**Location:** `crates/goad/src/diagnostics.rs:276-283`

**Expected:** `draft-spec.md` R-6 and §6: what the host reports is *the
instruction it holds*, and the reported value must not read as a prediction of
when it will fire.

**Observed:** `jiff::Timestamp::round(Unit::Second)` defaults to half-expand,
so an instruction at `04:34:14.987Z` renders as `04:34:15Z` — an instant up to
500 ms **later** than the one the host holds, and one it never stored. The unit
test at `:453-459` pins that behaviour as intended. Truncation reports a value the
host actually holds and errs, if at all, in the direction that cannot be read
as promising a later check. The magnitude is trivial; the direction is the
point, given how much of §5.2 and §6 is spent on the line not overstating what
it knows.

**Evidence:** `diagnostics.rs:276-283`; `diagnostics.rs:453-459`;
`draft-spec.md` §6.

**Disposition:** `fix-now`
**Response:**

Accepted. The direction is the point, as the finding says. `next_check_line` now
truncates: `TimestampRound::new().smallest(Unit::Second).mode(RoundMode::Trunc)`
in place of the default half-expand. The reported value is one the host actually
holds, and it can no longer read as promising a later check.

Red first: the unit test was rewritten to expect `04:34:14Z` from
`04:34:14.987654321Z` and failed against the old rounding; renamed
`an_ordinary_instant_is_truncated_to_second_precision_never_rounded_up`, with
the reasoning in its doc comment. Recorded as D-24.

**Outcome:** `verified` — round 2. `TimestampRound::new().smallest(Unit::Second).mode(
RoundMode::Trunc)`. The unit test is renamed
`an_ordinary_instant_is_truncated_to_second_precision_never_rounded_up` and
expects `04:34:14Z` from `04:34:14.987654321Z`, with the direction argument in
its doc comment. The host can no longer display an instant it does not hold.
### F-13 — the stray `(F-1)` citation in `schedule.rs` predates the slice and is still there

**Severity:** nit
**Location:** `crates/goad-semantics/src/schedule.rs:327`

**Expected:** `docs/memory/cite-requirements-not-finding-ids.md`, as F-9.

**Observed:** *"fired is not one: it is consumed, and the default poll applies
(F-1)."* It sits inside the `#[cfg(test)]` module and predates this slice
(`notes.md` records it at `ad811c6d`). PHASE-06 named it and correctly declined
to fix it: `schedule.rs` is a source file and PHASE-06's Surfaces line is *"No
source file, no manifest, no markup."* Raised here so it has a disposition
rather than a mention. It is the same class as F-9 and should travel with it.

**Evidence:** `schedule.rs:327`; `notes.md` PHASE-06 Findings.

**Disposition:** `tolerated`
**Response:**

`docs/memory/cite-requirements-not-finding-ids.md` is explicit that slice 001's
existing citations are kept by user decision — *"they grep to the slice's own
ledgers, and a sweep would trade a pointer for a paraphrase"* — and this is one
of them: `git log -L` puts it at `ad811c6d`, slice 001's own audit, and it sits
inside `#[cfg(test)] mod tests`. PHASE-06 was right to decline it and right
about why.

So it is knowingly left, with the memory as the written rationale. This is the
disposition the audit proposed when it raised F-16: *leave the residue, fix the
class*. Removing it would be an edit to a slice-001 comment made under a slice
003 ledger, and the class fix — F-9 and F-16 — is what actually stops the
practice spreading.

**Outcome:** `verified` — round 2. The disposition is the one the memory itself writes:
slice 001's citations are kept by user decision, and `git log` puts this one in
slice 001's own audit. Leaving it while fixing the class (F-9, F-16) is the
right way round — a sweep of slice-001 comments under a slice-003 ledger would
be the larger liberty. The rationale is written down, which is what `tolerated`
requires.
### F-14 — the renderer-tier test for the next-check line asserts non-emptiness, not content

**Severity:** nit
**Location:** `crates/goad/tests/renderer/wiring.rs:230-236`

**Expected:** PHASE-04/VT-2's own doc comment: *"the standing next-check line
reaches its own window property."*

**Observed:** the assertion is `assert_ne!(window.get_next_check(), "")`. A
glass that wrote a constant non-empty string, or the wrong instant, passes.
The backend's instruction is `"90 minutes"` against a fixed `now`, so the exact
expected string is available and cheap: `next_check_line` is unit-tested for
format at `diagnostics.rs:453-459`, but nothing checks that the **value** reaching
the property is `Frame::next_check`. The empty-before-first-exchange half of
the test is exact and does its job; only the populated half is weak.

Related, and not worth its own id: `app.slint:73`'s `Text { text:
root.next-check; }` is unconditional, so an empty row is laid out before the
first exchange resolves anything.

**Evidence:** `wiring.rs:210-245`; `glass.rs:104-105`; `diagnostics.rs:453-459`.

**Disposition:** `fix-now`
**Response:**

Accepted. The populated half of the assertion is now exact:
`assert_eq!(window.get_next_check(), "next check (instructed):
2026-01-01T01:30:00Z")` — the harness's fixed `now` plus the backend's
instructed 90 minutes, rendered. A glass writing a constant non-empty string, or
the wrong instant, now fails.

The related observation is taken too: `app.slint`'s next-check row is now
`if root.next-check != "":`, so no empty row is laid out before the first
exchange resolves anything. The empty-before-first-exchange half of the test was
already exact and is unchanged.

**Outcome:** `verified` — round 2. The assertion is now
`assert_eq!(window.get_next_check(), "next check (instructed):
2026-01-01T01:30:00Z")` — the harness's fixed `now` plus the instructed 90
minutes, rendered. A glass writing a constant, or the wrong instant, fails. The
related observation is taken as well: the markup row is `if root.next-check !=
"":`, so no empty row is laid out before the first exchange.
### F-15 — `wait_for` carries no `#[must_use]` where its neighbours do

**Severity:** nit
**Location:** `crates/goad-semantics/src/schedule.rs:254`

**Expected:** consistency with the surrounding surface. `Stimulus::kind`,
`next_check_line`, `Controller::frame` and the rest of the crate's pure
returns-a-value functions all carry `#[must_use]`.

**Observed:** `wait_for` does not. `must_use_candidate` is `allow` workspace-wide
(`Cargo.toml:214`), so nothing catches it; discarding the wait is meaningless
and would be worth a compiler complaint. One attribute.

**Evidence:** `schedule.rs:247-257`; `wire.rs:50`; `diagnostics.rs:276`;
`Cargo.toml:214`.

**Disposition:** `fix-now`
**Response:**

Accepted; one attribute. `wait_for` now carries `#[must_use]`, like every other
pure returns-a-value function on the surface. Discarding a computed wait is
meaningless and now draws a compiler complaint.

**Outcome:** `verified` — round 2. `#[must_use]` is on `wait_for`.
### F-16 — the slice added eight `D-N` design-decision citations and one `F-N` to production source, which a standing user decision forbids

**Severity:** minor
**Raised by:** the audit, `audit.md` §Reconciliation. Appended to this ledger so
it travels with F-9 and F-13, which are the same class.
**Location:** production source across `crates/goad/src/` — the `D-N` citations
F-9 enumerates — plus the one `F-N` at `crates/goad/src/controller.rs:319`

**Expected:** `docs/memory/cite-requirements-not-finding-ids.md` records a
standing user decision, settled at slice 001's audit and *"in force from slice
002"*: a comment in `src/`, `tests/` or `examples/` cites a spec requirement, a
spec section, an ADR or the brief, and *"does not cite a slice-local review
finding (`F-N`) or design decision (`D-N`)"*. Slice 001's own citations are
tolerated by that decision; its closing instruction is *"Do not extend the
practice."*

**Observed:** production source carried **zero** `D-N` citations at the plan
commit `0b2e50f`. At `3777c22` it carries eight, plus one `F-N`. The practice
was extended, not held. The pre-existing `(F-1)` at
`crates/goad-semantics/src/schedule.rs:327` is slice-001 residue the memory
explicitly tolerates and is not part of this finding.

**Proposed disposition** (the auditor's, not binding on the responder): leave
the slice-001 residue where it is, and fix the class rather than the two
instances F-9 names — replace every `D-N` and `F-N` citation in production
source with the requirement it stands for (`SPEC-001/R-n`, or `SPEC-002/R-n` as
drafted in `draft-spec.md`) or with the reason stated in words. Where no
requirement states the reason, that is a gap in the spec, which the memory says
is not a licence to cite the ledger.

**Evidence:** `docs/memory/cite-requirements-not-finding-ids.md`;
`git diff 0b2e50f..3777c22 -- crates` for the eight added citations;
`audit.md` §Reconciliation.

**Disposition:** `fix-now`
**Response:**

Accepted, and the auditor's proposed disposition is taken as proposed: the
slice-001 residue at `schedule.rs:327` is left (F-13), and the class is fixed
rather than the instances F-9 names. The sweep, its scope and its boundary are
recorded in F-9's response; the two findings were repaired as one piece of work
because they are one class.

Count after the sweep: **zero** `D-N` or `F-N` citations added by this slice
remain in production source, and the repair round added none of its own.

**Outcome:** `verified` — round 2. Same evidence as F-9: the count of slice-003-added `D-N`
and `F-N` citations in production source is zero, and the repair round added
none of its own. Treating the two findings as one class rather than two lists
was the right call.
### F-17 — the item-scoped `#[cfg(test)]` skip counts braces in text that still contains string literals, so one unbalanced brace in a test module blinds the scan to the rest of the file

**Severity:** minor
**Location:** `crates/goad-boundary/tests/checks/structure.rs`, `Skip::consume` and `production_lines`

**Expected:** F-1's repair (D-23) exists because a whole-tail cut *"left 101 of
`state.rs`'s 217 lines and 48 of `process.rs`'s 342 unread"*, and because
*"nothing made that a checked fact."* The replacement should not reintroduce a
blinding of the same kind by another route.

**Observed:** it does, narrowly. `Skip::consume` counts `{` and `}` in
`code_of`'s output, and `code_of` strips comments but deliberately leaves
**string literal contents intact** (its own doc comment says so — the
vocabulary scan needs them). A single unbalanced brace inside a string or char
literal in an inline test module therefore leaves `depth` above zero for the
rest of the file, and every line after that module goes unread.

Run, not reasoned. Adding one line to the on-disk fixture —
`let _opening = "a JSON fragment: {";` inside its `mod tests` — makes
`production_after_an_inline_test_module_is_still_read` fail with *"the cut took
the whole tail"*; removing it restores green. The fixture caught it only
because it is the one file in the workspace with production code after its test
module. In a real subject file the same desync is silent.

**Neither new guard closes it.** The per-file check is `read > 0`, and the file
still yields every line *before* its test module. The directory floors are
1000 and 900 against measured 1907 and 1224 — roughly half the tree in slack —
so several hundred unread lines pass. Both guards catch a *total* blinding, and
this is a partial one.

Not live today: every inline test module in both subject directories runs to
the end of its file, so a desync changes nothing that is read. It is the same
shape of latent hazard the repair set out to remove, and the repair's own
argument — *"nothing made that a checked fact"* — applies to it unchanged. The
cheap closure is to count braces outside string literals; `scan.rs`'s `code_of`
already has the state machine that knows where they are.

**Evidence:** `scan.rs`'s `code_of` doc comment, *"The line with comments
removed, string literals intact"*; `Skip::consume`'s `code.matches('{')`;
the fixture probe above, run and reverted.

**Disposition:** `fix-now`
**Response:**

Accepted, and the probe reproduced exactly as described. The class fix is the
one the finding names: count braces over text with **literals cut as well as
comments**.

`scan.rs` now carries `code_without_literals`, `code_of`'s sibling over the
**same** state machine — `code_of` is `strip(line, Literals::Kept)` and the new
function is `strip(line, Literals::Cut)`, so there is no second machine to keep
in step. A cut literal becomes one space, never nothing, for the same reason
the block-comment cut does; a literal still open at end of line is cut from its
opening delimiter, which is where `code_of` deliberately returns the line
intact. The block-comment arm and the three literal arms now share one
`cut_out` helper rather than spelling the buffer move four times.
`production_lines` feeds `Skip::consume` the stripped text and still stores
`code_of`'s output for the matchers that read words, so no existing scan's
behaviour moves — the other 37 boundary checks pass unchanged.

The fixture carries the hazard: `production_after_tests.rs`'s test module now
holds an unbalanced `{` in a plain string, one in a raw string, and one in a
char literal. It failed before the change with *"the cut took the whole tail"*
and passes after. Four controls in `counting_itself` pin the strip directly —
braces in all three literal forms are gone, a real brace survives, an
unterminated literal is cut where `code_of` keeps it, and neighbours are not
joined across a cut.

**Outcome:** `verified` — round 3. `scan::code_without_literals` is a second entry into
`code_of`'s own state machine — one `strip` with a `Literals` flag — replacing
each string, raw string and char literal with a single space, and cutting an
unterminated one from its opening delimiter. `production_lines` counts braces
over that; `code_of` and every scan that reads words are untouched, which the
43 green boundary tests confirm.

Re-run of round 2's own probe, now across all three literal kinds: a plain
string, a raw string and a char literal each holding an open brace, all three
inside the fixture's `mod tests` — `production_after_an_inline_test_module_is_
still_read` **stays green**, where one plain string reded it in round 2. Five
new controls pin the strip itself, including that a real brace survives and
that a cut literal leaves a space rather than joining its neighbours.
### F-18 — instrument (b) now pins a number that changes for reasons AC-6 does not care about, including the wording of a user-facing error message

**Severity:** minor
**Location:** `crates/goad-boundary/tests/checks/structure.rs`, `the_identifier_resolve_is_confined_to_the_hosts_resolution_path`

**Expected:** AC-6 is *"the timer never resolves a schedule"*, and SPEC-002 R-2
is *"the host MUST NOT resolve a next check anywhere but the one resolution
SPEC-001/R-26 describes."* An instrument holds a property; the strength of a
boundary instrument is that a failure means the property broke.

**Observed:** the assertion is `found.len() == 9`, and the nine include things
R-2 has no view on. `error.rs:163` is a **string literal** — *"is not a
duration this host can resolve"* — counted because `code_of` keeps string
contents. Six more are `resolve_from` and `resolve_to`, counted because
`mentions` splits identifiers on `_`. So rewording a diagnostic message, or
renaming a private helper, turns the boundary suite red without anything having
resolved a schedule anywhere.

The response calls this a *change detector over the word* and states both
residues at the instrument, which is honest and is why this is minor rather
than a contest of F-1. The concern is the failure mode: an instrument that goes
red on a message reword invites the repair *bump the number*, and a number
bumped once is a number nobody reads the report for again. That is how an
instrument stops holding anything, and it is a slower version of the failure
F-1 described.

The count is genuinely load-bearing — it is what catches a third call site
added inside `host.rs`, which the file-set assertion alone would not. So the
repair is not to drop it but to make a drift legible: assert the count
alongside a per-file breakdown, or exclude string literals from this one
matcher so a message is not a boundary fact.

**Evidence:** `structure.rs`'s `assert_eq!(found.len(), 9, …)` and its file-set
assertion; `goad-shell/src/error.rs:163`; `scan.rs`'s `mentions` splitting on
non-alphanumerics; the instrument's own doc comment naming both residues.

**Disposition:** `fix-now`
**Response:**

Accepted. The failure mode is the point, and *bump the number* is exactly what
the previous shape invited.

Instrument (b) is now about the resolving **call**, which is what R-2 states.
`calls_resolve` matches `resolve(` with no identifier byte before it, over
`code_without_literals`, so `my_resolve(`, `resolve_from(`, `resolve_to(` and a
`resolve(` inside a message all fail to count. **Measured: 2 calls, both in
`host.rs`** — back to the number the criterion always claimed, now reached by a
matcher that is blind to import shape, which is the property the path substring
lacked.

The narrowing's one cost is closed rather than conceded: a second assertion
requires every production line naming the path `schedule::resolve` to be a call
line, so `let f = schedule::resolve;` followed by `f(…)` is caught even though
no call form appears. A rename-import stays outside both, as §5.5 I-1a already
concedes.

The count stays load-bearing where the finding says it must: a third call added
inside `host.rs` passes the file-set check and is caught by the count alone.
The test keeps its original name, `schedule_resolve_is_called_only_from_host`,
which is true again.

Three probes, run and reverted. A brace-grouped `use` plus a bare `resolve(…)`
in `state.rs` takes the count to 3 and reds it. `let _f =
goad_semantics::schedule::resolve;` reds the second assertion with *"named
without being called"*. Rewording `error.rs`'s *"can resolve"* message to *"can
understand"* — the reword that used to red the suite — leaves all 43 checks
green. D-23 restated; the walk's helpers are now one function with three
predicates rather than two near-copies.

**Outcome:** `verified` — round 3, and the repair is better than the finding asked for.
Instrument (b) counts the **call form**: `resolve(` with no identifier byte
before it, over `code_without_literals`. Measured 2, both `host.rs`. The
message string in `error.rs`, `resolve_from`, `resolve_to` and a bare `use`
line are all out, and a control test pins each of those five shapes as
not-a-call.

The narrowing a call matcher costs — the path taken as a value and called
elsewhere — is closed rather than conceded: a second assertion requires every
production line naming `schedule::resolve` to also be a call line.

Three break-and-reverts. A third resolving call spelled bare after
`use goad_semantics::schedule::{parse, resolve};` → red at 3 vs 2, so the
import-shape blindness F-1 named is still closed. `let via_value =
schedule::resolve;` replacing a call → red at 1 vs 2. And the case that tests
the second assertion on its own: `let _f = schedule::resolve;` added
**alongside** both calls, leaving the count at 2 → red with *"`schedule::
resolve` is named without being called"*, naming `host.rs:128`. That assertion
is live, not decorative.

The rename-import residue (`use … as r;`) remains, as §5.5 I-1a already
concedes and the instrument's doc comment repeats.
### F-19 — `slice-003.md`'s AC-6 still states the instrument that was replaced, and is checked `[x]`

**Severity:** minor
**Location:** `docs/slices/003/slice-003.md`, AC-6

**Expected:** an acceptance criterion is what the slice is judged against, so it
is the one document that must be true about the tree at close. `design.md`
§9's AC-6 row and `draft-spec.md` §7's R-2 row were both rewritten for the new
instrument in this round, which shows the sweep was understood.

**Observed:** AC-6 was not. It still reads *"**(b)** the path
`schedule::resolve` occurs exactly twice in `crates/goad-shell`'s production
code, both in `host.rs`. Both counts are measured against the tree, not assumed:
0 over 12 files and 2 over 8 respectively"*, and its preamble still says the
scans read *"production code only — cut at each file's own `#[cfg(test)]`"*.
Three statements, all now false: the instrument matches the identifier not the
path, the measured figure is 9 over 3 files, and the walk skips the item rather
than cutting the file. The criterion is checked `[x]` and marked **Discharged
by:** PHASE-04/VT-4.

A reader auditing AC-6 against the tree finds a test whose name, subject and
number all differ from the criterion that claims it. `audit.md:152` carries the
old test name too, though that file is the audit's to correct.

**Evidence:** `slice-003.md` AC-6 against `structure.rs`'s
`the_identifier_resolve_is_confined_to_the_hosts_resolution_path`;
`design.md:682` and `draft-spec.md:189`, both updated, for contrast.

**Disposition:** `fix-now`
**Response:**

Accepted. AC-6 is rewritten: instrument (b) is *"`schedule::resolve` is
**called** from exactly two places … and is nowhere taken as a value without
being called"*, the measured figures are 0 over 12 files and 2 calls over 8, and
the preamble says the walk skips each `#[cfg(test)]` **item** with comments and
literal contents handled by `scan.rs`. Its vacuity clause now names all three
guards rather than the file count alone, and PHASE-04/VT-4 is described as *the
call count*.

Swept with it: `design.md` §9's AC-6 row and D-23, and `draft-spec.md` §7's R-2
row, all of which named the identifier change-detector this round replaced.
`audit.md:152` is left alone — it is the audit's own record and this session
does not write it; the round-2 verifier should expect to correct it there.

**Outcome:** `verified` — round 3. `slice-003.md` AC-6 now describes the instrument that
exists: the item-scoped skip with literals handled by `scan.rs`, absence over
stratum 3, and *"`schedule::resolve` is **called** from exactly two places …
and is nowhere taken as a value without being called"*, measured 0 over 12 and
2 calls over 8. It also now states the vacuity guards. The chain is consistent
end to end — `design.md` §9's AC-6 row and `draft-spec.md` §7's R-2 row carry
the same claim and the same test name.
### F-20 — `design.md` §5.5 E-6 still says the far-future case is not mitigated, which D-20 mitigated

**Severity:** minor
**Location:** `docs/slices/003/design.md` §5.5, E-6

**Expected:** E-6 is the section a reader goes to for *"an instruction further
out than a timer can hold"*, and D-20's own log column cites E-6 as what it
answers. F-7's repair established that a design paragraph left false about the
tree is worth fixing.

**Observed:** E-6 is unchanged. It still explains the case entirely through
tokio's own clamp — *"tokio clamps a deadline at `MAX_SAFE_MILLIS_DURATION`,
roughly two years … so the host idles at roughly two-year intervals"* — and
closes *"not mitigated, because there is nothing to mitigate."* Since D-20 there
is a host-side clamp in front of tokio's: `deadline_after` bounds the wait at
`LONGEST_WAIT`, 365 days, and falls back to `now`. On Linux that branch is
unreachable and E-6's two-year figure still describes what happens; on a
platform where `Instant`'s representation is narrower it is one year, and the
sentence saying nothing is mitigated is wrong on every platform.

Only D-20's table row records the clamp. A reader looking up the far-future case
where the design files it will not find it.

**Evidence:** `design.md` §5.5 E-6 against `controller.rs`'s `deadline_after`
and `LONGEST_WAIT`; `design.md` D-20, whose log column is *"E-6"*.

**Disposition:** `fix-now`
**Response:**

Accepted. E-6 conflated two clamps, and said the case was unmitigated because
only one of them existed when it was written.

It now separates them. **The host's own (D-20)**: `deadline_after` uses
`checked_add`, clamps an overflowing wait to `LONGEST_WAIT` and falls back to
`now` — this is the mitigation, and it is about the `Add`, which panics on
overflow rather than returning an `Option`, and which is reachable from backend
input. **tokio's, further in**: a clamp applied to a deadline it is *given*,
never to the `Add` that produces one, which is why it was never the answer to
the first case. The Linux-versus-narrower-platform difference is stated, and
the closing sentence now reads *mitigated, by D-20*, saying plainly what the
old text got wrong and why.

**Outcome:** `verified` — round 3. E-6 is rewritten around **two** clamps, named and kept
apart: the host's own `deadline_after` with `LONGEST_WAIT`, which is about the
`Add` and is the mitigation SPEC-001/R-45 requires; and tokio's, further in,
which is about a deadline it is given. It states that on Linux the host clamp
is unreachable and the timing is unchanged, and that on a narrower platform the
host idles at one-year intervals rather than panicking. The sentence saying
nothing is mitigated is gone.
### F-21 — widening the liveness bound removed the guarantee that one anti-spin window closes before the floor expires, and §9 still claims anti-spin rows cannot fail under load

**Severity:** minor
**Location:** `crates/goad/tests/renderer/scheduling.rs`, `a_person_acting_mid_cadence_does_not_clear_the_floor`; `docs/slices/003/design.md` §9 and R1

**Expected:** `design.md` §9 states, unqualified: *"Anti-spin and anti-fire rows
cannot fail under load, because load can only reduce a count or delay a
firing."* R1 repeats it. F-7's repair widened the liveness bound *for* load
safety.

**Observed:** for one test the claim no longer holds, and the widening is why.
Its shape is `until(bound, >= 2)` → send → `until(bound, >= 3)` →
`sleep(500 ms)` → `assert_eq!(count, 3)`. The floor expires three seconds after
the *second* invocation's firing, but the window opens after the *third*
invocation returns. At a two-second bound the window provably closed first
(2 s + 0.5 s < 3 s). At five seconds it does not (5 s + 0.5 s > 3 s): a loaded
gate that delays the person's exchange past ~2.5 s lets the floor expire inside
the window, a fourth invocation lands, and the count assertion fails.

Every other anti-spin window is anchored to the same event the floor is
measured from, so all of them are still one-sided. This one is anchored to a
later event, which is what makes the bound load-bearing for its correctness.

**It is not a new red.** In that band the old bound would have panicked on the
liveness wait first, so the test was going to fail either way. What changes is
*which* assertion reports and what it says: the message is *"R-4: the person's
exchange must not have cleared or reset the floor"*, which accuses the
production floor of a defect when the cause is machine load. A misdirecting
failure on a gate is how a real floor bug gets dismissed as flake.

Cheap either way: anchor the window to the second invocation, shorten it, or
give that one `until` a bound that keeps the old inequality.

**Evidence:** `scheduling.rs`'s `a_person_acting_mid_cadence_does_not_clear_the_floor`;
`design.md:702` and `design.md:665` (R1); `controller.rs`'s `floor_until` write
site.

**Disposition:** `fix-now`
**Response:**

Accepted, and the finding is right that the misdirection is the real cost: a
message accusing the production floor of a defect the machine caused is how a
real floor bug gets dismissed as flake.

The third option is taken, and made checkable. VT-6's two waits take their own
`FLOOR_SAFE_BOUND` (2 s) rather than the workspace's `LIVENESS_BOUND` (5 s),
its window is the named `ANTI_SPIN_WINDOW` (500 ms), and

    const _: () = assert!(FLOOR_SAFE_BOUND.as_millis() + ANTI_SPIN_WINDOW.as_millis() < FLOOR_MILLIS, …);

sits beside them. Widening the shared bound broke this inequality silently
once; it now fails the build with *"VT-6's window must close before the floor
expires, or a loaded gate reports load as a floor defect"* — verified by
setting the bound back to 5 s and watching the compile fail, then reverting.
The doc comment says why this test is exempt from the widening, so the
exemption reads as a decision rather than an oversight.

`design.md` §9's unqualified *"anti-spin and anti-fire rows cannot fail under
load"* now carries the exception as its own paragraph, R1 names it, and the
margin table gains a VT-6 row whose margin column is the inequality rather than
a ratio. Every other window in the module opens at the same event its floor is
measured from, so the general argument still holds for all of them.

**Outcome:** `verified` — round 3, and the guarantee is now the compiler's rather than a
reader's. VT-6 takes `FLOOR_SAFE_BOUND` (2 s) for both its liveness waits and
`ANTI_SPIN_WINDOW` (500 ms) for its window, with
`const _: () = assert!(FLOOR_SAFE_BOUND + ANTI_SPIN_WINDOW < FLOOR_MILLIS)`.
Break-and-revert: raising `FLOOR_SAFE_BOUND` to the shared five seconds fails
the build — *"VT-6's window must close before the floor expires, or a loaded
gate reports load as a floor defect"* — so the silent widening that caused this
finding cannot recur.

`design.md` §9 no longer claims the anti-spin argument covers every row: a
paragraph names VT-6 as the exception, says why its window opens later than the
floor's origin, and the margin table gains a row whose margin column is the
inequality itself.

One residue, noted rather than raised: `FLOOR_MILLIS` is a hand-mirror of the
private `controller::MINIMUM_SPACING`, and its own comment says the mirror is
*"checked by nothing but this comment"*. Changing the floor would leave the
compile-time assertion passing against a stale three seconds. It is a smaller
version of what this finding was about, one level up.
### F-22 — the far-future test synchronises on the start of an exchange and asserts a fact that only exists after it ends, so `just check` is intermittently red

**Severity:** blocker
**Location:** `crates/goad/tests/renderer/scheduling.rs`, `an_instruction_at_the_far_edge_of_time_arms_the_sleep_without_panicking`

**Expected:** `slice-003.md` AC-12 — *"`just check` exits 0 … and no test whose
passing depends on machine load. A timing-sensitive test that can be flaky under
a loaded gate is a design defect, not a tolerated cost."* POL-001 forbids
weakening the gate, and an intermittently red gate is the weakening it is most
worth preventing.

**Observed:** the test is red about two runs in three when the `renderer` target
runs in full, and green every time it runs alone.

```
test scheduling::an_instruction_at_the_far_edge_of_time_arms_the_sleep_without_panicking ... FAILED
assertion `left == right` failed: SPEC-001/R-28: the instruction is stored and
reported as given, whatever the timer does with it
  left: None
 right: Some(Timestamp(9999-12-01T00:00:00Z))
```

Three consecutive `cargo test -p goad --test renderer` runs on an otherwise idle
machine: **red, green, red.** Filtered to itself: green. `cargo test
--workspace` is red on the same assertion.

**The race, and it is not a timing margin.** The test waits with
`until(LIVENESS_BOUND, || invocations(&log) >= 1)` and then calls
`stopper.stop()` at once. `answers-as-instructed.sh` writes its `invoked` line
**before** `cat >/dev/null` — that is, before it has even read the request — so
the log reaches one while the exchange is still in flight. The stop then wins
the second `select!`, `call` is dropped exactly as the cancel arm's own comment
says, `absorb` never runs, and `Controller.next_check` is still `None`. Whether
the exchange or the stop wins is decided by whether a `bash` subprocess finishes
first, which is precisely what a loaded gate changes. Widening the bound cannot
help: the bound is not what the race is against.

**Why the sibling tests do not have it.** Every other test in the module that
reads `frame().next_check` waits for an event that *implies* an absorb already
happened — a second invocation, a view on the window, a tray string. This one
waits for the first invocation, which implies only that the exchange started.

**The fix is one line and does not weaken anything.** The production glass
writes the rendered next check to the window on every `present`, so
`until(LIVENESS_BOUND, || window.get_next_check().contains("9999-12-01"))`
synchronises on the absorb itself rather than on the exchange's start, and keeps
the assertion the test exists to make.

Raised at `blocker` on the severity table's own terms — *"must not proceed"* —
because it is not a latent hazard but a gate that is red now, and because the
slice cannot claim AC-12 while it is. It arrived with F-5's repair in round 2
and this review did not catch it then: two full-target runs happened to land
green.

**Evidence:** the three full-target runs above; `scheduling.rs:836-850`;
`tests/backends/answers-as-instructed.sh`'s `printf 'invoked\n' >>"$log"`
preceding its `cat`; `controller.rs`'s second `select!`, *"`call` is DROPPED
here"*. Confirmed against the owner's own file — checksummed against a copy
taken before any probe in this round, and `FLOOR_SAFE_BOUND` back at two
seconds.

**Disposition:** `fix-now`
**Response:**

Accepted without qualification. Mine, from round 1's F-5 repair, and a blocker
correctly: a gate that reds three runs in five is worse than no test.

Reproduced before repairing — 3 of 5 full renderer runs red, 12 of 12 green in
isolation, which is why it survived the round-1 checks. `answers-as-instructed.sh`
appends its log line **before** reading the request, so `invocations(&log) >= n`
proves only that the nth exchange *began*. `Cancel` is the first arm of the
second `select!`, so a stop on that signal drops the call in flight, `absorb`
never runs, and `next_check` is `None`.

**The class, swept.** The observable that proves absorption is the rendered
next-check line: `glass.present(controller.frame())` runs at the top of the
iteration *after* `absorb`, so it cannot be read early. `absorbed_line(rfc3339)`
names it, with the reasoning in its doc comment. Six sites in
`scheduling.rs` now wait on it — the far-edge case, and five that had been
resting on a fixed sleep between the log line and an assertion about what the
exchange resolved. One of those, VT-8's *"short settle so the loop is genuinely
back at that `select!`"*, was a 20 ms delay used as synchronisation, which is
the same defect at a different magnitude.

**Deliberately left.** `event_loop_schedule` asserts only facts about the log
itself, and says so in a comment. `wiring.rs`'s two `@hang` cases want the
exchange **in flight** when the stop lands, so the log line is the correct
observable there — the opposite requirement, not the same shape.

**Ten consecutive full renderer runs: 10 green, 138/138 each.**

**Outcome:** `verified` — round 4. Swept as a class, not patched as an instance, and the
exceptions are right.

**The observable is sound.** `absorbed_line` renders through the production
`diagnostics::next_check_line`, and the glass writes that property during the
`present` at the top of the iteration *after* `absorb`. It cannot be read early,
which is exactly the property the log line lacked.

**The sweep is complete, checked by sorting rather than by reading the
response.** Every test in the module was classified by whether it asserts a
post-absorb fact — `frame().next_check`, `diagnostics.lines()`, or
`CLOCK_READS` — and by what it waits on before `stopper.stop()`. All five that
do now wait on `absorbed_line`; the sixth site is VT-8, where a 20 ms sleep had
been standing in for synchronisation. The eight that still wait on the log
assert only log facts — invocation counts and `event.kind` — which a log line
does prove. No test asserts a post-absorb fact on a log-line wait.

**Both exceptions hold.** `event_loop_schedule` asserts `Ending::Stopped`, which
its watcher guarantees, and an invocation count, which is a log fact; it has no
post-absorb assertion to lose. `wiring.rs`'s two `@hang` cases need the exchange
**in flight** when the stop lands, so the log line — which fires before the
request is read — is the correct observable there and `absorbed_line` would be
the wrong one. The opposite requirement, as the response says.

**Re-run.** Six consecutive full `renderer` runs: **6 green, 138/138 each**,
2.64–2.66 s. Three `cargo test --workspace` runs: green, 16 result blocks each,
no failures. Against three runs before the repair that went red, green, red on
this assertion.
## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

**Done. Twenty-two findings, all `verified`, no `blocker` outstanding.
Outstanding: 0.**

**What the review changed.** Two majors in round 1, at the root rather than at
the symptom. AC-6's stratum 2 instrument was a path substring that a
brace-grouped `use` walked straight past — the defeat the design had already
identified and closed on the sibling instrument. It is now a matcher for the
resolving **call** itself: two, both in `host.rs`, with a second assertion
catching the path taken as a value, and with neither an error message's wording
nor a private helper's name able to red it. And canon-delta's R-56 no longer
closes the set of event kinds against every future host; it names three, fixes
their meaning, leaves the set open, and adds the backend tolerance clause
SPEC-001 never had.

Below those: `serve` has one refusal site where it had three, two of which no
input could reach, and the fold is tight enough that deleting the condition
fails to compile. The re-arm's arithmetic is checked and total, with a clamp and
a floor-backed fallback, so a year-9999 instruction cannot panic the host. The
poll loop exists once. The production-topology test fails in five seconds
instead of wedging the gate. Brace counting runs over text with literals
stripped, so one `{` in a test module's string can no longer blind the scan to
the rest of a file. The next-check line truncates rather than rounding up, and
its test asserts the rendered instant. Every `D-N` and `F-N` citation the slice
put into production source is gone.

**What the review confirmed.** The loop held under four readings: the always-armed
invariant, the single floor write site, cancellation's precedence, and R-9 by
construction. F-6's deferral is a scope boundary — both repairs would have the
host judge that a view is worth protecting, which the first invariant puts in
the backend — not a fix postponed for being large.

**What the last round was actually about.** Round 3 found the gate red two runs
in three, from a test the review's own earlier repair had introduced: it waited
for a backend log line written *before* the request was read, then stopped the
loop, so the cancel arm dropped the exchange and the assertion read `None`. That
is now swept as a class. Six tests wait on the rendered next-check line, which
only exists after `absorb`; the tests that still wait on the log assert only log
facts; and the two `@hang` cases keep the log line because they need the
exchange in flight, which is the opposite requirement. Six consecutive full
renderer runs green, three workspace runs green.

**The risks knowingly left standing.** Three, all named and none in the loop. A
rename-import (`use … as r;`) in stratum 2 is outside both AC-6 instruments, as
§5.5 I-1a concedes. `FLOOR_MILLIS` in the renderer tier is a hand-copy of the
private `controller::MINIMUM_SPACING`, so changing the floor leaves the
compile-time assertion that protects VT-6 passing against a stale number. And a
scheduled evaluation can still supersede a view a person is mid-answering,
bounded only by the three-second floor — documented in D-22, `draft-spec.md` §5
and OQ-4, and carried as a follow-up because both candidate repairs would put
domain judgement in the host.

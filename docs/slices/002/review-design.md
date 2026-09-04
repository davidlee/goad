# Review — design — Slice 002

**Subject:** design — `docs/slices/002/design.md`, with `slice-002.md` and
`canon-delta.md` in scope as its companions.
**Reviewer:** rounds 1 and 2 — codex, `gpt-5.6-sol`, read-only, briefed for
implementation feasibility rather than intent.
**Opened:** 2026-09-05
**State:** open — twenty-eight findings raised over three rounds, all currently
`verified`. It stays open because round 3's repairs have not themselves been
reviewed. Round 3 exists because round 2's had not been, and round 2 because
round 1's had not been; the pattern is the point, and the ledger says so rather
than declaring itself finished.

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

**Round 1** — 2026-09-05 — the design's *feasibility*, not its intent.

Intent was settled before the review opened (`design-log.md` 2026-09-05, three
user decisions), so the reviewer was pointed away from it and at the seam where
a design most often fails here: the distance between what a document says the
code does and what the code does. `design.md` was written partly from
`research.md`'s summaries rather than from the source, and summaries lose exactly
the details an implementing agent trips over.

Specifically probed: every type and field name the design borrows from slice 001;
whether the proposed `Presentation` shape is constructible through the accessors
`canonical.rs` actually grants; whether the runtime ownership story is
expressible in Rust given `&mut self` on both `Host` entry points and `!Send`
Slint handles; whether the chosen test tier can reach what the acceptance
criteria demand of it; and what the split does to a gate the design describes in
pre-split terms.

The bodies were expected in the runtime seam and in the gate. Both delivered.

**Reviewer:** codex `gpt-5.6-sol`, read-only, given the design, the slice doc,
`research.md`, SPEC-001, both ADRs, and `src/`.

**Round 2** — 2026-09-05 — the *repairs*, on the same terms.

Same reviewer, same materials, pointed at what round 1 changed rather than at the
design as a whole. The question was narrow and the right one: is each repair
something an agent could type? A repair that restates a problem in better prose
is worse than the original defect, because it looks done.

It reopened **seven of twelve** and raised **seven more**. The pattern is
different from round 1's and worth naming: round 1 found places where the design
had been written from summaries rather than from source. Round 2 found places
where the *repair* had been written at the level of intent rather than of
signatures — a reducer table that reads correct and is not disjoint, a shutdown
path that cannot be received, a diagnostic reducer whose signature cannot see the
field its contract promises, a case table handed to the plan.

Closed at round 2: F-2, F-3, F-10, F-11, F-12.

**Round 3** — 2026-09-05 — the round-2 repairs, on the same terms.

Same reviewer, same materials, pointed at what round 2 changed, and additionally
at the three things round 2 had not reached: the repaired `canon-delta.md`, the
rewritten acceptance criteria, and §9 item 12 read against the fixtures it names.
That reading list was set by round 2's own synthesis, which named them as what it
had not reached.

It **held ten** of the fourteen round-2 repairs, reopened **four** under their
original ids — F-6, F-9, F-16, F-17 — and raised **seven** more, F-20…F-26. Two more — **F-27** and **F-28** — were raised by the responder while repairing
F-22 and F-17, by building the things rather than reasoning about them. Both are
recorded under the raiser's hat with their measurements, because a defect found
while repairing is still a defect, and both were in *this round's own new text*.

The pattern shifted again, and the shift is worth naming. Round 1 found a design
written from summaries rather than from source. Round 2 found repairs written at
the level of intent rather than of signatures. Round 3 found something narrower
and more specific: **repairs that are correct where they are written and
contradict something else that was left alone.** A-5 still pointing at the crate
override D8 had just forbidden; a `Closed` arm still quitting a loop the same
paragraph says has exactly one quit; an `engaged` flag with a setter and no
clearer; a slice non-goal that says slice 003 owns the clock beside a design
that adds one. Every one of those is a *seam* between a repaired passage and an
unrepaired neighbour — which is the failure mode a repair round creates, and the
reason a fourth round is owed rather than assumed.

Closed at round 3: nothing new closes, because every round-2 finding that was
held was already `verified`; what closes is the *class* — see the Synthesis.

## Findings

Severity is the raiser's, at raise time. A finding reopened in a later round
keeps its id and its severity; its earlier outcome became `contested` and the
fresh disposition and outcome are appended below it.

| id | severity | raised | r1 disp | r1 out | r2 disp | r2 out | r3 disp | r3 out |
|----|----------|--------|---------|--------|---------|--------|---------|--------|
| F-1 | blocker | r1 | fix-now | **contested** | fix-now | verified | — | held |
| F-2 | blocker | r1 | fix-now | verified | — | — | — | — |
| F-3 | blocker | r1 | fix-now | verified | — | — | — | — |
| F-4 | blocker | r1 | fix-now | **contested** | fix-now | verified | — | held |
| F-5 | major | r1 | fix-now | **contested** | fix-now | verified | — | held |
| F-6 | major | r1 | fix-now | **contested** | fix-now | **contested** | fix-now | verified |
| F-7 | major | r1 | fix-now | **contested** | fix-now | verified | — | held |
| F-8 | major | r1 | fix-now | **contested** | fix-now | verified | — | held |
| F-9 | major | r1 | fix-now | **contested** | fix-now | **contested** | fix-now | verified |
| F-10 | major | r1 | fix-now | verified | — | — | — | — |
| F-11 | minor | r1 | fix-now | verified | — | — | — | — |
| F-12 | minor | r1 | fix-now | verified | — | — | — | — |
| F-13 | blocker | r2 | — | — | fix-now | verified | — | held |
| F-14 | major | r2 | — | — | doc-wrong | verified | — | held |
| F-15 | major | r2 | — | — | fix-now | verified | — | held |
| F-16 | major | r2 | — | — | fix-now | **contested** | fix-now | verified |
| F-17 | major | r2 | — | — | fix-now | **contested** | fix-now | verified |
| F-18 | major | r2 | — | — | fix-now | verified | — | held |
| F-19 | minor | r2 | — | — | fix-now | verified | — | held |
| F-20 | major | r3 | — | — | — | — | fix-now | verified |
| F-21 | major | r3 | — | — | — | — | fix-now | verified |
| F-22 | major | r3 | — | — | — | — | fix-now | verified |
| F-23 | major | r3 | — | — | — | — | doc-wrong | verified |
| F-24 | major | r3 | — | — | — | — | fix-now | verified |
| F-25 | minor | r3 | — | — | — | — | fix-now | verified |
| F-26 | minor | r3 | — | — | — | — | fix-now | verified |
| F-27 | major | r3 | — | — | — | — | fix-now | verified |
| F-28 | major | r3 | — | — | — | — | fix-now | verified |

**held** = round 3 audited that repair against the code and did not reopen it.
The finding's terminal outcome stays the one in its own round's column; "held"
is evidence about the repair, not a fresh disposition.

### F-1 — The outcome reducer destroys a still-live interaction

**Severity:** blocker
**Location:** `design.md` §5.4 (the sequence diagram), §5.5 I-4

**Expected:** the renderer's response to an `Outcome` follows `Host`'s state
machine.
**Observed:** the design treats every `Outcome { view: None }` as "nothing shown"
and hides the window after every `respond`. `Host` distinguishes four cases, not
two, and the design would hide the only UI able to answer an interaction `Host`
still considers live. It also contradicts AC-7's "no failure closes the window".

**Evidence:** `WhenNothingToShow` is the one place `evaluate` and `respond`
differ once the bytes are in — evaluate leaves the interaction outstanding,
respond closes it (`src/shell/host.rs:98-108`, `:139`, `:169`). A failed
`respond` returns through `no_action` without closing state (`:152`, `:158`,
`:191`); state closes only when a successful normalized response reaches `accept`
with `CloseOutstanding` (`:208`, `:231`, `:238`). `State::verify` leaves the
outstanding interaction intact after a rejection (`src/shell/state.rs:89`,
`:102`).

**Disposition:** fix-now
**Response:** the design was wrong, and wrong in the way that matters — it
described the host's behaviour from the brief rather than from the code. §5.4
now carries an action-sensitive reducer over `(command, previous presentation,
Outcome)` with all four transitions named, and §9 gains the four-transition test
including a failed `respond` followed by a successful retry on the same
`ViewId`.

**Outcome:** verified

### F-2 — There is no constructible startup or tray evaluation

**Severity:** blocker
**Location:** `design.md` §6 OQ-7, §5.4

**Expected:** OQ-7's answer lets an agent write the first call.
**Observed:** "process start" and "a tray action" name stimuli, not values.
Construction needs a `Config`, a backend and a `Timestamp`; evaluation needs a
complete `Event`. The slice excludes the clock as slice 003's, yet cannot make
its first call without reading time, and the binary has no stated way to find its
configuration. An implementing agent must invent clock policy, event vocabulary
and config discovery — decisions the design claims are closed.

**Evidence:** `Host::new` requires `Config`, a backend and `Timestamp`
(`src/shell/host.rs:127`); `evaluate` requires `Timestamp` and `Event` (`:137`).
`Event` requires `source`, `kind`, `timestamp`, `data`
(`src/semantics/protocol/canonical.rs:489-495`). `Config::load` requires a
caller-supplied path (`src/shell/config.rs:115`).

**Disposition:** fix-now
**Response:** the real gap was conflating *reading a clock* with *owning the
schedule*. Slice 003 owns the schedule; stratum 3 has always had to timestamp its
own calls, and saying so does not give this slice a timer. §6 OQ-7 now specifies
the config path source, a stratum 3 wall-clock adapter explicitly distinct from
slice 003's loop, both event envelopes verbatim, and the startup failure
behaviour. D15 records the distinction so slice 003 does not read it as a timer
already built.

**Outcome:** verified

### F-3 — `OptionRow` is two incompatible Rust types

**Severity:** blocker
**Location:** `design.md` §5.2, §5.3, D4, D5

**Expected:** one row type, constructible and passable to the markup.
**Observed:** the mapper declares a hand-written `OptionRow { id: OptionId, label:
String }` while the `.slint` property `[OptionRow]` generates a distinct Rust
struct over `SharedString`. `VecModel<view_model::OptionRow>` cannot be handed to
a setter expecting `ModelRc<generated::OptionRow>`. D4 forbids the mapper
returning generated types; D5 reasons as though the row *is* the generated one.

**Evidence:** `OptionId` is an opaque newtype exposing `as_str` and `Clone`
(`src/semantics/protocol/canonical.rs:57-67`). A `.slint` struct generates a
distinct Rust struct, `string` maps to `SharedString`, arrays to `ModelRc<T>`
(`research.md:562`); the verified model idiom uses the generated row
(`research.md:568`).

**Disposition:** fix-now
**Response:** two layers, named separately, with an explicit adapter between
them. `PresentationOption { id: OptionId, label: String }` is the canonical
retained row; the generated `OptionRow { id: SharedString, label: SharedString }`
is the display row; the `VecModel` holds the generated one. The canonical rows
are retained by the host task so that a `chosen(string)` callback resolves to an
`OptionId` the host already owns and never mints (F-5's channel is where that
resolution happens). D4 and D5 are restated against the two layers.

**Outcome:** verified

### F-4 — The chosen test tier cannot test the shutdown mechanism

**Severity:** blocker
**Location:** `design.md` D9, §5.4, §9 item 12

**Expected:** validation item 12 proves AC-12.
**Observed:** D9's `block_on` tier is right for item 11 and cannot reach item 12.
Production shutdown depends on `spawn_local`, a live loop, `on_close_requested`,
and quitting only after the owner task ends. `init_no_event_loop()` rejects
`spawn_local`, so the cheap target bypasses the mechanism AC-12 exists to prove.
"Drains" is also the wrong word: the verified spike *cancelled and dropped* the
in-flight future; it did not await a returned `Outcome`.

**Evidence:** `spawn_local` and `invoke_from_event_loop` both return
`NoEventLoopProvider` in the cheap tier (`research.md:888`); event-loop wiring
needs `init_integration_test_*`, once per process, hence one target per case
(`research.md:898`). The shutdown spike dropped the exchange future and closed in
17 ms rather than awaiting the 2 s timeout (`research.md:499`, `:504`). The
transport relies on `kill_on_drop` for cancellation
(`src/shell/backend/process.rs:70`).

**Disposition:** fix-now
**Response:** D9 becomes two tiers with a stated boundary, not one. The
`block_on` target keeps ordinary Host-to-view wiring; one dedicated event-loop
target carries the shutdown seam and nothing else, so the per-target cost is paid
once rather than per test. §5.4 and AC-12 now say *cancel*, not *drain*, and name
the observable: the in-flight exchange is dropped, the owner task ends, and the
child is reaped as far as the existing cancellation contract permits.

**Outcome:** verified

### F-5 — The runtime ownership story omits its only viable bridge

**Severity:** major
**Location:** `design.md` §5.3, §5.4

**Expected:** the ownership story is expressible in Rust.
**Observed:** the lifecycle diagram sends "stimulus" and an `OptionId` straight to
the owner task with no seam defined. A Slint callback is a `'static` synchronous
closure: it cannot capture `&mut Host`, build a future borrowing it, and leave
that future pending after it returns; nor can several callbacks own one `Host`.
The design credits research for this architecture while omitting the component
that made it work.

**Evidence:** both entry points borrow the host mutably across the exchange
(`src/shell/host.rs:137`, `:152`, `:179`); the transport seam enforces one
mutable exchange (`src/shell/backend/transport.rs:18`, `:30`). The verified
runtime has the owner task consuming a `tokio::sync::mpsc` receiver with
callbacks holding sender clones (`research.md:471`).

**Disposition:** fix-now
**Response:** the omission was mine — the channel was in the research and did not
survive into the design, which is exactly the failure this review was briefed to
find. §5.3 now specifies a bounded `mpsc` carrying `Evaluate(Stimulus)`,
`Choose(String)` and `Shutdown`; the single `spawn_local` future owns `Host`, the
retained canonical presentation and the receiver; callbacks own sender clones
only and mint nothing. Full-channel and closed-channel behaviour are stated
rather than left to drop UI actions silently.

**Outcome:** verified

### F-6 — The split and the gate have mutually incompatible definitions

**Severity:** major
**Location:** `design.md` §5.1, §5.6, §9; `slice-002.md` AC-1, AC-3

**Expected:** the design describes the gate the split produces.
**Observed:** the design demands "both feature columns" throughout, and the
measured split *retires the feature that creates them*. It also claims direction
becomes compiler-enforced without qualification, which is false at the manifest
level.

**Evidence:** the two columns exist because of the optional `shell` feature
(`Cargo.toml:25`, `:38`) enforced by a seven-command gate (`justfile:17`, `:55`).
Post-split there is no `shell` feature, `--no-default-features` stops being a
distinct column, the gate is five commands, and `cargo test -p goad-semantics` is
its successor — deliberately kept *out* of the gate as a diagnostic
(`research.md:817-835`). Adding `tokio.workspace = true` to the stratum 1
manifest still leaves `cargo build --workspace` at exit 0 (`research.md:806`).

**Disposition:** fix-now
**Response:** the design was describing a gate that will not exist. §5.6 and §9
now carry the five commands verbatim, `cargo test -p goad-semantics` is promoted
*into* the gate rather than kept beside it — a purity claim that is not run is
not a claim — and the compiler-enforcement claim is narrowed to crate edges.
The residue is real and now named: two of `boundary.rs`'s three forbidden tokens
become compile errors, and the third moves from source into the manifest, so the
`tokio` grep is retired in favour of a manifest-reading test. AC-1 and AC-3 are
restated in `slice-002.md`. CLAUDE.md's own "both feature columns" wording
becomes false the day this lands and joins the canon delta.

**Outcome:** verified

### F-7 — The diagnostic surface is not specified enough to implement or test

**Severity:** major
**Location:** `design.md` §5.4, §6 OQ-8, §9 item 13

**Expected:** an agent can implement and test the diagnostic surface from the
design.
**Observed:** "bounded detail", "two states" and "each fact once" do not decide
the reducer. Undecided: the display bound; non-UTF-8 conversion; truncation
marker and newline escaping; ordering when failure, cleanup, stderr, discarded
and `Undrawn` coexist; whether a later clean outcome clears prior diagnostics;
whether a successful `view: null` is a diagnostic at all; the exact strings and
icon assets. Costly because `Outcome`'s members are owned and not `Clone`, so the
consumption point must be explicit rather than assumed.

**Evidence:** `Outcome` has six owned fields and derives only `Debug`
(`src/shell/host.rs:69-95`). `Captured` holds arbitrary bytes plus a
transport-truncation flag and is not `Clone`
(`src/shell/backend/transport.rs:72`). The transport's cap is 256 KiB, not
necessarily the glass bound (`src/shell/backend/process.rs:23`). `Discarded`
carries an arbitrary `serde_json::Value` and its `Display` includes the raw value
for one reason only (`src/semantics/protocol/normalize.rs:48`, `:59`). Cleanup is
an independent channel (`src/shell/host.rs:92`). Research named retention,
bounding and report-channel as three unresolved questions
(`research.md:928`).

**Disposition:** fix-now
**Response:** §5.4 now specifies a pure `Diagnostics::from_outcome(Outcome) ->
Diagnostics` reducer — consuming, because `Outcome` is not `Clone` and the
consumption point had to be chosen — with named bounds, deterministic ordering,
lossy-decoding policy and marker, newline escaping, retention and clearing rules,
and the exact user-visible strings. Being pure and consuming, it is tested
directly, and its rendering is asserted once. The two icon assets get a
generation rule rather than a description.

**Outcome:** verified

### F-8 — "Extend the configuration, not the walk" cannot implement `.slint` scanning

**Severity:** major
**Location:** `design.md` D13, §9 item 14

**Expected:** D13 is implementable as stated.
**Observed:** `Scan` has no extension configuration and the walk hard-codes
`.rs`, so adding `.slint` necessarily changes the scanner's structure — D13's
stated shape is not available. Worse, `code_of` truncates at the first `//`
without respecting string literals, which is a *recorded* blind spot in `.rs` and
becomes materially unsafe in markup containing URLs: a URL hides every
user-visible string after it on that line.

**Evidence:** `Scan` holds only `root` and `forbidden`
(`tests/protocol/boundary.rs:13-20`); the extension check is hard-coded in `walk`
(`:116`); `code_of` truncates at the first `//` (`:181`) and the file records the
string-literal case as known and accepted precisely because nothing in `src/` has
one (`:166`). The present configuration scans root `src` only (`:240`), while the
markup will live in a different workspace member.

**Disposition:** fix-now
**Response:** D13's stated shape was wrong and the reason is instructive — the
file's header prescribes "extend the configuration" for *vocabulary*, and I
generalised it to file types, which the structure does not support. D13 now adds
an explicit extension set to `Scan`, changes the walk once to consult it, and
makes the comment cut string-literal aware for both languages. The `//`-inside-a-
string blind spot was tolerable while it was hypothetical; a URL in markup makes
it reachable, so slice 001's follow-up on the text scan is discharged here rather
than deferred again. One guarded scan per workspace member, with positive
controls planting a forbidden word in a component name, an accessible label, an
ordinary string, and a string following a URL.

**Outcome:** verified

### F-9 — "Each failure class" has no executable case definition

**Severity:** major
**Location:** `design.md` §9 item 11; `slice-002.md` AC-7

**Expected:** an agent can enumerate the cases item 11 requires.
**Observed:** R-44 is a prose list, not an enum to iterate. The implementation has
two `Failure` variants, seven `BackendError` variants, two `StateError` variants,
independent cleanup failures and many protocol errors. Some are not inducible by a
real backend; state refusals deliberately never reach one. Item 11 also asserts
only one rendering per failure, where AC-7 demands the host remain invocable
afterwards — a strictly stronger claim, absent from the validation step.

**Evidence:** `Failure::{Backend, State}` (`src/shell/host.rs:40`);
`BackendError` includes transport-internal cases such as `PipeMissing` and `Io`
(`src/shell/error.rs:17`); `StateError` has idle and stale cases (`:83`); cleanup
failures are independent of `Failure` (`:49`, `src/shell/host.rs:92`). R-44
enumerates in prose (`docs/specs/001-host-backend-protocol.md:149`). Slice 001
already established the stronger pattern — nineteen failures through one `Host`,
then a success (`docs/specs/001-host-backend-protocol.md:394`).

**Disposition:** fix-now
**Response:** §9 item 11 becomes a case table the plan must fill: every R-44 case,
its fixture or script, the expected Rust variant, the expected diagnostic text,
and whether it contacts a process — with separate rows for cleanup-only and for
combined cleanup-plus-exchange failure. All cases run through one retained
`Host`, and a successful exchange after the sequence is required, which is AC-7
as written rather than a weaker paraphrase. State failures and unreachable
transport-internal safeguards are explicitly exempt from the real-process
requirement. Slice 001's precedent is cited so the shape is inherited rather than
reinvented.

**Outcome:** verified

### F-10 — The content mapping contradicts itself

**Severity:** major
**Location:** `design.md` §5.2, §9 item 4; `slice-002.md` non-goals

**Expected:** one exhaustive mapping from `Content` to what appears.
**Observed:** `Body::Degraded` says forms with no element render as plain text;
`Undrawn::ContentForm` says HTML and URI have no element; the slice says they stay
"undrawn"; and validation item 4 calls all four variants "the four the renderer
does not draw" while two of them are explicitly rendered. An agent cannot tell
whether an HTML body is omitted, shown literally, or shown literally with only
its semantics counted as undrawn.

**Evidence:** `Content` is exactly `Text`, `Markdown`, `Html`, `Uri`, each
carrying a string (`src/semantics/protocol/canonical.rs:212-217`). The slice says
HTML and URI "stay admitted by the protocol and undrawn"
(`slice-002.md:68`). Research establishes only that they have no native renderer
(`research.md:656`) and separately establishes the two explicit conversion paths
(`research.md:646`).

**Disposition:** fix-now
**Response:** three statements of the same thing had drifted apart, which is what
happens when a rule is written three times instead of tabulated once. §5.2 now
carries one exhaustive table — absent, text, accepted markdown, rejected
markdown, HTML, URI — and §9 item 4 and the slice's non-goals are restated
against it. HTML and URI render as literal text with `Undrawn::ContentForm`,
because omitting a body the backend authored is the silent drop R-20 forbids at
normalization and CD-3 forbids at the glass; the `Undrawn` wording says the
content *form* was undrawn, not its bytes.

**Outcome:** verified

### F-11 — The Slint property type is spelled incorrectly

**Severity:** minor
**Location:** `design.md` §5.2 markup sketch

**Expected:** the sketch compiles if copied.
**Observed:** `in property <StyledText> body` uses the Rust type name; the Slint
property type is `styled-text`. Copying it literally fails in code generation —
in the *build script*, which `docs/memory/slint-build-mechanics.md` already warns
is where Slint errors surface.

**Evidence:** the accepted declaration is `in property <styled-text> body`,
generating `set_body(slint::StyledText)` (`research.md:613`).

**Disposition:** fix-now
**Response:** corrected. The first phase compiles one root `.slint` before
anything else, so the class of error is caught at the cheapest point rather than
by inspection.

**Outcome:** verified

### F-12 — The event-loop rationale conflates windows with top-level components

**Severity:** minor
**Location:** `design.md` §3, §5.4

**Expected:** the stated reason for `run_event_loop_until_quit()` is the measured
one.
**Observed:** the design asserts both that a visible tray keeps the loop alive
with no window *and* that the loop returns when the last window hides. With the
tray visible the second is not the measured behaviour, so the API choice is
defended by a reason that does not hold under the design's own architecture.

**Evidence:** a visible `SystemTrayIcon` keeps `run_event_loop()` alive with no
window, and hiding the tray ends it (`research.md:709`, `:714`). The last-window
exit was established by the separate no-tray experiment (`research.md:531`).

**Disposition:** fix-now
**Response:** the decision stands and its reason is replaced. `run_event_loop()`
exits when no visible top-level component remains; the tray currently prevents
that, so `run_event_loop_until_quit()` is chosen to make process lifetime depend
on an explicit quit rather than on the incidental visibility of a component — a
stronger reason than the false one, and one that survives the tray becoming
hideable.

**Outcome:** verified


## Round 2 — re-dispositions

Round 2 reopened seven of the twelve round-1 findings. Ids are immutable and
findings are append-only, so the round-1 entries above stand unedited: their
outcome became **contested**, and the fresh disposition and outcome are recorded
here, one block per finding.

*Raiser:* codex `gpt-5.6-sol`, round 2. *Responder:* the design agent under the
autonomy grant (`design-log.md` 2026-09-05); the outcome is set wearing the
raiser's hat, deliberately and with the switch stated, which is why round 3 is
owed rather than assumed.

### F-1 — re-disposed (round 1 outcome: contested)

**Round 2's objection.** The repaired four-row table is **not disjoint**, because
cleanup is orthogonal to success: `Host::accept` can return a successful new view
*and* a cleanup failure together. Cleanup enters at `host.rs:185-189`, the view is
minted at `:231-237`, and both are returned at `:246-253`. So a successful
`evaluate` with `view: Some` and `cleanup: Some` matched both row 1 (retain) and
row 2 (replace); and a successful `respond` with `view: None` and `cleanup: Some`
closes the interaction at `:241` while row 1 said retain. Separately, a
`State::verify` refusal (`host.rs:158-162`, `state.rs:102-112`) belongs to the
failure-retain transition, and validation item 11 never required proving the
former `ViewId` went stale.

**Disposition:** fix-now
**Response:** the four-row table is replaced by a **seven-row** one keyed on
`(Exchanged, view.is_some(), failure.is_some())` — eight combinations, all
covered, disjoint by construction. **`cleanup` appears in no row.** It is copied
through `accept` and `no_action` untouched and reaches no state writer:
`State::issue`, `State::close` and `State::verify` are the only three
(`state.rs:64-113`). Reading it as a selector was reading a host-disposal fact as
an interaction fact, which R-54 forbids in words and the code forbids by
construction. `Received::refused` is therefore a `bool`, and the failure's
stratum — the row 5 / row 6 split — changes only the diagnostics.

Row 7, `(view: Some, failure: Some)`, is unreachable (`accept` writes
`failure: None`) and is written `Replaced` rather than `unreachable!()`: a
`Prepared` exists only because `State::issue` ran, so the arm can never become a
panic on a value the host produced.

Validation item 11 is rewritten against the seven rows and gains the staleness
observable item 11 lacked: after a `Replaced` fold, a queued `Choose` bearing the
previous token is refused as `Refused::SupersededView` with no backend contact
and no advance of the invocation log (item 11d), with the no-intervening-evaluate
negative control beside it. `design.md` §5.4 "The reducer, derived from the
code"; D19.

**Outcome:** verified

### F-4 — re-disposed (round 1 outcome: contested)

**Round 2's objection.** Three defects, not one. (a) The owner task loops on
`recv()` and then awaits the exchange to completion, so **while awaiting an
exchange it cannot receive `Shutdown`**; the claim that shutdown "drops whatever
exchange is in flight" was therefore false as designed. (b) AC-12 and validation
item 14 required that no child outlive the process, which is stronger than
SPEC-001's cancellation contract (R-48) and than what the transport can observe —
`kill_on_drop` (`process.rs:64-72`) cannot await a reap once its future is
dropped. (c) The cheap-tier reducer test was disconnected from production: the
loop lived inside the `spawn_local` task while D9 bypassed that task with
`block_on`, so no function existed that both production and item 11 could call,
and the test could only duplicate the logic.

**Disposition:** fix-now
**Response:** all three, at their causes.

(a) **Shutdown leaves the command channel entirely.** A `Cancel` handle over
`tokio::sync::watch::<bool>` is level-held and separately pollable; the loop
`select!`s on it `biased`, both while idle and while awaiting an exchange, and
the losing branch's future is dropped by `select!` itself. `Command` has no
`Shutdown` variant. *Rejected:* a bare `Notify` (a waiter arriving after the trip
never completes) and `tokio_util::sync::CancellationToken` (a new dependency — a
hard stop). `watch` and `mpsc` come from tokio's `sync` feature, which the runtime
seam already adds alongside `rt-multi-thread` (`research.md:548`).

(b) **AC-12 is rewritten to what the host can honestly observe**: the task ends
far inside the configured timeout, `serve` *returns* rather than being abandoned,
the drop happens inside the still-entered runtime, and `quit_event_loop` has
exactly one call site on the completion path. That the child is gone is
explicitly not asserted — R-48 concedes drop-time cleanup is the only mechanism,
and SPEC-001 §7 records the same concession for slice 001's
`a_cancelled_exchange_leaves_nothing_of_the_host_behind`.

(c) **`serve(host, controller, commands, cancel, clock, glass) -> impl
Future<Output = Served<B, G>>` holds the whole loop.** Production wraps it in the
one `spawn_local` block — plus a single `quit_event_loop()` — and the cheap tier
drives the identical call under `block_on`. Everything is taken by value because
`spawn_local` needs a `'static` future, and handed back in `Served` so a test can
read what it did. `Controller::absorb` is the fold and is called only from
`serve`. D9's tier boundary moves accordingly: cancellation is now cheap-tier
work, and the event-loop target holds only the wiring that exists nowhere else —
a real close request reaching `Wire::stop`, `serve` returning, the loop quitting.

`design.md` §5.4 "The loop, and the one function both tiers call",
"Cancellation, concretely", "What AC-12 can honestly observe"; D9, D18; §9 item
14; `slice-002.md` AC-12.

**Outcome:** verified

### F-5 — re-disposed (round 1 outcome: contested)

**Round 2's objection.** Awaiting `mpsc::recv()` inside `spawn_local` is
workable and the `EnterGuard` does supply the runtime context — but three things
in the repair were wrong. The bridge could not receive shutdown mid-exchange
(F-4); **capacity 1 does not mean "an exchange is already running"**, since once
the task takes the current command the queue is empty and one more is accepted
during the exchange; and a full-channel callback must change the UI while a
closed-channel callback must ask the loop to quit, so callbacks demonstrably need
more than a sender clone — and nothing owned those handles.

**Disposition:** fix-now
**Response:** the callback's whole capability becomes one named, cloneable value:
`Wire { commands: mpsc::Sender<Command>, cancel: Cancel, window:
slint::Weak<PromptWindow> }`. `Wire::send` maps `Full` → write the busy notice
through the weak handle, `Closed` → ask the loop to quit; `Wire::stop` trips the
cancel signal. The handle is **weak** because the component owns the callback and
a strong capture is a reference cycle that leaks the window.

The queue policy is restated as **four mechanisms with one job each**, and the
false claim is struck: the controls are disabled while an exchange is in flight;
the channel holds one, so at most one command survives an exchange; `Full`
reports rather than drops; and a survivor bearing a superseded view token is
refused (F-13). **The token is the safety mechanism; the bound and the disable
only reduce how often a stale command is produced, and nothing rests on them.**

One consequence round 2 did not name but which follows from its own argument:
reporting `Full` by setting `busy` reports nothing, because `busy` is already set
during an exchange. So the report is a second property, `notice`, carrying
`BUSY_NOTICE`; it is the one property `Glass::present` does not read from the
frame, and `present` clears it — which is exactly as long as the statement is
true for. `design.md` §5.3; D14, D24.

**Outcome:** verified

### F-6 — re-disposed (round 1 outcome: contested)

**Round 2's objection.** The gate contradicted itself on its own command count —
§5.6 said "five commands, plus one" and printed six, item 1 said five, AC-1 said
six. Worse, `cargo test -p goad-semantics` was argued to add **no** pass/fail
coverage after `cargo test --workspace`, and it demonstrably rejects no unwanted
dependency (`research.md:806`), so the claim that it "holds purity" was false.
The manifest test still lacked a concrete allowlist or classification, a test
target and an owning member, and `boundary.rs` still had no defined final home.

**Disposition:** fix-now
**Response:** four repairs, and one of round 2's premises is refuted by
measurement.

**Six, said the same way in four places** — `design.md` §5.6, §9 item 1,
`slice-002.md` AC-1, `canon-delta.md` CD-5.

**The sixth command stays, on a measured ground that replaces the false one.**
`cargo test --workspace` unifies Cargo features across every member it builds, so
stratum 1 is compiled *there* with whatever features stratum 2 and 3 switch on in
shared dependencies. Probe, two members: `a` depends on `serde` with
`default-features = false` and derives `Serialize`; `b` depends on `serde` with
`features = ["derive"]`. `cargo build --workspace` succeeds; `cargo build -p a`
and `cargo test -p a` fail `error[E0433]` on `serde::Serialize`. So `--workspace`
does not cover what `-p` covers, and `-p goad-semantics` is the only gate command
that builds stratum 1 with exactly the features its own manifest asks for. Round
2 is right about the rest: it is **not** a purity check, and that claim is
withdrawn.

**The manifest test is an allowlist, not a denylist.** `goad-semantics` may
depend on `jiff`, `serde`, `serde_json`; `goad-shell` on those plus
`goad-semantics`, `tokio`, `toml`. A denylist requires classifying an unbounded
universe and lets a new dependency through by not being on it — the failure mode
the test exists to prevent. "Runtime, renderer or filesystem-shaped" becomes the
*reason* recorded beside a rejection, never the decision procedure. Every table
named `dependencies`, `dev-dependencies` or `build-dependencies` at any depth is
read, a renamed entry is checked by its `package` value, and zero entries is a
`Vacuous` breach. Seven positive controls, listed in §9 item 3.

**Both checks get an owning member.** `crates/goad-boundary` — a fourth
workspace member, test-only, depending on no other member (D17). The checks are
facts about the *workspace*, not about any member; any member that hosts them
reaches upward, and a member above all of them reaches nowhere.

`design.md` §5.1, §5.6, §9 items 1 and 3; D17; `slice-002.md` AC-1, AC-3.

**Outcome:** verified

### F-7 — re-disposed (round 1 outcome: contested)

**Round 2's objection.** `Diagnostics::from_outcome` **cannot implement its own
contract.** `undrawn` belongs to `Presentation` and the function receives only an
`Outcome`, which has no renderer-degradation field (`host.rs:70-96`). So it could
not include `undrawn` in the promised ordering, and a successful view with
undrawn parts would read as clean and *clear* the surface. The repair also
claimed exact strings and icon assets were specified when it gave only "one
marker", "one line", "one glyph and two colours". And the bound was not
executable without deciding whether bytes are bounded before or after lossy
decoding and newline escaping — both change displayed length, and truncating
arbitrary bytes can split UTF-8.

**Disposition:** fix-now
**Response:** the signature is replaced by one that cannot omit `undrawn`, and
everything else is written out literally.

**One consumption point.** `receive(outcome: Outcome) -> Received` is the only
consumer of an `Outcome` in the process. It calls `present` itself and hands the
resulting `undrawn` to `Diagnostics::of(reported: Reported, undrawn: &[Undrawn])`
in the same expression, so **I-2 stops being a rule an agent must remember and
becomes the only path the types admit**. *Rejected:* a four-argument free
function and letting the controller assemble `Reported` itself — both leave the
mapper's output and the outcome's residue joinable by a caller who can forget
one, which is I-2 by vigilance. `Received` carries `prepared`, `refused`,
`next_check` and `diagnostics`, which is also where F-13's public `ViewId` copy
comes from.

**The bound is applied last, and counted in characters.** Compose, decode
(stderr only), escape, then bound. Bounding bytes first lets escaping re-expand
the result past the budget — one 0x0A becomes two characters, one invalid byte
becomes a three-byte U+FFFD — and truncating arbitrary bytes before a lossy
decode can split a codepoint and manufacture a replacement character the backend
never wrote. `chars()` yields whole scalar values, so a split codepoint is not
representable. Limits: 4096 characters for the stderr line, 1024 for every other
line, 120 for the tooltip. A grapheme cluster can still split; fixing that needs
a segmentation dependency, which is a stop, and the failure is cosmetic.

**Every string is now literal**, including the two truncation statements, which
are deliberately distinct: `stderr was cut at the host's capture limit; …` for
`Captured::truncated`, and ` [{n} more characters not shown]` for the display
bound. The capture line names no number, because `STDERR_LIMIT` is a private
const in stratum 2 (`process.rs:26`) and copying its value to the glass is a
second statement of one fact.

**The icon is a rule with no artefact**: one pure function rasterising a 32×32
RGBA8 disc in integer arithmetic, the two states differing in form (ring versus
disc) as well as hue, so the pair survives a colour-blind viewer and a monochrome
panel. *Rejected:* two checked-in PNGs, a build-time PNG generator (needs an
image encoder — a dependency stop), and SVG (an unmeasured Slint cargo feature).

`design.md` §5.2 "The reception seam", §5.4 "The diagnostic surface"; D16, D21,
D22, D23; §9 item 13.

**Outcome:** verified

### F-8 — re-disposed (round 1 outcome: contested)

**Round 2's objection.** The extension set resolves the hard-coded `.rs`
problem, but "string-literal aware for both languages" does not *define* Rust raw
strings, escaped quotes, multiline strings, Slint strings, or lexical state
crossing lines — and the scanner is line-based and cuts at the first `//`
(`boundary.rs:137-147`, `:182-184`), so those choices determine what it catches.
The final workspace owner and paths were missing, as under F-6.

**Disposition:** fix-now
**Response:** the cut is defined as a **per-line state machine** over `Code` /
`Str{hashes}` / `Block`, with the transition table written out: `"` opens
`Str{0}`; `r` + *n* `#` + `"` opens `Str{n}`; `\` skips one byte inside `Str{0}`
and there are no escapes inside a raw string; `//` ends the line's code; `/*`
enters `Block`; `'` is **never** a delimiter, because in Rust it opens a lifetime
as often as a char. Ending a line inside a string returns the line intact — an
unterminated string is content, and content is scanned. String *contents* are
scanned throughout; only comments are cut.

It stays **line-based**, and the three residual costs are named rather than
hidden: a Rust string spanning lines is not tracked across the break (the one
surviving false negative); a `/* */` block spanning lines leaves later lines
scanned as code (a false positive in the safe direction, unchanged from today);
and `r"` is recognised in `.slint`, where the construct does not exist and the
rule is inert. *Rejected:* a stateful multi-line lexer — a different program, for
a defect that is not the reachable one; the reachable defect is a `//` inside a
single-line string, which is fully covered line-based.

Owner and paths: `crates/goad-boundary`, `src/` for the machinery and `tests/`
for the token list, the allowlists, the configured scans and every control (D17).
Members are read from `workspace.members` rather than listed, which retires R7 in
this slice. `design.md` D13, D17, §5.6, §9 item 15; `slice-002.md` AC-13.

**Outcome:** verified

### F-9 — re-disposed (round 1 outcome: contested)

**Round 2's objection.** The missing case definition was **delegated to the
plan** rather than supplied. It gave neither the rows nor the expected diagnostic
text, and F-7 left that text undefined. An autonomous planner would still have to
decide which R-44 phrases collapse to one fixture, which fixture or script drives
each case, which yield `Failure` versus `Discarded` versus cleanup-only, how one
`Host<ProcessBackend>` executes multiple process behaviours, and the exact
expected UI text.

**Disposition:** fix-now
**Response:** the table is written into the design, in full, as §9 item 12 — 12.1
through 12.8. Thirty rows in five groups with their exact `Display` text; the
vehicle inherited unchanged from slice 001 (one command, the invocation log as
argv[2], instructions as argv[3…]) with **three new sentinels** quoted from the
grandchild scripts, whose absence of `exec` is the whole fixture; the ordered
sequence through one retained `Host`, opening with the idle-state refusal and
ending with a coda that mints a view *and* fails cleanup (the row that falsifies
a cleanup-as-selector reducer, F-1); what every row asserts, including
**occurrence count rather than presence**, which is what fails if a `source()`
walk is added; the four rows that additionally read the element tree, one per
channel; the exemptions with a reason each; and the collapse table saying which
R-44 phrases are one row and which are six.

Two decisions inside it worth naming. Rows assert the **rendered text**, not the
Rust variant: slice 001's display tests already prove every variant names every
value it carries (`src/semantics/error.rs:270-286`), so a distinct line implies a
distinct variant, while a `matches!` proves nothing about what a person is shown
— and re-asserting the variant would be slice 001's `failure_matrix.rs` claim
copied into a second tier. And the table drives the `Host` directly, folding each
`Outcome` through the production `receive`, because `Controller::answer` would
refuse a fabricated `ViewId` locally and rows S1/S2 would never reach
`State::verify`; item 11 is where the command channel is driven.

12.8 settles the prerequisite round 2 did not raise but the table needs: the
host-driving half of slice 001's `harness.rs` becomes one file both test crates
include by `#[path]`, cut at the intersection of what the two tiers use, because
an included helper neither tier calls is dead code and dead code fails the gate.

`design.md` §9 item 12.

**Outcome:** verified

## Round 2 — new findings

### F-13 — A queued choice loses the identity it came from

**Severity:** blocker
**Location:** `design.md` §5.3 and §5.4 (as repaired at round 1)

**Expected:** an answer reaches the interaction it was an answer to.
**Observed:** `Command::Choose` carried only the option-id string, resolved
against whichever presentation was current when it was processed. During a slow
exchange another command can queue; an intervening `evaluate` returning a new
view makes `Host` replace the outstanding interaction and mint a new `ViewId`. If
both views reuse a backend-authored option id — legal, and common — the delayed
click is sent as an answer to the **new** interaction, because `Host::respond`
validates the supplied `ViewId` but not that the option belonged to that view.
The command must carry the `ViewId`, or a generation captured when the control
was created. `Presentation` contained no `ViewId`, and the ownership table
wrongly left the only copy inside private `Host::State`.

**Evidence:** minting replaces the outstanding interaction and the replaced id is
stale immediately (`src/shell/host.rs:231-237`,
`src/shell/state.rs:64-81`). `respond` verifies the id it is handed and nothing
about the option's provenance (`src/shell/host.rs:152-172`,
`src/shell/state.rs:102-112`). The public copy of the pair is
`Presented::view_id` (`src/shell/host.rs:35-38`), not the private
`State::outstanding` (`src/shell/state.rs:22-25`). R-14 permits two options to
share a label and nothing forbids two views sharing an option id.

**Disposition:** fix-now
**Response:** `Command::Choose { view: String, option: String }`, with the view
token carried on every `OptionRow` as a third column and passed by the
`chosen(string, string)` callback. The controller compares it to the retained
`Prepared::view_id.as_str()`; on a mismatch nothing is sent and
`Refused::SupersededView` is reported, with no backend contact.

The token is a **string selector**, matched against retained state and never
parsed back into a value — the treatment the option id already gets, so it is one
rule rather than two, and `ViewId::new` is never called from a callback. Ids are
unique by construction, `{now}#{seq}` (`state.rs:76`), so a match cannot be
accidental. *Rejected:* a numeric generation counter — Slint's `int` is `i32`, so
a `u64` counter truncates, which is exactly the numeric identity loss I-3 exists
to forbid; and the bare option id, which is the defect.

`receive` now returns the `ViewId` beside the presentation in `Prepared`, so the
ownership table states two copies with one owner rather than claiming there is
one. `design.md` §5.2, §5.3, §5.4 row 1; D19; §9 item 11d; `slice-002.md` AC-5.

**Outcome:** verified

### F-14 — AC-6 contradicts the repaired reducer

**Severity:** major
**Location:** `slice-002.md` AC-6

**Expected:** the acceptance criteria accept the intended behaviour.
**Observed:** AC-6 said every backend `view: null` produces no window. The
repaired reducer correctly says an `evaluate` returning `view: None` retains an
outstanding presentation and leaves the window unchanged. The criterion would
reject the design's own intended behaviour.

**Evidence:** `WhenNothingToShow::LeaveOutstanding` for `evaluate` and
`CloseOutstanding` for `respond` (`src/shell/host.rs:100-109`, `:137`, `:152`,
`:239-241`); SPEC-001 §5's account of the same distinction.

**Disposition:** doc-wrong
**Response:** AC-6 is restated to follow the interaction rather than the message.
A successful `respond` with `view: null` closes the interaction and leaves goad
with no window; a successful `evaluate` with `view: null` leaves any outstanding
interaction — and therefore the question on screen — exactly as it was, and with
nothing outstanding leaves goad a tray icon and no window. Validation item 11b
asserts both halves in one test, because asserting only the second is the shape
this finding showed to be wrong.

**Outcome:** verified

### F-15 — The reducer omits diagnostic-window mode

**Severity:** major
**Location:** `design.md` §5.4 (as repaired at round 1)

**Expected:** every transition of a one-window, two-mode surface is defined.
**Observed:** the design specified one window with two modes, but reducer state
was only presentation and visibility. Undefined: a clean `evaluate` → `view: None`
while the diagnostic window is visible; a failure while the prompt window is
visible; a new view while diagnostic mode is visible; diagnostics clearing while
their window is open; and returning from diagnostic mode to a retained prompt.
"Window unchanged" is insufficient — preserving visibility and preserving mode
are different things.

**Evidence:** the design's own "one window, two modes, not a second window",
against a reducer whose only outputs were the presentation and whether the window
was shown.

**Disposition:** fix-now
**Response:** the surface becomes **one derived value**, not two stored flags:

```rust
fn surface(&self) -> Surface {
  match (self.focus, self.shown.is_some()) {
    (Focus::Diagnostics, _)   => Surface::Diagnostics,
    (Focus::Automatic, true)  => Surface::Prompt,
    (Focus::Automatic, false) => Surface::Hidden,
  }
}
```

Three inputs, three outputs, no combination unnamed — so "window unchanged" never
has to be written down, because there is no independent visibility flag to
preserve or lose. `Focus::Diagnostics` is set only by `OpenDiagnostics` and
cleared by `CloseDiagnostics` and by a `Replaced` fold: a question the person must
answer outranks a record they can reopen.

The five cases are then DT-1 … DT-5, stated twice on purpose — once as a state
diagram over `(Shift, Focus)` in the reducer section and once from the
diagnostics' side — and the two statements are required to agree, which is a
review obligation rather than a hope. DT-1 is the one that decides a real
question: clearing never closes a window a person opened, so the window stays and
shows `Nothing to report.` while the tray returns to idle at once. Auto-closing
would destroy the record at the moment the next quiet exchange succeeded, so the
surface could never say "there was a fault, and it is now clear".

`design.md` §5.3 "The surface is derived, never stored", §5.4 "The window mode is
part of the fold" and "Diagnostic-mode transitions"; D20; §9 item 11f.

**Outcome:** verified

### F-16 — The renderer lint contract has two incompatible definitions

**Severity:** major
**Location:** `design.md` §5.1, A-2, D8

**Expected:** one answer to "what lint table applies to `crates/goad`".
**Observed:** §5.1 said the renderer owns a laxer `[lints]`; A-2 assumed the full
goad lint table applies to hand-written renderer code; D8 required twelve active
expectations around generated code. An agent must choose whether `crates/goad`
inherits `[workspace.lints]`, defines its own, or inherits with exceptions — and
that determines whether the twelve `expect`s are fulfilled.

**Evidence:** the three passages, mutually exclusive as written. The twelve lints
are tripped by *generated* code (`research.md:395-425`), and
`clippy::allow_attributes` does not fire on inner module attributes, which is what
makes a module-scoped suppression available at all.

**Disposition:** fix-now
**Response:** `lints.workspace = true`, no crate-level override, identical to
every other member. The only laxity is the twelve-lint `#![expect(...)]` on the
one module that wraps `include_modules!()` — a module-scoped expectation is
exactly as wide as the problem. A laxer crate table would extend a suppression
earned by generated code across every hand-written renderer file, which is the
largest body of new code in the slice. §5.1's "the renderer owns its own laxer
`[lints]`" was loose phrasing of a true measurement and is corrected rather than
implemented; D3's measured grounds for crate-not-feature are untouched, since the
ungateable Slint dev-dependency (16 → 223 crates) remains the decisive one.

A-2 keeps its status as an assumption and gains a **stop rule**: the third
distinct lint needing an `expect` outside the generated-code quarantine is not an
implementation detail; it is the table being wrong for this stratum, and it stops
the phase. `design.md` §5.1, A-2, A-5, D8; `canon-delta.md` CD-1.

**Outcome:** verified

### F-17 — The entry point is incomplete and names a removed type

**Severity:** major
**Location:** `design.md` §5.4, §5.2 markup sketch

**Expected:** an agent can write `main` from the design.
**Observed:** the declared component was `PromptWindow` but startup and
no-display handling said `MainWindow::new()`. The runtime sequence created only
`Tray`: it never created `PromptWindow`, installed its callbacks, constructed the
process-lifetime `VecModel`, or enqueued the startup evaluation. `dismissed()` was
declared with no producer and no transition while shutdown referred to
`on_close_requested`; tray `quit()` was declared and never connected to the
shutdown protocol.

**Evidence:** the design's own §5.2 declares `PromptWindow`; `MainWindow` appears
nowhere else in the slice. `research.md:527-530` is the no-display error path, and
it is the window's constructor that returns it.

**Disposition:** fix-now
**Response:** §5.4 now carries the entry point as a fifteen-step ordered
sequence, naming `PromptWindow` throughout, with every failure before the loop
reported on stderr and exiting **2** — one code for every startup failure, kept
distinct from a future non-zero meaning "ran, then failed". It creates the
window and the tray, sets the app id before any show, builds the process-lifetime
`VecModel`, mints one `Wire` and installs every callback from it, constructs the
glass, **enqueues the startup evaluation through the ordinary channel** so item
11 exercises the real path, and only then spawns the one task.

A callback table gives every producer: `chosen`, `close-diagnostics`,
`on_close_requested`, `check-now`, `show-diagnostics`, `quit`. Four shutdown
sources, one path — the window close and tray `quit()` are the identical
`Wire::stop()` call.

`dismissed()` is **deleted**. It had no producer and no transition because it had
no meaning: under SPEC-001 a view cannot be withdrawn — it is answered, or
replaced, or it stands. `close-diagnostics()` replaces it, and it has both.
Closing the window quits, in either mode: the alternative would leave an
interaction `Host` still considers outstanding with no way on screen to answer
it, which is the shape I-4 forbids, and a close that meant "quit" in one mode and
"go back" in the other is a gesture nobody can predict.

`design.md` §5.2 markup, §5.4 "The entry point, in order", "Callbacks", "Shutdown
is cancellation, not draining"; E-6; F-4's `Cancel`.

**Outcome:** verified

### F-18 — The canon delta preserves claims the repaired design refutes

**Severity:** major
**Location:** `canon-delta.md` CD-1, CD-5

**Expected:** promoting the delta puts nothing false into canon.
**Observed:** CD-1 said Cargo mechanically enforces ADR-001's direction rule; the
design expressly narrows that, because `tokio` in stratum 1's manifest still
builds. CD-5 said "the renderer adds a column" and then said the gate has no
feature matrix. Both cannot describe the same change. Promoting this would put an
overclaim and a contradiction into canon.

**Evidence:** `research.md:806` — `tokio.workspace = true` in the stratum 1
manifest leaves `cargo build --workspace` at exit 0. The split retires the
`shell` feature, which is what creates the two columns
(`research.md:817-835`), and a workspace member is built and linted by the same
`--workspace` commands as every other.

**Disposition:** fix-now
**Response:** `canon-delta.md` is repaired in place — it is a slice-folder draft,
not canon, so nothing under `docs/specs/`, `docs/policy/` or `docs/adr/` is
touched. CD-1's Cargo-enforcement bullet is narrowed to **crate edges** and now
says where the claim stops, that the manifest is held by a test, and that the
vocabulary scan is not made redundant; its crate list names four members
including `goad-boundary` and states that `[workspace.lints]` is inherited with no
crate-level override. CD-5 drops "the renderer adds a column" — the split
*retires* the matrix — and replaces the `-p goad-semantics` justification with the
measured feature-unification one, probe included, explicitly saying it is not a
purity check. CD-6 gains D13's final shape and the `goad-boundary` home. CD-7
lists the three enforcement mechanisms separately with their distinct jobs
instead of merging them into one sentence that would have carried the same
overclaim into `CLAUDE.md`.

*Rejected:* leaving the wording for audit. CD-1 and CD-5 are already endorsed as
*decisions*, so a reviewer at promotion is reading text rather than re-deciding,
and an overclaim that survives to promotion is canon.

**Outcome:** verified

### F-19 — `ContentForm` is undefined

**Severity:** minor
**Location:** `design.md` §5.2

**Expected:** every type the design names is either defined here or present in
the code.
**Observed:** `Undrawn::ContentForm { form: ContentForm }` named a type neither
defined in the design nor present in the code. The agent must invent the type,
its derives, its visibility and its rendering vocabulary.

**Evidence:** canonical `Content` is exactly `Text`, `Markdown`, `Html`, `Uri`
(`src/semantics/protocol/canonical.rs:212-218`); nothing named `ContentForm`
exists.

**Disposition:** fix-now
**Response:** `pub enum ContentForm { Html, Uri }` in `view_model.rs`, deriving
`Debug, Clone, Copy, PartialEq, Eq`, with a `Display` yielding the noun phrase
`HTML` or `a URI` and no other rendering, so it drops into the undrawn sentence
without an article being chosen at the call site.

It is deliberately **not** a mirror of `Content`: naming only the two forms that
reach the glass undrawn makes a third content form added to SPEC-001 a compile
error in the mapper's `match` rather than a silent omission. It carries **no
payload**: the bytes already reach the glass as the body, so carrying them here
would render one value twice — principle 4. *Rejected:* two `Undrawn` variants
instead, which would write the shared sentence twice.

`design.md` §5.2; §9 item 13k.

**Outcome:** verified

## Round 3 — re-dispositions

Four of the fourteen round-2 repairs were reopened. Ids are immutable and
findings are append-only, so both earlier blocks stand unedited: their round-2
outcome became **contested**, and the fresh disposition and outcome are recorded
here.

*Raiser:* codex `gpt-5.6-sol`, round 3. *Responder:* the design agent under the
autonomy grant (`design-log.md` 2026-09-05). The outcome is again set wearing the
raiser's hat, deliberately and with the switch stated — which is why round 4 is
owed and why this ledger does not read done.

### F-6 — re-disposed (round 2 outcome: contested)

**Round 3's objection.** The gate still does not enforce stratum 1's purity, and
AC-3 still says it does. Both instruments AC-3 names constrain only Cargo:
dependency resolution and dependency *names*. Neither rejects `std::fs`,
`std::process`, `std::time::SystemTime`, a socket, or any other I/O reachable
with no manifest entry — while ADR-001 asks for "No I/O and no async runtime"
(`adr/001-one-way-strata.md:34-35`). The design's own §5.4 acknowledges a
feature-unification hole the manifest test cannot see.

**Disposition:** fix-now
**Response:** round 3 is right, and the fix takes **both** halves of the
alternative it offered rather than choosing between them.

**The claim is narrowed, everywhere it is made.** §5.1's table, §5.6, §9 item 3
and `slice-002.md` AC-3 now enumerate instruments with a stated boundary each,
and none of them says "purity is enforced". This is the third raising of one
defect and the pattern in it is the useful part: each repair narrowed the *word*
and left the *sum* implying the whole. So the documents now say what the sum is
**not**, in the same breath as what it is.

**And a third instrument is added, because one boundary was not residue but a
gap.** A direct `std::fs` call needs no manifest entry and is not a crate edge,
so neither Cargo nor the allowlist can see it, and no source scan was looking.
The instrument is **one more configured `Scan`** in `crates/goad-boundary` —
root `crates/goad-semantics/src`, forbidden `std::fs`, `std::process`,
`std::net`, `std::os`, `std::env`, `std::thread`, `std::io`,
`std::time::SystemTime`, `std::time::Instant`. It needs no new machinery:
`mentions` already matches a `::`-bearing token as a substring
(`boundary.rs:166-176`, slice 001's F-45), and `code_of` already cuts comments,
which is why `schedule.rs:215`'s doc-comment mention of `` std::time:: `` is not
a hit today — verified against the tree. `std::time::Duration` is deliberately
not forbidden: a duration is a quantity, not a clock.

Its three misses are named beside it rather than assumed away —
`use std::{fs, process};`, an alias introduced by any other route, and I/O a
permitted dependency performs on stratum 1's behalf — and it is described as a
**regression tripwire** wherever it is cited, never as a proof.

**The feature residue stays residue, in writing.** Nothing in the gate rejects a
feature switched on by stratum 2 or 3 in a shared dependency. The design states
the *rule* (a dependency shared with stratum 1 keeps `default-features = false`
where it already has it; adding a feature to one is a slice decision) and states
that nothing enforces it. §9 item 3 says so explicitly, so a reader cannot infer
coverage from the list's length. *Rejected:* inventing a feature-graph test —
a fifth instrument arriving on argument rather than on measurement, which is how
this finding was created in the first place.

`design.md` §5.1, §5.6 "The three enforcement residues", §9 item 3; D25;
`slice-002.md` AC-3; `canon-delta.md` CD-1, CD-7; `draft-policy.md` Verification.

**Outcome:** verified

### F-9 — re-disposed (round 2 outcome: contested)

**Round 3's objection.** The failure matrix's schema cannot express its own
assertions. `Case` has one expected line and one channel, but C2 requires a
timeout **and** a cleanup failure; T3 and P2 require a failure line **and**
captured stderr; T1's tail is the OS's, although `Prefixed` was described as
being for the two serde rows only; assertion 3 requires a per-row presentation
transition and there is no field for it; and T1 runs in "its own `Host`" while
AC-7 requires every failure through one retained `Host`. An implementer must
invent special cases outside the claimed one-loop data model.

**Disposition:** fix-now
**Response:** the schema is replaced by one that can state every assertion the
rows make, and the AC-7 exemption is written into the data instead of into prose
beside it.

`Case` now carries: `cohort` (`Retained` | `Own { command }`), the instruction,
the turn, a **list** of `Observed { channel, text }` rather than one expectation,
the `shift` the fold must produce, and the `invocations` delta. `Channel` gains
`Stderr`, which is what lets T3 and P2 assert R-42 at the glass — the assertion
that had no field. `Expect::Prefixed` is redefined as *any line whose tail is
owned outside this repository*, which is the three rows P2, P3 and T1, and the
12.3 note is corrected to say so.

**T1's exemption is now a cohort with a checkable meaning.** A `Host` owns one
command (`host.rs:119-133`, `process.rs:46-49`), so a command that cannot be
spawned cannot first have succeeded and no sequence through the retained `Host`
reaches `BackendError::Spawn`. For that one row, "the backend is invocable
again" is asserted in the only form available: a **second** `evaluate` on the
same `Host` attempts a second spawn and fails identically — the host neither
died nor latched. AC-7 in `slice-002.md` now states the cohort and that weaker
meaning, so the criterion no longer claims something one of its rows cannot do.
Assertion 2 (`next_check` unchanged since exchange 1) is skipped for `Own`,
which is what slice 001 already recorded for its equivalent row
(`failure_matrix.rs:420-435`); and the two zero-invocation cases are
distinguished by cohort rather than by the count, because they mean different
things.

`design.md` §9 12.3, 12.4, 12.6; `slice-002.md` AC-7.

**Outcome:** verified

### F-16 — re-disposed (round 2 outcome: contested)

**Round 3's objection.** A-5 directly contradicts D8. D8 requires
`lints.workspace = true` "and nothing else" and rejects a laxer crate table;
A-2 says "never a crate-level `[lints]` override"; and A-5 says that if
`future_not_send` fires, the answer is precisely such an override.

**Disposition:** fix-now
**Response:** the contradiction is real and the repair went to the wrong half of
it — round 2 rewrote §5.1 and D8 and left A-5 pointing at the thing they had
just forbidden.

A-5 now says the answer is **A-2's** answer, because this is an instance of A-2
and not an exception to it: the narrowest `#[expect(clippy::future_not_send,
reason = …)]` that works, on `serve`, whose future is `!Send` by construction —
it owns `Rc`-bearing Slint handles and is driven by `spawn_local`, which never
moves it across threads. It **counts toward A-2's three-exception stop rule**,
and a crate-level override is stated as unavailable. If a site-local `expect`
genuinely will not do, that is not an implementation detail: it is D8 and AC-1
being wrong for this stratum, and it stops the phase.

`design.md` A-5 (and A-2, D8 unchanged — they were already right).

**Outcome:** verified

### F-17 — re-disposed (round 2 outcome: contested)

**Round 3's objection.** The entry point is still incomplete and internally
unconstructible. The ordered sequence never constructs `ProcessBackend` or
`Host`, yet passes an undefined `host` into `serve`; its `Wire` literal names no
`commands` field; and the markup declares callbacks but defines none of the
controls or event bindings that produce them — so AC-4's "one activatable
control per option" has no producer and `close-diagnostics()`'s claimed producer
does not exist.

**Disposition:** fix-now
**Response:** round 3 is right twice over, and the second half is the one that
matters: a *numbered sequence* is a format that can omit a step and still read
complete. The repair changes the format as well as the content.

**The entry point is now Rust**, not a list: `main -> ExitCode` over
`run() -> Result<(), StartupError>` over `start(&Path)`. It loads the `Config`,
reads the clock, **clones the command out of the config**, builds
`ProcessBackend::new(command, timeout)`, and **moves** the config into
`Host::new(config, backend, now)` — which is exactly the composition slice 001
already wrote and this design had been paraphrasing
(`harness.rs:249-254`, `process.rs:46-49`, `host.rs:119-133`,
`config.rs:47-51` for the `Clone`). It builds the runtime and holds the
`EnterGuard`, sets the app id, constructs both components, mints one `Wire`
through `Wire::new` — not a field literal over private fields — installs the six
callbacks, builds the glass, enqueues the startup command through the ordinary
channel, spawns the one task, and runs the loop. `main` uses no `?`, because it
returns `ExitCode`.

**The markup now contains the controls**, and each is grounded in the Slint
compiler's own sources rather than inferred: a `for` repeater over `options`
producing a stock `Button` per row, with `accessible-description` carrying the
`OptionId` and `clicked => root.chosen(option.view, option.id)`; a `Close`
`Button` invoking `close-diagnostics()`; and the tray's single `Menu` child with
three `MenuItem`s whose `activated` callbacks are `check-now`,
`show-diagnostics` and `quit`. `Button` already declares `accessible-role:
button`, `accessible-label: root.text` and `accessible-action-default`, and its
inner `Text` declares `accessible-role: none`
(`widgets/fluent/button.slint:29-34`, `:86`) — which is what makes it one handle
rather than T-E's two, and gives keyboard operation without a per-option
`FocusScope` (`:104-117`, T-D). `SystemTrayIcon` takes exactly one `Menu` child
and `MenuItem` carries `title` and `activated()`
(`builtins.slint:1296-1319`, `:3121-3134`). `accessible-description` and
`accessible-item-index`/`-count` are reserved properties settable on any element
with a role, and on a *component instance* only when its own root declares one —
which `Button` does and a bare `Rectangle` does not
(`typeregister.rs:256-283`; `tests/syntax/accessibility/accessible_properties.slint:24-41`).

One consequence found while writing it, and recorded rather than left to the
phase: `StyledText`'s property is `text`, it carries `link-clicked(link)`, and
it declares **no** accessible role (`builtins.slint:731-755`), unlike `Text`
(`:599-600`). So the body's *value* is not addressable through the accessibility
tree, and the design says where it is asserted instead — on `Presentation`, in
the mapper tier — rather than adding a second `body-text` property that would
render one value twice.

`design.md` §5.2 markup, §5.4 "The entry point, in full" and "Installing the
callbacks, exactly"; F-26's strings; §9 item 17.

**Outcome:** verified

## Round 3 — new findings

### F-20 — The design requires both one and two `quit_event_loop` call sites

**Severity:** major
**Location:** `design.md` §5.3 `Wire::send`, §5.4 the entry point, §9 item 14f

**Expected:** one owner for the event-loop quit.
**Observed:** `Wire::send` is required to call `quit_event_loop` on `Closed`
while the task's completion path also calls it; the prose calls the first "the
single exception" to having exactly one site, and validation requires a
source-level count of exactly one. Both cannot pass.

**Evidence:** the two passages, mutually exclusive as written, against a
validation item that counts call sites in `crates/goad/src/`.

**Disposition:** fix-now
**Response:** the completion path owns the quit, and `Wire::send`'s `Closed` arm
does nothing.

The argument is stronger than "pick one". `Wire` holds a `Sender`, so the
channel is closed only when the **receiver** is gone — and the receiver is owned
by `serve` and dropped when `serve` returns, one line before the task's own
`quit_event_loop`. A `Closed` arm that quit would therefore be a second call for
a quit already in flight, never a first one. The arm is written out and matched
with that reasoning as its comment, so "it does nothing" is a decision on the
page rather than a `_ =>`; and item 14f now says the count is one **with no
exception**, because a source scan asserting "one" against a design admitting
two is a check that cannot pass.

`design.md` §5.3 `Wire::send`, §5.4 entry point, §9 item 14f.

**Outcome:** verified

### F-21 — `engaged` is set and never cleared

**Severity:** major
**Location:** `design.md` §5.3 `Controller`, §5.4 the loop

**Expected:** the controls are re-enabled after an exchange.
**Observed:** `engage()` sets `engaged`; no method is specified to clear it; and
the loop asserts the next top-of-loop frame carries `busy = false`. An
implementation following only the stated mutations disables every control
permanently after the first exchange.

**Evidence:** `absorb` is documented only as consuming and reconciling an
outcome, and the loop returns directly to `glass.present(controller.frame());
// busy = false here`.

**Disposition:** fix-now
**Response:** `absorb` clears `engaged` before it returns, unconditionally and
whatever the `Shift`: the exchange it is folding is the exchange that has just
ended, and there is no outcome for which the controls should stay disabled. One
setter, one clearer, one pair, in one place — `serve`'s exchange arm.

*Rejected:* a separate `disengage()`, which is a third call the loop can forget
on a path it does not take; and moving `busy` out of `Controller` into a
parameter of `frame()`, which would make the caller responsible for a fact the
controller is otherwise the sole owner of, and would break the "everything the
glass needs, from one value" property `Frame` exists for.

The one path that sets and does not clear is `Ending::Stopped` arriving
mid-exchange, and it is unobservable: `serve` breaks out and never presents
again. That is stated rather than left as a hole. Validation item 11i asserts
`busy = false` and `accessible_enabled == true` after **both** a successful and a
failed exchange, which is the pair that fails if the clear is dropped.

`design.md` §5.3 (`absorb`, `engage`, and the retained-state paragraph); §9 item
11i.

**Outcome:** verified

### F-22 — The "exact" loop body contains an untyped placeholder

**Severity:** major
**Location:** `design.md` §5.4 the loop

**Expected:** a body described as exact is one an agent can type.
**Observed:** `let call = /* host.evaluate(...) | host.respond(...) */;`.
`evaluate` and `respond` take different arguments and have distinct opaque
future types while both borrow `&mut Host`, so the implementer must choose
between duplicating the cancellation `select!`, boxing, an operation enum, or an
`async` block — a choice that controls borrowing, cancellation and lint
behaviour.

**Evidence:** `src/shell/host.rs:136-170`, the two signatures.

**Disposition:** fix-now
**Response:** `Pending` — an enum carrying the exact arguments of whichever
entry point is about to be called, plus `exchanged()` derived from it — and one
`async` block that matches on it. `select!` therefore has **one** future to race
the stop signal against, and that future *is* the exchange rather than a wrapper
around it, which is what makes "the exchange future is dropped" a statement
about the exchange.

The command dispatch that produces the `Pending` is written out too, exhaustive
and with no `_` arm: `OpenDiagnostics`/`CloseDiagnostics` fold and `continue`;
`Evaluate` stamps; `Choose` resolves identity **before** stamping, so a
superseded click is refused for the reason that is true of it and a broken clock
does not relabel it. One `stamp(clock) -> Result<Timestamp, Refused>` helper, so
`ClockError` becomes a refusal in one place.

*Rejected:* two duplicated cancellation `select!`s (the contract stated twice,
and the copy rots); `Pin<Box<dyn Future>>` (an allocation, and a wrapper between
`select!` and the exchange); and leaving it to the implementer, which is the
finding.

`design.md` §5.4 "What is pending, as a value" and the loop body; D27.

**Outcome:** verified

### F-23 — The design violates the slice's stated clock non-goal

**Severity:** major
**Location:** `slice-002.md` non-goals and OQ-7; `design.md` §1, §5.4, §6 OQ-7

**Expected:** the slice document accepts the design's behaviour.
**Observed:** the non-goal says "Slice 003 owns the clock"; the design adds a
production wall-clock adapter to slice 002, and OQ-7 calls this "a slice with no
clock" while introducing that adapter in the same answer.

**Evidence:** `slice-002.md:69-71` against `design.md`'s `Clock` /
`wall_clock()`. `docs/AGENTS.md:90-95` requires the slice to be revised for
consistency after a design change.

**Disposition:** doc-wrong
**Response:** the slice document is the defect, not the design. Slice 003 owns
**scheduling and timers** — when to evaluate, how `next_check` is consumed, what
happens on failure — and slice 002 is authorized to read wall time solely to
stamp the events it sends and the calls that carry them. Every `Host` entry point
has always required a caller-supplied `Timestamp`, including slice 001's tests
(`host.rs:119`, `:136`, `:152`), so supplying one is not owning a schedule.

The non-goal is restated in those terms, and the same distinction is used in the
slice's OQ-7, in `design.md` §1, and in §6's OQ-7 — where the original question
was itself asked with the wrong word, which is how the inconsistency was
manufactured. D15 already drew the distinction correctly and is unchanged.

`slice-002.md` non-goals and OQ-7; `design.md` §1, §6 OQ-7.

**Outcome:** verified

### F-24 — New policy canon is drafted in the wrong vehicle

**Severity:** major
**Location:** `canon-delta.md` preamble and CD-5; `docs/AGENTS.md:29-40`

**Expected:** promoting the delta puts nothing in the wrong place.
**Observed:** `canon-delta.md` says it covers existing canon only and that new
canon belongs in a slice-local draft; CD-5 then defines an entirely new policy
under `docs/policy/`.

**Evidence:** `canon-delta.md:6-8` against CD-5's own "canon creation" label.
AGENTS.md assigns new canon to the slice's draft artefact and `canon-delta.md`
to changes to existing canon.

**Disposition:** fix-now
**Response:** the policy is drafted where the methodology puts it, and CD-5
becomes the amendment it should always have been.

`docs/slices/002/draft-policy.md` is created from `docs/templates/policy.md` —
copied, not written from memory — carrying the six-command block, the rationale
for `cargo test -p goad-semantics`, what each of the four enforcement instruments
holds and where it stops, and the feature residue stated as residue. It is
`POL-NNN` until promotion, says in its own header that it is not canon, and is
the slice's working authority for the gate in the meantime.

AGENTS.md names `draft-spec.md` because it assumes new canon is a specification;
this new canon is a policy, so it is drafted from the policy template and named
`draft-policy.md`. The rule that matters — new canon is drafted in the slice
folder from its governing template, never written into `docs/` mid-slice — is
honoured. The deviation and its reason are recorded in `design-log.md` rather
than taken silently.

CD-5 is rewritten as an amendment to `CLAUDE.md`: its pointer moves off
`docs/slices/001/design.md` §9 and onto the promoted policy. It states
explicitly that it does not create the policy, and that the two moves are two
endorsements and two Reconciliation rows — both land, or neither does, since
applying one alone leaves either a dangling pointer or two claimants to the gate.
`design.md` §10 C-5 says the same, once.

`canon-delta.md` preamble and CD-5; `docs/slices/002/draft-policy.md`;
`design.md` §10 C-5; `slice-002.md` Scope.

**Outcome:** verified

### F-25 — The tray-icon "rule" omits its geometry

**Severity:** minor
**Location:** `design.md` §5.4 the tray icon; D21

**Expected:** a rule that can regenerate the exact asset.
**Observed:** edge size, colours, form, supersampling and alpha arithmetic are
fixed; the centre, the outer radius, the idle inner radius, the sample positions
and the boundary comparison are not — so many visibly different rasterisers
conform, while D21 calls it a rule intended to regenerate the exact asset.

**Evidence:** "idle is an annulus, fault is a filled disc. One rasteriser, one
parameter — the inner radius", with no value for that parameter or for the outer
radius.

**Disposition:** fix-now
**Response:** the geometry is pinned, in integer eighth-of-a-pixel units chosen
so a 4×4 sample grid has integral sample centres: centre `128`, `OUTER_SQ`
`14_400` (15 px), `INNER_SQ` `5_184` (9 px) for `Idle` and `0` for `Fault`;
sample `(i, j)` of pixel `(x, y)` at `(8x + 2i + 1, 8y + 2j + 1)`; covered iff
`d2 <= OUTER_SQ && d2 >= inner_sq`, **both comparisons inclusive**, stated so two
implementations cannot differ by one sample on the rim.

The pinning pays for itself immediately: the centre pixel's assertion stops being
a description and becomes arithmetic — every sample of pixel `(16, 16)` has
`d2 <= 98`, below `Idle`'s inner radius and at or above `Fault`'s zero, so the
centre is fully transparent for one and fully opaque for the other. Validation
item 16 now asserts three pixels with computed values rather than "the centre
differs".

`design.md` §5.4 "The tray icon"; §9 item 16.

**Outcome:** verified

### F-26 — Startup, usage and clock-error text is undefined

**Severity:** minor
**Location:** `design.md` §5.4 (the clock, argument handling, the startup line)

**Expected:** every user-visible string in this renderer is written out, which
is what the design claims of its own surface.
**Observed:** `ClockError` has no `Display` text, the usage block has no content,
and the usage error has no wording — while all three are interpolated into the
visible `goad: {error}` line. An implementer must author user-facing policy
during execution.

**Evidence:** the `ClockError` variants and "carries a `Display`"; "one usage
block" and "usage error" with no text; `goad: {error}`.

**Disposition:** fix-now
**Response:** all of it is written out, and the outlet is decided with it —
because the outlet is not a free choice here.

`StartupError` is named as the type `{error}` renders, with its eight variants
and their exact text; `ClockError`'s two renderings are given; the usage block is
written verbatim, on **stdout**, one trailing newline, exit 0, produced by one
`const` with `--help` as its only destination — a usage error names the flag
rather than reprinting the block, so one fact stays in one place. Both
`StartupError` and `ClockError` implement `std::error::Error` with the default
`source()` returning `None`, which is F-47's defect refused at the one outlet
that has no window to lose.

The outlet: `print_stdout` and `print_stderr` are both `deny`
(`Cargo.toml:143-144`), so neither `println!` nor `eprintln!` is available, and
the write's own `Result` is discarded by matching because `let _ =` trips
`let_underscore_must_use` (`:152`) and `.ok();` trips `unused_must_use`. That is
three lint interactions an implementer would otherwise meet one at a time inside
a phase; it is settled here, with A-2's stop rule as the fallback if the first
`cargo clippy` disagrees.

`Launch` and `arguments(argv, env)` are given signatures, so §5.4's discovery
table becomes a test (§9 item 17) rather than a claim.

`design.md` §5.4 (the clock, "Config discovery, exactly", "The exact strings");
D26; §9 item 17.

**Outcome:** verified


### F-27 — `serve`'s shape does not avoid the lint it was chosen to avoid

**Severity:** major
**Location:** `design.md` §5.4 `serve`, A-5, D18
**Raised by:** the responder, wearing the raiser's hat, while repairing F-22 —
by compiling the loop rather than reasoning about it.

**Expected:** `serve`'s signature is the shape that passes goad's lint table.
**Observed:** it is not, and the assumption that it was survived two rounds of
review because the earlier reading tested a `Send` future, against which
`future_not_send` has nothing to fire.

`clippy::future_not_send` **does** reach a plain `fn` returning an `async` block
once that future is genuinely `!Send`. Worse, that shape additionally trips
`clippy::manual_async_fn`, which is in `clippy::all` and therefore `deny`
(`Cargo.toml:120`). So the signature chosen to avoid one deny-level lint trips
two, and A-5 — an assumption carried unchanged through rounds 1, 2 and 3 — is
false.

**Evidence:** a standalone crate of exactly this shape, run offline against
tokio 1 under `deny(clippy::all)` + `deny(clippy::future_not_send)`
(`research.md` Thread 7): `fn -> impl Future` with a `Send` future is clean;
with an `Rc`-bearing `!Send` future it produces **two** errors; `async fn` plus
one `#[expect(clippy::future_not_send, reason = …)]` is clean **and** the
expectation is fulfilled, so it self-clears if the situation ever changes.

**Disposition:** fix-now
**Response:** `serve` becomes an ordinary `async fn` carrying one
`#[expect(clippy::future_not_send, reason = "the loop owns Rc-bearing Slint
handles and is driven by slint::spawn_local, which never moves it between
threads")]`. That is exactly the answer F-16's repair had already specified for
"if it fires"; what changes is that "if" is now "does". It is the **first** of
A-2's three permitted expectations outside the generated-code quarantine, and
A-2's stop rule is updated to say two remain.

A-5 stops being an assumption and becomes a measured fact with a table. D18
drops the shape from its rationale. `Cancel::stopped()` keeps
`-> impl Future<Output = ()> + use<>` and trips nothing, because
`manual_async_fn` fires only when the body *is* a single `async` block and
`stopped` clones its receiver first — recorded so the next reader does not
"fix" it.

**What this says about the review, and it is the useful part.** A-5 was listed
as a *risk knowingly left standing* in two synthesis sections, on the grounds
that it "is settled by running the gate on the first renderer commit". That was
true and it was also a way of not finding out: the thing was cheap to measure —
a scratch crate, minutes — and measuring it changed a signature at the centre of
the design. Three of the four remaining assumptions (A-2, A-6, A-7) are of the
same kind. The stated mitigation for all of them is still "the first renderer
commit", and that is now a weaker answer than it looked.

`design.md` §5.4 `serve`, A-2, A-5, D18; §5.3 `Wire::send` (`try_send`, the
deadlock the same spike found); `research.md` Thread 7.

**Outcome:** verified


### F-28 — The tray cannot be written, and E-4's trap is wider than recorded

**Severity:** major
**Location:** `design.md` §5.2 markup (`Tray`), §5.3 `Glass::present`, E-4
**Raised by:** the responder, wearing the raiser's hat, while discharging A-7 —
by compiling §5.2's markup and reading the generated Rust.

**Expected:** `SlintGlass::present` can write the tray's icon and tooltip, which
is the whole of the two-state tray the design rests on.
**Observed:** it cannot. The markup as written generates **no `set_icon` and no
`set_tooltip` on `Tray` at all**, and marks `icon`, `title`, `tooltip` and
`visible` `set_constant()`. Every tray behaviour in the design — the icon
changing with `TrayState`, the four-form tooltip, `Diagnostics::state()` read on
every `present` — had no route to the component.

This is a defect **this round introduced**, and it was introduced by a repair
that was right about its premise. F-17's repair removed
`in property <image> icon;` from `Tray` because redeclaring an inherited
property is an error — true, `error: Cannot override property 'icon'` — and
replaced it with `visible: true;` on the reasoning that setting rather than
declaring gives E-4's required binding. Also wrong: a literal is
constant-folded.

**Evidence** (`research.md` Thread 8), three variants through
`slint_build::compile`:

| markup | result |
|---|---|
| redeclare `icon` / `tooltip` / `visible` | `error: Cannot override property` ×3 |
| set them to literals | compiles; `set_constant()` on all four; **no setter generated** |
| declare `image` / `hover-text` / `shown` and bind `icon: root.image;` etc. | compiles; `set_image`, `set_hover_text`, `set_shown`; **zero** `set_constant()` on `visible` |

**Disposition:** fix-now
**Response:** the third variant, in §5.2's markup. `Tray` declares `image`,
`hover-text` and `shown`, and binds the three builtins to them.
`Glass::present` writes `image` and `hover-text`; E-4 is genuinely closed
because `visible` is bound to something that can change, and the generated code
confirms it is not folded. §5.3's `present` description, §5.5 E-4 and §5.2's
notes are all restated against the measurement rather than against
`builtins.slint`, which does not mention either behaviour.

The corrected block was re-extracted from `design.md` after the edit and rebuilt,
and generates the eleven setters the design's Rust names.

**What this says about the round, and it is the second time in an hour.** F-27
came from building the loop; F-28 came from compiling the markup. Both were in
text this round wrote, both were invisible to careful reading of the upstream
sources, and both changed the design rather than merely confirming it. A-7 was
listed as "settled by the first renderer commit" ninety minutes before it was
settled by a scratch crate in twenty seconds of compile time — and had it been
left, the first renderer phase would have met a tray it could not write, mid-
phase, with a standing instruction to stop.

`design.md` §5.2 markup and its notes, §5.3 `Glass::present`, §5.5 E-4, A-7;
`research.md` Thread 8.

**Outcome:** verified


## Synthesis

Twenty-eight findings over three rounds — five blockers, twenty majors, three
minors — all `fix-now` but two `doc-wrong`, all currently `verified`. No blocker
outstanding. The ledger does **not** read done: round 3's repairs have not been
reviewed, and the whole lesson of rounds 2 and 3 is that unreviewed repairs are
where this design fails.

**Three rounds, three different failure modes.** Naming them separately is the
useful part, because each round's pattern is what the next round should be
briefed against.

- **Round 1 — written from summaries, not from source.** `design.md` described
  `Host`'s state machine from the brief rather than from `WhenNothingToShow`
  (F-1); dropped details research had verified and a summary had smoothed away —
  `SharedString`, an `mpsc` channel (F-3, F-5); and described a gate the split
  deletes (F-6). Cheap to find with the code open; expensive to find with an
  agent halfway through a phase.
- **Round 2 — repairs written at the level of intent.** Intent reads as settled.
  Seven of twelve did not survive "could an agent type this?": a four-row table
  that reads correct and is not disjoint (F-1); a shutdown travelling as a
  queued message in a loop that cannot receive one (F-4); "the UI reports that it
  is busy" with nothing behind it (F-5); a reducer whose signature provably
  cannot honour its contract (F-7); "string-literal aware" as a phrase rather
  than a transition table (F-8); a case table handed to the plan (F-9).
- **Round 3 — repairs correct in place, contradicting an unrepaired
  neighbour.** This is the failure mode a repair round *creates*, and it is
  narrower and easier to miss than either of the others. A-5 still pointing at
  the crate-level override D8 had just forbidden (F-16). A `Closed` arm still
  quitting a loop the same page says has exactly one quit (F-20). An `engaged`
  flag with a setter and no clearer, three sections after the flag was introduced
  (F-21). A slice non-goal saying slice 003 owns the clock, beside a design that
  adds one (F-23). New canon drafted in the file whose own preamble forbids it
  (F-24). Every one is a **seam**: repaired passage against untouched
  neighbour.

**What round 3 held.** Ten of fourteen round-2 repairs were audited against the
code and stood: F-1's seven-row reducer against `Host::respond` and its
pre-transport refusal path; F-4's `serve` owning and dropping the exchange
future; F-5's `Wire`; F-7's single `Outcome` consumption point; F-8's line
scanner; F-13's two-token `Choose`; F-14's restated AC-6; F-15's derived
`Surface`; F-18's narrowed canon delta; F-19's `ContentForm`. That is the
strongest evidence in the ledger that the structural repairs — the ones that
turned a rule into a shape the types enforce — are the ones that hold.

**What the three rounds did to F-6, and what it cost to stop.** F-6 was raised
three times on one defect: the gate's guarantees, overstated. Round 1 narrowed
"both feature columns"; round 2 withdrew "holds purity" from one command; round 3
observed that the *sum* still implied the whole and that one boundary was not
residue at all but a gap — direct `std` I/O, which no manifest and no compiler
can see. The repair takes both halves: the claim is narrowed everywhere it is
made **and** a third instrument closes the gap, with its three misses named. The
generalisable lesson is in the shape of the failure: narrowing a claim one word
at a time leaves the reader adding the instruments up, and the fix is to say what
the sum is *not*, in the same breath as what it is.

**What all three rounds confirmed.** The three user decisions of 2026-09-05 were
never challenged on their merits, and no finding touched the guiding principles.
The mapper as one exhaustive `match`, `OptionRow` as a struct row,
degrade-and-report, the crate split, the tray-plus-window shape, and the two test
tiers all survived unchanged. What the rounds did was force each of them down to
signatures, values and numbers.

**What changed structurally, and is the real yield.** Six repairs turned a rule
an agent must remember into a shape the types or the arithmetic enforce, and
those are the ones least likely to rot:

- `receive` is the only consumer of an `Outcome`, so **I-2 cannot be forgotten**.
- `Surface` is derived from `(focus, shown)`, so **"window unchanged" has no two
  meanings**.
- `serve` is one function production and the cheap tier both call, so **a
  loop-body change cannot pass in test and fail in production**.
- `Command::Choose` carries its view token, so **a stale answer is refused by a
  comparison rather than by timing**.
- `Pending` makes the raced future *be* the exchange, so **"cancellation drops
  the exchange" is a fact about the code rather than about a wrapper** (F-22).
- The tray icon's geometry is integers, so **"a rule regenerates the asset" is
  arithmetic a test can assert** rather than a description two rasterisers could
  both satisfy (F-25).

**Risks left standing, and one that should not have been.** A-5 was on this
list for two rounds — "settled by running the gate on the first renderer commit"
— and it was false. Measuring it cost minutes and changed a signature at the
centre of the design (F-27). That is the sharpest lesson of round 3, and it
generalises: *"the first renderer commit will tell us"* is a real mitigation and
also a way of not finding out, and it should be spent only on things a scratch
crate cannot reach.

So the two that a scratch crate could reach were measured rather than carried,
and both paid:

| assumption | outcome |
|---|---|
| **A-5** | **false.** `future_not_send` fires; the shape chosen to dodge it also trips `manual_async_fn`. `serve` becomes an `async fn` with one expectation (F-27) |
| **A-6** | **discharged.** The `Window.title` conditional compiles; the fallback is dropped |
| **A-7** | **discharged, and it found F-28** — the tray had no icon or tooltip setter, and E-4's constant-folding trap is wider than recorded |
| **A-2** | still standing: ~75 unproven lints against hand-written renderer code. Two named instances — the stderr-outlet spelling and the six-clone `install` — are ordinary Rust and cheap to check; the rest needs the real crate. One of the three permitted expectations is already spent (F-27) |
| **A-4** | still standing: `just check` wall-clock with 411 crates (ADR-002 T3). Needs the real tree; not answerable by a spike |

Three assumptions measured, two of them wrong, both wrong in ways that would
have stopped a phase. That is the round's most transferable result, and it is
not about any one of them: **an assumption a scratch crate can reach should be
reached before a phase starts, not listed as a risk.** A-2 and A-4 are the two
that genuinely need the real tree, and they are the two the first renderer
commit is actually for.

**The residue that is not a risk but an admission.** No instrument in the gate
rejects a feature switched on by stratum 2 or 3 in a dependency shared with
stratum 1. The design states the rule and states that nothing enforces it (D25).
That is the honest shape: an unenforced rule written down beats an enforced-
sounding claim, and it is exactly the trade F-6 was raised three times to get
right.

**What this review has not reached.** Round 1 did not read the canon delta, which
is what let F-18 stand for a round. Round 2 did not read the acceptance criteria
against the repaired design, which is what let F-23 stand. Round 3 read all
three artefacts, but **it has not read its own repairs** — and four of the seven
new findings are in passages round 3 itself caused to be rewritten from scratch:
the entry point, the loop body, the failure-matrix schema, the startup strings.
Two of those four are now measured — the loop (Thread 7) and the markup
(Thread 8) — and measuring them produced F-27 and F-28. The two that are
**not** are the failure-matrix `Case` schema and the startup strings: both are
ordinary Rust that no spike has compiled and no reviewer has read.

A round 4 should read those two, plus the two documents this round created or
restructured — `draft-policy.md`, and `canon-delta.md` CD-5 against `design.md`
§10 C-5 — and should treat F-27 and F-28 as its brief: **two of this round's own
repairs were wrong in ways only building them revealed, and the reviewer should
assume the same rate applies to the two that were not built.**

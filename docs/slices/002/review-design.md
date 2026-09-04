# Review — design — Slice 002

**Subject:** design — `docs/slices/002/design.md`, with `slice-002.md` and
`canon-delta.md` in scope as its companions.
**Reviewer:** round 1 — codex, `gpt-5.6-sol`, read-only, briefed for
implementation feasibility rather than intent.
**Opened:** 2026-09-05
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

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | blocker | fix-now | verified |
| F-2 | blocker | fix-now | verified |
| F-3 | blocker | fix-now | verified |
| F-4 | blocker | fix-now | verified |
| F-5 | major | fix-now | verified |
| F-6 | major | fix-now | verified |
| F-7 | major | fix-now | verified |
| F-8 | major | fix-now | verified |
| F-9 | major | fix-now | verified |
| F-10 | major | fix-now | verified |
| F-11 | minor | fix-now | verified |
| F-12 | minor | fix-now | verified |

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

## Synthesis

Twelve findings, all `fix-now`, all `verified`. No `blocker` outstanding; the
ledger resolves at round 1 pending a round 2 against the repairs.

**What the review changed.** Four blockers, and the pattern behind them is one
thing: `design.md` was written from `research.md`'s summaries where it should
have been written from the source. F-1 described `Host`'s state machine from the
brief rather than from `WhenNothingToShow`. F-3 and F-5 dropped details —
`SharedString`, an `mpsc` channel — that research had verified and a summary had
smoothed away. F-6 described a gate the split deletes. Each was cheap to find
with the code open and would have been expensive to find with an agent halfway
through a phase.

**What it confirmed.** The three decisions of 2026-09-05 were not challenged on
their merits, and no finding touched the design's guiding principles. The mapper
as one exhaustive `match`, `OptionRow` as a struct row, degrade-and-report, and
the crate split all survived — F-3 and F-10 sharpened how they are expressed
without disturbing what they decide.

**Risks knowingly left standing.** A-2 (goad's ~75 unproven lints against
hand-written renderer code) and A-4 (`just check` wall-clock with 411 crates,
ADR-002's T3) are unchanged by this round: both are settled by running the gate
on the first renderer commit, and neither is answerable from a document.

**What the review did not reach.** It read the design against slice 001's code
and against `research.md`. It did not attack `research.md`'s own claims — the
three spikes and two adversarial verifiers did that — and it did not review the
canon delta's wording, which promotion at audit still owes.

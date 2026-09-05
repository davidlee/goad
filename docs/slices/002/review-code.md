# Review — implementation — Slice 002

**Subject:** implementation — branch `slice-002`, commit range `a6ae617..af4c6f2`
(PHASE-01 … PHASE-09, twenty-two commits), and the working tree at `af4c6f2`.
**Reviewer:** adversarial code reviewer — fresh agent, Opus 5, raiser only
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

<!-- Written BEFORE the review, so it is not shaped by what turned out to be easy
     to find. What this review is probing, and the invariants it holds the
     subject to. Where the bodies are likely buried. -->

**Round 1** — 2026-09-05 — the shipped code of slice 002: the workspace split,
`goad-boundary`'s instruments, and the renderer.

This review reads the production code, not the documents. Documents are read
only where the code must answer to them. The subject is what a user runs, and
the standard is `CLAUDE.md`'s five invariants — each of which slice 002 is the
first slice able to *break* rather than merely to promise. The design is long,
converged and reviewed twice; the risk is therefore not that it was under-thought
but that the code diverged from it, or that both are right in the small and wrong
where they meet.

Nine lines of attack, in the order of how much a failure would cost.

**1. Invariant 4 — a backend failure never takes the host down.** The renderer is
the first thing that can be killed by a backend, because it is the first thing
that *runs* between exchanges. I walk every path reachable from backend output —
`serve`, `receive`, `Diagnostics::of`, `SlintGlass::present`, `main.rs`'s
`start` — for a panic, an `unwrap`, an `expect`, an index, an arithmetic
overflow, or a `?` that ends the loop. Panic-avoidance lints are denied
crate-wide, so a panic here arrives by a path clippy does not see: slicing a
`String` by byte range, `char_indices` arithmetic, a `RefCell` double-borrow
across a Slint callback, a `Rc` cycle, `try_send` on a full channel treated as
fatal, a Slint `set_*` on a dead window. Second half: latched state. If
`engaged`, the retained `ViewId`, the notice property or the focus flag can enter
a state that no subsequent outcome clears, the host is up but no longer
answerable, which AC-7 forbids as squarely as a crash. The `Spawn` cohort's
stated exemption (AC-7) is checked for being the *stated* exemption rather than a
larger one taken quietly.

**2. Invariant 3 — no narrowing.** This is the failure the project exists to
avoid, and a renderer is exactly the pressure that causes it. Slice 002 draws a
strict subset: no fields, no HTML, no URI, markdown degraded. The question for
every one of those is whether the code *refuses* or *narrows*. Specifically:
does `present` have an arm that can reject a view SPEC-001 admits — a view with
option-scoped fields, an `html` body, a `uri` body, an empty body, a title-only
view, a hundred options, an option label that is empty or enormous? Does the
mapper report every undrawn part (R-20), or does an undrawn part fall out of a
`match` arm silently? Is `Undrawn` closed over the things the protocol admits, or
over the things this renderer happened to meet? A `_ => {}` in the mapper is the
shape of this defect.

**3. Invariant 2 — permissive wire, canonical internals.** Past the normalization
door nothing is unvalidated, so anything downstream that *re-reads* a canonical
value as text — parsing, splitting, guessing, string-matching a payload — is a
second, undeclared door. I look at `reception.rs`, `wire.rs`, `view_model.rs` and
`diagnostics.rs` for a re-parse, a `to_string()` round trip, or a branch on a
payload's content rather than its kind.

**4. Invariant 1 — no domain understanding.** The vocabulary scan is the
enforcement, so the attack is on the scan, not on the code it scans. `code_of`'s
four-state cut is the load-bearing part: I construct inputs that defeat it — a
`//` inside a string followed by a real comment, a Rust raw string `r#"…"#`, a
`'"'` char literal, an escaped quote, a `.slint` line with `//` inside a string —
and check each against the shipped function rather than against D13's list of
four known blind spots. `members()` is attacked with a multi-line `members = [`,
an `exclude` table, and a commented-out member.

**5. Invariant 5 — one-way strata.** `cargo tree -p goad-semantics` is the cheap
check. The interesting one is D25's residue: `cargo tree -e features -p
goad-semantics` under `--workspace` resolution, looking for a feature switched on
in `serde` or `jiff` by stratum 2 or 3 that unifies into stratum 1's build. I
report whether it *matters* — whether any unified feature admits I/O, a clock or
an allocator stratum 1's own manifest would not.

**6. Cancellation, AC-12 and R-48.** The `biased` select, the drop of the
exchange future inside the entered runtime, `_entered`, `kill_on_drop`,
`spawn_local`. What is retained across the drop is the question: a `JoinHandle`,
a channel sender, a `Rc` held by a Slint callback, a task spawned on the
`LocalSet` that outlives `serve`. R-48's second sentence is the target — an
exchange must leave behind nothing a drop would fail to cancel.

**7. Interaction identity, AC-5 and R-32/R-33.** The view token on a control. I
try to find a path by which a `Choose` minted against view A is *answered* after
B replaced it: the `install.rs` callback closure capturing a stale token, a
`Wire::send` that queues and is drained after the replacement, a
`Controller::answer` that compares against the wrong side, an ordering where the
token is refreshed before the reply is checked. `SupersededView` must be raised
locally with no spawn.

**8. Display bounds, AC-8.** A code-point split; the three limits and whether
each is applied after escaping as the AC requires or before it; `chars().count()`
versus grapheme clusters, judged against the design's own statement rather than
against Unicode purism; the tooltip projection; whether each fact is rendered
exactly once (F-42/F-47 are repaired, so the target is a *new* double-render).

**9. Test honesty, and the small things.** A test that would pass against a
broken implementation is worse than no test, because it is a claim. I look for:
a guard test that passes without debug info; absence asserted with no presence
control; `find_all().len()` against a virtualised list; a break-and-revert
claimed in notes but not demonstrable now; a test asserting an internal enum
where the AC names a user-visible window; timing assertions that flake under load
(VT-10's 250 ms, the `@slow-view` 0.2 s arm, every `until(…)` poll). Alongside:
`#[expect]` outside `generated.rs` and `#[allow]` anywhere (both should be zero),
`as` casts, bare arithmetic in `diagnostics.rs`, anything `pub` that need not be,
`clock.rs:13`'s comment, `describe_outcome` duplication, and documents that
assert something about the code that is not true.

I do not re-raise `review-plan.md` F-34…F-39, and I take `plan-log.md`
PL-13…PL-17 as adjudicated unless I think the adjudication itself was wrong, in
which case I say so and why. I raise nothing I have not evidenced by running a
command, constructing an input, or reading the path end to end.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | major | | |
| F-2 | major | | |
| F-3 | minor | | |
| F-4 | minor | | |
| F-5 | minor | | |
| F-6 | minor | | |
| F-7 | minor | | |
| F-8 | nit | | |
| F-9 | nit | | |
| F-10 | nit | | |
| F-11 | nit | | |

### F-1 — the body and its degradation marker are drawn by markup nothing asserts

**Severity:** major
**Location:** `crates/goad/ui/app.slint:31-37`; `crates/goad/tests/renderer/tree.rs:81-88`;
`crates/goad/tests/renderer/reception.rs:673-683`

**Expected:** AC-9 — "A `markdown` body that the renderer's parser rejects is
still **shown**, degraded, and the degradation is reported." AC-4 — a choice view
has its "title, **body**, and one activatable control per option" drawn. Both
clauses are claims about what a person sees. E-1's own rule is that an absence
assertion is paired with a presence assertion and that each is shown to fail
against a deliberately broken implementation — the discipline `tree.rs`'s
`the_diagnostic_empty_state_is_present_only_when_empty` follows for
"Nothing to report.".

**Observed:** the body path is asserted only at `Presentation`. No test in the
crate reads `PromptWindow::get_body()`, `get_body_degraded()`, or looks for the
`"shown as plain text"` marker element. `tree.rs:81-88` asserts a `StyledText`
element *exists* and explicitly declines to assert its content;
`reception.rs:673` is named
`an_html_body_reaches_the_glass_through_receive_with_the_bytes_the_backend_sent`
and touches no glass — it asserts `prepared.presentation.body`. So the two
markup lines that carry AC-9's user-visible half are unheld.

**Evidence:** two break-and-reverts, each leaving the whole `goad` suite green.

Deleting the marker outright — `if root.body-degraded: Text { text: "shown as
plain text"; }` removed from `app.slint`:

```
test result: ok. 117 passed; 0 failed  (renderer)
test result: ok. 1 passed; 0 failed    (event_loop)
test result: ok. 6 passed; 0 failed    (lib unit tests)
```

Unbinding the body entirely — `text: root.body;` commented out of the
`StyledText`:

```
test result: ok. 117 passed; 0 failed  (renderer)
test result: ok. 1 passed; 0 failed    (event_loop)
test result: ok. 6 passed; 0 failed    (lib unit tests)
```

Both reverted with `git checkout -- crates/goad/ui/app.slint`; the tree is
clean.

A renderer that draws no body at all, and never tells a person their markdown
was degraded, passes this slice's gate.

**Fix:** one test in `tree.rs`, in the shape
`the_diagnostic_empty_state_is_present_only_when_empty` already uses: set
`body_degraded` false and assert the `"shown as plain text"` Text is absent, set
it true and assert it is present; and one assertion that a `StyledText` carrying
a known plain body reads that text back (`set_body(StyledText::from_plain_text
("x"))` then a predicate on the element's accessible label, or `get_body()`
round-tripped). Both are element-tree claims the query API already supports —
`accessible_label` is what every other assertion in the file selects on.

**Disposition:**
**Response:**

**Outcome:**

### F-2 — item 14d's in-flight precondition is a bare 100 ms sleep, the exact race PL-17 repaired in 14a

**Severity:** major
**Location:** `crates/goad/tests/renderer/wiring.rs:1114-1118`

**Expected:** `plan-log.md` PL-17 records the repair as "14a's precondition
observes the invocation log rather than sleeping", and `wiring.rs:1090-1094`
states the reason in the code: the precondition "previously rested on a bare 100
ms sleep". The helper that replaced it, `until(bound, predicate)`
(`wiring.rs:126-138`), exists for exactly this.

**Observed:** `cancellation::on_stop_a_command_queued_behind_the_exchange_is_
left_unread` — item 14d, VT-13 — still sleeps a fixed 100 ms before tripping
`Cancel`, and then asserts `invocations(&log) == 1`. That assertion holds only
if the `@hang` backend has been spawned *and* has written its log line within
those 100 ms. Nothing enforces it; under load the count is 0 and the test fails
for a reason that has nothing to do with what it tests.

**Evidence:** shrinking the sleep to 1 ms turns the pass into a failure whose
message shows the precondition, not the property, is what broke:

```
thread 'wiring::cancellation::on_stop_a_command_queued_behind_the_exchange_is_left_unread'
panicked at crates/goad/tests/renderer/wiring.rs:1123:5:
assertion `left == right` failed: the queued second command must never reach the backend
  left: 0
 right: 1
```

Reverted with `git checkout -- crates/goad/tests/renderer/wiring.rs`. The
duration is the only thing separating the shipped test from that failure, and
`just check` is AC-1's gate.

**Fix:** replace the sleep with the helper the sibling test already uses —
`until(Duration::from_secs(1), || invocations(&log) >= 1).await;` — then
`stopper.stop()`. Same three lines as `wiring.rs:1090-1094`.

**Disposition:**
**Response:**

**Outcome:**

### F-3 — AC-14 names "crate name" as a covered surface; no instrument reads a manifest

**Severity:** minor
**Location:** `crates/goad-boundary/tests/checks/vocabulary.rs:28-35`;
`crates/goad-boundary/src/members.rs:32-52`

**Expected:** AC-14 — "No domain vocabulary appears in any **crate name**, module
name, type, markup component, accessible label, or user-visible string." AC-13
makes the scan's coverage mechanical so "a new member cannot arrive unscanned".

**Observed:** the vocabulary scan's only extensions are `rs` and `slint`
(`vocabulary.rs:31`), so no `Cargo.toml` is ever inspected — not the root's
`workspace.members` list and not a member's own `[package] name`. `members()`
returns the entries as paths and checks them only for `*`; it never calls
`mentions`. A module name is covered transitively, because `mod habit_view;`
must appear in a scanned `.rs` file. A crate name has no such source-side
obligation: a member declared and depended on by manifest alone is invisible.

**Evidence:** a member-shaped directory whose manifest names the domain, scanned
with the production configuration:

```
crates/…/fixture-member/Cargo.toml   → [package] name = "goad-habits"
crates/…/fixture-member/src/lib.rs   → pub fn ok() {}

Scan { extensions: ["rs","slint"], excluded_dirs: ["tests","target"],
       forbidden: ["habit","streak","site"] }.run()
  → CLEAN: inspected 1 file(s)
```

`mentions("crates/goad-habits", "habit")` returns `true`, so the matcher is not
the gap — nothing hands it the string.

**Fix:** either run `mentions` over each `workspace.members` entry and over each
member manifest's `[package] name` inside `members()`, or narrow AC-14's
sentence to drop "crate name". The first is four lines and keeps the criterion
honest.

**Disposition:**
**Response:**

**Outcome:**

### F-4 — a char literal holding a quote desynchronises `code_of`, and the cost is not among the four it names

**Severity:** minor
**Location:** `crates/goad-boundary/src/scan.rs:257-263`, `:264-321`

**Expected:** the function's own doc names its costs — "Four costs, named rather
than assumed away (D13)" — and AC-3 holds the sibling purity scan to "Its limits
are named, not assumed away". D13 chose not to treat `'` as a delimiter, and
gave the reason (a lifetime looks like a char literal), but neither D13 nor the
shipped doc records what that choice costs.

**Observed:** a Rust char literal containing a double quote — `'"'`, or `b'"'` —
puts the state machine into `Str` on a quote that never closes as a string. The
*next* real string literal's opening quote is then read as a *closing* one, the
machine re-enters `Code` inside that string, and a `//` inside it truncates the
line. Everything after is invisible to the vocabulary scan and to the purity
scan. The miss is in the false-negative direction, which is the direction D13
says matters.

**Evidence:** the shipped `code_of` and `mentions`, driven directly:

```
line   = let q = '"'; let s = "//"; let habit = 1;
code_of= let q = '"'; let s = "
mentions(line, "habit") = false

line   = if b == b'"' { let s = "//"; let site = 1; }
code_of= if b == b'"' { let s = "
mentions(line, "site")  = false
```

The false-positive direction is reachable too — `let q = '"'; // the call sites`
returns `mentions(.., "site") == true`, because the comment was never cut.
Nothing in the tree contains such a literal today, so no scan is currently
wrong; this is a named-limits defect, not a live miss.

**Fix:** the cheap correction is to add the case to the doc's cost list and to
D13, alongside the all-caps compound. The fuller one is a fifth transition —
`Code` on `'` followed by a character and a closing `'` skips three bytes — which
handles `'"'` and `b'"'` and still leaves `&'static` alone, since a lifetime has
no closing quote at that offset.

**Disposition:**
**Response:**

**Outcome:**

### F-5 — the production runtime topology is exercised by no shipped test

**Severity:** minor
**Location:** `crates/goad/src/main.rs:56-115`;
`crates/goad/tests/event_loop/closing.rs:34-114`

**Expected:** `closing.rs`'s header claims it is "The whole of `start`'s own
composition (§5.4), minus argument parsing and the process exit code". `main.rs`
records the hazard the composition exists to avoid: "Without the guard the first
poll of a `tokio::process` future on the Slint thread panics with *there is no
reactor running*" — the panic `research.md` Thread 4 measured, which unwinds out
of the event loop and exits 101. That is `CLAUDE.md` invariant 4 failing.

**Observed:** `closing.rs` instantiates the composition and then sends no
command at all — its channel stays empty and its backend (`true`) is never
spawned, so no `tokio::process` future is ever polled on the Slint thread. Every
test that *does* drive `serve` against a real `ProcessBackend`
(`wiring.rs`'s `interaction`, `serving` and `cancellation` modules) runs under
`#[tokio::test]` with `tokio::task::spawn_local` inside a `LocalSet` that a
current-thread runtime is blocking on — a different executor from production,
where Slint's own executor polls the future and only an `EnterGuard` supplies
the runtime context. The two arrangements are exactly the pair Thread 4 measured
as differing, and only the untested one ships.

**Evidence:** I rebuilt the production arrangement in a scratch binary against
the shipped library — multi-thread runtime, `runtime.enter()` held,
`slint::spawn_local(serve(…))` over a real `ProcessBackend`,
`run_event_loop_until_quit` — and it **works**:

```
RESULT: view drawn through the production topology after 0 ms
ending = Some((Stopped, true))
```

and, against `@hang` with `Cancel` tripped 300 ms into the exchange:

```
RESULT: stopping mid-exchange at 300 ms
ending = Some((Stopped, false))
exit 0;  no surviving `sleep` child (`ps -eo comm | awk '$3=="sleep"'` empty)
```

So this is a gap, not a live defect: the shipped code is correct in the
arrangement it ships in, and nothing in the repository would notice if it
stopped being.

**Fix:** `closing.rs` already builds everything needed. Enqueue one
`Command::Evaluate(Stimulus::Startup)` into its channel and point its
`ShellCommand` at `tests/backends/answers-as-instructed.sh` with a pinned view,
then assert the heading before dispatching `CloseRequested`. That turns the
file's existing claim into a true one at the cost of two lines and one
instruction.

**Disposition:**
**Response:**

**Outcome:**

### F-6 — two `Refused` variants carry a payload nothing renders

**Severity:** minor
**Location:** `crates/goad/src/diagnostics.rs:51-60`, `:141-152`;
`crates/goad/src/controller.rs:177-192`

**Expected:** design.md §5.2 principle 4 — every fact has one renderer and one
place. `Diagnostics::of`'s own doc states the reducer is "total, because every
field of every input has a rendering".

**Observed:** `Refused::SupersededView { named }` and
`Refused::UnknownOption { named }` are constructed at three sites in
`controller.rs::answer`, each allocating with `to_owned()`, and matched at two
sites in `Diagnostics::refused` — both as `{ .. }`. The field is written three
times and read zero. Its host-side twin does the opposite:
`StateError::NoOutstandingView { named }` renders the id it carries, which is
what `reception.rs`'s `failure_line(named_len)` drives a bound test with. So the
renderer's own version of "you answered a question that is not outstanding"
silently drops the identifier its stratum-2 equivalent reports.

**Evidence:**

```
$ grep -rn "SupersededView\|UnknownOption" crates/goad/src
diagnostics.rs:54:  SupersededView { named: String },
diagnostics.rs:57:  UnknownOption { named: String },
diagnostics.rs:142:      Refused::SupersededView { .. } => {
diagnostics.rs:145:      Refused::UnknownOption { .. } => {
controller.rs:177:    let prepared = self.shown.as_ref().ok_or_else(|| Refused::SupersededView {
controller.rs:181:      return Err(Refused::SupersededView {
controller.rs:190:      .ok_or_else(|| Refused::UnknownOption {
```

The design specifies both the field (`design.md:2082-2090`) and the two
sentences that omit it (`design.md:2186-2188`), so the code is faithful and the
contradiction is upstream of it.

**Fix:** one of two, and it is a design call rather than a repair: drop `named`
from both variants, or render it — bounded, since it is a string the markup
handed back. Dropping it is the smaller change and matches the sentences already
pinned.

**Disposition:**
**Response:**

**Outcome:**

### F-7 — the two stderr outlets' exact strings are asserted nowhere

**Severity:** minor
**Location:** `crates/goad/src/diagnostics.rs:274-278`, `:296-316`

**Expected:** design.md §5.4's *The exact strings* pins `goad: {error}` and
`goad: the window could not be drawn: {detail}` alongside every diagnostic
sentence, and every other string in that section has a test. `line_to` takes
`impl std::io::Write` rather than writing to a fixed sink, which is the shape
that makes a sink injectable.

**Observed:** no test in the tree names either string.
`startup.rs` covers every `StartupError` `Display` arm — the `{error}` half —
and `print_usage_does_not_panic` covers the stdout outlet, but the `goad: `
prefix and the whole of `report_platform`'s sentence are unasserted.
`report_platform`'s only caller is a `show()`/`hide()` failure inside
`SlintGlass::present`, which no test induces, so a typo there ships unseen.
`line_to`'s sink parameter buys nothing here: the function is private, so no
test can reach it, and both public wrappers hard-code `std::io::stderr()`.
Separately, `report_platform` is `pub` with no caller outside the crate.

**Evidence:**

```
$ grep -rn "could not be drawn\|goad: " crates/goad/tests tests/
(no matches)
$ grep -rln "\breport_platform\b" crates/goad/src crates/goad/tests
crates/goad/src/glass.rs  crates/goad/src/diagnostics.rs  crates/goad/src/wire.rs
```

(`wire.rs`'s is a doc-comment mention, not a call.)

**Fix:** make `line_to` `pub(crate)` and assert both sentences against a
`Vec<u8>` sink in `startup.rs`, beside `print_usage_does_not_panic` — two tests,
no new surface outside the crate. `report_platform` becomes `pub(crate)` in the
same change.

**Disposition:**
**Response:**

**Outcome:**

### F-8 — `driving.rs::no_view` restates `describe_outcome`'s sentences, against the file's own rule

**Severity:** nit
**Location:** `tests/support/driving.rs:190-203`;
`crates/goad-shell/tests/integration/harness.rs:210-216`

**Expected:** the same file, twelve lines earlier, states the rule it is
breaking: "Stated **once**, here, rather than a second time per tier: two of
three statements of one number are what nothing updates (D23, applied to a
test)" (`driving.rs:28-30`).

**Observed:** `no_view` reproduces two of `describe_outcome`'s three literal
sentences — `"a failure: {failure}"` and `"nothing to show, and no failure"` —
in a second file. The doc comment argues the case honestly (the `renderer`
target no longer calls `describe_outcome`, and `driving.rs` cannot reach into
`harness.rs`), but the outcome is two statements of one sentence that nothing
keeps in step.

**Evidence:** `harness.rs:212` and `:214` against `driving.rs:200` and `:201` —
identical text, no shared constant, no test comparing them.

**Fix:** move `describe_outcome` back into `driving.rs` and have `harness.rs`
call it, or lift the two sentences into `pub(crate) const`s there. The third arm
(`"a view carrying {id}"`) is `harness.rs`'s alone and can stay.

**Disposition:**
**Response:**

**Outcome:**

### F-9 — `clock.rs` attributes `serve` to the wrong phase

**Severity:** nit
**Location:** `crates/goad/src/clock.rs:11-13`

**Expected:** `serve` landed in PHASE-10 — `wire.rs:5-6` says so ("`serve`
itself lives in `controller.rs`, PHASE-10"), and so does `controller.rs:1-5`.

**Observed:** "a test supplies a fixed instant in one line, and **PHASE-07's**
`serve` takes one of these rather than a `dyn Fn`."

**Evidence:** `git log --oneline` — `6fd3ee3 PHASE-07: the glass, the wiring,
and back-pressure`; `c2cacf5 PHASE-10: serve, and the stop that drops the
exchange`.

**Fix:** one word.

**Disposition:**
**Response:**

**Outcome:**

### F-10 — `structure.rs` is non-recursive and says that is not a silent gap

**Severity:** nit
**Location:** `crates/goad-boundary/tests/checks/structure.rs:30-45`

**Expected:** the file is item 14f's instrument — `quit_event_loop` has exactly
one call site, the renderer holds no `tokio::spawn` handle. Its own doc:
"Non-recursive — the directory is flat today … — so a subdirectory arriving
unscanned is a defect the next `git ls-files` diff surfaces, **not a silent
gap**."

**Observed:** it is a silent gap by the ordinary meaning. `subject_files()`
takes one `read_dir` and filters for `.rs`, so a second `quit_event_loop` under
`crates/goad/src/<anything>/` leaves the count at one and every assertion
passes. What the doc offers instead is a human reading a diff, which is the
review-gate-not-build-gate arrangement ADR-001's Verification section is
explicit about not trusting.

**Evidence:** `structure.rs:37-44` — a single `std::fs::read_dir(&dir)` with no
descent, and no test asserting the directory is flat.

**Fix:** recurse, excluding `target`, in the six lines `Scan::walk` already
demonstrates; or assert that `crates/goad/src` contains no subdirectory, so the
premise fails loudly when it stops being true.

**Disposition:**
**Response:**

**Outcome:**

### F-11 — the design still heads the rasteriser block with a file that does not exist

**Severity:** nit
**Location:** `docs/slices/002/design.md:2538`

**Expected:** the design's own module list (`design.md:458-469`) gives
`crates/goad/src/lib.rs` in full — ten `pub mod` lines, no `tray_icon` among
them — and §5.2 principle 4 puts every user-visible string in one file, which is
why the rasteriser ships in `diagnostics.rs`.

**Observed:** the code block presenting `ICON_EDGE`, `IDLE`, `FAULT` and
`tray_icon` is headed `// crates/goad/src/tray_icon.rs — stratum 3`. No such
file exists, and the design contradicts itself eighty lines apart.

**Evidence:**

```
$ grep -n "crates/goad/src/tray_icon.rs" docs/slices/002/design.md
2538:// crates/goad/src/tray_icon.rs — stratum 3
$ ls crates/goad/src/
clock.rs controller.rs diagnostics.rs generated.rs glass.rs install.rs
lib.rs main.rs reception.rs startup.rs view_model.rs wire.rs
```

**Fix:** retitle the block `// crates/goad/src/diagnostics.rs` at design
reconciliation.

**Disposition:**
**Response:**

**Outcome:**

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

**Round 1, raiser's synthesis** — 2026-09-05. Superseded when the ledger
resolves.

**Blockers: 0. Majors: 2.** Minors 5, nits 4.

The two that matter are both about what the suite holds rather than about what
the code does. **F-1**: the body and the "shown as plain text" marker can each
be deleted from `app.slint` outright and all 124 tests in the `goad` crate stay
green, so AC-9's user-visible half — a degraded body is *shown*, and the person
is told — is asserted only at `Presentation` and never at the glass. **F-2**:
item 14d still gates on a bare 100 ms sleep where the sibling test one screen
away was repaired to observe the invocation log, and shrinking that sleep to 1
ms turns the pass into a failure; the gate `just check` is AC-1's, and it has a
race in it. Behind those, **F-5** is the same shape one level up: the runtime
arrangement the binary actually ships — Slint's executor polling a
`tokio::process` future under nothing but an `EnterGuard` — is instantiated by
`closing.rs` and then never driven, so the panic `research.md` Thread 4 measured
as invariant 4 failing has no regression test.

**The five invariants hold, and four of them hold by more than argument.**
*No domain understanding*: the vocabulary scan genuinely reaches the shipped
markup — planting `streak_count` and `habit options` in `app.slint` fails
`no_workspace_member_names_the_users_domain` naming both lines — and its one
uncovered surface is a crate name declared only in TOML (F-3), with one unnamed
lexer blind spot (F-4). *Permissive wire, canonical internals*: nothing
downstream of `read_response` re-parses; `Body::Rich` carries the retained parse
and `glass.rs::styled` clones it rather than calling `from_markdown` a second
time; the only string round-trip is the option id, which travels as a selector
and is matched back against the retained `OptionId`. *No narrowing*: `present`
has no refusal path and no `_` arm, and `Choice`'s three fields and `Opt`'s
three are each either drawn or reported — `Undrawn::OptionFields`,
`::MarkdownUnsupported`, `::ContentForm` — so R-20 is met by construction rather
than by vigilance; an HTML or URI body reaches the glass as its own bytes.
*A backend failure never takes the host down*: no `unwrap`, `expect`, index or
unchecked arithmetic is reachable from backend output in `crates/goad/src`;
`engaged` is cleared unconditionally by `absorb`; and I confirmed by running the
production arrangement that a stop tripped mid-exchange returns `serve`, exits
0, and leaves no orphaned child. *One-way strata*: `cargo tree -p
goad-semantics` resolves to `jiff`, `serde`, `serde_json` and nothing else, and
D25's feature-unification residue is one feature wide — `serde_core`'s `alloc`
is switched on across the workspace and off standalone. It admits no clock, no
filesystem and no runtime, so the residue is real and currently harmless.

**AC-1 verified independently:** `git archive HEAD` into a fresh tree, then
`just check` under `nix develop`, exits 0 with 293 tests passing and no source
file missing from the archive.


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
| F-1 | major | fix-now | verified |
| F-2 | major | fix-now | verified |
| F-3 | minor | fix-now | verified |
| F-4 | minor | fix-now | verified |
| F-5 | minor | \<proposed\> follow-up | pending endorsement |
| F-6 | minor | \<proposed\> doc-wrong | pending endorsement |
| F-7 | minor | \<proposed\> | contested |
| F-8 | nit | fix-now | verified |
| F-9 | nit | fix-now | verified |
| F-10 | nit | fix-now | verified |
| F-11 | nit | doc-wrong | verified |

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

**Disposition:** fix-now
**Response:** Two tests added, and a documented residual.

`tree.rs::the_degradation_marker_is_present_only_when_the_body_is_degraded`
(`crates/goad/tests/renderer/tree.rs:167-193`) asserts the marker absent at
`body_degraded(false)` and present at `body_degraded(true)`, both by
`accessible_label` — the same pairing `the_diagnostic_empty_state_is_present_
only_when_empty` uses. Break-and-revert below.

The `accessible_label` predicate for the *body* half does not work: verified
directly (`init_no_event_loop`, set a body, query
`ElementQuery::from_root(&window).match_inherits("StyledText").find_first()`,
call `.accessible_label()`) that it returns `None` regardless of content.
Traced to source: `i-slint-compiler` 1.17.1's `lower_accessibility.rs::
apply_builtin` only synthesizes a default `accessible-role`/`accessible-label`
for `Text`, `TextInput` and `Image` — not for `StyledText` — and the
language's `styled-text` type has no `Type::StyledText` member function or
string conversion (`typeregister.rs`) to bind one by hand. Upstream's own
`slint-1.17.1/tests/styled_text.rs` confirms the ceiling: it asserts
`component.get_text() == greeting`, the property round trip, never an
element's rendered content. So the raiser's own fallback —
`get_body()` round-tripped — is what's actually implementable without
touching `app.slint`.

Added `wiring::body_content` (`crates/goad/tests/renderer/wiring.rs`,
after `mod rows`): three tests drive a real view through
`read_response` → `present` → `Glass::present` (not `Prepared::presentation`,
which `mapper.rs`/`reception.rs` already cover) and read `window.get_body()`/
`get_body_degraded()` off the window `SlintGlass` actually wrote —
`a_plain_body_reaches_the_window_and_is_not_degraded`,
`accepted_markdown_reaches_the_window_rich_and_is_not_degraded`,
`rejected_markdown_reaches_the_window_as_plain_and_is_degraded`. This closes
`glass.rs::styled`'s two live arms and the `body-degraded` flag end to end,
which nothing previously exercised past `Prepared`.

**Residual, not closed:** neither addition detects `app.slint`'s
`StyledText { text: root.body; }` binding itself being severed.
`PromptWindow::get_body()` reads the `in property` `set_body` wrote,
independently of whether any element still binds to it — confirmed by
re-running the break below. Closing that specific break needs either a new
production property carrying the plain text alongside `body` (a design-shape
change) or an `i-slint-backend-testing` capability that does not exist in
1.17.1. I did not make an app.slint change beyond the break-and-revert itself,
per the brief. Recommend `follow-up` once a fix is possible, or accepting the
residual as `tolerated` with this rationale — the raiser's call.

Break-and-revert, both re-run against the expanded suite (`nix develop
--command cargo test -p goad --test renderer`):
- Marker deleted (`if root.body-degraded: Text { text: "shown as plain text"; }`
  removed): `the_degradation_marker_is_present_only_when_the_body_is_degraded`
  **fails** — `"the marker must appear once the body is degraded"`. Goes red.
- `text: root.body;` commented out: all 118 tests, including the three new
  `body_content` ones, **still pass** — confirmed still green, the residual
  above.

Both breaks reverted; `git status --short crates/goad/ui/app.slint` is clean.

**Outcome:** verified, with the residual accepted as stated.

I re-ran both breaks. Deleting the marker turns
`tree::the_degradation_marker_is_present_only_when_the_body_is_degraded` red at
`tree.rs:189` — 120 passed, 1 failed — so the marker half now goes red where it
went green in round 1. The residual is real, and I confirmed it independently
rather than on the responder's word: with a body of `"BODYMARKERTEXT"` set on a
live `PromptWindow`, `ElementQuery::from_root(&w).match_predicate(|_| true)`
returns ten elements, and none of `accessible_label`, `accessible_value`,
`accessible_description` or `accessible_placeholder_text` carries the text on
the `StyledText`:

```
type=Some("Text")        label=Some("HEADINGTEXT")   value=None desc=None
type=Some("StyledText")  label=None                  value=None desc=None
ANY ACCESSOR CARRIES THE BODY TEXT: false
```

The sibling `Text` element does expose its content, which is what makes the
`StyledText` result a capability gap rather than a query mistake. So the
responder's trace to `lower_accessibility.rs` is borne out at the API surface,
and the fallback they took is the only one available in 1.17.1.

**What AC-9 now holds, precisely.** Two chains, one complete and one not. The
*degradation* chain is complete: `body_content`'s
`rejected_markdown_reaches_the_window_as_plain_and_is_degraded` proves
`SlintGlass::present` writes `body-degraded`, and `tree.rs`'s marker test
proves the markup draws a marker exactly when that property is set — glass to
property to element, with a break-and-revert at each end. The *shown* chain
stops at the property: three tests prove the right `StyledText` reaches
`window.get_body()`, and nothing proves an element still binds to it. So AC-9's
"the degradation is reported" is held, and "is still shown" is held as far as
the renderer's own code and argued for the last hop. That is a smaller residual
than the finding raised — round 1 had neither chain — and it is the right place
to stop given the library.

I do not ask for `follow-up`: closing the last hop needs a production property
that exists only to be tested, which is a worse trade than the residual. If the
user prefers it recorded, `tolerated` with this rationale is the honest label.


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

**Disposition:** fix-now
**Response:** Applied exactly as prescribed
(`crates/goad/tests/renderer/wiring.rs:1207-1219`, in
`on_stop_a_command_queued_behind_the_exchange_is_left_unread`): the sleep is
replaced with `until(Duration::from_secs(1), || invocations(&log) >= 1).await`
ahead of `stopper.stop()`. The outer `run_until(async move { ... })` block
lost its `move` (to `async { ... }`) so `log` is borrowed by the `until`
closure rather than captured by value — it is still needed by the assertion
after the block returns; the inner `tokio::task::spawn_local(async move
{ ... })` keeps its own `move` and still takes ownership of `backend`,
`controller`, `rx`, `cancel`, `glass` as before. No other line changed.

There is no sleep left in this test for the 1 ms-shrink experiment to apply
to — confirmed by grep: the only `tokio::time::sleep` remaining in
`wiring.rs` is inside the `until` helper's own poll loop (`wiring.rs:145`, 5
ms, unrelated to this precondition). Ran the test in isolation and inside the
full `cancellation` module ten times (`cargo test -p goad --test renderer
cancellation:: -- --test-threads=1`, looped): 30/30 pass.

**Outcome:** verified.

The precondition is now observed, not slept:
`until(Duration::from_secs(1), || invocations(&log) >= 1).await;` at
`wiring.rs:1216-1219`, identical to VT-10's, with `run_until(async move {…})`
relaxed to `run_until(async {…})` so `log` can be borrowed. The round-1
experiment no longer applies because there is no duration left to shrink — the
test now waits on the fact it depends on, and fails with `until`'s own
"condition did not become true" message if the backend never spawns, which
names the precondition instead of misreporting it as the property. The
comment records the lineage to PL-17. `cargo test -p goad --test renderer`
passes.

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

**Disposition:** fix-now
**Response:** Added `goad_boundary::members::crate_name(manifest: &Path, text:
&str) -> Result<String, Breach>` (`crates/goad-boundary/src/members.rs`) —
text in, not a path read internally, the same shape
`crate::manifest::unpermitted` already takes, so a fixture can supply literal
text with no file on disk. Parses `[package].name`; an unparsable manifest or
one with no name is a `Breach::Unreadable`, never a panic (library code).

Test `vocabulary::no_member_manifest_names_the_users_domain_in_its_own_crate_
name` (`crates/goad-boundary/tests/checks/vocabulary.rs:60-81`) reads every
real workspace member's manifest and checks its `[package].name` against
`DOMAIN` with `mentions`. Positive control,
`a_forbidden_word_in_a_crate_name_is_caught` (`vocabulary.rs:83-96`): a
fixture manifest text with `name = "goad-habits"` is caught naming "habit" —
never a file on disk, matching `crate_name`'s text-in shape.

**Outcome:** verified.

The positive control is a fixture, so I checked the real direction too:
renaming `crates/goad/Cargo.toml`'s `[package].name` to `goad-habits` fails the
new check, naming both the value and the manifest —

```
panicked at crates/goad-boundary/tests/checks/vocabulary.rs:75:7:
"goad-habits" (…/crates/goad/Cargo.toml) names the domain word "habit"
```

— reverted after. Half my Fix was applied and the omitted half is right to
omit: I asked for `mentions` over each `workspace.members` *entry* as well, and
a directory name is not a crate name, which is what AC-14 lists. `crate_name`
taking text rather than a path matches `unpermitted`'s existing shape and keeps
the control off disk.

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

**Disposition:** fix-now
**Response:** Built the fuller fix, the fifth transition, not just the doc
note — kept the four-state design, no lexer.

`char_literal_len(bytes, at)` (`crates/goad-boundary/src/scan.rs:384-399`)
gives the byte length of a char literal opening at a `'`, if it is one rather
than a lifetime: an escape (`'\''`, `'\n'`) at four bytes
(quote-backslash-char-quote), a single byte at three (`'"'`, `'x'`, and
`b'"'`'s quoted half — the `b` itself is ordinary code one byte earlier).
Neither shape matches a lifetime (`'a`, `'static`), which has no closing `'`
at either offset, so it falls through unconsumed as before. Wired into
`code_of` as a new `State::Code if b == b'\''` arm (`scan.rs:300-306`) ahead
of the `//` and `/*` arms, consuming `char_literal_len(...).unwrap_or(1)`
bytes without ever entering `State::Str`.

Doc updated in place (`scan.rs:242-245`, `:260-275`) rather than D13 itself
(design.md is out of scope this round): the "Four costs" list is unchanged,
and a new paragraph states the fifth was open and is now closed, not merely
named, naming `char_literal_len`.

Positive controls, `vocabulary::a_char_literal_holding_a_quote_does_not_
desynchronise_the_cut` (`crates/goad-boundary/tests/checks/vocabulary.rs:
261-286`): the reviewer's own two false-negative fixtures (`'"'` and `b'"'`
hiding `habit`/`site` behind a `"//"`), plus `'\''` doing the same, all now
caught by `mentions`; and the false-positive fixture (`'"'; // the call
sites`) still correctly cut as a comment. `cargo test -p goad-boundary --test
checks vocabulary`: passes (this finding's own module; the whole-binary
count is stated once, at the end of the ledger's Response note, since it
moves as later findings' tests join it).

**Outcome:** verified — closed rather than merely named, which is more than
the finding asked for.

Fourteen inputs through the shipped `mentions`/`code_of`, all as expected:

```
ok  F-4 case 1                          let q = '"'; let s = "//"; let habit = 1;   → true
ok  F-4 case 2 (byte char)              if b == b'"' { let s = "//"; … site …   }   → true
ok  escaped-quote char literal          let q = '\''; let s = "//"; let habit = 1;  → true
ok  escaped double-quote char literal   let q = '\"'; let s = "//"; let habit = 1;  → true
ok  real comment past a char literal    let q = '"'; // the call sites              → false
ok  lifetime opens no string            let x: &'static str = "ok"; // habit        → false
ok  two lifetimes / impl<'a> Foo<'a>                                                → false
ok  multi-byte, hex-escape, ordinary char literals                                  → true
```

Both directions are fixed: the false negative that hid the rest of a line, and
the false positive where a real `//` past a char literal was not cut. Lifetimes
are untouched, including the pre-existing control. `char_literal_len`'s two
shapes are the only Rust constructs that put a bare `"` in code, so the class is
closed, not just the instances I supplied.

**One input that still defeats it, and D13 names it.** Line two of a
multi-line string: `   continued string line // habit` returns `false`, because
the cut is per-line and does not track a string across the break. That is cost
one of the four in `code_of`'s own list and in D13, unchanged and correctly
named. The doc now records the char-literal case as a fifth cost *and* says it
is closed, which is the right way round.

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

**Disposition:** <proposed> follow-up — awaiting user endorsement
**Response:** Not repaired here, per the brief: a test driving one exchange
through the real production topology (multi-thread runtime, `runtime.enter()`
held, `slint::spawn_local`, `run_event_loop_until_quit`) in the `event_loop`
target is a new test tier this crate does not otherwise carry — every other
test either runs headless with no event loop (`renderer`) or, per
`closing.rs`'s own header, exercises the composition minus the part that
actually drives an exchange. Adding it is a real flakiness surface (a real
event loop, a real spawned process, real wall-clock scheduling) that the
brief's dispositioning rule reserves for the user rather than a unilateral
`fix-now`.

Rationale for `follow-up` over `tolerated`: the reviewer's own reproduction
shows the gap is real and cheap to close (their scratch binary already proves
the shape works), so the risk is worth carrying forward as owned work rather
than accepted permanently with no plan to close it. If endorsed, the
follow-up note belongs in `slice-002.md`'s Follow-ups at close, in the shape
the reviewer's Fix already gives: extend `closing.rs` to enqueue one
`Command::Evaluate(Stimulus::Startup)` against
`tests/backends/answers-as-instructed.sh` with a pinned view, assert the
heading, then dispatch `CloseRequested`.

**Outcome:** pending endorsement — I would accept `follow-up`.

The reasoning is sound and I do not contest it. A test that drives one exchange
through a real event loop and a real spawned process is a new flakiness surface,
and PL-14's rule puts that with the user. Two things for whoever endorses it.

First, the gap is narrower than "untested": my round-1 reproduction shows the
shipped code is correct in the arrangement it ships in, including cancellation
mid-exchange with no orphaned child. What is missing is a regression test, not a
working path.

Second, the flakiness objection is smaller than it looks. `closing.rs` already
runs a real headless event loop and already awaits an unbounded
`run_event_loop_until_quit`; adding one `@slow-view`-style instruction adds a
process spawn to a file that already blocks on the loop. If the user declines,
`tolerated` with the reproduction recorded would be honest; abandoning it with
no note would not.

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

**Disposition:** <proposed> doc-wrong — awaiting user endorsement
**Response:** Changed nothing, per the brief. Both options, and my pick:

- **Drop `named`** from `Refused::SupersededView`/`::UnknownOption`
  (`diagnostics.rs:54,57`) and from the three construction sites in
  `controller.rs::answer` (`:177-192`). Matches the two rendered sentences
  design.md already pins (`:2186-2188`) exactly as they stand; the design's
  own declared shape for the two variants (`:2082-2090`) is what would need
  correcting to match.
- **Render `named`**, bounded the way a markup-supplied string already is
  elsewhere (R-14's label bound, say), in both sentences. Matches the
  declared shape; the two pinned sentences (`:2186-2188`) are what would
  need correcting.

**My pick: doc-wrong, in the drop-the-field direction.** The design's two
declared sentences are dated `:2186-2188`, after the field's own declaration
at `:2082-2090` in the same document — nothing suggests the sentences are the
stale half. And the asymmetry with `StateError::NoOutstandingView`
(stratum 2, host-side) rendering its own `named` is not obviously a mistake:
that error reaches a person positioned to act on a stale identifier (a
backend integrator debugging a protocol mismatch), where `SupersededView`/
`UnknownOption` are host-internal races a user cannot act on regardless of
which id is named — "no action taken" is the whole of what matters to them.
So I read this as the sentences being right and the field being the
oversight, not the reverse. Held rather than fixed, since it is a design's
declared shape and canon is amended only with explicit endorsement — never
mid-slice on the responder's own initiative.

**Outcome:** pending endorsement — I would accept `doc-wrong` in the
drop-the-field direction, which is the responder's own pick.

Their argument is better than the one in my Fix. I reasoned from the asymmetry
with `StateError::NoOutstandingView`, which renders its `named`; they point out
that the two are addressed to different readers — a backend integrator who can
act on a stale identifier, versus a person watching a host-internal race — and
that "no action taken" is the whole of what the second reader can use. That is
the right distinction and it settles which half of `design.md` is stale.

One correction to their reasoning, which does not change the conclusion: line
order in a document is not evidence of which statement is later in time.
`design.md:2082-2090` and `:2186-2188` are one document written and rewritten
across four review rounds. The conclusion stands on the reader argument alone.

If endorsed, the change is: drop `named` from `Refused::SupersededView` and
`::UnknownOption`, drop the three `to_owned()` sites in `controller.rs::answer`,
and amend `design.md:2082-2090` to match. The two pinned sentences do not
move.

**Round 3 — disposition: `doc-wrong` (endorsed PL-18).** `named` dropped from
both variants — `SupersededView` and `UnknownOption` are now unit variants —
in `crates/goad/src/diagnostics.rs:54,57`. The three `to_owned()` construction
sites in `crates/goad/src/controller.rs::answer` (`:177`, `:181`, `:190`)
drop the field with them. Four call sites that built the removed field
updated to match: `crates/goad/tests/renderer/reception.rs:282,577,629,639`
and `crates/goad/tests/renderer/wiring.rs:177` (a `{ .. }` pattern, now the
bare variant). No test asserted the field's value; nothing was deleted, only
the field literals at construction sites.

`design.md:2082-2090` (§5.4, "The diagnostic surface") still declares the
field on both variants — the drift is not reconciled here, per the brief:
canon is amended only with explicit endorsement during audit, not mid-repair.
`audit.md`'s *Design drift not reconciled* list carries this.

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

**Disposition:** <proposed> — awaiting user endorsement
**Response:** Checked the prescribed fix against the actual visibility
boundary, and it does not reach: `report_startup(error: &StartupError)` and
`report_platform(detail: &str)` (`diagnostics.rs:296,313`) hardcode
`std::io::stderr().lock()` internally — neither takes a sink at all, only
`line_to` does, and `line_to` is a lower-level primitive with no
`"goad: {error}"`/`"goad: the window could not be drawn: {detail}"`
formatting of its own. A test that calls `line_to` directly with a
hand-written expected string would test that `line_to` writes what it is
given, not that `report_startup`/`report_platform` compute the right string
— exactly the vacuous-test shape review brief item 9 warns against.

Also: `pub(crate)` would not be reachable from where the brief says to put
the test. `tests/renderer/startup.rs` is an integration test — a separate
crate that already writes `use goad::diagnostics::{USAGE, print_usage};` —
and `pub(crate)` items in `goad`'s lib crate are invisible across that crate
boundary. Confirmed by the same pattern already established throughout this
codebase: every existing seam these tests reach (`USAGE`, `print_usage`,
`StartupError`, `arguments`) is `pub`, not `pub(crate)`, for exactly this
reason.

So this needs a real, if small, signature change to be testable at all,
which the brief's own fallback covers: propose it and hold rather than
`fix-now`. Smallest proposal: add `pub fn report_startup_to(sink: impl
std::io::Write, error: &StartupError)` and `pub fn report_platform_to(sink:
impl std::io::Write, detail: &str)`, each one line delegating to `line_to`;
`report_startup`/`report_platform` become one-line callers of these with
`std::io::stderr().lock()`, unchanged in their own signature and behaviour.
Two new `pub` functions, additive, no existing string or signature disturbed
— but new production surface, which is why I did not add it unilaterally.
Changed nothing in `src/`.

**Outcome:** contested — narrowly, and I withdraw two-thirds of my own Fix.

The responder is right twice and wrong once.

Right that `line_to` is not the seam. It receives an already-composed string —
`report_startup` calls `line_to(sink, &format!("goad: {error}"))` — so a test
driving `line_to` would assert that it writes what it is handed, which is the
vacuous shape I named in the brief's own item 9. Right, too, that `pub(crate)`
is unreachable from `tests/renderer/startup.rs`, which is a separate crate. Both
halves of my Fix are withdrawn.

Wrong that this "needs a real, if small, signature change to be testable at
all". For `report_startup` it needs none. Cargo sets `CARGO_BIN_EXE_goad` for
every integration test target in the crate, so `startup.rs` can run the real
binary and assert the real stderr and the real exit code:

```
$ ./target/debug/goad a b
goad: too many arguments: goad takes at most one, the path of the configuration
file; run `goad --help` for usage
exit=2
$ ./target/debug/goad /nonexistent/goad.toml
goad: configuration could not be read: No such file or directory (os error 2)
exit=2
```

Both were run against the built binary. That is stronger than a sink test: it
covers the `goad: ` prefix, `StartupError`'s `Display`, the `ExitCode::from(2)`
decision, and the fact that a usage error does not reprint the usage block —
and it is the only route that touches `main`, `run` and `start`, which
`design.md:475-477` records as "the only items in the crate no test drives".
Nothing in production changes.

`report_platform` is a different case and the responder's proposal is the right
one there: it fires only when `show()`/`hide()` fails after the loop starts,
which no test can induce and no subprocess can reach, so `report_platform_to`
(or an equivalent sink parameter) is the only way. That half stays held.

**What I am handing back:** split the finding's repair. `report_startup` is a
`fix-now` with no production change, by subprocess. `report_platform` remains
`<proposed>` for endorsement, and the proposal should say it is one function,
not two — the case for `report_startup_to` is gone.

**Round 3 — disposition: `fix-now` (endorsed PL-18, §9 item 17 kept — no test
runs the binary).** PL-18 rejected the subprocess route this finding's own
Outcome preferred for `report_startup` ("relaxing §9 item 17 ... the rule was
chosen in design"), so both outlets take the same seam rather than splitting
across two testing strategies. Each outlet gains a pure sibling that composes
the exact string with no destination, and the outlet itself becomes a
one-line caller of it:

- `report_startup_line(error: &StartupError) -> String` and
  `report_platform_line(detail: &str) -> String`
  (`crates/goad/src/diagnostics.rs:301`, `:318`), each `#[must_use]` and
  `pub` (required: the asserting tests live in the separate
  `tests/renderer` integration crate, which cannot see `pub(crate)`).
  `report_startup` and `report_platform` (`:306-308`, `:322-324`) are
  unchanged in signature and behaviour, now delegating to their `_line`
  half before handing the result to `line_to`. `line_to` itself, its sink
  parameter, and its `Ok(()) | Err(_) => ()` spelling are untouched.
- Tests, `crates/goad/tests/renderer/startup.rs`, new `stderr_outlets`
  module: `report_startup_line_is_the_error_prefixed_with_goad` (two
  `StartupError` variants, against the literal `"goad: ..."` sentences
  design.md §5.4 pins, not re-derived through `Display`) and
  `report_platform_line_is_the_detail_in_its_sentence` (one literal). No
  sink faked, no subprocess, no `CARGO_BIN_EXE_*`.

`main.rs` and `glass.rs` call sites are unchanged; `main.rs` still holds only
`main`, `run`, `start`.

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

**Disposition:** fix-now
**Response:** Took the `const`-lift option's spirit rather than moving
`describe_outcome` wholesale: PL-4 moved it to `harness.rs` specifically
because `driving.rs`'s own callers stopped needing it, so moving it back
would reopen that. Instead, extracted the two sentences `no_view` and
`describe_outcome` actually share into one function, `driving.rs`'s own
`no_view` renamed to `pub(crate) fn failure_or_nothing(outcome: &Outcome) ->
String` (`tests/support/driving.rs:190-204`) — legitimately shared, since
both files really do call it, unlike the third arm.

`harness.rs::describe_outcome` (`crates/goad-shell/tests/integration/
harness.rs:200-215`) now matches `(&outcome.failure, &outcome.view)`: its
`(None, Some(presented))` arm keeps "a view carrying {id}" locally, and the
`(_, _)` arm calls `crate::driving::failure_or_nothing(outcome)` — this
preserves the original three-way match's exact precedence (a failure
present, with or without a view, still reports the failure first) since
`failure_or_nothing`'s own two arms are exactly the other two original
sentences.

`crate::driving::choice`/`presented` (`driving.rs:210-256`) call
`failure_or_nothing` at their two former `no_view` call sites; no text
changed. Ran both consumers: `cargo test -p goad-shell --test integration`
(58 passed) and `cargo test -p goad --test renderer` (121 passed) — the
second confirms `driving.rs`'s other consumer is unaffected.

**Outcome:** verified.

The two sentences are stated once, in `driving.rs::failure_or_nothing`, and
`harness.rs::describe_outcome` now calls it for both arms rather than restating
them; the third arm, `"a view carrying {id}"`, stays with the tier that is its
only user. The rewritten match is behaviour-preserving: `(Some(_), _)` and
`(None, None)` both fall to the wildcard, which is exactly what
`failure_or_nothing` distinguishes. The file's own D23 citation is now
consistent with what the file does. `cargo test -p goad-shell --test
integration`: 58 passed, unchanged.

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

**Disposition:** fix-now
**Response:** `crates/goad/src/clock.rs:12`: "PHASE-07's" → "PHASE-10's".

**Outcome:** verified. `clock.rs:12` reads PHASE-10, matching `wire.rs:5-6`
and `controller.rs:1-5` and the commit that landed `serve`.

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

**Disposition:** fix-now
**Response:** Recursed rather than asserting flatness — a walk that keeps
working when a subdirectory arrives is stronger than one that refuses to
run once it does. `subject_files()` now calls a new `walk_rs_files(dir,
files)` (`crates/goad-boundary/tests/checks/structure.rs:29-59`), a small
recursive descent restated locally rather than reusing `Scan::walk`:
that method is private and coupled to `Scan`'s own token-breach collection,
where this one only lists paths (checked directly — `walk` is not `pub`,
so `crate::scan::Scan::walk` is not reachable from this test target at all).
The prior non-vacuity control, `the_subject_directory_is_found_and_is_not_
empty`, is unchanged.

Added the presence half E-1 pairs with it:
`the_walk_descends_into_a_subdirectory` (`structure.rs:96-115`) runs
`walk_rs_files` against a new fixture,
`crates/goad-boundary/tests/fixtures/structure/nested/marker.rs` (a `.rs`
file one directory down), and asserts it is found — proving the recursion
actually descends rather than only being written to, the same discipline
the reviewer's own finding named. `cargo test -p goad-boundary --test
checks structure`: 8 passed, 0 failed (up from 7).

**Outcome:** verified, and the stronger of the two repairs I offered was
taken. `subject_files()` now descends, so the check keeps working when a
subdirectory arrives rather than refusing to run once one does, and
`the_walk_descends_into_a_subdirectory` is the presence control E-1 pairs with
the existing vacuity guard — it finds `nested/marker.rs` one level down, so the
recursion is demonstrated rather than asserted. The docstring no longer claims
that a human reading a diff closes the gap. `cargo test -p goad-boundary --test
checks`: 32 passed.

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

**Disposition:** doc-wrong
**Response:** Confirmed already tracked: `audit.md`'s *Design drift not
reconciled* list already names this exact drift. Changed nothing here, per
the brief — `design.md` is out of scope this round, and reconciliation is
this project's own named later step, not mid-slice on the responder's
initiative.

**Outcome:** verified. `audit.md:257-262` names this drift under *Design
drift not reconciled*, including the confirmation that no `tray_icon.rs` exists
in the tree, so it is owned by the step that owns design reconciliation.
Nothing more is owed here.

## Round 1 — response

Responder's note, 2026-09-05. Six findings repaired (`fix-now`): F-2, F-3,
F-4, F-8, F-9, F-10 — each verified green against the affected test
binaries, and the workspace-wide gate (`just check`) re-run after all of
them together, exiting 0. F-1 (`fix-now`) is repaired for the
degradation-marker half and partially for the body-content half; a residual
is documented in its own Response rather than hidden, since closing it fully
hit a real limit in the testing library this repair could not lift without a
production markup or property change outside the scope given.

Three findings are held for the user rather than dispositioned unilaterally,
each because the repair (or the only repair that actually closes the gap)
would change something canon or the brief reserved to the user:

- **F-5** (`<proposed> follow-up`) — the fix is a new test tier (a real event
  loop, a real spawned process), which is a standing risk this project treats
  as a call for the user, not the responder.
- **F-6** (`<proposed> doc-wrong`) — both repairs (drop the field, or render
  it) change a shape or a sentence `design.md` itself declares; I say which
  I'd pick and why, but change nothing.
- **F-7** (`<proposed>`, no doc-wrong/fix-now call made) — the prescribed
  repair does not actually work as specified (the visibility level named is
  unreachable from the crate the test would need to live in, and the two
  functions in question take no sink to begin with), so this is a smallest-
  signature-change proposal, not a disposition choice.

One finding is disposed with no repair needed: **F-11** (`doc-wrong`) is
already tracked in `audit.md`'s own drift list, so nothing further is added
by this response.

Test totals after every `fix-now` repair above, each binary run whole and
each verified against the original commit (`git archive 226a923`, run
unmodified, for the "was" figure — not estimated): `cargo test -p
goad-boundary --test checks` — 32 passed (was 28: F-3 +2, F-4 +1, F-10 +1);
`cargo test -p goad --test renderer` — 121 passed (was 117: F-1 +4 — the
marker test plus three `body_content` tests; F-2 is a rewrite, no count
change); `cargo test -p goad-shell --test integration` — 58 passed,
unchanged (F-8 is a rewrite). `just check` exits 0.

`#[expect]` count outside `generated.rs`: 0 (unchanged). No `#[allow]`
introduced. No dependency added. `flake.lock` untouched by any of the
above — its modification predates this response and is left exactly as
found, per instruction.

## Round 2 — verification

Raiser's note, 2026-09-05. Subject: commit `a2f3a7e` on `slice-002`, and the
working tree at it. Every claim below was re-run rather than read.

**Verified: eight.** F-1 (with a named residual), F-2, F-3, F-4, F-8, F-9,
F-10, F-11. **Contested: one.** F-7, narrowly — see its Outcome; two-thirds of
my own round-1 Fix is withdrawn there, and the part I hand back is that
`report_startup` needs no signature change to be tested. **Pending the user:
two.** F-5 and F-6, both held correctly under PL-14, both of which I would
accept as proposed.

**Nothing new raised.** No F-12. I looked for defects the repairs introduced and
found none worth a finding: `crate_name` is additive `pub` API in the test-only
member; `walk_rs_files` is a local recursion that lists paths and cannot collect
a breach, so it does not duplicate `Scan::walk`'s contract; the rewritten
`describe_outcome` match is behaviour-preserving; and `char_literal_len` leaves
every lifetime shape in the tree untouched, which the pre-existing control and
three new ones assert.

**Independent re-runs.**

| what | result |
|---|---|
| `just check` after all repairs | exits 0, 301 tests (was 293), no compiler warning |
| marker deleted from `app.slint` | `the_degradation_marker_is_present_only_when_the_body_is_degraded` fails at `tree.rs:189`; reverted |
| `[package].name` set to `goad-habits` | `no_member_manifest_names_the_users_domain_in_its_own_crate_name` fails naming the word and the manifest; reverted |
| every accessible accessor on a live `StyledText` | all four `None`; the sibling `Text` returns its content |
| fourteen `code_of` inputs, both directions | all as expected; the one remaining miss is D13's named cost one |
| `./target/debug/goad a b` and a missing config path | both print `goad: …` on stderr and exit 2 |

`git status --short` is `M flake.lock` only. Two stashes on the stack, untouched.
Nothing committed.

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

**Round 2, raiser's synthesis** — 2026-09-05.

**No blockers, and none were ever raised.** Both majors are closed. F-1's
degradation chain now runs end to end — the glass writes `body-degraded`, the
markup draws a marker exactly when it is set, and deleting either end turns a
test red — where in round 1 the whole body path could be deleted without the
suite noticing. F-2's precondition is observed rather than slept, so the gate no
longer carries a race. The three minors that were `fix-now` are closed and, in
two cases, closed harder than asked: F-4 fixes the lexer instead of documenting
the blind spot, and F-10 recurses instead of asserting flatness.

**What is left standing.** One residual inside a verified finding: the shipped
markup's `text: root.body` binding can be severed without any test noticing,
because `i-slint-backend-testing` 1.17.1 exposes no accessor on a `StyledText`
element and the language has no `styled-text`-to-`string` conversion to build
one. I verified that ceiling directly. Closing it needs a production property
whose only purpose is to be tested, which is the worse trade; so AC-9's "the
degradation is reported" is held by test and "is still shown" is held to the
property boundary and argued for the last hop. That is the honest statement of
where the renderer's evidence stops.

**What awaits the user: three decisions, none of them urgent.** F-5, whether the
production runtime topology earns a regression test in `closing.rs` — the code
is correct there today, verified by reproduction; only the guard is missing.
F-6, whether to drop `named` from two `Refused` variants and amend the design's
declared shape, or to render it and amend two pinned sentences; both the
responder and I would drop the field. F-7, now split: `report_startup`'s exact
string and exit code can be asserted by running the built binary with no
production change at all, which is a `fix-now`; only `report_platform` needs a
new sink seam and needs endorsement.

**The five invariants stand where round 1 left them, with invariant 1 better
held than before.** The vocabulary scan now reads each member's crate name as
well as its sources, closing the one surface AC-14 named and nothing checked,
and its comment cut no longer loses a line to a char literal. Nothing in the
repairs touched production behaviour: `crates/goad/src` is byte-identical to the
commit round 1 reviewed apart from one word in a doc comment.

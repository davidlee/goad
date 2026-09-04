# Notes — Slice 002

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| — | no phases. `plan.md` is not begun; the slice is still at the design gate. | 2026-09-05 |

## Phase sheets

<!-- One block per phase, written at phase-plan time, immediately before
     execution. Disposable detail — it exists to get one agent through one
     phase. -->

### PHASE-01 — <name>

**Objective:** <copied from plan.md>

**Reading list**
<!-- path:line references, the design sections that bind, prior art. -->

**Assumptions & STOP conditions**
<!-- What is being taken on faith, and the specific conditions under which the
     agent must stop and consult the user rather than improvise. -->

**Tasks**
<!-- [ ] todo · [~] in progress · [x] done · [!] blocked -->
- [ ]

**Decisions taken during execution**
<!-- Small and local: how, within what the design already settled. A choice that
     changes the design is not one of these — stop, consult the user, and record
     it in `design-log.md`. -->

**Findings**
<!-- Things noticed in passing that are not this phase's job: a defect
     elsewhere, drift from the design, a surprise. Defects in this phase's own
     work get fixed, not recorded. These feed the audit; the ones that outlive
     the slice become Follow-ups. -->

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** <yyyy-mm-dd> · <phase or stage> · <commit>

### Produced
<!-- What now exists: modules, contracts, docs. -->

### Learned
<!-- Durable facts a future agent would otherwise rediscover. Candidates for
     `docs/memory/`. -->

### Open
<!-- Still unresolved at this point. Candidates for follow-ups. -->

---

## Handover

**Written:** 2026-09-05, end of session 1. **Branch:** `slice-002`, six commits
ahead of the last slice-001 commit `a6ae617` (four of them round 3's). **Gate:** `just check` exits 0.
**Tree:** clean apart from a pre-existing unstaged `flake.lock` edit (a `bun2nix`
input repointed to a `Mic92` fork) that predates this session and was not
touched.

The slice reached the design gate and stopped there. `plan.md` is still the
template; no phase has been written or run; no code has changed. What exists is
a design that survived three adversarial review rounds, and a fourth round it
has not had.

**Why that was the right call.** The design gate exists to stop a half-built
tree, and two of round 3's own repairs were measured against a compiler and
found wrong — a defect rate of 2 in 3 in text that round 3 itself wrote from
scratch, which makes overnight execution against the two rewrites *nobody* has
built a bet on unread text. Converging the design costs one more session; a
phase that stops mid-flight against a wrong signature costs the phase plus the
repair of whatever it half-landed.

### 1. What the three review rounds changed

Reviewer for all three: codex `gpt-5.6-sol`, read-only, briefed for
implementation feasibility rather than intent. Ledger: `review-design.md`.

| round | raised | reopened | pattern found |
|---|---|---|---|
| 1 | F-1…F-12 | — | the design was written from `research.md`'s summaries rather than from `src/` |
| 2 | F-13…F-19 | 7 of 12 | repairs written at the level of *intent* rather than of signatures |
| 3 | F-20…F-28 | 4 of 14 | repairs correct in place, contradicting an **unrepaired neighbour** |

Round 3's pattern is the one a repair round *creates*, and it is the reason a
fourth is owed rather than assumed: `A-5` still pointing at a crate-level lint
override `D8` had just forbidden (F-16); a `Closed` arm still quitting a loop
the same page says has exactly one quit (F-20); an `engaged` flag with a setter
and no clearer (F-21); a slice non-goal saying slice 003 owns the clock beside a
design that adds one (F-23); new canon drafted in the file whose own preamble
forbids it (F-24).

**The structural yield.** Six repairs turned a rule an agent must remember into
a shape the types or the arithmetic enforce. These are the ones round 3 audited
against the code and held, and they are the real product of the three rounds:

- `receive` is the **only** consumer of an `Outcome`, so I-2 (every `Undrawn`
  reaches the diagnostic surface) cannot be forgotten.
- `Surface` is *derived* from `(focus, shown)`, so "window unchanged" has one
  meaning.
- `serve` is one function that production and the cheap test tier both call, so
  a loop-body change cannot pass in test and fail in production.
- `Command::Choose` carries its view token, so a stale answer is refused by a
  comparison rather than by timing.
- `Pending` makes the raced future *be* the exchange, so "cancellation drops the
  exchange" is a fact about the code, not about a wrapper.
- The tray icon's geometry is integers, so "a rule regenerates the asset" is
  arithmetic a test can assert.

**Three assumptions were measured rather than carried, and two were wrong.**

| assumption | outcome |
|---|---|
| **A-5** — `fn serve(..) -> impl Future` dodges `clippy::future_not_send` | **false** (F-27). It fires once the future is really `!Send`, and that shape *additionally* trips `clippy::manual_async_fn` from `clippy::all`. `serve` is now an `async fn` carrying one `#[expect]`. Building it also pinned a deadlock: `Wire::send` must be `try_send`, never `send().await`. Evidence: `research.md` Thread 7 |
| **A-6** — `Window.title` bound to a conditional compiles | **discharged.** The fallback is dropped |
| **A-7** — §5.2's markup compiles and its generated API is what §5.3/§5.4 assume | **discharged, and it found F-28**: `Tray` generated **no** `set_icon` and **no** `set_tooltip`, with `icon`/`title`/`tooltip`/`visible` all `set_constant()`. Every tray behaviour had no route to the component, and E-4's `hide()` panic trap was still open because a literal `visible: true` constant-folds. Repaired by declaring `image`/`hover-text`/`shown` and binding the builtins to them, then re-extracting and rebuilding. Evidence: `research.md` Thread 8 |

The transferable result, and the one to carry into round 4: **an assumption a
scratch crate can reach should be reached before a phase starts, not listed as a
risk.** "The first renderer commit will tell us" is a real mitigation and also a
way of not finding out.

### 2. Findings — closed, held, and unreviewed

All 28 findings are `verified` or terminal. **No blocker is outstanding.** The
ledger is nonetheless `open`, and the distinction matters:

- **Closed and audited (15).** F-2, F-3, F-10, F-11, F-12 closed at round 2.
  Ten round-2 repairs were audited against `src/` in round 3 and **held**: F-1,
  F-4, F-5, F-7, F-8, F-13, F-14, F-15, F-18, F-19.
- **Repaired in round 3, unreviewed by anyone but their author (13).** F-6, F-9,
  F-16, F-17 (reopened under their original ids) and F-20…F-28. Of these, F-27
  and F-28 rest on built evidence; the other eleven rest on reading.
- **Unbuilt *and* unreviewed (2 passages).** These are the concrete blockers:
  - **F-9's repair** — the failure-matrix `Case`/`Observed`/`Cohort` schema,
    `design.md` §9 item 12.4. It is ordinary Rust: a scratch crate can
    instantiate the ~30 rows against the real `Display` strings and prove the
    schema can state every assertion the rows make (two-channel rows C2, T3, P2;
    T1's own-`Host` exemption; the `Exact`/`Prefixed` split).
  - **F-26's repair** — the startup surface, `design.md` §5.4 "The exact
    strings": `StartupError`'s eight variants (`Usage`, `NoConfigPath`,
    `Config`, `Clock`, `Runtime`, `Platform`, `EventLoop`, `Enqueue`), `Launch`,
    `arguments(argv, env)` and its four-row table, and the `writeln!` outlet. The
    outlet spelling in particular is a **guess against a deny-all lint table**:
    `print_stdout`, `print_stderr`, `let_underscore_must_use` and
    `unused_must_use` are all `deny`, and the chosen
    `match writeln!(..) { Ok(()) | Err(_) => () }` has never been compiled under
    them.

### 3. What else is open

- **A-2 is still standing** — ~75 unproven lints against hand-written renderer
  code — and has already spent **one of its three permitted expectations** on
  F-27's `future_not_send`. Two named instances are ordinary Rust and cheap to
  check and have not been: the stderr outlet above, and the **six distinct
  `Wire` clone bindings in `install()`** (`design.md`:1117) against
  `shadow_unrelated`, which is `deny`. The stop rule stands: the third distinct
  `expect` outside the generated-code quarantine stops the phase.
- **A-4 is still standing** — `just check` wall-clock with 411 crates in the
  tree (ADR-002 T3). Not answerable by a spike; it needs the real tree, and it is
  what the first renderer commit is actually for.
- **D25 is an admission, not a risk.** No instrument in the gate rejects a
  feature switched on by stratum 2 or 3 in a dependency shared with stratum 1.
  The design states the rule and states that nothing enforces it. That shape is
  deliberate — it is what F-6 was raised three times to get right.
- **`plan.md` is not begun.** Under the 2026-09-04 scope extension every phase
  must be executable with no user present: entry and exit criteria
  machine-checkable, STOP conditions stated as conditions an agent can
  recognise. A phase that cannot be written that way is a finding against the
  design.

### 4. What the user must decide

Everything here is a **canon act**, and canon is the one thing the autonomy
grant explicitly withholds. Nothing under `docs/specs/`, `docs/policy/` or
`docs/adr/` was created or edited this session.

| # | movement | vehicle | needed |
|---|---|---|---|
| CD-1 | a new ADR **superseding** ADR-002 (the split) | `canon-delta.md` | the *decision* is endorsed (`design-log.md` 2026-09-05); the **wording** is not |
| CD-2 | ADR-002's stated reason for expecting T1 is measurably false; the superseding ADR states the real ground | `canon-delta.md` | with CD-1 |
| CD-3 | SPEC-001 has no rule at the glass — R-20's no-silent-dropping stops before the renderer | `canon-delta.md` | at audit |
| CD-4 | SPEC-001 §7 names the fixture directory normatively; moving it is a canon change | `canon-delta.md` | at audit |
| CD-5 | `CLAUDE.md`'s gate pointer moves off a **closed slice's design** (`docs/slices/001/design.md` §9) | `canon-delta.md` | **lands with the policy below, or neither lands** |
| CD-6 | `CLAUDE.md` invariant 1 — the boundary test must grow to grep `.slint` | `canon-delta.md` | at audit |
| CD-7 | `CLAUDE.md`'s "both feature columns" becomes false the day the split lands | `canon-delta.md` | at audit |
| — | **new** policy: the phase gate, six commands, four enforcement instruments and one named residue | `draft-policy.md` | **paired with CD-5** |

Two things to decide, then:

1. **Endorse the canon movements** (or the timing of them). CD-5 and
   `draft-policy.md` are a pair by construction: applying CD-5 alone leaves
   `CLAUDE.md` pointing at nothing, promoting the draft alone leaves two
   claimants to the gate.
2. **Accept or refuse a fourth review round before `plan.md`.** The
   recommendation is to run it; §5 says what it should read.

Already granted and needing nothing further: the crate split (2026-09-05), the
tray-plus-window shape (2026-09-05), the font package in `flake.nix`
(2026-09-05), and the dependency set — `slint`, `slint-build`, the Slint testing
dev-dependency.

### 5. Decisions taken under the autonomy grant, and where each is recorded

The grant is `design-log.md`, 2026-09-04, "Gate autonomy: an explicit deviation
from `docs/AGENTS.md`" — decide everything except canon, and record it as a user
decision would be recorded. Four vehicles were used, and nothing was decided
outside them:

| vehicle | count | what it holds |
|---|---|---|
| `design-log.md` | **41** design decisions, each marked *"Autonomy grant"* with Asked / Decided / Why / Rejected / Consequence | the decisions themselves |
| `review-design.md` | **28** dispositions across three rounds, ids immutable, each with its reason | finding dispositions |
| `canon-delta.md` + `draft-policy.md` | **7** movements + 1 new policy, **drafted, not applied** | canon consequences |
| `notes.md` | 0 | phase-local decisions — there are none, because there are no phases |

Three decisions on this branch are the **user's own**, not the agent's, and are
marked as such: the ADR-002 T1 split, tray-not-window, and the devshell font.
Round 3's synthesis records that none of the three was ever challenged on its
merits, and that no finding touched the guiding principles.

The 41 grant decisions, by heading (`grep -n '^### ' docs/slices/002/design-log.md`):

*The split and the gate* — the gate is six commands and `-p goad-semantics`
earns its place · the workspace invariant checks get their own member · members
are enumerated, not listed; R7 is retired · the renderer inherits the workspace
lint table unchanged · Round 3: four instruments for stratum 1, and the claim
narrowed to fit · Round 3: the new gate policy is drafted from the policy
template.

*The state machine* — the presentation transition is a total function, and
cleanup is not one of its inputs · `(view: Some, failure: Some)` is unreachable
and is still written total · `Command::Choose` carries the view token · the
window has one derived surface value, and a new question outranks a record · one
consumption point for an `Outcome`, and it runs the mapper.

*The runtime seam* — shutdown leaves the command channel; `serve` is one
function both tiers call · the queue policy is four mechanisms, and only one of
them is the safety mechanism · a callback holds a `Wire`, and `busy` and
`notice` are two properties · four shutdown sources, one path; `dismissed()` is
deleted · `serve` is a plain fn returning `impl Future` **(superseded)** ·
Round 3: the loop was built, and it changed `serve`'s signature · Round 3: three
seams closed, one shape each.

*The glass* — the glass is one total method, and the component is never
recreated · Markdown is parsed once and the parse is retained · `ContentForm` is
two variants and no payload · the tray icon is a rule with no artefact · Round 3:
the icon has numbers and the startup surface has strings · Round 3: the Slint API
is read from the compiler, not inferred · Round 3: the markup was compiled, and
the tray could not be written.

*The diagnostic surface* — the display bound is applied last, and counted in
characters · two truncations, two statements · stderr alone reports without
raising fault; a renderer refusal does raise it.

*Startup and the clock* — the clock is a `fn` pointer returning `Result` ·
config discovery, exactly; and one startup exit code · the xdg app id is
`"goad"`, and the window rule lives beside the binary · Round 3: the entry point
is Rust, not a numbered list · Round 3: the slice document was wrong about the
clock, not the design.

*Validation* — the failure case table is written into the design, not delegated ·
rows assert the rendered text, not the Rust variant · four rows read the element
tree, one per channel · where the exemptions, the refusals and the F-1 coda sit
in the sequence · the failure table drives the `Host`, and item 11 drives the
channel · the driving helpers are shared by one included file, cut at the
intersection · integration: one vocabulary, one pair type, one home for
`Refused` · AC-12 asserts what the host holds, not that the child is gone.

### 6. What the next session does first

In this order. The first three are the cheap measurements that decide whether
round 4 is reading text worth reading.

1. **Build F-9's `Case` schema in a scratch crate.** Instantiate the ~30 rows of
   §9 item 12.3 against the real `Display` strings from `src/semantics/error.rs`
   and `src/shell/error.rs`, and prove the schema states every assertion the rows
   make — the two-`Observed` rows (C2, T3, P2), T1's own-`Host` exemption, the
   `Exact`/`Prefixed` split, and the `invocations(&log) == instructions.len()`
   accounting. Record in `research.md` as Thread 9.
2. **Build F-26's startup surface in the same crate**, under the real lint table
   (`print_stdout`, `print_stderr`, `let_underscore_must_use`, `unused_must_use`
   all `deny`). The `writeln!` outlet spelling is the specific thing to settle;
   `arguments(argv, env)`'s four-row table becomes a test in the process.
3. **Check A-2's two named instances** while that crate is open: the stderr
   outlet, and six distinct `Wire` clone bindings in one function against
   `shadow_unrelated`. Both are ordinary Rust. Two expectations remain before the
   stop rule fires.
4. **Run review round 4.** Its brief is already written into the ledger's
   Synthesis: the two rewrites above, plus `draft-policy.md` and
   `canon-delta.md` CD-5 read against `design.md` §10 C-5. Brief the reviewer
   with F-27 and F-28 as the frame — *two of round 3's own repairs were wrong in
   ways only building them revealed; assume the same rate applies to the two that
   were not built.*
5. **Put the canon decisions in §4 to the user**, and only then write `plan.md`.
   Every phase must be executable with no user present, and PHASE-01 is the
   split, because the split moves 111 files and nothing else should be moving at
   the same time.

**Reading list for whoever picks this up:** `docs/AGENTS.md`;
`docs/slices/002/slice-002.md` (15 acceptance criteria, Stage: design);
`design.md` §5.4 and §9 item 12; `review-design.md` Brief and Synthesis;
`design-log.md` from 2026-09-04; `research.md` Threads 6, 7 and 8.

---

## Handover addendum — session 2, 2026-09-05

**Written:** end of session 2. **Branch:** `slice-002`. **Gate:** `just check`
exits 0. **Tree:** clean apart from the same pre-existing unstaged `flake.lock`
edit, still untouched. **Stage:** still the design gate; `plan.md` is still the
template and no code has changed.

Session 2 did one thing: it executed the instruction session 1's handover wrote
for it. **Item 6's first three steps are done, and all three found defects.**

| step | thread | verdict |
|---|---|---|
| 1. build F-9's `Case` schema | `research.md` Thread 9 | corrected — 14 changes, two of them expected strings that disagree with the fixtures |
| 2. build F-26's startup surface | Thread 10 | corrected — 5 changes, including an `arguments` call site that does not compile |
| 3. check A-2's named instances | Thread 11 | corrected — 13 errors across 9 lints on the design's own text, plus 8 in the rasteriser |

**The score, carried forward as the thing to brief round 5 with.** Round 3
measured three assumptions and two were false. Round 4 built three passages and
all three were defective. That is five out of six across two sessions, in text
written by agents with the sources open. The handover's lesson — *an assumption
a scratch crate can reach should be reached before a phase starts* — has now
been paid for twice and is the strongest empirical claim this slice has made
about its own process.

### What changed, and where to read it

- **`design.md`** — §5.1 gains the crate shape (D28: lib plus thin bin); §5.2,
  §5.3 and §5.4 gain nine lint-forced shapes, tabulated once in a new §5.4
  subsection *The shapes the lint table requires*; §5.4's startup surface is
  corrected end to end; §5.5's A-2 and A-5 are rewritten (both were refuted, and
  neither is left standing with a note beside it); §9 gains a preamble stating
  two test-target lint rules; §9 item 12 is rewritten in full.
- **`review-design.md`** — round 4 appended: brief, F-9 and F-26 re-verified on
  built evidence, F-29 … F-33 raised and verified, synthesis. Thirty-three
  findings, no blocker outstanding, ledger still `open`.
- **`research.md`** — Threads 9, 10 and 11 appended; Thread 7's third table row
  corrected in place, because it was refuted rather than merely dated.
- **`design-log.md`** — seven entries, all under the autonomy grant.
- **`slice-002.md`** — the renderer surface names the lib/bin shape.

### The one judgement call worth flagging to a reader

**Thread 10's prescription was rejected and its measurement kept.** It measured
`unreachable_pub` correctly and concluded "`pub(crate)`, not a lib target",
which is right for a crate with no integration tests and wrong for this one:
§12.8 runs the cheap tier in a `tests/` target that `pub(crate)` locks out. A
measurement is evidence about the shape it was taken on. Every other correction
this session was greped against the whole design before it was applied, for
exactly that reason — round 3's failure mode is a repair correct in place
against an unrepaired neighbour, and it does not stop being available just
because the repair came from a compiler.

### What is still open, unchanged from session 1 except where noted

- **A-4** is now the *only* assumption the first renderer commit is genuinely
  for: `just check` wall-clock with 411 crates (ADR-002 T3). A-1 and A-3 also
  need Slint in the graph. A-2 is largely discharged; its expectation budget is
  **unspent, three remain** — F-27's spend was refunded by F-29.
- **The canon decisions in §4 of session 1's handover are untouched.** Nothing
  under `docs/specs/`, `docs/policy/` or `docs/adr/` was created or edited this
  session either. CD-1 … CD-7 and `draft-policy.md` still need the user.
- **Round 4 did not read `draft-policy.md` or `canon-delta.md` CD-5 against
  `design.md` §10 C-5.** Round 3's synthesis asked for that and round 4's brief
  was narrower on purpose — it was a measurement round. That reading is still
  owed, and it is the cheapest remaining item.
- **`plan.md` is still not begun**, for the same reason as before: the canon
  decisions come first, and PHASE-01 is the split.

### What the next session does first

1. **Read `draft-policy.md` and `canon-delta.md` CD-5 against `design.md` §10
   C-5.** The last unreviewed artefact pair, and a reading job rather than a
   building one.
2. **Put the canon decisions to the user** (session 1's handover, §4). They are
   the one thing the autonomy grant withholds, and `plan.md` waits on them.
3. **Then `plan.md`.** PHASE-01 is the split, because it moves 111 files and
   nothing else should move at the same time. Two constraints the builds added
   to phase planning, both of which decide where a boundary can fall:
   `dead_code` is fatal under `-D warnings`, so the phase that lands
   `StartupError` must land a construction site for all eight variants in the
   same commit; and §5.4's *shapes* table plus §9's preamble are the two lists a
   phase reads before writing renderer code or a test target.

---

## Handover addendum — session 3, 2026-09-05

**Written:** end of session 3. **Branch:** `slice-002`. **Gate:** `just check`
exits 0. **Tree:** clean apart from the same pre-existing unstaged `flake.lock`
edit, still untouched. **Stage:** still the design gate; `plan.md` is still the
template and **no code has changed** in three sessions.

Session 3 ran **review round 5** and repaired what it found. Thirteen repairs:
seven new findings (F-34…F-40) and six reopened under their existing ids (F-6,
F-8, F-9, F-16, F-17, F-30). No blocker outstanding.

### The two blockers, and why one of them matters more than its severity

**F-30 regressed.** It was raised at round 4 *by a compiler*, repaired
correctly — D28, the lib-plus-thin-binary shape — and §5.1 was made to say
*"everything a test can reach, `install` included, lives in the library."* Nine
hundred lines later the prescription still read `fn install(…)`. Private. A rule
stated and not applied to the site the rule came from.

**F-37 was never raised at all**, by four rounds. The design could state the
exact `Display` of thirty-three diagnostic lines and could not say what the two
test targets were called, where `build.rs`'s input lived, or which modules
`lib.rs` declares. Four rounds asked *does this work?*; none asked *can this be
typed?*

Both are now closed. F-37's repair — `design.md` §5.1, *The artifact map* — is
the largest single addition this slice has made and is what PHASE-01 executes
against.

### What is now specified that was not

| what | where |
|---|---|
| the split's source→destination table, 111 files | `design.md` §5.1, the artifact map |
| four member manifests, dependency by dependency, with features | same |
| six `[[test]]` targets by name, path and `main.rs` module list | same |
| the shared helper `tests/support/driving.rs` and its literal `#[path]` | same |
| `crates/goad/src/lib.rs`, ten `pub mod` lines | same |
| which of §9's seventeen validation items runs in which target | same |
| `SlintGlass` — module, fields, constructor, `impl Glass` — and what a `show`/`hide` failure does | `design.md` §5.3 |
| `StartupError`'s module and derives | `design.md` §5.4 |
| `pub fn install`, in `install.rs` | `design.md` §5.4 |
| `code_of -> Cow<'_, str>`, and `goad-boundary`'s whole public API | `design.md` D13, §5.6 |
| the 33-row `CASES` array, verbatim from the crate that compiled it | `design.md` §9 item 12.9 |
| the three `@lingers*` bash arms, and the four success bodies | same |
| four numeric thresholds and eight STOP conditions, S-1…S-8 | `design.md` §5.5, §8 R2, §9 item 14a |
| the font: `pkgs.dejavu_fonts` + `makeFontsConf` + `FONTCONFIG_FILE` | `design.md` D12 |
| the niri `window-rule`, validated | `design.md` §5.4 |
| the counting rule — four instruments, plus the vocabulary scan, plus one residue — with **one** home | `design.md` §5.1 |

### What was measured this session, and what was only read

Three things were **built or validated**, and all three found something:

| # | measurement | result |
|---|---|---|
| 1 | round 4's `f9-schema` scratch crate re-run | still green — `--test table` 8/8, clippy `-D warnings` exit 0. The array is preserved verbatim in §12.9, and its `<A>` placeholder is now documented as a placeholder rather than left to be copied literally |
| 2 | `makeFontsConf { fontDirectories = [ dejavu_fonts ]; }` evaluated, built, and `fc-list`'d | 39 DejaVu faces; the conf carries the store path as an explicit `<dir>`. It also found the trap: **`buildInputs` alone does nothing** |
| 3 | `niri validate` on the proposed `window-rule`, niri 26.04 | *config is valid* |

**Four repairs rest on reading**: F-8's `Cow` signature, F-17's `SlintGlass`
declaration, F-37's artifact map, F-38's four thresholds. The ledger's round-5
synthesis argues why that is a different bet from round 3's — these are paths,
numbers and two signatures, all of which fail visibly on the first `cargo build`
rather than silently — but it is a bet, and it is written down as one.

### The running score, which is this slice's best evidence about itself

| session | measured | wrong |
|---|---|---|
| 1 (round 3) | 3 assumptions | 2 |
| 2 (round 4) | 3 passages built | 3 |
| 3 (round 5) | 3 artefacts built/validated | 1 (the `buildInputs` trap) — and the other two confirmed |

Six of nine, across three sessions, in text written by agents with the sources
open. The rate is dropping, which is what convergence looks like, and it is the
first session where a measurement mostly **confirmed** rather than corrected.

### What is still open

- **The canon decisions are untouched**, for the third session running. Nothing
  under `docs/specs/`, `docs/policy/` or `docs/adr/` was created or edited.
  CD-1…CD-7 and `draft-policy.md` still need the user; session 1's handover §4
  is the list and it is still accurate. **`plan.md` waits on this and on nothing
  else.**
- **Round 5's own repairs are unreviewed**, which is the debt every round in this
  ledger has left and why it stays `open`.
- **A-4 is still the only assumption the first renderer commit is genuinely
  for**, and it now has a protocol and three numbered bands rather than the word
  "tolerable".

### What the next session does first

1. **Put the canon decisions to the user** (session 1's handover §4, unchanged).
   They are the one thing the autonomy grant withholds.
2. **Then `plan.md`.** PHASE-01 is the split. Its surface is `design.md` §5.1's
   artifact map, read top to bottom; its exit is `just check` at 0 and AC-2's
   content-change list; and the first thing PHASE-02 does after `slint` lands is
   A-4's timing protocol, because that number decides whether T3 has fired.
3. **Read §5.5's STOP table (S-1…S-8) into every phase sheet.** It is the list a
   phase agent needs to recognise a condition it is not allowed to improvise
   past.

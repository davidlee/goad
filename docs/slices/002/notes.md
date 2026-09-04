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

**Written:** 2026-09-05, end of session 3. This section **replaces** the session-1
handover and the session-2 and session-3 addenda. It is the whole state, not a
delta; nothing earlier in this file needs reading first.

| | |
|---|---|
| branch | `slice-002`, 9 commits ahead of `a6ae617` (slice 001's close) |
| head | `d880801` — *review round 5: seven findings raised, six reopened, all repaired* |
| gate | **verified this session:** `just check` exits 0, 1.836 s warm |
| tree | clean apart from the pre-existing unstaged `flake.lock` edit (a `bun2nix` input repointed to a `Mic92` fork). Untouched for three sessions. Leave it alone |
| canon | **untouched.** `git diff a6ae617..HEAD -- docs/specs docs/policy docs/adr CLAUDE.md` is empty |
| stage | the design gate. `plan.md` is still the template. **No code has changed in three sessions** |

The slice has produced 12,569 lines of documentation and 0 lines of code. That
is the fact this handover has to justify or condemn, and §5 does the arithmetic.

---

### 1. What rounds 4 and 5 built, and what the compiler said

Round 3 ended by writing itself a lesson: *an assumption a scratch crate can
reach should be reached before a phase starts, not listed as a risk.* Rounds 4
and 5 spent themselves discharging it. Six scratch measurements, all recorded in
`research.md` as numbered threads.

**Round 4 — three passages built, three defective.** This is the round where the
compiler became the raiser rather than the reviewer.

| built | thread | what the compiler said |
|---|---|---|
| F-9's failure-matrix `Case`/`Observed`/`Cohort` schema | Thread 9 | **14 corrections**, two of them expected strings that disagree with the fixtures on disk |
| F-26's startup surface — `StartupError`'s eight variants, `arguments(argv, env)`, the `writeln!` outlet | Thread 10 | **5 corrections**, including an `arguments` call site that does not compile |
| A-2's two named lint instances — the stderr outlet, six `Wire` clone bindings against `shadow_unrelated` | Thread 11 | **13 errors across 9 lints** on the design's own text, plus 8 more in the rasteriser |

Five findings followed (F-29…F-33), two of them blockers: `serve` carried an
`#[expect]` for a lint that does not fire (F-29), and **every `pub` item in the
renderer was a lint error because the crate's shape had never been stated**
(F-30). F-30's repair is D28 — a library plus a thin binary — and it is load
bearing for the whole test strategy.

One judgement call from that round is worth a reader's attention. **Thread 10's
prescription was rejected and its measurement kept.** It measured
`unreachable_pub` correctly and concluded *"`pub(crate)`, not a lib target"*,
which is right for a crate with no integration tests and wrong for this one:
§12.8 runs the cheap tier in a `tests/` target that `pub(crate)` locks out. A
measurement is evidence about the shape it was taken on, not about a shape it
never saw.

**Round 5 — three artefacts built or validated, one corrected.** The first round
in which measurement mostly *confirmed*.

| measured | result |
|---|---|
| round 4's `f9-schema` scratch crate, re-run | **still green.** `--test table` 8/8, clippy `-D warnings` exit 0. The 33-row `CASES` array is now copied into `design.md` §9 item 12.9 **verbatim from the crate that compiled it**, not paraphrased; its `<A>` placeholder is documented as a placeholder |
| `makeFontsConf { fontDirectories = [ dejavu_fonts ]; }` evaluated, built, `fc-list`'d | 39 DejaVu faces; the conf carries the store path as an explicit `<dir>`. **Found the trap: adding the font to `buildInputs` alone does nothing** — this is the one correction |
| `niri validate` on the proposed `window-rule`, niri 26.04 | *config is valid* |

Round 5 raised seven findings (F-34…F-40) and reopened six under their existing
ids (F-6, F-8, F-9, F-16, F-17, F-30). All thirteen `fix-now`, all `verified`.
Two were blockers, and the second is the one that matters:

- **F-30 regressed under integration.** Round 4 raised it with a compiler,
  repaired it correctly, and made §5.1 say *"everything a test can reach,
  `install` included, lives in the library."* Nine hundred lines later the
  prescription still read `fn install(…)` — private, reachable from neither the
  binary crate nor a `tests/` target. A rule stated and not applied to the site
  the rule came from.
- **F-37 was never raised by four rounds.** The design could state the exact
  `Display` of thirty-three diagnostic lines and could not say what its two test
  targets were called, where `build.rs`'s input lived, or which modules `lib.rs`
  declares. Four rounds asked *does this work?*; none asked *can this be typed?*

**What is now specified that was not.** F-37's repair, `design.md` §5.1 *The
artifact map* (`design.md`:291–445, ~155 lines), is the largest single addition
this slice has made and is what PHASE-01 executes directly against.

| what | where |
|---|---|
| the split's source→destination table, 111 files, with a "change permitted" column AC-2 reads against | `design.md` §5.1, the artifact map |
| four member manifests, dependency by dependency, with per-member feature sets | same |
| six `[[test]]` targets by name, path and `main.rs` module list | same |
| the shared helper `tests/support/driving.rs` and its literal `#[path = "../../../../tests/support/driving.rs"]` | same |
| `crates/goad/src/lib.rs`, ten `pub mod` lines | same |
| which of §9's seventeen validation items runs in which target | same |
| `SlintGlass` — module, fields, constructor, `impl Glass` — and what a `show`/`hide` failure does | `design.md` §5.3 |
| `pub fn install`, in `install.rs`; `StartupError`'s module and derives | `design.md` §5.4 |
| `code_of -> Cow<'_, str>`, and `goad-boundary`'s whole public API | `design.md` D13, §5.6 |
| four numeric thresholds and eight STOP conditions, S-1…S-8 | `design.md` §5.5, §8 R2, §9 item 14a |
| the font — `pkgs.dejavu_fonts` + `makeFontsConf` + `FONTCONFIG_FILE` | `design.md` D12 |
| the validated niri `window-rule` | `design.md` §5.4 |
| the counting rule — four ADR-001 instruments, the vocabulary scan, one named residue — with **one** home, cited by six documents | `design.md` §5.1 |

Writing the artifact map forced two design decisions: **one** `ui/app.slint`
(the shape `research.md` Thread 8 actually compiled), and `slint`/`slint-build`
pinned `= 1.17.1`, so A-1 and A-3 change on a deliberate upgrade rather than on
resolver drift.

---

### 2. The previous handover's four open items — one closed, three not

Session 2's handover left four. Checked against the tree, not against the report:

| # | item | state |
|---|---|---|
| 1 | **`draft-policy.md` and `canon-delta.md` CD-5 read against `design.md` §10 C-5** — the last unreviewed artefact pair | **CLOSED.** Round 5 did exactly this reading and it produced two findings. F-36: the draft legislated repository-wide rules the design never derived — three of four clauses are now derived clause by clause in a new block under §10 C-5, and the fourth (lint discipline) is **cut**, which is also F-16's structural fix. F-35 corrected a context-dependent evidence path in `canon-delta.md`, and the class fix went into its preamble |
| 2 | **A-4** — `just check` wall-clock with 411 crates in the tree (ADR-002 T3) | **OPEN, and not closable by a spike.** It needs `slint` in the graph. It now has a protocol and three numbered bands instead of the word "tolerable" (S-4: ≤ 120 s local, ≤ 300 s, stop). Today's baseline, measured this session: **1.836 s warm, pre-split** |
| 3 | **The canon decisions, CD-1…CD-7 and `draft-policy.md`** | **OPEN, for the third session running — and larger than it was.** See §3 |
| 4 | **`plan.md` is not begun** | **OPEN.** It waits on item 3 and on nothing else |

Round 5 **enlarged** item 3 rather than shrinking it. F-36's repair rewrote
`draft-policy.md`'s Scope and Compliance and added a derived scope block to §10
C-5; F-6's repair changed the wording that CD-1 and CD-7 will transcribe into a
new ADR and into `CLAUDE.md`. The user must endorse the movements as they now
read, not as session 1 described them.

---

### 3. What is open, and what only the user can decide

#### 3a. Canon — the one thing the autonomy grant withholds

Nothing under `docs/specs/`, `docs/policy/` or `docs/adr/` has been created or
edited on this branch. Verified: the diff is empty. Every movement below is
drafted in the slice folder and waits.

| # | movement | vehicle | needed |
|---|---|---|---|
| CD-1 | a new ADR **superseding** ADR-002 (the split) | `canon-delta.md` | the *decision* is endorsed (`design-log.md`, 2026-09-05); the **wording** is not, and F-6 changed it |
| CD-2 | ADR-002's stated reason for expecting T1 is measurably false; the superseding ADR states the real ground | `canon-delta.md` | with CD-1 |
| CD-3 | SPEC-001 has no rule at the glass — R-20's no-silent-dropping stops before the renderer | `canon-delta.md` | at audit |
| CD-4 | SPEC-001 §7 names the fixture directory normatively; moving it is a canon change | `canon-delta.md` | at audit |
| CD-5 | `CLAUDE.md`'s gate pointer moves off a **closed slice's design** (`docs/slices/001/design.md` §9) | `canon-delta.md` | **lands with the policy below, or neither lands** |
| CD-6 | `CLAUDE.md` invariant 1 — the boundary test must grow to grep `.slint` | `canon-delta.md` | at audit |
| CD-7 | `CLAUDE.md`'s "both feature columns" becomes false the day the split lands | `canon-delta.md` | at audit; F-6 changed the wording |
| — | **new** policy: the phase gate — six commands, four enforcement instruments, one named residue, and a scope now derived clause by clause | `draft-policy.md` | **paired with CD-5** |

**Two things to decide, and the first is a pair by construction.** Applying CD-5
alone leaves `CLAUDE.md` pointing at nothing; promoting the draft alone leaves
two claimants to the gate.

1. **Endorse the canon movements**, or their timing.
2. **Decide what discharges the design gate** — §5 argues that "no reviewer
   objects" has no fixed point here and proposes a replacement.

Already granted and needing nothing further: the crate split, the
tray-plus-window shape, the font package in `flake.nix`, and the dependency set
(`slint`, `slint-build`, the Slint testing dev-dependency).

#### 3b. Unbuilt claims the design still rests on

Four of round 5's thirteen repairs rest on **reading by their own author**, and
the ledger says so rather than letting five rounds of accumulated rigour imply
otherwise:

| repair | what it is | why it is a smaller bet than round 3's |
|---|---|---|
| **F-37**, the artifact map | ~155 lines: 111 paths, four manifests, six target names, one literal `#[path]`, `lib.rs` | a table of file paths fails **loudly** on the first `cargo build` |
| **F-38**, four thresholds | four numbers and the S-1…S-8 STOP table | wrong only if a measurement disagrees, and the measurement is the first thing PHASE-01 does |
| **F-8**, `code_of -> Cow<'_, str>` + `goad-boundary`'s public API | two signatures | the smallest surface a wrong repair can have |
| **F-17**, `SlintGlass` + the third stderr outlet | one module declaration | same |

None is a *behaviour* specified from summaries, which is what rounds 1–4 kept
finding. That is the honest case for them, and it is written down as a bet.

#### 3c. Standing assumptions

- **A-1, A-3** — standing; A-1 now has a local/stop line, A-3 is now a **stop**
  rather than a decision (F-38, S-2 and S-3). Both need `slint` in the graph.
- **A-2** — ~75 unproven lints against hand-written renderer code. Largely
  discharged by Thread 11. **Expectation budget unspent; three remain.** F-27's
  spend was refunded by F-29. The stop rule stands: the third distinct `expect`
  outside the generated-code quarantine stops the phase.
- **A-4** — see §2. The only assumption the first renderer commit is genuinely
  for.
- **A-5, A-6, A-7** — discharged. A-5 measured twice; `serve` is an `async fn`
  with no attribute. A-6's residual fallback deleted at F-34.
- **D25 is an admission, not a risk.** No instrument in the gate rejects a
  feature switched on by stratum 2 or 3 in a dependency shared with stratum 1.
  The design states the rule and states that nothing enforces it. That shape is
  deliberate — it is what F-6 was raised four times to get right.

---

### 4. Every decision taken under the autonomy grant

The grant is `design-log.md`, 2026-09-04, *"Gate autonomy: an explicit deviation
from `docs/AGENTS.md`"* — decide everything except canon, and record it as a
user decision would be recorded. Four vehicles were used and nothing was decided
outside them.

| vehicle | count | holds |
|---|---|---|
| `design-log.md` | **55** design decisions, each marked *Autonomy grant* with Asked / Decided / Why / Rejected / Consequence | the decisions themselves |
| `review-design.md` | **59** dispositions over five rounds — 40 findings, 19 of them re-dispositions — ids immutable, each with its reason | finding dispositions |
| `canon-delta.md` + `draft-policy.md` | **7** movements + 1 new policy, **drafted, not applied** | canon consequences |
| `notes.md` | 0 | phase-local decisions — there are none, because there are no phases |

`design-log.md` holds 61 entries. Six are **not** the agent's: three frame the
grant itself (*How the slice is driven*, *Gate autonomy*, *Scope extended*) and
three are the **user's own** and marked as such — the ADR-002 T1 split, the
tray-not-window empty state, and the devshell font. No review round has
challenged any of the three on its merits, and no finding has touched the
guiding principles.

The 55, by heading. `grep -n '^### ' docs/slices/002/design-log.md` gives line
numbers.

*The split and the gate* — the gate is six commands and `-p goad-semantics`
earns its place · the workspace invariant checks get their own member · members
are enumerated, not listed; R7 is retired · the renderer inherits the workspace
lint table unchanged · R3: four instruments for stratum 1, and the claim
narrowed to fit · R3: the new gate policy is drafted from the policy template.

*The state machine* — the presentation transition is a total function, and
cleanup is not one of its inputs · `(view: Some, failure: Some)` is unreachable
and is still written total · `Command::Choose` carries the view token · the
window has one derived surface value, and a new question outranks a record · one
consumption point for an `Outcome`, and it runs the mapper.

*The runtime seam* — shutdown leaves the command channel; `serve` is one
function both tiers call · the queue policy is four mechanisms, and only one is
the safety mechanism · a callback holds a `Wire`, and `busy` and `notice` are
two properties · four shutdown sources, one path; `dismissed()` is deleted ·
`serve` is a plain fn returning `impl Future` **(superseded)** · R3: the loop
was built, and it changed `serve`'s signature · R3: three seams closed, one
shape each.

*The glass* — the glass is one total method, and the component is never
recreated · Markdown is parsed once and the parse is retained · `ContentForm` is
two variants and no payload · the tray icon is a rule with no artefact · R3: the
icon has numbers and the startup surface has strings · R3: the Slint API is read
from the compiler, not inferred · R3: the markup was compiled, and the tray
could not be written.

*The diagnostic surface* — the display bound is applied last, and counted in
characters · two truncations, two statements · stderr alone reports without
raising fault; a renderer refusal does raise it.

*Startup and the clock* — the clock is a `fn` pointer returning `Result` ·
config discovery, exactly; and one startup exit code · the xdg app id is
`"goad"`, and the window rule lives beside the binary · R3: the entry point is
Rust, not a numbered list · R3: the slice document was wrong about the clock,
not the design.

*Validation* — the failure case table is written into the design, not delegated
· rows assert the rendered text, not the Rust variant · four rows read the
element tree, one per channel · where the exemptions, the refusals and the F-1
coda sit in the sequence · the failure table drives the `Host`, and item 11
drives the channel · the driving helpers are shared by one included file, cut at
the intersection · integration: one vocabulary, one pair type, one home for
`Refused` · AC-12 asserts what the host holds, not that the child is gone.

*Round 4 — what the compiler decided* — the three unbuilt passages were built,
and all three were wrong · the renderer crate is a library plus a thin binary
(D28) · `serve` carries no attribute, and A-2's budget is unspent · the escape
step is a `Display` adapter, and the outlet is not · `HOME` as given, XDG
absoluteness as one test, and the `argv[0]` skip · the failure table's instants,
its sentinel body, and a total channel partition · the house test standard
yields to `unnecessary_wraps`, on the standard's own terms.

*Round 5* — the count has one home, and every other document cites it · a
display failure goes to stderr, and the glass stays total · the artifact map,
and the two decisions writing it forced · every threshold is a number, and the
stops are one table · the font is `dejavu_fonts`, and `buildInputs` alone does
nothing · the window rule is validated KDL, not remembered KDL · the phase-gate
policy's scope is derived, clause by clause.

---

### 5. Is this converging? — the numbers, and the honest answer

The question a reader is owed after five rounds and no code: **is the design
converging, or is the review finding new work as fast as it closes old work?**

#### The raise rate

| round | new | reopened | total dispositions | new blockers | rested on built evidence |
|---|---|---|---|---|---|
| 1 | 12 (F-1…F-12) | — | 12 | 4 | 0 of 12 |
| 2 | 7 (F-13…F-19) | 7 | 14 | 1 | 0 of 14 |
| 3 | 9 (F-20…F-28) | 4 | 13 | 0 | 2 of 13 |
| 4 | 5 (F-29…F-33) | 2 | 7 | 2 | **7 of 7** |
| 5 | 7 (F-34…F-40) | 6 | 13 | 1 | 3 of 13 |
| **total** | **40** | **19** | **59** | **8** | **12 of 59** |

New findings per round: **12, 7, 9, 5, 7.** That is flat, not falling.
Re-dispositions per round: **0, 7, 4, 2, 6.** Also flat. A **blocker** was raised
at round 5 (F-37) that four earlier rounds did not see. Nineteen of fifty-nine
dispositions — **32%** — were reopenings of findings a previous round had
already marked `verified`. `F-9` has been disposed five separate times; `F-6`
four; `F-8`, `F-16` and `F-17` three each.

**On the raw counts the answer is: the review is finding new work about as fast
as it closes old work, and has been for four rounds. As a reading process it is
not terminating, and there is no number in the table that predicts round 6 would
be the last.**

#### The one series that does fall

| session | measurements taken | wrong |
|---|---|---|
| 1 (round 3) | 3 assumptions | 2 |
| 2 (round 4) | 3 passages built | 3 |
| 3 (round 5) | 3 artefacts built/validated | 1 |

Six of nine, then 1 of 3. **Building converges. Reading does not.** That is the
whole finding, and it is consistent with the other thing the record shows: round
4, the only round where every disposition rested on built evidence, is also the
only round whose raise count dropped.

#### Why the raise rate stayed flat — and why that is not an excuse

Each round asked a different question. Rounds 1–3 asked *is this right?*; round 4
asked *does this compile?*; round 5 asked *can this be typed?* Every new question
opened a fresh seam, which explains a flat rate without redeeming it — because
the corollary is that **there is no evidence the current question set is
complete.** A sixth question would plausibly find a sixth seam. That is precisely
why "run another round until it comes back clean" cannot be the exit criterion:
the criterion has no fixed point, and five rounds is the evidence.

#### What must change

**1. Stop reviewing by reading; execute instead.** The four unbuilt repairs
(§3b) are paths, two signatures and four numbers — the class that fails loudly at
first compile. Do not spend a session reading them. **Execute the artifact map in
a worktree.** `research.md`'s dry run put the split at about six minutes to a
green gate; `git reset --hard` reverses it; and performing it proves every path,
every manifest, every target name and the `#[path]` arithmetic at once. It is
also PHASE-01's actual work, so the cost is not a review round — it is the first
phase, done where a mistake is free.

**2. Replace the design gate's exit criterion.** Not *"no reviewer objects"* —
that has no fixed point here. Instead: **every claim in the design is either
built, or its failure mode is loud at first compile.** By that criterion the
design is met on everything except the four repairs in §3b, and step 1
discharges all four.

**3. Clear the canon endorsement**, which is the only genuine blocker and is not
the agent's to clear. It has been open for three sessions while the agent found
five rounds of other work to do. That sequencing is itself a symptom.

**The risk of the status quo, stated plainly.** An endless design gate is a
failure mode of the same family as a half-built tree. Five rounds, three
sessions, 5,049 lines of design and zero lines of code is what it looks like from
outside, and the review's own numbers do not promise a sixth round would end it.

---

### 6. What the next session does first

1. **Put §3a's canon decisions to the user.** They are the one thing the autonomy
   grant withholds, `plan.md` waits on them by the methodology's own rule, and
   they are now three sessions old. CD-5 and `draft-policy.md` go together or not
   at all.
2. **Execute the artifact map in a worktree**, per §5's recommendation — *not* a
   round 6 reading pass. Entry: `design.md` §5.1 *The artifact map*, top to
   bottom. Exit: `just check` at 0, and AC-2's content-change list matching the
   map's "change permitted" column. If it comes back green, the last unbuilt
   repair of consequence is built rather than read, and nothing stands between
   the slice and `plan.md` except item 1.
3. **Then `plan.md`.** PHASE-01 is the split, because it moves 111 files and
   nothing else should be moving at the same time. Two constraints the builds
   added to phase planning, both of which decide where a boundary can fall:
   `dead_code` is fatal under `-D warnings`, so the phase that lands
   `StartupError` must land a construction site for all eight variants in the
   same commit; and §5.4's *shapes* table plus §9's preamble are the two lists a
   phase reads before writing renderer code or a test target.
4. **The first thing PHASE-02 does after `slint` lands is A-4's timing
   protocol.** That number decides whether ADR-002's T3 has fired. Baseline
   measured 2026-09-05, pre-split: `just check` 1.836 s warm.
5. **Read §5.5's STOP table (S-1…S-8) into every phase sheet.** It is the list a
   phase agent needs in order to recognise a condition it is not allowed to
   improvise past.

**Reading list for whoever picks this up:** `docs/AGENTS.md`;
`docs/slices/002/slice-002.md` (15 acceptance criteria, Stage: design);
`design.md` §5.1 *The artifact map* (`:291–445`), §5.4, §5.5's STOP table, §9
item 12, §10 C-5; `review-design.md` Brief and the round-4 and round-5
syntheses; `design-log.md` from 2026-09-04; `research.md` Threads 7–11;
`canon-delta.md` and `draft-policy.md`.

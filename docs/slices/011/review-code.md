# Review — the refused arrival's present, implementation — Slice 011

**Subject:** implementation — `d2617c1..fdc2229` over `crates/` (`crates/goad/src/controller.rs`, `crates/goad/tests/renderer/ingress.rs`), plus `docs/slices/011/flood.py`
**Reviewer:** fresh agent (Claude Opus 5.5), adversarial code reviewer, did not build it
**Opened:** 2026-09-26
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

<!-- Written BEFORE the review, so it is not shaped by what turned out to be easy
     to find. What this review is probing, and the invariants it holds the
     subject to. Where the bodies are likely buried. -->

**Round 1** — 2026-09-26 — the implementation of SPEC-003/R-15's update guarantee as drafted in `canon-delta.md`.

Probing, before reading the code:

1. **Control flow of `serve`'s `'idle` loop.** Every entry and exit: does each exit present exactly when R-15 and design D5/D6/D8 require, and never when they forbid? Labelled `break`/`continue` targets — does a `continue 'outer` skip a present it owed? `biased` `select!` ordering — can the refusal-present arm be starved by a steady arrival stream (the very load R-15 guards), or can it starve the arrival arm? Loop end (channel closed, backend exit) with a pending stale surface. Deadline arithmetic at `Instant` overflow.
2. **Canon against code.** Does R-15's guarantee, as `canon-delta.md` drafts it, hold of the code in every case the rule names — and does the code do anything the delta does not say (an extra present, a reset of the interval clock the rule does not admit)?
3. **The tests.** For each of T1–T4 and VT-7: does it assert the property it names, or a proxy a regression would survive (memory: *tests asserting proxies*, *negative control must compile*)? Mutation-check at least the central mechanism. Timed bounds: is each bound's load direction stated, and right (load must move the bound toward passing a correct build, not a broken one)? Flakiness under a loaded CI box. `RecordingGlass` — does it record what the test then asserts about, or something adjacent?
4. **Doc comments.** True of the code? Sentences elsewhere in `controller.rs` the change falsified (grep for the old behaviour's vocabulary). Counts nothing checks; line-number citations.
5. **Invariants.** Strata purity (`src/semantics/` untouched?), domain vocabulary, no new coupling between controller and renderer.

Where the bodies likely are: the interaction of the interval deadline with arrivals that reset or do not reset it; the exit paths of `'idle`; test timing bounds.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | minor | | |
| F-2 | minor | | |
| F-3 | minor | | |
| F-4 | nit | | |
| F-5 | minor | | |
| F-6 | minor | | |
| F-7 | minor | | |
| F-8 | minor | | |
| F-9 | nit | | |

### F-1 — T2(a)'s ceiling carries a whole spare present; a 30 % shorter interval passes it

**Severity:** minor
**Location:** `crates/goad/tests/renderer/ingress.rs`, `a_flood_of_refusals_updates_the_window_once_per_interval_with_the_latest`, the `let ceiling = 1 + span_ms.div_ceil(interval_ms)` line and the case's doc paragraph *(a)'s resolution*; `canon-delta.md` Change 3, the T2 bullet ("one more than the number of intervals their span covers").

**Expected:** (a) holds I-1 (consecutive arm firings at least `I` apart) as tightly as the stated allowance `ε = I/2` permits. With `k + 1` flood presents from firings at least `I` apart, `span ≥ kI + (δ_last − δ_first)` where `δ` is each present's lag behind its firing, so `count ≤ 1 + floor((span + ε) / I)` whenever `δ_first − δ_last ≤ ε` — the same margin, in the same direction, that the case's comment already states.
**Observed:** the case uses `div_ceil`. Because `(span + ε)/I` is never an integer (`span ≈ kI`, `ε = I/2`), the ceiling is always `k + 2`: one present more than I-1 allows, every run. So the case admits one extra refusal-caused present per flood, and its resolution is coarser than it needs to be. `review-design.md` Round 4 accepted the resulting ~0.65 s resolution on the ground that "a sharper bound costs real time" (more flood intervals). The floor form costs none.
**Evidence:** scratch copy of HEAD `fdc2229` (`…/scratchpad/rc1`), T2 instrumented with an `eprintln!` of count/ceiling only:

| production `REFUSAL_PRESENT_INTERVAL` | ceiling form | count / ceiling | result |
|---|---|---|---|
| 1000 ms (unchanged) | `div_ceil` (as shipped) | 4 / 5, 3/3 runs | green |
| 800 ms | `div_ceil` | 5 / 5, 3/3 runs | **green** |
| 700 ms | `div_ceil` | 5 / 5, 3/3 runs | **green** |
| 1000 ms | floor (`1 + span_ms / interval_ms`) | 4 / 4, 2/2 | green |
| 900 ms | floor | 4 / 4, 2/2 | green |
| 800 ms | floor | 5 / 4, 2/2 | red |
| 700 ms | floor | 5 / 4, 2/2 | red |

The floor form also widens M9 (600 ms) from red-by-one (`notes.md` M9: "6 presents against a ceiling of 5", one dropped firing from green, `review-design.md` Round 4) to red-by-two (ceiling 4 by the same arithmetic). Canon-delta Change 3's words "one more than the number of intervals their span covers" describe the `div_ceil` form, so the canon text carries the same slack and would need the matching edit ("one more than the whole intervals their span holds").

**Disposition:**
**Response:**

**Outcome:**

### F-2 — Three doc comments describe the interval as a delay from the refusal; the leading edge presents at once

**Severity:** minor
**Location:** `crates/goad/src/controller.rs` — the doc of `REFUSAL_PRESENT_INTERVAL`; the doc of `refuse_arrival`; the doc of `ingest` (its `None` paragraph).

**Expected:** `design.md` D3 and §5.4 — the first refusal after a quiet interval presents without waiting (the elapsed deadline *is* the leading edge); the interval spaces refusal-caused presents from each other, and R-15 (canon-delta Change 1) states both halves: a latency bound *and* "MUST NOT exceed one per interval". The throttle is the slice's reason to exist (`design.md` §1).
**Observed:**
- `refuse_arrival`: the fold "reaches the top present once `REFUSAL_PRESENT_INTERVAL` allows it **rather than at once**" — on the leading edge it is at once.
- `ingest`: "that present is the coalescing arm's job, **no sooner than `REFUSAL_PRESENT_INTERVAL`**" — false for the same reason; the interval is measured from the arm's last firing, not from the refusal.
- `REFUSAL_PRESENT_INTERVAL`: "How long a refused arrival's fold may sit on the diagnostics surface before the surface is updated to show it" — states only the latency half, not the one-per-interval rate it exists to enforce, and says the fold sits "on the diagnostics surface" when it sits in the retained `Diagnostics`, *off* the surface — which is the whole distinction this slice's canon draws ("update of the surface").
**Evidence:** `serve`'s `next_refusal_present` starts at `sleep_until(started)` and is reset only by the arm; T3's R1 bound (`at − sent ≤ I/2`) and T4's A are green precisely because the leading edge does not wait (M5 reds T3 when it does, `notes.md`).

**Disposition:**
**Response:**

**Outcome:**

### F-3 — `surface_stale`'s comment claims only the arm ends the wait with the surface stale; every exit does

**Severity:** minor
**Location:** `crates/goad/src/controller.rs`, `serve`, the comment above `let mut surface_stale = false;`.

**Expected:** a true statement of why the flag needs no clear — `design.md` §5.3: "Every exit from `'idle` either presents (at the top, or at the engage present) or ends the loop."
**Observed:** "the only thing that ends the wait with the surface still stale is the arm below, and it does not need a read-then-clear because nothing loops back into `'idle` after it." A command, a scheduled firing, the ingress-stopped fold, `cancel.stopped()` and a closed command channel all leave `'idle` with `surface_stale` possibly `true` (the last two are D8's unmade update). What makes the missing clear safe is that the flag is re-declared on every entry and each non-ending exit reaches a present — not that the arm is the only stale exit.
**Evidence:** the `select!` arms inside `'idle`: `break 'serving Ending::Stopped`, `break 'serving Ending::Closed`, `Fired::Command`/`Fired::Scheduled` → `break 'idle`, ingress `None` → `continue 'serving`. None of them reads or clears the flag.

**Disposition:**
**Response:**

**Outcome:**

### F-4 — The rewritten `let Some(attempted)` comment keeps "an edit the drain applied", which cannot reach it

**Severity:** nit
**Location:** `crates/goad/src/controller.rs`, `serve`, the comment above `let Some(attempted) = attempted else { continue; };`.

**Expected:** `design.md` §5.2 lists this comment among those to rewrite; the rewrite should name what can arrive there.
**Observed:** "A diagnostics command, or an edit the drain applied." The drain loop (`while drained.is_none()`) only ever hands over a `Some`; an edit the drain applies yields `None` and the drain carries on. The `None` here is a diagnostics command or an `Edit` taken by the `'idle` `select!`'s `commands.recv()` arm. The sentence predates the slice, but the slice rewrote the paragraph around it and added "this `None` is only ever a command that resolved without an exchange", which is true and sits next to the false half.
**Evidence:** `serve`: `let (attempted, refusal_re_arms) = if let Some(drained) = drained { (Some(drained), false) } else { … }`.

**Disposition:**
**Response:**

**Outcome:**

### F-5 — Test doc comments still narrate the pre-PHASE-02 loop as "today's"

**Severity:** minor
**Location:** `crates/goad/tests/renderer/ingress.rs` — the doc of the mirror `REFUSAL_PRESENT_INTERVAL`; the docs of `a_too_soon_refusal_decided_while_idle_reaches_the_window_at_once` (T3), `a_flood_of_refusals_updates_the_window_once_per_interval_with_the_latest` (T2), `a_command_during_a_coalesced_interval_presents_at_once_and_carries_the_refusal` (T4); and the doc of `a_flat_out_writer_raises_no_evaluation_rate` (T1).

**Expected:** doc comments true of the code they sit in.
**Observed:**
- Mirror constant: "mirrored ahead of PHASE-02, which gives `controller::REFUSAL_PRESENT_INTERVAL` its production value … T3 (below) states its bounds against this value while today's loop still presents every refusal at once — PHASE-02's mutations are what exercise the throttle this constant **will then** gate." PHASE-02 has landed; the loop no longer presents every refusal at once.
- T3: "**Green on today's loop** …: nothing throttles a refusal's presentation yet … PHASE-02's M5, M6 and M8 are its controls, once the coalescing loop exists to mutate." False at HEAD.
- T2: "every refusal presents today, so the count is the flood's own size". T4: "B presents at once today". Both false at HEAD; each is a record of a red-before-green run, which belongs in `notes.md`, not in a comment that reads as present tense.
- T1 cites "slice 011 `design.md` VT-2": `design.md` has no VT-2; the id is `plan.md` PHASE-01/VT-2.
- Also `flat_out`'s doc: "such as **this phase's** own two" — phase-relative, and wrong in a file every later slice edits.
**Evidence:** the quoted lines, against `serve` at `fdc2229`; `grep -n "VT-2" docs/slices/011/design.md` → no match.

**Disposition:**
**Response:**

**Outcome:**

### F-6 — T3's and T4's timed bounds carry no load-direction comment

**Severity:** minor
**Location:** `crates/goad/tests/renderer/ingress.rs`, T3 (`a_too_soon_refusal_decided_while_idle_reaches_the_window_at_once`: the R1 and R2 `≤ I/2` asserts) and T4 (`a_command_during_a_coalesced_interval_presents_at_once_and_carries_the_refusal`: the precondition `< I/4` and the `≤ I/2` lag).

**Expected:** `plan.md` (execution notes): "**Every timed assertion states which way load moves it** in a comment beside the bound"; `design.md` §8 R1 says the same of §9's assertions, and §9's table gives the directions (T3 "toward red; margin about `I/2`", T4 "toward red … the precondition fails only after a stall longer than `I/4`").
**Observed:** T2 (a), (b), (c) and VT-7's assertion 3 carry one; the four bounds in T3 and T4 carry none. The directions in `design.md` §9 are right (all four are toward red); they are just not where the plan put them.
**Evidence:** `grep -n -i "toward red" crates/goad/tests/renderer/ingress.rs` → only T2's three comments and VT-7's doc.

**Disposition:**
**Response:**

**Outcome:**

### F-7 — `Debounce::tick`'s doc cites `serve`'s present sites by line number, now further adrift, and counts them

**Severity:** minor
**Location:** `crates/goad/src/pending.rs`, the doc of `Debounce::tick` ("`serve` presents at three sites: `controller.rs:898` … `:994` … `:1046`").

**Expected:** `CLAUDE.md` *Name, never count — cite by symbol*. The slice moved every line this cites.
**Observed:** the three `glass.present` calls in `serve` are at 921, 1055 and 1108 at HEAD (they were at 900, 996, 1048 at `d2617c1`, so the citation was already off by two before this slice; the slice widened it to 23–62 lines). "three sites" is a count nothing holds. The semantic claim still holds — I checked it against the new loop: the engage present is still reached synchronously from the `select!` that yields the firing (now the `'idle` one), and the new arm's present is the top present, which the drain precedes. So the repair is naming: *the top present*, *the engage present*, *the inner `select!`'s ingress-`None` arm*.
**Evidence:** `grep -n "glass.present" crates/goad/src/controller.rs` at HEAD and at `d2617c1`; `grep -rn "controller.rs:[0-9]" crates docs/{specs,policy,adr}` finds this as the only such citation (class is one instance).

**Disposition:**
**Response:**

**Outcome:**

### F-8 — R-15's new clock-overflow clause describes a regime in which the adjacent timer arm panics

**Severity:** minor
**Location:** `crates/goad/src/controller.rs`, `serve`: the initial `sleep` (`sleep_until(started + MINIMUM_SPACING)`) and the scheduled arm's write `floor_until = tokio::time::Instant::now() + MINIMUM_SPACING`; `canon-delta.md` Change 1 ("where the end of an interval cannot be represented on the host's clock, the interval is not held and every refusal updates the surface").

**Expected:** the rule the file states for itself (`ingest`, `deadline_after`): "`Instant + Duration` panics on overflow, and a panic here takes the host down", hence `checked_add` at every such site. The new arm follows it (`now.checked_add(REFUSAL_PRESENT_INTERVAL).unwrap_or(now)`).
**Observed:** two sites in the same `serve` still use the panicking `+`. In the only regime where the new arm's fallback matters (a clock within `REFUSAL_PRESENT_INTERVAL` of its end), `now + MINIMUM_SPACING` also overflows, so the next scheduled firing panics — the host goes down within one schedule, and R-15's degraded-but-live clause is not a state the host can sit in. Pre-existing (not introduced by this slice), practically unreachable on a monotonic-since-boot clock, but the slice promotes a canon sentence about exactly this regime, and the class was fixed site-by-site rather than whole.
**Evidence:** `grep -n "Instant::now() +\|started +" crates/goad/src/controller.rs` → the two `serve` sites (plus one in a `#[cfg(test)]` test, harmless).

**Disposition:**
**Response:**

**Outcome:**

### F-9 — `flood.py`'s "last key" is the last key *recorded after its reply*, which can sit either side of what the host shows

**Severity:** nit
**Location:** `docs/slices/011/flood.py` — module docstring ("prints the refusal count so far and the last key sent"), `Tally.record`, `writer_loop`.

**Expected:** the printed key is what a person compares with the window (`plan.md` PHASE-03 hand-over step 6, loosened in `notes.md` to "the last key printed, or a few above it").
**Observed:** `writer_loop` claims an index, connects, reads the reply, then `record`s. Across four writers, the host decides in *accept* order while `last_key` is set in *record* order, and indices are claimed before connecting. So the host's last-decided key can be **below** the printed one (writer B claims 10, writer A claims 11, A is accepted first, B decided last → window shows `flood-10`; A records after B → prints `flood-11`), not only "a few above it". The docstring's "sent" is also wrong: it is the last *replied*. Harmless to VH-1 (already run), but the hand-over wording is the one a person would repeat.
**Evidence:** `writer_loop`: `index = tally.claim(); write_one(path, index); tally.record(index)`.

**Disposition:**
**Response:**

**Outcome:**

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

**Round 1 (raiser, before dispositions).** No blocker, no major. The mechanism is
correct; every finding is in the tests' tightness, the prose around the code, or
a pre-existing neighbour.

**Checked and found complete** (the next round can narrow past these):

- **Every exit from `'idle`.** Traced each `select!` arm at `fdc2229`: stop and
  closed channel `break 'serving` and make no update (D8); a command, a
  scheduled firing and an accepted arrival `break 'idle` and reach either the
  top present (`None`, refusal site) or the engage present; the ingress-stopped
  fold `continue 'serving`s (D6); the new arm `continue 'serving`s; a refused
  arrival sets the flag and `continue 'idle`s. Every non-ending exit presents,
  so a stale fold is never dropped except by R-15's named overwrite and
  loop-end exceptions. Bare `continue`s after the `'idle` block bind to
  `'serving`, as intended.
- **R-15 as drafted holds of the code.** Leading edge (elapsed deadline, D3),
  trailing edge (arm at `F`), "unless another update has come sooner" (the flag
  is re-declared `false` on re-entry after any present), loop end, overwrite,
  and the clock-overflow fallback. `next_refusal_present` is written only by
  its arm (D5), so no person's present moves it. The code does nothing the
  delta does not say.
- **`biased` ordering.** No starvation: the arm self-disables (the flag is
  fresh on the re-entry that follows its present), and the arms above it each
  leave `'idle`. D4's claim that I-2 rests on the timer and not on
  goad-shell's `channel(1)` holds in production, because `main.rs` drives
  tokio's timers on a separate multi-thread runtime while `serve` runs on
  Slint's executor; tokio's `Sleep` is ready on driver state, not on a clock
  compare (`TimerEntry::poll_elapsed`), so in the renderer tier's current-thread
  runtime the same guarantee also leans on tokio's coop budget. Not raised.
- **I-4.** `floor_until`, `event_floor_until` and `sleep` keep their single
  write sites; the new arm touches none of them.
- **The tests read the window.** `RecordingGlass::present` delegates first,
  then reads `get_mode` / `get_diagnostic_lines` (D12). T2's key matching is
  exact: `names_key` wraps the key in backticks, matching
  `EnvelopeFault::Unknown`'s ``unknown key `{key}` `` Display in goad-shell. T3's
  and T4's substring keys (`t3-r2`, `t4-a`, `t4-b`) cannot collide.
- **Load directions** in `design.md` §9 are right for every bound (all toward
  red; T2(a)'s only red-ward term is the first present's lag, margin `ε`).
- **Suite at rest.** `cargo test -p goad --test renderer -- ingress::` on a
  scratch copy of HEAD: 15/15, twice.
- **Mutations run here** (scratch copy only): interval 800 ms and 700 ms (green
  on `div_ceil`, red on floor — F-1); interval 900 ms (green either way — the
  floor form's resolution limit); a leading-edge double present gated on
  `Instant::now() >= deadline()` (red by an order of magnitude, because the
  timer driver lags the deadline — so it does not isolate the +1 case, and F-1
  rests on the interval mutations instead). The recorded M0–M9, M8b and R2
  rows in `notes.md` were read, not re-run.
- **Invariants.** Only `crates/goad/src/controller.rs` and the renderer test
  changed under `crates/`; `goad-semantics` is untouched; no domain vocabulary
  in the new host identifiers.

**Doubtful, raised:** F-1 (T2(a)'s slack; the only finding touching a claim the
canon-delta makes), F-2/F-3/F-4 (controller.rs prose), F-5/F-6 (test prose),
F-7 (line-number citation class), F-8 (pre-existing overflow sites next to the
new canon clause), F-9 (`flood.py`).

**Process note.** While this review copied the tree to a scratch directory,
`crates/goad/src/controller.rs` in the **live** worktree carried the R2
mutation (the ingress-stopped fold `continue 'idle`), with `git status` clean
before and after — another agent mutating in place. The first scratch run went
red on VT-7 for that reason; the copy was re-taken from `git show HEAD:` before
any result above was recorded. Memory `one-writer-per-worktree` applies.

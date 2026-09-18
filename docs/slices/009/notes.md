# Notes — Slice 009

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Handover — design review, after round 2's findings were integrated

Written 2026-09-18 for a fresh agent. Delete once the design closes.

### Where the slice is

Design drafted and accepted by the user (sections 1-10), then substantially
rewritten across two review rounds. **The review loop is open: round 3 is owed,
and it is the next thing to do.** Plan not started.

Round 2's seven live findings — F-30, F-32 … F-37 — are **integrated into
`design.md`** at this commit. F-31 is `withdrawn`. Every finding in the ledger
carries a disposition; F-19 … F-30 and F-32 … F-37 carry `_pending round 3_` as
their outcome, which is round 3's job to set.

### What holds the truth

| file | state |
|---|---|
| `review-design.md` | **the ledger, and the live one.** F-1 … F-37. The Responses are written to be complete without the session that wrote them |
| `design.md` | current truth. Nothing is outstanding against it |
| `design-log.md` | D-1 … D-22, the user's decisions. Append-only. Note the header: `D-n` here is **not** `Dn` in `design.md` §7, and from 18 they overlap on adjacent subjects |
| `research.md` | Thread 3 carries every measured fact |
| `canon-delta.md` | CD-1, CD-2 |
| `spike-fields/` | committed at `4f93d41`. Delete when the design closes; the facts are in Thread 3 |

### What is owed, in order

1. **Round 3.** See *What round 3 must attack* below. Use a genuinely fresh
   reviewer — rounds 1 and 2 shared one Codex thread, and round 3's subject is
   largely text that thread wrote the objections to.
2. **Revise `slice-009.md`**: it still lists all five OQs as open, its
   §Governing canon omits `canon-delta.md`, and §Scope owes
   `crates/goad/Cargo.toml` (§7 D18's `jiff` feature). `AGENTS.md` puts this
   after the findings are integrated, which they now are.
3. **Re-ask the user for acceptance.** The design has changed substantially
   since theirs.
4. **Plan**, with a fresh agent.
5. **Delete `spike-fields/`** when the design closes.
6. `just check` is owed once repairs reach code. Nothing under `crates/` has
   been touched, so the gate's subject is unchanged.

### What round 3 must attack, and why

The rate of new defects is not falling. Round 1 found 18; round 2 found 11 more;
**round 2's integration found four, two of them blockers**; dispositioning those
found three more; and integrating *those* found F-37, a blocker. Each pass over
the same text has found something the last one did not.

Four live risks to point round 3 at:

- **F-37 is new design surface, written at integration time and reviewed by
  nobody.** A second value type (`Reported`) and a kind-directed `resolve` in
  `view_model.rs` now carry every edit. That is the largest single addition since
  the design was accepted and it has had one pair of eyes.
- **§5.2's numeric account is long and was wrong twice.** The guard's comparand,
  the parse rule and F-34's "last representable number stands" are now one
  argument spanning three paragraphs and two types. The third answer was measured
  rather than argued, which is why it is probably right — but the *composition*
  was not measured.
- **§5.4 was rewritten on a mechanism that was wrong twice** (F-31 wrong about
  persistence, F-35 wrong about assignment). A third reading of the same widget
  deserves suspicion.
- **F-30's repair puts display text in `draft.rs`**, which declares itself pure
  and canonical. A real cost, taken deliberately (D-18), and the kind of thing a
  reviewer should push on rather than wave through.

### Facts verified by hand or by measurement, because they overturn things

1. **No `PopupWindow` state survives a close.** `show-popup` compiles to a fresh
   `::new()` per show; the closed instance is dropped. F-31 is wrong on that
   ground, and §5.4, §7 D21, §5.5 A-3 and §9 are rebuilt on the measured fact.
2. **A `PopupWindow`'s properties cannot be assigned from an enclosing
   component's handler** — a hard compile error. They can be *bound* at the
   popup's declaration site; `show()` from outside is fine, which is how the
   first spike missed it (F-35).
3. **The guard as designed at round 2 corrupts ordinary typing**: `1.05` becomes
   `105`, `-3` becomes `3`. Measured, injection-passed (F-30).
4. **`input-type: decimal` admits exactly three texts no parse accepts** — `-`,
   the locale separator, and `-` followed by it (`items/text.rs:2202-2229`).
   `--` is **not** one of them; the design's old edge example could not be typed.
   Verified by hand this session.
5. **`Command::Edit` could not carry two of `Edited`'s five variants** — an
   `AlternativeId` cannot be minted in a callback, and a number's fallback lives
   in the draft. That is F-37, found by trying to write F-30's repair down.
6. **Popups *are* reachable under `init_no_event_loop`** (round 2's finding, still
   true). F-13 was wrong and §9 was rebuilt on it. But no case in this repository
   has yet needed a popup **laid out**, which is what `mock_single_click` depends
   on — §8 R9.
7. **The command channel is capacity 1** (`main.rs:86`) and `serve` shares the UI
   thread, so the second `try_send` of any flush is certain of `Full`.
8. **`reception.rs:753` does not exist** — the file is 103 lines. F-10's contest
   cites it and is upheld anyway on `draft.rs:82` and `view_model.rs:31`.

### Citations known bad, and what that means

The ledger is append-only, so bad citations inside a Response stay as written.
Four are known:

- F-10's re-disposition — one `wiring.rs` site, not two.
- F-23's Response — `wire.rs:130`, not `:126`.
- the pre-repair §9's `set_accessible_value` claim, which appears nowhere here.
- F-33's Response cites `fluent/components.slint:15-19` for `ListItem`'s
  accessible properties. They are at **`:49-53`**. `research.md` and `design.md`
  both carry the corrected form.

All four were written by the **responder**, not by the reviewer. Round 2's own
citations checked out. **The responder's citations are the unreliable ones —
verify those first.**

### How this review has been run, and why

- Rounds 1 and 2: Codex (`gpt-5.6-sol`) via the `codex` MCP, sharing thread
  `01a0ad06-ba40-7821-8d33-016b8cc4b0ee`. Right for setting outcomes, wrong for
  a fresh round.
- **Do not integrate your own dispositions.** Round 1's responder was wrong about
  four of its own repairs; round 2's integrator found four more defects; this
  session's integration found F-37. The rule is discharged by handing over
  between sessions rather than by spawning a subagent underneath the session that
  decided (D-21).
- Prompt the reviewer with **surfaces, not conclusions**
  (`docs/memory/dont-feed-the-raiser-your-finding.md`), and have it write to a
  file — agent reports truncate, and this one truncated twice.
- **Spike anything a spike can answer** (D-19). Round 2's spike refuted a
  blocker, found two defects nobody had raised, and shrank a third.
- The user asked for plainer prose: fewer punchy one-liners, more the way an
  engineer explains something to a colleague. §5.1-§5.3 are the model.

### Traps worth naming

- `design-log.md` is append-only; `design.md` §7 is current truth and its entries
  are rewritten in place under immutable ids. Two rules, two id sequences one
  hyphen apart.
- **A Slint `changed <property>` handler fires on a *change*, not on a write**,
  and the comparison happens at flush time against the last value the tracker
  stored (`i-slint-core/properties/change_tracker.rs:138-141`). So writing a
  perturbation and then the real value *inside one handler* fires nothing. This
  has caught the design twice and a finding once.
- **Reading a widget's source tells you what an instance does, never how long the
  instance lives.** That is F-31, and it is F-13's failure mirrored.
- One event-loop **arrangement**, one `[[test]]` target
  (`docs/memory/slint-testing-backend-initialises-once-per-process.md`).
- Two 64-to-32-bit narrowings were found in one round (§8 R7). Treat any
  host↔markup conversion as guilty until checked.
- **The guard's comparand was wrong twice**, and the third answer was measured
  rather than argued. Treat a fourth proposal the same way.
- **A type that only the controller can construct cannot be built in a Slint
  callback.** That is F-37, and `AlternativeId` is not the only such type in
  `goad-semantics`.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 | pending / in progress / done / blocked | |

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

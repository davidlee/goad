# Notes — Slice 009

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Handover — design review, after round 2's integration

Written 2026-09-17 for a fresh agent. Delete once the design closes.

### Where the slice is

Design drafted and accepted by the user (sections 1-10). **The review loop is
open**, two rounds run, a third owed. Plan not started. HEAD carries the design,
round 2's integration, the spike, and this session's ledger work.

**Every finding now carries a disposition.** F-1 … F-18 have terminal outcomes;
F-19 … F-30 and F-32 … F-36 are disposed and pending round 3; F-31 is
`withdrawn`. Nothing in F-30 … F-36 has been **integrated into `design.md`** —
that is your first job.

### What holds the truth

| file | state |
|---|---|
| `review-design.md` | **the ledger, and the live one.** F-1 … F-36. The Responses are written to be complete without the session that wrote them: they are your brief |
| `design-log.md` | D-1 … D-21, the user's decisions. Append-only. Note the header: `D-n` here is **not** `Dn` in `design.md` §7, and from 18 they overlap on adjacent subjects |
| `design.md` | current truth as of round 2's integration. Seven findings are outstanding against it |
| `research.md` | Thread 3 carries every measured fact, including this session's three new sections |
| `canon-delta.md` | CD-1, CD-2 |
| `spike-fields/` | committed at `4f93d41`. Delete when the design closes; the facts are already in Thread 3 |

### What is owed, in order

1. **Integrate F-30 and F-32 … F-36 into `design.md`.** Dispositioned and
   confirmed by the user (D-20); not applied. F-30 and F-35 are blockers.
2. **Round 3.** See *How this review has been run* below.
3. **Revise `slice-009.md`**: it still lists all five OQs as open, its
   §Governing canon omits `canon-delta.md`, and §Scope owes
   `crates/goad/Cargo.toml` (§7 D18's `jiff` feature). `AGENTS.md` puts this
   after the findings are integrated.
4. **Re-ask the user for acceptance.** The design has changed substantially
   since theirs.
5. **Plan**, with a fresh agent.
6. **Delete `spike-fields/`** when the design closes.
7. `just check` is owed once repairs reach code. Nothing under `crates/` has
   been touched, so the gate's subject is unchanged.

### What round 3 must attack, and why

Round 2's shape was: eleven of eighteen repairs rewrote a section, and five were
wrong. **Round 2's integration then found four more defects, two of them
blockers** — and dispositioning those found three more (F-34 … F-36). The rate
is not falling. Round 3 gets the round-2 repairs *and* everything F-30 … F-36
changes.

Two live risks to point it at:

- **§5.4 and §7 D21 are being rewritten on a mechanism that was wrong twice.**
  F-31 said a shared popup goes stale; the measurement says no popup state
  survives a close at all. F-36 rewrites two arguments on the new fact. A third
  reading of the same widget deserves suspicion.
- **F-30's repair puts display text in `draft.rs`**, which declares itself pure
  and canonical. That is a real cost, taken deliberately (D-18), and it is the
  kind of thing a reviewer should push on rather than wave through.

### Facts verified by hand or by measurement, because they overturn things

1. **No `PopupWindow` state survives a close.** `show-popup` compiles to a
   fresh `::new()` per show; the closed instance is dropped. So F-31 is wrong,
   D21's "field B opens on field A's pick" rationale is wrong, and §5.4's
   loop-tier argument for a re-seed is wrong (F-36). Seeding is still needed,
   for the *picked* field.
2. **A `PopupWindow`'s properties cannot be assigned from an enclosing
   component's handler** — a hard compile error. §5.4 specifies exactly that
   (F-35). They can be bound at the popup's declaration site; `show()` from
   outside is fine, which is how the first spike missed it.
3. **The guard as designed corrupts ordinary typing**: `1.05` becomes `105`,
   and `-3` becomes `3`. Measured, injection-passed (F-30).
4. **Popups *are* reachable under `init_no_event_loop`** (round 2's finding,
   still true). F-13 was wrong and §9 was rebuilt on it.
5. **The command channel is capacity 1** (`main.rs:86`) and `serve` shares the
   UI thread, so the second `try_send` of any flush is certain of `Full`.
6. **`reception.rs:753` does not exist** — the file is 103 lines. F-10's
   contest cites it and is upheld anyway on `draft.rs:82` and `view_model.rs:31`.

### Citations known bad, and what that means

The round-2 integrator opened every `path:line` in the ledger. Three did not
check out, **all three written by the responder, not by Codex**: F-10's
re-disposition (one `wiring.rs` site, not two), F-23's Response (`wire.rs:130`,
not `:126`), and the pre-repair §9's `set_accessible_value` claim, which appears
nowhere in this repository.

Round 2's own citations checked out. **The responder's citations are the
unreliable ones — verify those first.**

### How this review has been run, and why

- Reviewer: Codex (`gpt-5.6-sol`) via the `codex` MCP. Rounds 1 and 2 shared
  thread `01a0ad06-ba40-7821-8d33-016b8cc4b0ee`, which is right for setting
  outcomes — that is the raiser's job. **Use a genuinely fresh reviewer for
  round 3**: a thread that has agreed twice is cheap to agree with a third time,
  and round 3's subject is largely text that thread wrote the objections to.
- **Do not integrate your own dispositions.** Round 1's responder was wrong
  about four of its own repairs; round 2's integrator found four more. This has
  been run by handing over between sessions rather than by spawning a subagent
  underneath the session that decided (D-21).
- Prompt the reviewer with **surfaces, not conclusions**
  (`docs/memory/dont-feed-the-raiser-your-finding.md`), and have it write to a
  file — agent reports truncate, and this one truncated twice.
- **Spike anything a spike can answer** (D-19). This session's spike refuted a
  blocker, found two defects nobody had raised, and shrank a third. It cost less
  than the argument it replaced.
- The user asked for plainer prose: fewer punchy one-liners, more the way an
  engineer explains something to a colleague. §5.1-§5.3 are the model.

### Traps worth naming

- `design-log.md` is append-only; `design.md` §7 is current truth and its
  entries are rewritten in place under immutable ids. Two different rules, and
  two id sequences one hyphen apart.
- **A Slint `changed <property>` handler fires on a *change*, not on a write**,
  and the comparison happens at flush time against the last value the tracker
  stored (`i-slint-core/properties/change_tracker.rs:138-141`). So writing a
  perturbation and then the real value *inside one handler* fires nothing. This
  has now caught the design twice and a finding once.
- **Reading a widget's source tells you what an instance does, never how long
  the instance lives.** That is F-31, and it is F-13's failure mirrored.
- One event-loop **arrangement**, one `[[test]]` target
  (`docs/memory/slint-testing-backend-initialises-once-per-process.md`).
- Two 64-to-32-bit narrowings were found in one round (§8 R7). Treat any
  host↔markup conversion as guilty until checked.
- **The guard's comparand has now been wrong twice**, and the third answer was
  measured rather than argued. Treat a fourth proposal the same way.

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

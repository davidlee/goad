# Notes — Slice 009

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Handover — design review, after round 3's integration

Written 2026-09-18 for a fresh agent. Delete once the design closes.

### Where the slice is

Design accepted by the user at draft, then rewritten across three review rounds.
**The review loop is open.** Round 3's thirteen findings — F-6, F-20, F-26, F-29,
F-32 and F-38 … F-45 — are **integrated** into `design.md`, `slice-009.md` and
`canon-delta.md`. Rounds 1 and 2's are integrated too, and round 3 verified 18 of
them. Plan not started.

Integrating raised four more, F-46 … F-49, dispositioned and **not yet confirmed
by the user**. None is a blocker. What is outstanding, all reading
`_pending round 4_`: the thirteen, awaiting round 4's terminal outcomes; and
F-46 … F-49, awaiting both the user's confirmation and round 4.

**What changed in the integration**, in one line each, because the ledger's
Responses do not all say where the text landed:

- the F-40 family became one mechanism: a pending entry carries the view it was
  made on (§7 D27), the value channel is the draft overlaid with it (§7 D26), the
  timer delivers one entry per tick and re-arms, and `Wire::send` reports whether
  the command was enqueued. §5.5 states it as **I-H**, which is new.
- `resolve` takes `held: Option<&Edited>` and applies `as_drawn` itself, so the
  two `as_drawn` sites are now `answer` and `resolve` rather than `answer` and
  `edit`.
- §9 gained four rows — two fields in one window, the numeric exception measured
  against the overlay, the picker re-seed (F-44), and a rewritten AC-6 driver
  (F-47) — and §8 gained **R10**.
- `slice-009.md` §Scope gained `main.rs` and the overlay; AC-6 gained one
  sentence of precision.

### Why a fresh agent, and not the session that dispositioned these

D-21. The session that writes a disposition does not integrate it, and this has
now paid for itself three times: round 1's responder was wrong about four of its
own repairs, round 2's integrator found four more defects, and the session that
integrated round 2 found F-37 — a blocker — by trying to write the repair down.
Round 3 then found that same session had inverted F-32's citation and overclaimed
a type property (F-42). **Expect to find something. Integrating is a form of
review.**

### What holds the truth

| file | state |
|---|---|
| `review-design.md` | **the ledger.** F-1 … F-45, with round 3's outcomes set and its probed-and-sound list. Every Response is written to be complete without the session that wrote it: they are your brief |
| `design.md` | current truth as of round 2's integration. Thirteen findings are outstanding against it |
| `design-log.md` | D-1 … D-24. Append-only. Note the header: `D-n` here is **not** `Dn` in `design.md` §7 |
| `research.md` | Thread 3 is everything measured |
| `canon-delta.md` | CD-1, CD-2. F-45 touches CD-2 |
| `spike-fields/` | committed at `4f93d41`. Delete when the design closes |

### What is owed, in order

1. **Confirm F-46 … F-49 with the user.** Dispositioned and integrated, not yet
   confirmed. The protocol wants the confirmation before the repair; this one ran
   the other way because all four were found *by* writing the repair and three of
   them are corrections to round 3's own dispositions. Say so when asking.
2. **Round 4**, with a fresh reviewer again. Round 3 was a new Codex
   (`gpt-5.6-sol`) thread and was worth its cost; thread
   `01a0b212-239e-70d3-9a99-09729c82b971` is *its* thread and is therefore the
   right one for setting outcomes and the wrong one for raising. Round 4's brief
   is not written yet — write it before the reviewer runs, not after.
3. **Re-ask the user for acceptance.** The design has changed twice since theirs.
4. **Plan**, with a fresh agent.
5. **Delete `spike-fields/`** when the design closes.

### Carried forward, outside this slice

`docs/memory/a-popup-is-rebuilt-on-every-show.md` cites
`widgets/fluent/components.slint:15-19` for `ListItem`'s accessible properties.
They are at `:49-53` — the same bad citation F-33 found in the design, and the
memory doc has it too. Not fixed mid-slice; lift it at close.

### Integration notes the ledger did not carry — now applied

Worked out while dispositioning, and applied during the integration. Kept because
each one records *why* the text reads as it does. **One of them was wrong**: the
first bullet's claim that pushing `as_drawn` inside `resolve` "puts `as_drawn`'s
two call sites back in `view_model.rs`" is false — `answer` is in
`controller.rs`. Its own next clause has it right, and §5.2 says `answer` and
`resolve`.

- **`resolve`'s signature should take `held: Option<&Edited>`, not `&Edited`.**
  §5.2 as integrated has the caller do `state_of(..).unwrap_or_else(|| as_drawn(kind))`.
  Push that inside: `resolve` already has the kind, so it can consult `as_drawn`
  itself, and then both callers pass `state_of(..)` straight through. This also
  puts `as_drawn`'s two call sites back in `view_model.rs` where it lives, and
  §5.2's sentence about which sites apply it changes for the second time — it is
  now `answer` and `resolve`.
- **The overlay should go through `resolve` rather than through a second
  mapping.** `pending.rs` holds `Reported`, and `glass.rs` displays from `Edited`.
  Resolving the pending entry and using the result in place of the draft's value
  keeps **one** display mapping; writing a `Reported` → `FieldValue` mapping
  beside the existing `Edited` → `FieldValue` one is the duplication to avoid.
- **F-38's rule applies at three sites, not the two its Response names.** An
  entry is *shown*, *sent* and *drained* only where its view is the retained one.
  The display site matters: on a new view the rows are rebuilt and slots
  renumbered, so a stale entry keyed `(option, field)` whose ids happen to match a
  new field would otherwise be overlaid onto the new view's widget. State it once,
  as one rule over three sites.
- **The overlay creates a property worth stating as an invariant:** what the
  screen shows is what an answer would submit. A drained entry reaches the draft;
  a kept entry is still displayed and still travels in the next `Choose`; a stale
  entry does neither. That is new and it is better than what the design had.
- **Construction order is already right.** `main.rs:85-101` installs the callback
  table before building `SlintGlass`, so the `Rc<Pending>` is created at step 6
  and cloned into `install` and `SlintGlass::new` both. One field on
  `SlintGlass`, no reordering.
- **`Wire::send`'s result is already in hand.** `wire.rs:127-133` binds
  `TrySendError::Full(_returned)` and drops it deliberately (D8). F-39 is a return
  type, not a mechanism.
- **CD-2's `R-16` mention (F-45)** is in its `**Document:**` line only; the three
  changes below it cover `R-57`, `R-58`, `R-55`. `SPEC-001`'s `R-13, R-14, R-16`
  row is about wire forms and is untouched by drawing.

### Facts verified by hand, because they overturn things

1. **The serve loop presents before every command** (`controller.rs:738-739`:
   `glass.present(...)` is the first statement of `'serving: loop`). That is what
   makes F-40 real: any handled command inside a debounce window repaints from a
   draft that does not yet hold the person's typing.
2. **`Wire::send` returns `()`** and swallows `Full` (`wire.rs:127-133`). F-39.
3. **`increment()` is `set-value(value + step)`**
   (`common/slider-base.slint:126-131`), so F-20's ulp case freezes the slider:
   at `minimum = 2^100` the `f32` ulp is `2^77` and a one-ulp span gives a step
   below half an ulp.
4. **The ICU decimal separator is live in this build.** `i-slint-core`'s default
   `std` feature enables `i-slint-common/locale-decimal-separator`
   (`i-slint-core/Cargo.toml:82-95`), and `string_to_float` replaces *that*
   character, rejecting `.` outright when the separator is not `.`
   (`i-slint-core/string.rs:398-412`). It is **not** reachable from host code:
   `SlintContext::locale_decimal_separator` is `i-slint-core`, which `crates/goad`
   does not depend on, and `slint` re-exports neither it nor `string_to_float`.
   F-26.
5. **`input-type: decimal` admits exactly three texts no parse accepts** — `-`,
   the locale separator alone, and `-` followed by it
   (`i-slint-core/items/text.rs:2202-2229`). `--` is not among them.
6. **No `PopupWindow` state survives a close**, and a popup's properties cannot be
   assigned from an enclosing handler. Both measured (F-31 withdrawn, F-35).
7. **Popups are reachable under `init_no_event_loop`** — `find_all` walks
   `active_popups` (`search_api.rs:291-312`) — but no case here has yet needed one
   **laid out**, which `mock_single_click` depends on (§8 R9).
8. **The command channel is capacity 1** (`main.rs:86`) and `serve` shares the UI
   thread, so one command per timer tick is the most that is available.

### Citations known bad

The ledger is append-only, so a bad citation inside a Response stays as written.
Five are known: F-10's re-disposition (one `wiring.rs` site, not two); F-23's
Response (`wire.rs:130`, not `:126`); the pre-repair §9's `set_accessible_value`
claim, which appears nowhere here; F-33's Response (`fluent/components.slint:15-19`
for `ListItem`; they are at `:49-53`); and F-42's location line, which cites §5.5
I-G for a claim that is in §5.2.

All were written by a **responder**, not by a reviewer. Rounds 2 and 3's own
citations checked out. **Verify the responder's first.**

### How this review has been run, and why

- Rounds 1 and 2: one Codex (`gpt-5.6-sol`) thread,
  `01a0ad06-ba40-7821-8d33-016b8cc4b0ee`. Round 3: a fresh thread,
  `01a0b212-239e-70d3-9a99-09729c82b971`. Use a **new** thread to raise; reuse a
  round's own thread only to set that round's outcomes.
- Prompt the reviewer with **surfaces, not conclusions**
  (`docs/memory/dont-feed-the-raiser-your-finding.md`) and have it write to a
  file — reports truncate, and this one has truncated twice.
- **Spike anything a spike can answer** (D-19).
- The user asked for plainer prose: fewer punchy one-liners, more the way an
  engineer explains something to a colleague. §5.1-§5.3 are the model.

### Traps worth naming

- `design-log.md` is append-only; `design.md` §7 is current truth and rewrites an
  entry in place under its own immutable id. Two rules, two id sequences one
  hyphen apart.
- **A Slint `changed <property>` handler fires on a *change*, not on a write**,
  compared at flush time against the last value the tracker stored
  (`i-slint-core/properties/change_tracker.rs:138-141`). Writing a perturbation
  and then the real value inside one handler fires nothing.
- **Reading a widget's source tells you what an instance does, never how long the
  instance lives** (F-31, F-13's failure mirrored).
- One event-loop **arrangement**, one `[[test]]` target
  (`docs/memory/slint-testing-backend-initialises-once-per-process.md`).
- Two 64-to-32-bit narrowings were found in one round (§8 R7). Treat any
  host↔markup conversion as guilty until checked — and F-20 is the same class one
  level down: exact endpoints are not an operable range.
- **The guard's comparand has been wrong three times**, twice on reasoning and
  once corrected by measurement. D-23 touches it again. Re-run
  `numeric_guard.rs` rather than arguing about the exception.
- **A type only the controller can construct cannot be built in a Slint
  callback** (F-37). `AlternativeId` is not the only such type in
  `goad-semantics`.
- **Prose outside §9's obligations table binds nothing** (F-11, and then F-44 for
  exactly the same reason one round later).

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

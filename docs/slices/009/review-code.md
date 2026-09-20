# Review — implementation — Slice 009

**Subject:** implementation — `a698217..HEAD` on `main`, 107 commits.
35 files and +7535/-595 under `crates/`. The subject is the code, not the
record: document-truth divergences are the audit's reconciliation, and appear
here only where a doc comment or a declared surface makes a *code* claim that
is false.
**Reviewer:** fresh agents, three dimensions, round 1 — Opus 5
**Opened:** 2026-09-20
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

Written before the review. This slice took a renderer that drew one of five
field kinds and made it draw all five, and in doing so replaced the mechanism
by which a present reaches the form. Three surfaces carry that, and each has a
different way of being wrong.

**Where the bodies are likely buried.**

- **The pure layer decides what reaches the wire.** `R-57` types what each kind
  submits and `R-58` says which fields carry a value at all. Everything between
  a widget and the wire is `draft.rs`, `view_model.rs`, `instant.rs` and
  `controller.rs`, and every one of those is total by construction *on the
  author's account*. Totality claimed in a doc comment is not totality.

- **Two channels and a guard replaced a rebuild.** `glass.rs`, `app.slint`,
  `install.rs`, `wire.rs`, `pending.rs` and `main.rs` now share mutable host
  state across a Slint callback boundary, an `Rc`, a timer and an async serve
  loop. The design names re-entrancy, write order and handle-splitting as risks
  and argues each away in prose.

- **A suite that grew by 1770 lines in one file.** The project's recorded
  failure mode is a green case that asserts a proxy for the thing it names
  (`docs/memory/a-green-test-can-assert-a-proxy.md`; slice 004 shipped four).
  Every new case here claims an injection pass. An injection pass proves a case
  can go red — not that it goes red for the defect the criterion is about.

**Invariants the subject is held to.** `CLAUDE.md`'s five, in full. In
particular: **no backend input may take the host down or leave a refusal
unattributed**, and **wire compatibility is never narrowed to fit the
renderer** — which in this slice runs the other way, since the renderer subset
is what was deleted. Plus `SPEC-001` R-16, R-17, R-18, R-35, R-52, R-53, R-55,
R-57, R-58; `ADR-001` and `POL-001` (stratum 1 stays pure, and the gate's four
instruments do not reach the dependency-feature residue).

**Round 1** — 2026-09-20 — three dimensions in parallel: the pure layer and the
wire contract; the renderer, its shared state and its markup; the suite and
what each case would survive. Each reviewer was given a surface and no
conclusion.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-A1 | blocker | | |

### F-A1 — every field control is deaf for the whole backend round trip, so a keystroke typed during an exchange is discarded, not deferred

**Severity:** blocker
**Location:** `crates/goad/src/controller.rs:921-922` and `:195`;
`crates/goad/ui/app.slint:128`, and the seven `enabled: !root.busy` sites at
`:423`, `:476`, `:547`, `:610`, `:683`, `:743`, `:784`

**Expected.** `slice-009.md` **AC-4**: *"Typing into a `text` field records
every character. Stated as an observable because the defect it excludes — one
character per click — is what the re-present work exists to prevent."* And
**AC-5**: *"A present that changes nothing about a field does not disturb it:
no destroyed element, no moved caret, no interrupted drag."*

**Observed.** `Controller::engage` sets `engaged` and `serve` presents with
`busy = true` before **every** exchange (`controller.rs:921-922`), a routine
scheduled poll included. `engaged` is not cleared until `absorb` folds the
completed exchange (`controller.rs:195`), so `busy` is true for the **entire
backend round trip**, not for a frame. `frame.busy` disables all seven field
controls.

Slint does not queue input for a disabled item — it drops it. `TextInput::
key_event` returns `EventIgnored` when `!enabled` before any handling
(`i-slint-core-1.17.1/items/text.rs:954`), and `TextInput::input_event` does
the same for pointer events (`:848`). `LineEdit` binds `enabled <=>
text-input.enabled` (`widgets/common/lineedit-base.slint:11`), and a `Slider`
reaches `enabled <=> touch-area.enabled`.

So the class is one sentence: **any key or pointer event delivered to any field
control while an exchange is in flight is discarded.** Three distinct costs,
one cause:

- a `Slider` mid-drag loses its grab — VH-1's measured failure, AC-5;
- a `LineEdit` mid-word **loses the characters typed during the exchange** —
  AC-4, and the one defect AC-4 was written to exclude;
- every other control flashes disabled — cosmetic.

**What VH-1's run could not see, and why it is not evidence against this.** The
caret *is* preserved: nothing issues a focus-out on disable, and `text.rs:1990`
only stops *rendering* the cursor (`cursor_visible() && self.enabled()`). So
*"the caret stayed where it was put"* is true and is a different claim from
*"every character was recorded"* — the same proxy shape the drag finding turned
on. The rig's scratch `backend.sh` answers in milliseconds, so the deaf window
was too short to type into. It is the exchange's duration, not the present
interval, and a backend that takes 300 ms makes this routine.

**Evidence.** Concrete and checkable without a person: hold a backend's reply
for 500 ms, present, type during the wait, and the characters are absent from
the draft and from the wire. No case in any tier delivers an input event to a
control while `busy` is true — `tests/renderer/fields.rs` drives
`set_accessible_value`, which assigns `text` and calls `edited` directly
(`fluent/lineedit.slint:16`) and therefore bypasses `key_event` and its
`enabled` guard entirely. That is why the suite is green over a criterion it
does not reach.

**Relation to the record.** `design.md` **A-6** — *"Disabling a widget while an
exchange is in flight does not destroy it … being wrong costs focus, not
data"* — is the assumption this falsifies. `notes.md`'s VH-1 block already
records the drag half and prices the class as *"a one-frame flash of the whole
form when idle, a dead drag when a gesture is in flight."* The flash is not one
frame and the loss is not only a drag: it is an exchange-long deafness, and
AC-4 is inside its blast radius.

**Disposition:**
**Response:**

**Outcome:**

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

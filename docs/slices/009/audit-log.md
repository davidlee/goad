# Audit log — Slice 009

Append-only, time-ordered. What the user was asked during audit and
reconciliation, and what they decided. The reasoning that produced a finding
lives in `review-code.md`; the closing argument lives in `audit.md`. This file
holds only decisions, cited to the finding or question that prompted them.

Same shape as `design-log.md` and `plan-log.md`, and created for the same
reason: `docs/AGENTS.md` §*Where it goes* puts a user decision in a log, and
the audit stage had no log of its own.

---

## 2026-09-20 — four decisions, taken together after round 1 of `review-code.md`

### F-A1 — the `busy` class: **fix now, by narrowing `busy`**

`busy` is true for the whole backend round trip and Slint discards rather than
queues input for a disabled item, so **AC-4 and AC-5 are both unmet from one
cause**. Four options were put: narrow `busy`, suppress the busy present, waive
both criteria, or defer the class to a follow-up slice.

**Decided: narrow it** — `busy` comes to mean *your answer is in flight* rather
than *the host is talking to the backend*. Both criteria are then **met rather
than waived**, which is why this was preferred to the waiver the Closure
checklist would have admitted. The two rejected fixes are rejected for stated
reasons and not on cost: suppressing the busy present fixes the flash and
leaves the deafness, now invisible until a slow backend (VH-1's lead 1 says so
in as many words); and `AGENTS.md` forbids deferring a fix merely because it is
large, which this is not — `engaged` is read exactly once, into `frame.busy`
(`controller.rs:400`), and `engage()` has one production call site (`:921`).

### The slider gap — **readout now, aiming as a follow-up**

Two problems were priced separately once the audit found the readout is nearly
free: `glass.rs:568` already writes the `text` slot for a slider, from the
host's own `spelled(number)` — the exact string `submitted` sends.

**Decided: land the readout in this slice**, because screen and wire then agree
by construction and §4's P-3 is satisfied with no rounding and no wire change.
**Quantisation is a follow-up slice**, because snapping the reported value to
`step` makes `step` normative for the value and `slice-009.md` §Non-goals
declined exactly that. The readout will display the full unrounded spelling,
which is ugly — and is the argument for the follow-up, in a form a person can
see rather than one that has to be explained.

### CD-1 — **promote descriptively, and gain a cleared-number clause**

**Decided: the descriptive form.** A backend MAY NOT rely on the epoch to mean
*untouched*; it is what this host sends, `R-58`'s existing instruction — do not
send the field — remains the way to distinguish unanswered, and `OQ-2` stays
the real answer to the question a sentinel would half-answer. This keeps the
slice's *no protocol change* non-goal intact.

**And a clause CD-1 did not have**, exposed by **F-P2**: a cleared bounded
`number` shows an empty box and submits **its minimum**, not `0`. That is a
fourth screen/wire divergence; `design.md` §5.5 I-H does not list it, three
sentences in `design.md` state the opposite, and CD-1 as drafted does not reach
it because CD-1 is about *untouched* rather than *cleared*. A backend author
reading an empty box and a `2.5` on the wire can discover it nowhere.

### The record — **CD-2, the roadmap and the example all land; `SPEC-001` OQ-4 to be discussed**

**Decided: fix before close.**

- **CD-2, corrected and then promoted.** Not optional: `SPEC-001`
  §Verification's `R-58` row cites two test functions PHASE-09 deleted, so
  canon is untrue about the tree as it stands. Two corrections first — CD-2's
  Change 3 cites `undrawn_form`, renamed to `drawn_form` at PHASE-05, and the
  `R-57` row's closing sentence names an arm that moved to the same place.
- **`examples/shell/backend.sh`.** Outside every phase's Surfaces and now false
  in three places, one of which describes runtime behaviour a person running
  `just demo` watches the host contradict.
- **`docs/roadmap.md`.** `design.md` §10 says this is owed at close: the
  roadmap names 009 as `SPEC-001`/OQ-4's answerer and OQ-4 stays shut.

**`SPEC-001` OQ-4's own wording: opened for discussion rather than decided.**
The audit's position, put to the user and not yet answered: OQ-4's fork — *its
own kind, or a hint on `datetime`* — is **asymmetric**, because `R-18` already
permits a renderer and only a renderer to branch on a hint, so the hint half
needs no protocol change at all. The clause *"no evidence asks for one yet"* is
true on its own terms and is now the wrong sentence: what 009 found is not
inexpressibility but an affordance cost. The recommendation is an **evergreen**
replacement — no slice number, no history, since `roadmap.md:519-547` already
carries the narrative and canon carries no revision history.

---

## 2026-09-20 — round 1 completed, and every finding dispositioned

Round 1 grew from fourteen findings to **twenty-one** when the renderer
dimension's two unattacked areas were run as a fresh agent rather than read as
a clean surface (`review-code.md`, `F-R3`–`F-R9`). Two results changed what the
repairs have to do.

### F-R1 — **fix now**, and it is settled by measurement rather than argument

F-R1 was raised *reasoned, not run*. It is now run: a loop-tier target drives
it with controls on both sides of every claim, and it is **confirmed and wider
than it was written** — the picker survives `hide()` as well as a view
replacement, so a person can be left with a picker over nothing, and the form
beneath is unreachable by pointer *and* by keyboard while the widgets
themselves stay alive. The injection pass fixed the repair's shape at **one
call site**: `present`'s `self.shown != showing` branch covers `Shift::Replaced`
and `Shift::Closed` together. The case is red by design and lands with the
repair.

### F-R3 — **fix now, by draining `commands` before the present**

Narrowing `busy` (F-A1, decided last session) makes typing during an
`Evaluate` the ordinary case — which is exactly the window in which a debounce
tick enqueues an edit that `serve` has not yet served, leaving the present at
`controller.rs:839` to write the draft's stale value back over the widget.
**Repairing F-A1 without this one trades an exchange-long deafness for a
per-poll revert**, so the two land together.

Four shapes were put. **Decided: drain `commands` before the present** — apply
every queued command that resolves without an exchange, stopping at the first
that needs one. The reason it is preferred to the other three is that it is
what the three loop-tier harnesses already do (`overlay.rs:238-249`), which is
*why* the rig cannot see the defect: production is being made to do what the
rig does. Rejected on their shape, not their cost — moving the present is the
same effect in a less obvious form; holding the entry until the edit is served
reverses PHASE-05/EX-4, which is the only reason `Wire::send` returns a `bool`;
and a `commands` arm in the inner `select!` reopens the re-entrancy the design
closed.

### The `busy` narrowing needs no second flag, and no new decision

Put to the user as a design question and withdrawn as one: `Command::Edit` is
not an exchange (`controller.rs:766-776` returns `None`), and `Command::Choose`
has exactly one origin (`install.rs:40`). So `Pending::Respond` holds **iff the
person clicked an option button** — there is no `Respond` they did not
initiate, and the option `Button`'s slice-003 double-submit guard wants exactly
the narrowed meaning. One flag, one line: `controller.engage(exchanged)`.
`F-R2`'s sites 5 and 6 are answered — 5 becomes unreachable (a click cannot
reach the button beneath an open dropdown, `window.rs:843-871`), and 6 is F-R1
rather than a `busy` problem.

### Every remaining finding, dispositioned together

**fix-now:** `F-A1`, `F-R2`, `F-R1`, `F-R3`, `F-S2`, `F-S1`, `F-S3`, `F-S4`,
`F-S5`, `F-R4`, `F-R5`, `F-P3`, `F-P4`, `F-R8`, `F-R9`, `F-S6`.
**doc-wrong:** `F-P1`, `F-P2`, `F-R6`, `F-R7`.
**settle, then dispose:** `F-S7` — one mutation decides whether I-F is
unobservable or merely untested.

`F-S3` is dispositioned against the audit's re-derivation rather than the
finding's headline: the stated mutation does **not** lint clean, and the hole
is one shape only — a wildcard absorbing an existing kind with a body that
agrees with it. A boundary scan closes it.

### The budget, revised again — **four sessions**

Round 1 nearly doubled and two of the new findings want real code. Session 2
takes the three repairs that gate — `F-A1`/`F-R2`, `F-R1`, `F-R3` — and
checkpoints. Session 3 takes the remaining repairs and round 2. Session 4 takes
reconciliation and close.

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

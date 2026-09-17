# Canon delta — Slice 009

Changes this slice owes to existing canon. Not canon itself: nothing outside
this slice may cite it. Promoted during audit and reconciliation with explicit
user endorsement, and recorded in `audit.md`'s Reconciliation table
(`docs/AGENTS.md` §*Canon that does not exist yet, or must change*).

The slice changes **no protocol**. Both entries below make the record true
about a host that now draws all five field kinds; neither widens what a backend
may send or must accept.

---

## CD-1 — `SPEC-001` §7: what an untouched field submits

**Document:** `docs/specs/001-host-backend-protocol.md`, §7, in the
neighbourhood of `R-58`.

**Change.** State what this host submits for a field nobody touched, per kind:
`boolean` → `false`; `text` → `""`; `number` → its declared minimum, or `0`
where none was declared; `choice` → its first alternative's id; `datetime` →
the Unix epoch, `1970-01-01T00:00:00+00:00`.

State one consequence of the `number` rule alongside it: a range carrying only a
`max` is legal (`R-17`), so a field declared `max: -10` and no `min` submits
`0` — **outside the bound its own backend declared**. That breaches nothing:
`R-35` puts the judgement of whether an answer is acceptable in the backend, and
`R-58` requires a value for every drawn field. But a backend author cannot
discover it from either rule, and will otherwise read it off the wire as a host
defect. `R-58`'s existing instruction — do not send the field when *unanswered*
must be distinguishable — remains the way to avoid it.

**Why.** `R-58` forbids omitting a value for a drawn field, and
`SPEC-001/OQ-2` (`field.value`) is unlanded, so the host must supply one. Four
of the five have an obvious neutral; `datetime` has none, and the host submits
a value nobody would pick rather than one that looks like an answer
(`design-log.md` D-6). A backend author reading `R-58` today cannot discover
any of this, and the `datetime` case in particular will be read off the wire as
a bug unless the spec says otherwise.

**Open — settle before promoting.** Whether the epoch is stated **normatively**
(a backend MAY rely on it to mean *untouched*) or **descriptively** (this is
what this host sends; it is not a sentinel, and `R-58`'s existing instruction —
do not send the field — remains the way to distinguish unanswered). The
descriptive form keeps this slice's *no protocol change* non-goal intact and
leaves `OQ-2` as the answer to the question the sentinel would half-answer. The
normative form is more useful to a backend author and harder to withdraw.

---

## CD-2 — `SPEC-001` §Verification: what drawing all five kinds makes false

**Document:** `docs/specs/001-host-backend-protocol.md`, §Verification — the
`R-57` row, the `R-58` row, the `R-55` row, and the `R-16` row, which gains
cases.

Three distinct changes, not one. The first was the whole of this entry when it
was written; the other two were found by the design review (F-10) and are the
ones that **remove** something.

**Change 1 — `R-57` stops being review-only.** The row currently reads, of the
`text`, `number`, `choice` and `datetime` clauses: *"**review, not a test**: no
renderer in this repository draws those kinds, so there is no code to assert
against and a test would have to construct the very mapping it checked."* After
this slice that is false for all four. Replace it with the cases that now
assert them, and name the new event-loop targets alongside the existing
renderer cases.

**Change 2 — the `R-58` row names two cases whose premise stops existing, and
one half of the rule stops being observable.** The row cites
`crates/goad/tests/renderer/fields.rs::a_view_carrying_an_undrawn_field_is_still_shown_and_still_answers_its_drawn_keys`
as where R-58 meets R-55, and
`crates/goad/tests/renderer/wiring.rs::an_answer_carries_no_value_for_another_option_or_for_an_undrawn_field`
(`wiring.rs:1340`) as the MUST NOT. Both need a view carrying an undrawn
**field**, and once all five kinds draw there is no way to build one: a
`group`-hint field is still drawn, and every surviving `Undrawn` variant is
body-level. The `wiring.rs` case says as much about itself — it opens with a
guard assertion that *"the fixture must actually carry an undrawn field for its
absence below to mean anything"*, which becomes unsatisfiable by construction.

R-58's MUST NOT prohibits two things: submitting a value for a field the host
did not draw, and submitting a field of an option it is not answering. After
this slice the **first is unobservable** — there is no substitute construction,
and the row must say so rather than quietly renaming a case. The second survives
unchanged and stays asserted by the same fixture, whose two options share the
field id `read`.

The row must therefore state three things: which case now carries the surviving
half of the MUST NOT; that the other half is held by the shape of the walk
(`Controller::answer` iterates the drawn fields, so a value for an undrawn field
has no path to `values`) rather than by a case; and that the R-55 meeting point
moves to the content forms.

**Change 3 — the `R-55` row's field clause becomes untrue.** It says
`Presentation::undrawn` is the general mechanism and that *"option fields, HTML
and URI content are admitted and undrawn the same way"*. After this slice no
option field is undrawn on account of its kind. The mechanism is unchanged and
still asserted by the content-form and markdown cases; the sentence naming
option fields is what has to go, and the row should say instead where the
sixth-kind path is held — `view_model.rs::undrawn_form`'s exhaustive match,
which is a compile-time guard rather than a case.

**Why.** A Verification row that cites a reason which has expired is worse than
one that cites nothing: it tells a future reader the clause cannot be tested. A
row that cites a *case that no longer exists* is worse again — it will be
discovered as a broken build by whoever next touches the suite, with no record
that it was foreseen. And a row that silently swaps one case for another hides
the thing most worth recording here: this slice **reduces** what the suite can
assert about a normative rule, permanently, and the reduction is a consequence
of drawing every kind rather than an oversight anyone can go back and repair.
All three are record-truth, not protocol changes.

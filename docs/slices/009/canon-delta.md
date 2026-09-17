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

## CD-2 — `SPEC-001` §Verification: four clauses stop being review-only

**Document:** `docs/specs/001-host-backend-protocol.md`, §Verification, the
`R-57` row (and the `R-16` / `R-58` rows, which gain cases).

**Change.** The `R-57` row currently reads, of the `text`, `number`, `choice`
and `datetime` clauses: *"**review, not a test**: no renderer in this
repository draws those kinds, so there is no code to assert against and a test
would have to construct the very mapping it checked."* After this slice that is
false for all four. Replace it with the cases that now assert them, and name
the new event-loop target alongside the existing renderer cases.

**Why.** A Verification row that cites a reason which has expired is worse than
one that cites nothing: it tells a future reader the clause cannot be tested.
This is record-truth, not a protocol change.

# Canon delta — Slice 001

Changes this slice makes to canon that **already exists**. New canon is drafted
in `draft-spec.md`, not here.

One entry per affected document: the document, the section, the change as it will
be stated, and why. Applied during audit and reconciliation with explicit user
endorsement, and recorded in `audit.md`'s Reconciliation table.

---

## CD-1 — ADR-001, "Verification"

**Document:** `docs/adr/001-one-way-strata.md`
**Section:** Verification
**Kind:** record accuracy. The **decision** is untouched.

### Why

ADR-001's Verification section says the one-way rule is checked "by inspection
during review, and by the placement discipline recorded in each slice's design",
and concludes that "until [the strata become separate crates], the honest answer
is that this is a review gate, not a build gate."

Slice 001 adds AC-15: a test asserting that no file under `src/semantics/` names
`crate::shell`, `crate::bin` or `tokio`. That makes part of the rule mechanical.
It does **not** make it a build gate — three tokens is the common case, not the
class, and the test cannot see a downward type leak, a re-export that flattens
the boundary, or `std::fs` appearing in stratum 1. So the ADR's conclusion is
still right and its description of the means is now incomplete.

Per `docs/AGENTS.md`, an ADR's decision is fixed while its record is kept
accurate as consequences are learned. This is that case, which is why it is a
delta rather than a supersession.

### Change as it will be stated

Replace the first paragraph of ADR-001's Verification section with:

> By inspection during review, by the placement discipline recorded in each
> slice's design, and — since slice 001 — by a test asserting that no file under
> `src/semantics/` names `crate::shell`, `crate::bin` or `tokio`.
>
> That test catches the common upward reference, not the class. It cannot see a
> downward type leak, a re-export that flattens the boundary, or I/O reaching
> stratum 1 through `std` rather than through a named crate. Treat a green run
> as the absence of the obvious violation, not as compliance.

Leave the second paragraph ("If the strata ever become separate crates …")
unchanged: it is still exactly true.

### Verified by

`docs/slices/001/design.md` §5.1 (what AC-15 checks and what it does not), §5.5
A2 (the same limit stated as an assumption), and R1 in §8.

---

## Considered and no delta owed

- **ADR-002.** Its Verification section requires the three split triggers to be
  checked and the answer recorded in the design of any slice that adds a
  dependency or a binary. Slice 001 does that in `design.md` §3 — T1, T2 and T3
  all negative, verdict one crate. The ADR asked for a record and got one; the
  document needs no change.
- **`docs/brief.md`.** Not canon by `docs/AGENTS.md`'s definition — it is intent.
  Three of its ambiguities were resolved by choice in this slice (`design.md`
  §5.5 A4); those are recorded as decisions, and if any should bind future
  slices it becomes a spec requirement or an ADR, not an edit to the brief.
- **Root `AGENTS.md`.** A deliverable of this slice (AC-10), not canon.

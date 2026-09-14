# Canon delta — Slice 007

**Status: draft.** Not endorsed, not applied. Design, plan and execution cite
the entry below exactly as they would the real thing; nothing outside this slice
may cite it. It is promoted during audit and reconciliation, with explicit user
endorsement, and the move is recorded in `audit.md`'s Reconciliation table. A
slice does not close holding an unpromoted draft: either it lands, or it is
abandoned with the reason written down (`docs/AGENTS.md`).

**Do not edit `docs/specs/001-host-backend-protocol.md` while the slice runs.**

This file exists because the slice **opened at tier 2**: drawing a field forces
the host to state what JSON type a submitted value has, and that is the wire
contract.

---

## CD-1 — SPEC-001: the JSON type of a submitted field value

**Document:** `docs/specs/001-host-backend-protocol.md`
**Sections:** §4 Requirements (one new row), §6.2 *Response messages* (the
`respond` example and a companion paragraph beside *Field forms*), §7
Verification (one row, fixtures per kind). Exact wording is design's, not
scoping's.
**Kind:** new requirement, appended as **R-57**. R-56 is that spec's current
highest id.

### The gap

§6.2 carries one submitted value — `"values": { "minutes": 20 }` — and no rule
mapping a field's `kind` to the JSON type its answer takes. R-9 declares
`response.values` opaque, but opacity is about the host never *reading* a
value. The host is nonetheless the only thing that can *write* one: it holds the
widget, and it alone turns a checkbox into JSON.

While no renderer drew a field, the gap cost nothing —
`crates/goad/src/controller.rs:216` sends an empty map and there was no value to
type. Drawing fields makes it a contract two backends could disagree about,
which is the definition of one.

### The change as it will be stated

A new requirement, in substance:

> **R-57.** A submitted field value's JSON type is determined by the field's
> `kind`, and by nothing else: `boolean` submits a JSON boolean, `text` a JSON
> string, `number` a JSON number, and `choice` the alternative's id as a JSON
> string. A host MUST submit a value for every field it drew, and MUST NOT
> submit one for a field it did not draw. A kind this table does not name has no
> defined submitted form; a renderer that cannot draw a kind reports it undrawn
> under R-55 and submits nothing for it.

with a table in §6.2 restating it beside *Field forms*, and the §6.2 `respond`
example extended to carry more than one value.

Three clauses and why each is load-bearing:

- **Type by kind, and nothing else.** Otherwise a backend must infer the type
  from the value it happens to receive, and `"true"` and `true` both arrive.
- **A value for every field drawn.** The alternative — omitting an untouched
  field — asks the host to distinguish *unanswered* from *false*, and both
  readings of an unticked box are domain meaning the host does not hold. It
  submits what it drew. The consequence is real and belongs in the spec's own
  words: a backend that needs *unanswered* distinct from *false* must not send
  the field, which is what SPEC-001/OQ-2 exists to fix properly.
- **Nothing for a field not drawn.** A host cannot invent a value for a control
  the person never saw. R-55 already requires the undrawn report; this says the
  answer stays silent about it rather than carrying a default.

### Why R-57 and not a clarification

Because it is falsifiable and a backend can be written against it. A rule that
only `design.md` states is a rule one host holds and the next does not.

### Deliberately out

`datetime`'s submitted form, if OQ-2 lands on leaving that kind undrawn. Naming
a wire form for a control nothing draws is a protocol decision no evidence is
asking for, and R-57's last sentence is written so its absence is a defined
state rather than a hole. If design draws `datetime` after all, this entry
grows a row and says what the string is.

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

## CD-1 — SPEC-001: the JSON type of a submitted field value, and the completeness of the map

**Document:** `docs/specs/001-host-backend-protocol.md`
**Sections:** §4 Requirements (two new rows), §6.1 *Request messages* (the
`respond` example), §6.2 *Response messages* (a type table beside *Field forms*),
§7 Verification (one row per new requirement, **and an amendment to R-18's
existing row** — see below), §8 *Open questions* (a new
**OQ-4**, whether a date without a time needs a kind of its own). Wording below is design's
(`design.md` §5.2, §10).
**Kind:** two new requirements, appended as **R-57** and **R-58**, and one new
open question, **OQ-4**. R-56 and OQ-3 are that spec's current highest ids;
SPEC-002's separate OQ-4 is a different document's numbering.

### The gap

§6.1 carries one submitted value — `"values": { "minutes": 20 }` — and no rule
mapping a field's `kind` to the JSON type its answer takes. R-9 declares
`response.values` opaque, but opacity is about the host never *reading* a
value; §7's own R-9 row says the requirement is that host code never *reads* a
payload. The host is nonetheless the only thing that can *write* one: it holds
the widget, and it alone turns a checkbox into JSON.

Research searched the document exhaustively and confirmed it: no rule, no
fixture and no verification row fixes the type of a submitted value for any
kind, and the only non-empty submission anywhere is §6.1's illustrative example
(`research.md` F2).

While no renderer drew a field, the gap cost nothing —
`crates/goad/src/controller.rs:216` sends an empty map and there was no value to
type. Drawing fields makes it a contract two backends could disagree about,
which is the definition of one.

### The change as it will be stated

> **R-57.** A submitted field value's JSON type is determined by the field's
> `kind` and by nothing else: `boolean` submits a JSON boolean, `text` a JSON
> string, `number` a JSON number, and `choice` the chosen alternative's id as a
> JSON string, and `datetime` an RFC 3339 `date-time` string carrying an
> offset.

> **R-58.** A `respond` carries values for exactly the fields the host drew of
> the option being answered: a host MUST submit a value for each of them, and
> MUST NOT submit a value for any other field — neither a field it did not
> draw, nor a field of an option it is not answering. A field a renderer cannot
> draw is reported undrawn under R-55, and the response is silent about it
> rather than carrying a default. A backend that needs *unanswered* to be
> distinguishable from *false* MUST NOT send the field — the additive mechanism
> for that is OQ-2.

with a table in §6.2 restating R-57 beside *Field forms*, and §6.1's `respond`
example extended to carry more than one value.

### Why two requirements and not one

Because they are two claims. *Type by kind* is a claim about a single value;
*a value for every field drawn of the answered option, and none for any other
field* is a claim about the shape of the map. §4's style is one falsifiable
claim per row, and R-52/R-53 are the precedent for splitting two rules that
travel together (`research.md` F2, F4; `design.md` D2).

R-58 is genuinely new rule rather than a restatement: research F4 established
that **no** requirement addresses a response that omits a field the view
declared. R-8 says a `respond` carries "a map of field id to submitted value"
and does not say the map is total over the drawn fields; R-35 forbids the host
*refusing* an incomplete answer but says nothing about how it *produces* one.

### Why R-57 covers kinds this renderer does not draw

007 draws `boolean` and reports the other four undrawn (`design.md` D4). R-57
nonetheless types all five, because what a `text` field
submits is a fact about the protocol and not about today's renderer. A table
that covered only the drawn kind would make the contract track the renderer —
which SPEC-001 §1 already names as the decay this document exists to prevent,
and which `CLAUDE.md`'s third invariant forbids outright.

The concrete case is in `design-log.md`: a trinary checklist item — yes / no /
ask later — is a `choice` **field**, conformant today, and it would have needed
a spec amendment to draw had R-57 been written to the drawn set.

### Why requirements and not a clarification

Because they are falsifiable and a backend can be written against them. A rule
that only `design.md` states is a rule one host holds and the next does not.

### Why `datetime` gets a form rather than a prohibition

Two earlier drafts of this section failed, in opposite directions, and the
record of both is why the current text is short.

The first left `datetime` with "no defined submitted form" beside R-58's MUST,
which required a host to submit a value whose shape nothing constrains — this
file's own definition of a contract two backends could disagree about (F-5).

The second closed that by forbidding it: no host may submit a `datetime` value,
and must report the field undrawn under R-55. That is worse, and in the way this
project cares about most (F-21). R-16 admits the kind, so a rule that no host may
answer it makes a conforming renderer that draws `datetime` impossible — the
protocol acquiring the shape of the one renderer being built in this slice, which
is what `CLAUDE.md`'s third invariant forbids and what R-55's second clause
("or produce the effect of") reaches. It also cited **R-55 for a report R-55
cannot license**: R-55's undrawn report is for a capability *the protocol admits
and a renderer does not implement*. Where the protocol itself forbids the answer,
a renderer not drawing it is complying, not exercising a subset, and the
mechanism was being borrowed to carry a prohibition it exists to prevent.

So R-57 defines the form. RFC 3339 `date-time` with a required offset settles
two of the three degrees of freedom named above — offset required, precision
unconstrained by the format — and excludes the third by the kind's own name: a
`date-time` carries both. The objection that no evidence asks for this format has
force, and it is outweighed: the alternative constrains harder, on equally no
evidence, by admitting nothing at all. Where a constraint must be chosen with no
demand to guide it, choose the one that admits more.

007 draws no `datetime` either way. The difference is that it now reports the
field undrawn as an ordinary **renderer** subset — which is exactly the case R-55
was written for, and leaves the distinction between "this renderer does not draw
it" and "the protocol forbids drawing it" intact.

> **OQ-4.** A date without a time. R-57 types `datetime` as an RFC 3339
> `date-time`, which carries both a date and a time with an offset. Whether a
> date-only field wants its own kind, or a hint on `datetime`, is open; no
> evidence asks for one yet, and deciding it is additive.

### R-18's existing §7 row states two facts this slice makes false

Found during plan review, not during design. SPEC-001 §7's R-18 row is *review,
not a test*, and its reasoning rests on two statements about this codebase:

> `hints` is read in `src/` only by `normalize.rs::normalize_field`, where the
> remaining keys are collected and passed through. … **The renderer, the one
> component that may, does not exist yet.**

After this slice, `view_model.rs` reads `hints` for the `group` key — which R-18
permits the renderer and only the renderer to do — and the renderer exists. The
row's *verdict* is unchanged and its requirement is unamended; what must change
is the two sentences of evidence beneath it, to name `view_model.rs` as the
second and only other reader and to drop the "does not exist yet" clause.

Stated here rather than left to audit's own reading, because a row whose
reasoning has quietly gone stale is the shape of decay SPEC-001 §1 exists to
prevent, and because R-18 is the requirement this slice's grouping rule is
licensed by.

### Deliberately out

**A clause on R-9's write direction.** Research F3 confirmed R-9 reaches reading
only, and that R-57 is what fills the gap. Folding a sentence into R-9 as well
was considered and dropped: one document, one change, and R-57 says it already.

# Slice 007: the renderer grows a form

**Stage:** design
**Tier:** 2 (full) — this slice states, for the first time, what JSON type a
submitted field value has, and how complete the map of them must be. That is the
wire contract, so it is canon, so it is tier 2. See `canon-delta.md` and *Why tier 2* below. Not the layout, and not the
300-line cap: both were candidates and neither is the reason.
**Depends on:** — (006 is independent in both directions)

## Purpose

An option may carry fields (SPEC-001/R-15) and `boolean` is a field kind
(R-16). This renderer draws neither. Every option's fields become
`Undrawn::OptionFields` (`crates/goad/src/view_model.rs:69`) and the answer goes
out with an empty `values` map (`crates/goad/src/controller.rs:216`).
SPEC-001/R-55 names that a renderer subset that must not narrow the protocol,
and slice 002 recorded it as a standing hazard. This slice discharges it.

Once it lands, a view with one option and fourteen boolean fields is answerable
in one exchange rather than fourteen — the shape a checklist wants, and the
shape the evidence asked for (`~/satan/goad/field-notes.md`, *The shape this
wants to be*, 2026-09-14; kept outside this repo because it is one person's
evidence and not canon).

Two things are true underneath that and are worth stating so nobody re-derives
them:

- **The view side needs no protocol change.** R-15 plus R-16 admit a form
  today. `Fields::new` accepts any count; `Options::new` requires one option.
- **The response side does need one.** §6.1 shows exactly one submitted value,
  `"minutes": 20`, and no rule maps a field kind to a JSON type. R-9's opacity
  is about the host not *reading* a value; the host is nonetheless the only
  thing that can *write* one, because it is the thing holding the widget.

## Why tier 2

`docs/AGENTS.md` §Tiers: tier 2 is required of a slice that writes or amends
canon, or changes the wire contract. This one does the second, and therefore
carries `canon-delta.md` from scoping.

The 300-line design cap is **not** the binding constraint and must not be used
as the argument. Tier 2 has no cap, so nothing needs splitting to fit one. What
is split out is the *look* — see Non-goals — and that is a scope decision, not
a line-count one.

## Scope

Surfaces this slice may touch:

- `crates/goad/ui/app.slint` — field rendering, per-option grouping, the submit
  path.
- `crates/goad/src/view_model.rs` — `Presentation` grows fields; `Undrawn`
  narrows from `OptionFields` to a per-field, per-kind variant.
- `crates/goad/src/controller.rs` — `edit()`, and `answer()` assembling `values`
  from the draft.
- `crates/goad/src/draft.rs` — **new**: the draft, its (option, field) keying,
  and `submitted()`, the single site where R-57 is applied.
- `crates/goad/src/reception.rs` — `Prepared` gains the draft.
- `crates/goad/src/wire.rs`, `crates/goad/src/install.rs` — `Command::Edit` and
  its callback.
- `crates/goad/src/glass.rs` — a second model beside `options`.
- `crates/goad/src/diagnostics.rs` — the wording of the narrowed undrawn report.
- `crates/goad/build.rs` — the style selection (OQ-5).
- `crates/goad/tests/renderer/` — the headless field tier.
- `examples/` — a backend that sends a form, so `just demo` can show one.
- `docs/slices/007/canon-delta.md` → SPEC-001 §6.1, §6.2 and two new
  requirement ids, R-57 and R-58.

Untouched, and each for a reason: `crates/goad-semantics/` already models
everything this slice needs — verify that, do not extend it; `crates/goad-shell/`
carries values it never reads; `crates/goad-emit/` and `crates/goad-boundary/`
are unrelated.

## Non-goals

- **The look — slice 008.** "It looks like ass" is the loudest field note, and
  the spike verdict was right: the ugliness is our layout, not the widget style.
  007 does only what drawing fields *forces* — a container per group, and a
  style whose checkbox is worth looking at (OQ-5). Typography, spacing, window
  sizing, the idle surface: 008's. Both at once yields a design nobody can
  review, and a diff in which a layout regression and a protocol regression look
  alike.
- **`field.value` / prefill, and per-field errors (SPEC-001/OQ-2).** Additive
  protocol fields needing a version bump or a capability declaration. Their own
  tier 2 slice, taken when use says a form must *reject* an answer. This slice
  makes that trigger sharper — see *What the workaround costs*.
- **SPEC-002/OQ-4** — a scheduled firing superseding a view mid-answer.
  **Answered during design and withdrawn on scope** (`design.md` D13): five lines
  of code, but a second spec amendment plus an ADR inside a slice already
  amending SPEC-001 twice, and a scheduling regression would look like a protocol
  regression in one diff. So **a scheduled firing can still discard a half-filled
  form**, and an ingested event likewise. The mechanism, the ADR-004 interaction
  and the ingress boundary are preserved in `design-log.md` so the follow-up
  re-derives nothing.
- **006's items** — `--version`, the unnamed config path, crane and
  `wrapProgram`.
- **Socket transport (009).** Worth recording that a form makes it *less*
  urgent: fourteen spawns per slot collapse to one.
- **The rest of the field notes**, each excluded as a decision rather than an
  oversight:
  - *"Waiting and dead look the same"* — the idle surface, → 008. Cheap there,
    incoherent here.
  - *`Check now` shows nothing and reads as broken* — the notes say the
    acknowledgement is arguably the backend's to word. It is: a backend with
    nothing to ask can return a view saying so. No host change; a line in 010's
    guide.
  - *The ingress lock names no pid* — real, and it is ingress, not the renderer.
  - *`Restart=always`, the two captured env vars* — deployment, → 006.
  - *`path:` flake ref versus the demo socket* — already in `docs/memory/`.
  - *`Later` and `Enough` mostly dissolve* — the backend's business. The host
    must keep drawing N options each with their own fields; a renderer that
    assumed one option would narrow the protocol, which is the failure this
    project exists to avoid.
  - *The standing questions* (are the 2h slots useful, is the day file read
    back) — not host questions.

## What the workaround costs

Recorded here because it was checked at scoping and the answer was not the
expected one.

SPEC-001 has no `field.value`, so a form cannot be pre-populated, and the way
around it is a backend that sends only the items it still wants answered —
`pending()` already does exactly that. That holds. What it costs is this: the
host submits a value for **every drawn field**, including a checkbox the person
left unticked. After one submission every item is answered, `pending()` is
empty, and the day's checklist closes early — unless the backend reads `false`
as *not yet* rather than *no*.

That reading is domain meaning, so it is the backend's to hold. The host must
not invent a tri-state, and must not omit untouched fields to manufacture one:
both readings of an unticked box are domain meaning, and the host cannot choose
between them. It submits what was drawn.

## Acceptance criteria

- [ ] **AC-1** A view with one option carrying N `boolean` fields draws N
      checkboxes and one button. Pressing it sends one `respond` whose `values`
      carries exactly N keys, each a JSON boolean matching what was on screen.
- [ ] **AC-2** A heading is drawn wherever the `group` value changes, and **no
      field is ever reordered**: screen order is declared order, always. A
      repeated group name draws its heading twice rather than collecting the
      fields under one; a field with no `group`, or one whose `group` is not a
      JSON string, is drawn ungrouped **in place**, the latter also reported. No
      sorting and no merging — either would be the host imposing meaning
      (`design.md` D8, D9; amended from the merge rule this card first carried).
- [ ] **AC-3** A field of a kind this renderer does not draw is reported
      undrawn — the **`Undrawn` value** naming the option, the field and the
      kind, which is what a test asserts; the rendered line's wording is held by
      review, as every diagnostic wording in this project is. The view is still
      shown and the option is still answerable (R-55). The values it does have
      are submitted; nothing is refused on the host's judgement (R-35).
- [ ] **AC-4** Values from one option's fields are never sent under another
      option's id. One option, one flat map (R-8).
- [ ] **AC-5** A `present` that does not change the outstanding view leaves a
      half-filled form intact. **This is the criterion that catches the real
      regression**: `serve` calls `glass.present` at the top of every loop
      iteration (`controller.rs:611`) and `Glass::present` is contracted to
      write every property, every time. **Both halves are asserted, and the
      on-screen one is load-bearing here** — the wire value is built from the
      draft, which no present writes, so a present that stops writing `checked`
      leaves the wire green and the screen wrong (`design.md` §9).
- [ ] **AC-6** Answering an option carrying **zero** fields is unchanged: no
      empty container, no stray control, and the existing option tests pass with
      **no change to what they assert**. Their diff is not empty and cannot be —
      `OptionRow` gains a member and `tree.rs` builds it with an exhaustive
      literal — so the criterion is on the assertions, not on the diff.
- [ ] **AC-7** A person runs the real backend under `just demo`, fills a
      multi-field form, submits once, and the record shows every answer from
      that one exchange. Recorded in `audit.md` under Evidence — `docs/AGENTS.md`
      §Tiers, and a green gate is not this evidence.
- [ ] **AC-8** `canon-delta.md` is promoted into SPEC-001 as **R-57 and R-58**,
      each with a §7 verification row, or abandoned in writing. The vehicles are
      stratum 3, where the kind still exists: `draft.rs::submitted` for R-57's
      `boolean` clause, `answer()` over a two-option view for R-58. R-57's
      `text`, `number` and `choice` clauses are **review, not a test** until a
      renderer draws them. A slice does not close holding an unpromoted draft.
- [ ] **AC-9** Standing: the domain-vocabulary scan and the four ADR-001
      instruments pass; `just check` exits 0. `group` is a hint key on the wire,
      never a host concept — and it is not on the scanned word list
      (`crates/goad-boundary/tests/checks/vocabulary.rs:18-26`), so the scan
      passing is necessary and not sufficient. Review holds the rest.
- [ ] **AC-10** A person runs the slice's output and reviews **how it looks**,
      iterating with the implementer until the form is legible: which fields
      belong to which option, and which heading covers which fields, are both
      apparent without reading the protocol. The bound is on what may be
      *changed*, never on what may be *said*, and it is **what drawing fields
      forces**: the block container and its separator, and the heading's own
      treatment. Typography, window sizing, the idle surface and the look of the
      controls are 008's — feedback about them is recorded **verbatim and not
      actioned**, and becomes 008's brief. Recorded in `audit.md` under Evidence
      beside AC-7.

## Governing canon

- **SPEC-001 (host/backend protocol)** — R-8 (one option, one flat map of field
  values), R-9 (submitted values are opaque to the host), R-15 (an option may
  carry fields), R-16 (the five kinds), R-18 (hints are flat; only the renderer
  may branch on one), R-35 (the host does not validate an answer beyond its
  `view_id`), R-50 (a named key where its position gives it no meaning is an
  error), R-52 (field ids unique within an option), R-53 (an alternative id is
  not an option id), R-55 (a renderer subset is not a narrowing of the
  protocol), §6.1 (the `respond` example), §6.2 *Field forms*, OQ-2 —
  sharpened, not amended: without prefill a fourteen-item form costs fourteen
  re-ticks to correct.
- **SPEC-002 (host scheduling)** — OQ-4, **sharpened and left open**. This slice
  answered it and withdrew the answer (§Non-goals, `design.md` D13).
- **ADR-001 (one-way strata)** — `view_model.rs` is the pure half of stratum 3
  and stays pure: the draft decision must put neither a clock nor a widget
  handle in it.
- **ADR-003 (workspace of strata)** — nothing new crosses a crate edge.
- **POL-001 (the phase gate)** — `just check`, and what each instrument does and
  does not reach.
- Checked and not applicable: SPEC-003 (event ingress — this slice adds no
  stimulus), ADR-005.
- **ADR-004 (scheduled firings are spaced from the previous scheduled firing)** —
  not applicable to the diff, and **not filed under "not applicable"**: the
  deferred hold interacts with its spacing rule, and a held firing must not write
  the floor because it is not a firing. `notes.md` Harvest carries the connection
  so the follow-up finds it (`research.md` delta 11).

## Open questions

All six are closed by a recorded user decision (`design-log.md`, 2026-09-14).
The reasoning behind each lives in `design.md` §6 and §7; it is not repeated
here.

| id | question | resolution |
|----|----------|------------|
| OQ-1 | where does a half-filled answer live? | **the controller**, inside `Prepared` (`design.md` D5) |
| OQ-2 | which field kinds does 007 draw? | **`boolean` only** — research inverted this card's four-kind recommendation (`design.md` D4) |
| OQ-3 | a firing lands while the form is half filled | **deferred** — answered, then withdrawn on scope (`design.md` D13). See §Non-goals |
| OQ-4 | what does a non-string `group` mean? | **ungrouped and reported**; `""` ungrouped and silent; grouping is by **runs**, which amended AC-2 (`design.md` D8, D9) |
| OQ-5 | is the style selection part of this slice? | **yes**, as a default — `material`, with `SLINT_STYLE` still overriding (`design.md` D12) |
| OQ-6 | several options each carrying fields | **a block per option**, each with its own button (`design.md` D7) |

Two things design added that this card did not anticipate: the `Edited` value
seam, landed now so that R-57 has a single enforcement site and the host cannot
grow a drawn kind without deciding what it submits (`design.md` D11 — it does not
guard the *protocol* growing a kind; that break lands in `present()`'s mapper
arm); and **AC-10**, because AC-7 asks a person to observe the
*behaviour* and nobody had been asked to judge the *appearance*.

## Summary

<!-- Written at close. -->

## Follow-ups

<!-- Written at close. Expected already: what 008 inherits; whether socket
     transport (009) is promoted or deferred, now that a form collapses fourteen
     spawns per slot into one; SPEC-001/OQ-2 with the cost of its absence
     written down. -->

- **Keyboard focus does not survive a present.** Every present resets the field
  model, which destroys and re-creates every element under `options` — the same
  mechanism that re-establishes `FieldRow.checked` from the draft (design §5.4,
  A-2). The window holds its focused item weakly, so focus is dropped once per
  tick: ticking the Nth box from the keyboard costs N tabs. No acceptance
  criterion is unmeetable and no requirement is breached — SPEC-001 addresses
  focus nowhere — and the form is fully usable by mouse.

  **Not repairable by writing rows in place** (`set_row_data`): that keeps focus
  and detaches the checkbox from the draft, which is the defect A-2 exists to
  exclude. The work is a focus identity that survives a rebuild — the row reports
  focus back, the host retains the focused field id, the row's `init` restores it
  — which is a design surface of its own and the reason this is owned here rather
  than absorbed. Raised as F-2 in `review-design.md`.

- **D3's rationale has a home outside this slice.** The decision that `datetime`
  is deliberately unspecified shaped R-57's text, and its reasoning lived only in
  `canon-delta.md`, which is consumed at promotion — so after reconciliation it
  would have existed nowhere. Recorded in `docs/roadmap.md` §Open decisions
  beside OQ-1 and OQ-2, with the trigger (the slice that first draws a `datetime`
  field) and the note that reversal reads as tidying in either direction.
  SPEC-001/OQ-4 carries the degrees of freedom themselves. **No ADR** — user
  decision, 2026-09-15. Raised as F-14 in `review-design.md`.

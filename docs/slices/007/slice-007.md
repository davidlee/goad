# Slice 007: the renderer grows a form

**Stage:** scoping
**Tier:** 2 (full) — this slice states, for the first time, what JSON type a
submitted field value has. That is the wire contract, so it is canon, so it is
tier 2. See `canon-delta.md` and *Why tier 2* below. Not the layout, and not the
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
- **The response side does need one.** §6.2 shows exactly one submitted value,
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
- `crates/goad/src/controller.rs` — the draft, and `answer()` assembling
  `values` from it.
- `crates/goad/src/glass.rs` — a second model beside `options`.
- `crates/goad/src/diagnostics.rs` — the wording of the narrowed undrawn report.
- `crates/goad/build.rs` — the style selection (OQ-5).
- `crates/goad/tests/renderer/` — the headless field tier.
- `examples/` — a backend that sends a form, so `just demo` can show one.
- `docs/slices/007/canon-delta.md` → SPEC-001 §6.2 and one new requirement id.

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
- **SPEC-002/OQ-4** — a scheduled firing superseding a view mid-answer. See
  OQ-3.
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
- [ ] **AC-2** Fields carrying a `group` hint are drawn under a heading of that
      name. Groups appear in order of first appearance; fields keep their
      declared order within a group; a field with no `group` is drawn ungrouped,
      in place. No sorting — sorting would be the host imposing meaning.
- [ ] **AC-3** A field of a kind this renderer does not draw is reported
      undrawn, naming the option, the field and the kind; the view is still
      shown and the option is still answerable (R-55). The values it does have
      are submitted; nothing is refused on the host's judgement (R-35).
- [ ] **AC-4** Values from one option's fields are never sent under another
      option's id. One option, one flat map (R-8).
- [ ] **AC-5** A `present` that does not change the outstanding view leaves a
      half-filled form intact. **This is the criterion that catches the real
      regression**: `serve` calls `glass.present` at the top of every loop
      iteration (`controller.rs:611`) and `Glass::present` is contracted to
      write every property, every time.
- [ ] **AC-6** Answering an option carrying **zero** fields is unchanged: no
      empty container, no stray control, and the existing option tests pass
      unmodified.
- [ ] **AC-7** A person runs the real backend under `just demo`, fills a
      multi-field form, submits once, and the record shows every answer from
      that one exchange. Recorded in `audit.md` under Evidence — `docs/AGENTS.md`
      §Tiers, and a green gate is not this evidence.
- [ ] **AC-8** `canon-delta.md` is promoted into SPEC-001 with a requirement id
      and a wire fixture per drawn kind, or abandoned in writing. A slice does
      not close holding an unpromoted draft.
- [ ] **AC-9** Standing: the domain-vocabulary scan and the four ADR-001
      instruments pass; `just check` exits 0. `group` is a hint key on the wire,
      never a host concept — and it is not on the scanned word list
      (`crates/goad-boundary/tests/checks/vocabulary.rs:18-26`), so the scan
      passing is necessary and not sufficient. Review holds the rest.

## Governing canon

- **SPEC-001 (host/backend protocol)** — R-8 (one option, one flat map of field
  values), R-9 (submitted values are opaque to the host), R-15 (an option may
  carry fields), R-16 (the five kinds), R-18 (hints are flat; only the renderer
  may branch on one), R-35 (the host does not validate an answer beyond its
  `view_id`), R-50 (a named key where its position gives it no meaning is an
  error), R-52 (field ids unique within an option), R-53 (an alternative id is
  not an option id), R-55 (a renderer subset is not a narrowing of the
  protocol), §6.2 *Field forms*, OQ-2.
- **SPEC-002 (host scheduling)** — OQ-4, sharpened by this slice. See OQ-3.
- **ADR-001 (one-way strata)** — `view_model.rs` is the pure half of stratum 3
  and stays pure: the draft decision must put neither a clock nor a widget
  handle in it.
- **ADR-003 (workspace of strata)** — nothing new crosses a crate edge.
- **POL-001 (the phase gate)** — `just check`, and what each instrument does and
  does not reach.
- Checked and not applicable: SPEC-003 (event ingress — this slice adds no
  stimulus), ADR-004, ADR-005.

## Open questions

- ~~**OQ-1 — where does a half-filled answer live?**~~ **Answered at scoping:
  (b), the controller owns the draft** (`design-log.md`, 2026-09-14). The
  question and its alternatives are kept below because design must honour what
  the choice costs, and because AC-5 is what proves it.

  The slice's real design question; everything else is downstream of it.

  `serve` presents at the top of every iteration and `Glass::present` is
  contracted as *"write every property, total and idempotent"* — the design's
  deliberate answer to a display server that fails partway through an update. A
  field holds user state between presents. The two collide.

  - **(a) Slint owns the draft.** `in-out` on a row struct; the glass writes the
    field model only when the `ViewId` changes. Cheapest. Costs the totality
    rule an exception, which must then be stated and tested rather than
    discovered.
  - **(b) The controller owns the draft.** An edit becomes a `Command`;
    `Controller` retains the draft *inside* `Prepared`, so a draft and the view
    it answers are one value and `shown = None` cannot leave one behind.
    `present` writes the field model from the draft every time, so totality
    survives. `answer()` assembles `values` from retained state.
  - **(c) The click carries the values.** `chosen(view, option, [FieldValue])`;
    no draft state anywhere. Fewest moving parts; the values arrive unverified.

  *Recommendation: (b).* `Controller` is already the named home of renderer
  state — "no Slint types, so it is testable without a platform", and "that is
  the complete retained state" (`controller.rs:100-107`). It keeps `present`
  total, keeps the draft in a pure reducer the headless tier can test, and lands
  the verification where `answer()`'s already is: it refuses an option the
  retained presentation does not carry, and filling `values` from retained state
  means every submitted key came from a field that view actually declared.
  Option (c) has to add that check back by hand.

  The cost is one channel message per widget edit — nothing for fourteen
  checkboxes. If text fields make it ugly, that is the argument for (c), and it
  should be made with a measurement.

  AC-5 is the criterion that distinguishes all three.

- **OQ-2 — which field kinds does 007 draw?** The pinned compiler
  (`i-slint-compiler-1.17.1/widgets/`) has a widget for four of the five;
  `datetime` has only `DatePickerPopup` and `TimePickerPopup`, so an inline
  control is a composite we write — *and* it is the one row of the canon-delta
  table that nothing in the spec implies.

  *Recommendation: draw `boolean`, `text`, `number` and `choice`; leave
  `datetime` undrawn and reported (AC-3), and leave its wire form out of the
  delta rather than guess at it.* That is a renderer subset R-55 explicitly
  permits, and it keeps the delta to four rows every one of which is already
  strongly implied. The alternative — draw all five — is defensible and makes
  the undrawn variant unreachable by construction, which is tidier, but it buys
  a composite widget and forces a protocol decision no evidence is asking for.

- **OQ-3 — a scheduled firing lands while the form is half filled.**
  SPEC-002/OQ-4, and this slice makes it expensive: today it costs an unclicked
  button; afterwards it costs ten minutes of ticking. SPEC-002 warns that
  suppression asks the host to judge a view worth protecting, which is domain
  meaning it does not hold.

  A counter-argument belongs on the record: *"a person is part-way through
  answering"* is interaction state, which the brief gives the host outright, and
  is not domain meaning. A dirty-draft suppression rule is defensible in a way
  "this view looks important" is not.

  *Recommendation: 007 does not answer it.* It is a SPEC-002 amendment plus an
  ADR, and the slots that produced the evidence are two hours apart, so the
  window is small. Record the sharpened case in `notes.md` Harvest. Do the cheap
  half: if a supersession discards a **non-empty** draft, say so on the
  diagnostic surface rather than losing it silently.

- **OQ-4 — what does a non-string `group` value mean?** `Hints` carries
  `serde_json::Value` (`canonical.rs:127`). R-18 lets the renderer branch on the
  key; it does not say what `{"group": 7}` means. A hint is presentation and
  must never fail the message — that would be a narrowing — so the choice is
  between treating it as ungrouped and coercing it to a display string.
  *Recommendation: ungrouped, and reported undrawn*; coercion invents a heading
  the backend did not author. Also to settle: two groups sharing a name (merge,
  at the first one's position), and a group name that is the empty string.

- **OQ-5 — is the style selection part of this slice?** A checkbox in the
  current style is a large part of what the notes complain about;
  `slint_build::CompilerConfiguration::with_style` is one line beside the
  `with_debug_info(true)` `build.rs` already sets. *Recommendation: yes — one
  line, and nothing else visual.* It needs a recorded decision because it
  changes the appearance of every existing widget and because the renderer tests
  run headless and will not notice.

- **OQ-6 — several options each carrying fields.** The backend in hand sends
  one, but the renderer may not assume it. Draw each option's fields with its
  own button, submitting only that option's values (AC-4)? Or reveal the
  selected option's fields on demand? *Recommendation: the former* — it is the
  literal reading of R-8, it needs no selection state, and it degrades to the
  single-option case the evidence actually wants.

## Summary

<!-- Written at close. -->

## Follow-ups

<!-- Written at close. Expected already: what 008 inherits; whether socket
     transport (009) is promoted or deferred, now that a form collapses fourteen
     spawns per slot into one; SPEC-001/OQ-2 with the cost of its absence
     written down. -->

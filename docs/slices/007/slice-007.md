# Slice 007: the renderer grows a form

**Stage:** done
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
- `crates/goad/src/controller.rs` — `edit()`, `answer()` assembling `values`
  from the draft, and `Frame` gaining `notice`.
- `crates/goad/src/draft.rs` — **new**: the draft, its (option, field) keying,
  and `submitted()`, the single site where R-57 is applied.
- `crates/goad/src/reception.rs` — `Prepared` gains the draft.
- `crates/goad/src/wire.rs`, `crates/goad/src/install.rs` — `Command::Edit` and
  its callback; and `Notice`, the back-pressure signal `Wire::send` raises
  (`design.md` §5.3).
- `crates/goad/src/main.rs` — constructs the `Notice` signal beside `Cancel`,
  clones it into `Wire` and passes it to `serve`. Not anticipated when this card
  was written: the notice repair arrived during design review (`design.md` §5.3,
  §5.4).
- `crates/goad/src/lib.rs` — declares the new `draft` module.
- `crates/goad/src/glass.rs` — a second model beside `options`, and `notice`
  written from the frame.
- `crates/goad/src/diagnostics.rs` — the wording of the narrowed undrawn report.
- `crates/goad/build.rs` — the style selection (OQ-5).
- `crates/goad/tests/renderer/` — the headless field tier.
- `crates/goad/tests/event_loop/`, `crates/goad/tests/event_loop_schedule/` —
  **call shape only.** `serve` and `Wire::new` change signature, and these two
  targets call both. No assertion and no fixture in them changes.
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
      that one exchange. **Its form carries at least one field of a kind this
      renderer does not draw**, so what a backend author copies is a
      protocol-shaped form rather than this renderer's subset — R-55's "or
      produce the effect of" clause, which no other artefact in this slice
      discharges (design §9/AC-7). Recorded in `audit.md` under Evidence —
      `docs/AGENTS.md` §Tiers, and a green gate is not this evidence.
- [ ] **AC-8** `canon-delta.md` is promoted into SPEC-001 as **R-57 and R-58**,
      each with a §7 verification row, or abandoned in writing. The vehicles are
      stratum 3, where the kind still exists: `draft.rs::submitted` for R-57's
      `boolean` clause, `answer()` over a two-option view for R-58. R-57's
      `text`, `number`, `choice` and `datetime` clauses are **review, not a
      test** until a renderer draws them. A slice does not close holding an unpromoted draft.
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
| OQ-4 | what does a non-string `group` mean? | **ungrouped and reported**; `""` is **its own untitled block** and silent — a string was sent and there is nothing in it to draw, which is not the same as no string at all (`design.md` §5.5's edge table, and `mapper::an_absent_group_and_an_empty_one_are_blocks_with_no_heading`); grouping is by **runs**, which amended AC-2 (`design.md` D8, D9) |
| OQ-5 | is the style selection part of this slice? | **yes**, as a default — `material`, with `SLINT_STYLE` still overriding (`design.md` D12) |
| OQ-6 | several options each carrying fields | **a block per option**, each with its own button (`design.md` D7) |

Two things design added that this card did not anticipate: the `Edited` value
seam, landed now so that R-57 has a single enforcement site and the host cannot
grow a drawn kind without deciding what it submits (`design.md` D11 — it does not
guard the *protocol* growing a kind; that break lands in `present()`'s mapper
arm); and **AC-10**, because AC-7 asks a person to observe the
*behaviour* and nobody had been asked to judge the *appearance*.

## Summary

**The renderer draws a form, and the protocol says what one answers with.**

A view whose option carries N `boolean` fields now draws N checkboxes under one
button, grouped into blocks wherever the `group` hint changes, and pressing the
button sends **one** `respond` carrying exactly N keys. The fourteen-exchange
checklist is a one-exchange checklist. A person ran it: five fields, three
ticked, one exchange, five correct keys, and no key for the `text` field this
renderer does not draw (Evidence, `audit.md`).

The half that made this tier 2 is the response, and it landed: **SPEC-001 R-57**
fixes a submitted value's JSON type by the field's `kind` — all five kinds, not
the one this renderer draws — and **R-58** fixes the map's shape: a value for
every field the host drew of the option answered, and none for any other. Both
carry a §7 verification row; §6.1's example grew past one value, §6.2 gained a
type table, and **OQ-4** opens the question R-57 leaves (a date without a time).
R-18's §7 row was amended where this slice falsified its evidence — `hints` now
has a second reader, `view_model.rs`, and it is the renderer, which is the one
component R-18 permits.

Three structural decisions carried the weight, and each is worth more than the
feature:

- **`answer()` walks the presentation's drawn fields and looks each up in the
  draft — never the draft.** `Draft` exposes no way to enumerate what it holds.
  So R-58's three properties are held by the shape of the walk rather than by a
  check someone must remember, and a draft key that outlived its view is not
  expressible on the wire rather than being filtered off it.
- **`draft::submitted` is the single site a widget's state becomes JSON**, a
  total match. The host cannot grow a drawn kind without deciding what it
  submits.
- **Grouping is by runs, and nothing is sorted, merged or moved.** The `Drawn`
  type makes *a run is over the drawn fields* a property of the types: an
  undrawn field never becomes one, so it cannot break a run or open a block.

**What the audit changed.** The gate was green at every phase boundary and was
not enough. A full-strength review found that the whole of the heading markup
could be deleted with all 206 tests passing — AC-2's chain was declared to have
three links and its third held the fields and not the heading they sit under —
and that `enabled: !root.busy` was held by nothing on either control, because
every enabled-state assertion in the workspace was in the one direction an
*unbound* property also answers. Both are now held at the screen, verified by
making the defect and watching the test fail. **Twenty-four findings across five
rounds, sixteen of them against the audit's own repairs**; all dispositioned,
none downgraded, no blocker raised. Round 2 alone found five defects in five of
round 1's repairs, two of them false statements in the ledger's own record — the
argument for unbounded rounds, demonstrated rather than asserted. Rounds 4 and 5
found no code defect at all, which is what let the ledger close.
`review-code.md` has them, and its Synthesis has the shape.

**And the gate itself was not deterministic.** Two pairs of cases shared one
invocation-log path — one pid, one file, cleared at handout — failing one run in
six. `just check` is the gate, and a gate green five times in six is not one. It
is now held by an instrument rather than by a naming convention: `claim` found
the second pair on its first run, then a third and fourth helper making the same
unheld promise, then a fifth and sixth. It holds four of the six; the two in
`goad-emit` are a follow-up with a measured obstacle. The durable repair is not
the count but the doc — it now carries the *class* and the grep that enumerates
it, because four rounds each wrote down a list and three of those lists were
short within the round that wrote them. Six consecutive runs green on both
claim-bearing targets.

`just check`: **exit 0, 537 tests across 21 binaries** (535 at the last phase;
the two additions are the repairs' own cases). What green does **not** reach is
unchanged and stated in `audit.md`: no ADR-001 instrument reaches stratum 3's
purity, and `group` is not on the vocabulary scan's word list. Both are held by
reading and by review.

## Follow-ups

> **Whether these are still open is `docs/follow-ups.md`'s**, not this
> section's. What is below is what slice 007 *raised* — the reasoning, and the
> price it was deferred against — and it stands as written, because it is a
> claim about what was decided then. The ledger carries the part that goes
> stale. This slice's rows: FU-4, FU-5, FU-10, FU-16, FU-31, FU-39.
>
> Swept 2026-09-23 at `3ecaa11`. Anything below that the ledger does not list is
> struck in its §Closed table, with what killed it.

Each of these becomes a future slice or a named piece of one. They are not
parked here instead of being decided — each has a decision behind it, recorded
in `audit.md`'s Reconciliation or in `review-code.md`.

- **008 inherits the look, and one defect sharper than it was given.** The
  window has **no content-derived preferred size at all** — measured at audit as
  **50×65 regardless of content**, identical for two plain options, two options
  with two fields each, and one option with five. At that size one of two option
  buttons is reachable. The mechanism is identified: the content sits inside a
  `ScrollView` (`app.slint:49`), which does not propagate its content's
  preferred size, and `PromptWindow` declares no width, height or min-size. So
  the repair is *nothing propagates a preferred size to the window*, not *make
  it taller*. It is invisible under any compositor that sizes windows itself,
  which is why two human acceptance criteria named as its observers could not
  have seen it. `docs/memory/a-fixtures-size-is-not-the-products.md`.

- **008's brief, recorded verbatim and not actioned** (AC-10's bound was on what
  could be *changed*, never on what could be *said*). The diagnostic surface
  wraps no line and its text cannot be selected; the layouts above the block
  container distribute the window's full height; the title and body are clipped
  at the top; and a captured stderr line's trailing newline renders as a visible
  `\n`. Full wording in `notes.md`'s PHASE-06 sheet.

- **`Diagnostics::of` builds unbounded lists, and this slice made one of them
  linear in the backend's choice.** The undrawn report went from one line per
  option to up to 2N lines for an option of N undrawn fields, and R-15 places no
  bound on N — so a *conforming* backend reaches it without misbehaving. The
  class, not the instance: `discarded` is unbounded the same way and is
  pre-existing. It wants one decision — what bound, and what an *"and N more"*
  line means when the **list** rather than the line overflowed — taken beside
  the three bounds the module already states, not a fourth added in passing.
  `review-code.md` F-4.

- **Every backend-authored string that reaches the screen is unbounded, and this
  slice added the third.** `title`, `option.label` and now `block.heading` — the
  `group` hint's value — bind straight to a `Text` with no bound, while **every**
  diagnostic line in the same binary passes through `finish(.., LINE_LIMIT)` at
  1024 bytes (`diagnostics.rs`, six call sites). Not an injection risk: Slint's
  `Text` renders a string as a string and interprets no markup. The exposure is
  layout, and a backend need not misbehave to reach it. The class is the work —
  *does a backend-authored string reaching the screen get the treatment one
  reaching the diagnostics pane gets?* — and bounding the new member alone would
  be fixing the instance. It sits with 008 because a heading bound is a decision
  about the look. `review-code.md` F-16.

- **`claim` reaches four of the six helpers that promise a unique temp path,
  and both stragglers are in `goad-emit`.** `tests/binary/exchange.rs`'s
  `socket_path` (six call sites) and `src/main.rs`'s `config_home` (five) —
  the second inside a `#[cfg(test)]` module in a production file, and
  destructive in `clear`'s exact way: it writes or removes `config.toml` under
  the directory as it hands it out, so two cases sharing a name have one
  clobber the other's fixture mid-run. Measured at one failure in six under a
  deliberate collision, with no panic and no name in the message — the `"vt8"`
  shape, in the one binary nothing had named.

  The obstacle is measured rather than assumed: neither target
  `#[path]`-includes `tests/support/scripting.rs`, and adding the include
  yields nine `dead_code` warnings — errors under the gate's `-D warnings`,
  which is the reason that file exists apart from `driving.rs` in the first
  place. `src/main.rs` also wants a `#[cfg(test)]` module to hang them on. The
  work is to move `claim`, `CLAIMED` and `spelled_at_the_call_site` into a
  support file of their own that a target can include without the
  scripted-backend helpers: a new file, a `#[path]` line per including target,
  and a decision about `spelled_at_the_call_site`, which strips
  `logging_backend`'s `invocations-` prefix and so couples the new file back to
  the one it left if extracted naively. Small, but a design question rather
  than a line, and taken at audit closure it is how a fifth review round
  becomes a sixth.

  **The enumeration came up short twice** — at F-17 by grepping `fn
  socket_path`, at F-22 because a helper of this class need not mint a socket
  or live under `tests/`. `claim`'s doc now carries the *class* and the grep
  that finds it (`process::id()`) rather than a list. That is the durable part
  of this item: whoever does the work should start from the grep, not from the
  six named here. `review-code.md` F-17, F-22.

- **Three states no round of this slice reached, all expressible with the
  existing harness.** Named here because the review named them rather than
  letting the silence read as coverage: nothing forced a genuine `Full` on the
  one-slot channel in a running `serve`; nothing killed a backend mid-form with
  a draft outstanding; nothing drove a real ingress arrival at a window holding
  a half-filled form. The second is 007's own — it is the state this slice
  introduced — and the other two are 003's and 004's mechanisms meeting 007's
  new one. This is budget, not tooling: the harness drives all three today.
  Relatedly, `choice`, `number` and `datetime` are covered as mapper values
  only; nothing drives one through `serve` to a `respond`, which will change the
  first time a renderer draws one. `review-code.md` §Negative results.

- **SPEC-001/OQ-2 — prefill and per-field errors — with the cost of its absence
  now written down.** Without `field.value` a fourteen-item form costs fourteen
  re-ticks to correct, and a backend must read `false` as *not yet* or its
  checklist closes early on the first submission. The trigger is sharper than it
  was: it fires when use says a form must **reject** an answer. `slice-007.md`
  §*What the workaround costs*.

- **SPEC-002/OQ-4 — a scheduled firing superseding a view mid-answer — is still
  open and was deliberately not answered here** (D13). A scheduled firing can
  discard a half-filled form, and that is decided behaviour. What bounds it is
  ADR-004's 3-second floor, anchored on the previous scheduled firing — **and
  its two exemptions reach exactly the person filling a form**: ADR-004 never
  delays an evaluation a person asked for, so pressing **Check now** mid-form
  replaces the form with no floor at all. Neither document says this; the
  connection is only visible from a slice holding both. `notes.md` Harvest.

- **Socket transport (009): promote or defer, now decidable.** A form collapses
  fourteen spawns per slot into one, which makes 009 measurably *less* urgent
  than when it was raised. That is the input the decision was waiting on.

- **`docs/memory/` is cited as if something checked it, and nothing does.**
  `path-flake-ref-breaks-on-demo-socket.md` was cited by `plan.md` and all six
  phase briefs while not existing; it is written now.
  `cite-requirements-not-finding-ids.md` exists and is unenforced across at
  least eight files. No compiler, gate step or reviewer resolves a `docs/` path.
  An instrument that resolves every `docs/memory/` citation in the repository is
  cheap; whether the *convention* wants enforcing is the larger question and is
  not this slice's to answer.

- **A phase cannot check its own Surfaces line.** Five of six were short, because
  a sheet is written before the work and nothing re-reads it after. The audit
  caught it four times. If this is worth fixing it is a methodology change —
  `docs/AGENTS.md`, not a slice — either by having the phase reconcile its
  Surfaces line at exit, or by dropping the per-phase enumeration in favour of
  the slice-level §Scope that no phase breached.

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

- **D3's rationale has a home outside this slice.** D3 shaped R-57's text and
  its reasoning lived only in `canon-delta.md`, which is consumed at promotion —
  so after reconciliation it would have existed nowhere. Recorded in
  `docs/roadmap.md` §Open decisions beside OQ-1 and OQ-2, with the trigger (the
  slice that first draws a `datetime` field). **The decision it records changed
  under F-21**: `datetime` is no longer deliberately unspecified but typed as an
  RFC 3339 `date-time` with an offset, and what the roadmap must carry is why a
  format was chosen on no demand rather than why none was. SPEC-001/OQ-4 carries
  the residue. **No ADR** — user decision, 2026-09-15. Raised as F-14 in
  `review-design.md`.

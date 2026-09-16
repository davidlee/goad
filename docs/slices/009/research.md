# Research — Slice 009

**Producers:** scoping agent (canon + code map); a throwaway Slint spike,
commit `a698217`, for every measured claim in Thread 3.
**As of:** 2026-09-16 · `a698217`

Evidence artefact for design and plan. Later stages cite this instead of
re-deriving. Refresh in place when it drifts; do not append rounds.

## Verification legend

- ✓ — independently verified by the *consuming* agent (a read or grep of the
  cited site).
- **M** — *measured*: a test in the spike asserts it, and the load-bearing ones
  are negative-controlled. Stronger than ✓; the test is named.
- unmarked — researcher claim: cited, not checked.

Design and plan may only load-bear ✓ / **M** rows, or rows they verify at point
of use. Verify what you lean on, not everything.

## Citation forms

Canon claims cite the document id (`SPEC-001/R-16`). Code claims cite
`path:line`. An uncited claim is unverifiable by definition.

## Thread 1 — governing canon

### Binding

- **SPEC-001/R-16** ✓ — field kinds are `text`, `boolean`, `datetime`, `number`
  and `choice`. A `number` **may** carry `min` and `max`; a `choice` **must**
  carry its own `options`; the other three carry no additional protocol keys.
  *The protocol already admits every kind this slice draws.* Nothing here adds
  one.
- **SPEC-001/R-57** ✓ — a submitted value's JSON type is fixed by the field's
  `kind` and nothing else: `boolean` → JSON boolean, `text` → string, `number` →
  number, `choice` → the chosen alternative's id as a string, `datetime` → an
  RFC 3339 `date-time` **carrying an offset**. All five are already typed; this
  slice implements four of them for the first time.
- **SPEC-001/R-55** ✓ — a renderer must not refuse a whole view because it
  cannot draw part of it, and a renderer subset must not narrow the protocol.
  This slice *discharges* a subset rather than adding a capability.
- **SPEC-001/R-58** ✓ — a `respond` carries values for exactly the fields the
  host drew of the option answered. Drawing four more kinds enlarges that set;
  the rule itself is unchanged.
- **SPEC-001/R-52** ✓ — a `choice` field's alternative ids are a **separate
  namespace** from option ids, and errors must name the right one.
- **SPEC-001/R-53** ✓ — a `choice` field's options carry an id and a label
  only; one carrying `fields` is rejected.
- **SPEC-001/R-17** ✓ — a `number`'s bounds must each be finite and min must not
  exceed max. Already enforced in normalization; the renderer may rely on it.
- **SPEC-001/R-18** ✓ — hints are flat keys, and **only the renderer** may
  branch on one. This is the clause that decides where `step`, and any
  slider-vs-spinbox choice, is allowed to live.
- **SPEC-001/R-35** ✓ — the host must not validate a submitted answer beyond its
  `view_id`. A widget may constrain what can be *entered*; the host may not
  refuse what was entered.
- **ADR-001 / POL-001** ✓ — `src/semantics/` stays pure; all of this slice is
  stratum 3.

### Checked, not applicable

- **SPEC-002** (scheduling) — untouched. One adjacency, recorded and not acted
  on: SPEC-002/OQ-4, a scheduled firing superseding a view mid-answer, is more
  visible once a form takes typed input. Still not this slice's.
- **SPEC-003** (event ingress) — untouched.
- **ADR-004, ADR-005** — scheduling and envelope normalization; neither is
  reached.
- **ADR-002 / ADR-003** — no new workspace member. The spike is deliberately a
  standalone workspace and is deleted at `a698217`'s successor.

### Amendment candidates

**None taken.** Two were identified at scoping and both are deliberately out:

- **`step` on a `number`.** The argument for making it protocol: a step
  constrains the submitted value, which is semantics, and R-18 forbids anything
  but the renderer branching on a hint. The argument for leaving it out: a
  slider with no step is continuous, R-57 requires no integrality, and no
  evidence has asked for one. Out until evidence asks.
- **SPEC-001/OQ-4** — whether a date without a time wants its own kind or a hint
  on `datetime`. `docs/roadmap.md` §Open decisions names *the slice that first
  draws a `datetime` field* as its answerer, which is this one. Held shut: the
  residue is additive either way, and drawing the control is what produces the
  evidence to answer it with. **If design finds that a date-only field cannot be
  expressed at all, that is the trigger to reopen it** — and raises no tier,
  because this slice is already tier 2.

## Thread 2 — code map

### Hotspots

| path | why |
|---|---|
| `crates/goad/src/view_model.rs:230` | `undrawn_form` — the exhaustive `match` over `FieldKind` that sorts a kind into drawn or `Undrawn::FieldForm`. Four arms move from undrawn to drawn. |
| `crates/goad/src/view_model.rs:131` | `FieldForm` — the undrawn-form enum. It is *not* a mirror of `FieldKind` by design; if every kind is drawn it may have no variants left, which is a decision, not a deletion. |
| `crates/goad/src/draft.rs:30` | `Edited` — one variant today, `Checked(bool)`. Its doc states outright that drawing a second kind should be *"a `submitted` arm rather than a reshaping of `Command`, `Draft`, `install.rs`"*. |
| `crates/goad/src/draft.rs:88` | `submitted` — the single application of R-57. Four arms to add. |
| `crates/goad/src/draft.rs:53` | `state_of` — returns `Edited::Checked(false)` as the as-drawn default. Needs a per-kind as-drawn value. |
| `crates/goad/ui/app.slint:14` | `FieldRow { id, label, checked }` — widens to carry a kind discriminant and a payload slot per kind. |
| `crates/goad/ui/app.slint:60` | `callback edited(string, string, string, bool)` — the bool is the value channel and must become one that carries five types. |
| `crates/goad/ui/app.slint:274` | the field repeater — the `if` chain per kind goes here. |
| `crates/goad/src/glass.rs:93` | `self.options.set_vec(options)` — the rebuild. This is the line the present-in-place work replaces. |
| `crates/goad/src/controller.rs:667` | `Command::Edit` — the edit path, and where a debounce would land or be rejected. |
| `crates/goad/src/controller.rs:739` | `glass.present(...)` at the loop top, unconditional. |
| `crates/goad/tests/renderer/fields.rs` | the field-drawing tier — see the tier constraint in Thread 3. |

### Cited facts

- ✓ Rows are built **from the draft**: `glass.rs::option_rows` →
  `field_block(&prepared.draft, option, block)`. A present regenerates the form
  with every value the person has entered, so **value persistence within a view
  already works** and is not this slice's problem.
- ✓ `Prepared` (`reception.rs`) holds `view_id`, `presentation` and `draft`, and
  its doc records that the draft is empty at construction because the protocol
  carries no `field.value`.
- ✓ `Command::Edit` returns `None` on success and falls through to the loop top,
  which presents — `controller.rs:661`'s comment states this is deliberate:
  *"writes retained state and the loop continues to the top, which presents and
  so writes the screen back from the draft."*
- ✓ `app.slint:279` records A-2's intent at the binding site: *"the next present
  rewrites this from the model, which is what corrects a click the host
  dropped."*

### Precedents

- **`draft.rs`'s inline `#[cfg(test)] mod tests`** is the shape for a
  stratum-internal pure function, shared with `controller.rs`,
  `goad-shell/src/state.rs` and `goad-semantics/src/schedule.rs`.
- **`ids()` in `draft.rs`'s tests** — ids can only be *read* off a normalized
  view, because `OptionId::new` / `FieldId::new` are `pub(super)`. Any new test
  building a field must parse a fixture view, not mint an id.
- **`tests/renderer/tree.rs::labels_in_option`** scopes by the **groupbox role**,
  and one case asserts exactly one groupbox exists per option with fields. Any
  new container drawn per field must not bind `accessible-role: groupbox`.
- **`tests/renderer/harness.rs::described`** — fields are found by accessible
  *description*, which carries `field.id`. Four new widget kinds must each carry
  it or become unfindable.
- **`crates/goad/build.rs`** — `with_debug_info(true)` is mandatory or
  `ElementQuery` returns empty and every renderer case passes vacuously.

## Thread 3 — the spike (all claims measured)

Commit `a698217`; six tests, all green, the load-bearing one
negative-controlled. Deleted by the following commit — read `a698217` for the
code.

### The present/draft mechanism

- **M** `set_row_data` preserves the repeater's element; `set_vec` destroys
  every row. Five presents in place construct **0** elements; one `set_vec`
  constructs one per row. `identity.rs::a_present_in_place_builds_no_new_elements`,
  `::a_present_by_set_vec_rebuilds_every_element`.
  Mechanism: `set_vec` → `ModelNotify::reset()` → `RepeaterTracker::reset` →
  `instances.clear()`; `set_row_data` → `row_changed` → `comp.update(row, data)`
  on the existing instance (`i-slint-core/model/repeater.rs`).
- **M** **Clicking a `CheckBox` destroys the use-site binding.**
  `fluent/checkbox.slint:27` is `root.checked = !root.checked`, and a property
  assignment removes the declared binding. After a click, `set_row_data` writing
  `false` leaves the widget `true`.
  `identity.rs::does_a_click_destroy_the_use_site_binding`.
  *This is 007's claim, previously read off the sources, now measured.*
- **M** A rebuild restores that binding — what A-2 rests on today, and never
  asserted until now. `identity.rs::a_rebuild_restores_the_binding_a_click_destroyed`.
- **M** A **guarded re-assert** driven by a root epoch converges a clicked
  widget to the draft *with the element preserved*, and writes nothing at all
  when the two already agree.
  `with_loop.rs::a_change_handler_fires_under_a_real_event_loop`, negative-controlled
  by deleting the re-assert (confirmed to compile and run before the red was
  believed — `docs/memory/a-negative-control-that-does-not-compile.md`).

  ```slint
  property <int> tick: root.epoch;
  changed tick => {
      if (self.checked != field.checked) { self.checked = field.checked; }
  }
  ```

  The guard is the point: writing nothing when nothing diverged is what lets a
  caret, a selection or a slider drag survive a present.

### The test-tier constraint — the finding with the sharpest consequence

- **M** **`changed` fires nowhere under `init_no_event_loop`** — not in a
  repeater and not on a root property.
  `identity.rs::changed_does_not_fire_without_an_event_loop`. Under
  `init_integration_test_with_system_time()` and a real
  `run_event_loop_until_quit()` it fires exactly as designed.
  Mechanism: `ChangeTracker::run_change_handlers_once()` is called from
  `WindowInner::ensure_tree_instantiated` (`i-slint-core/window.rs:805`), which
  that backend never reaches. An `ElementQuery` walk materialises the repeater
  but does **not** pump the trackers.
- **Consequence:** the whole of `crates/goad/tests/renderer/` uses
  `init_no_event_loop`, so a case written there asserting anything a `changed`
  handler does would be **green while measuring nothing**. The observable tier
  is the `tests/event_loop_schedule/` shape — a real loop driven by a
  `slint::Timer` that quits itself — and that tier is **one `#[test]` fn per
  binary**, because `init_integration_test_*` may be called once per process
  (`tests/event_loop/main.rs:5`).

### Widget and markup shape

- **M** A `PopupWindow` **cannot be repeated or conditional** — a hard compiler
  error. `DatePickerPopup` and `TimePickerPopup` must be single root instances,
  with the row recording which field opened one. Both being popups also means a
  `datetime` field has **no inline control**: the form holds a button showing
  the value.
- **M** One fat struct with a `kind` enum carries all five kinds in one
  repeater with declaration order preserved. `[string]` inside a struct works
  and a `ComboBox`'s `model` binds to it directly.
- **M** One callback taking that struct hands an edit back with no parsing: a
  Slint struct literal may name a subset of fields and the rest default, so the
  host reads the slot the `kind` names.
- ✓ `Slider` carries `minimum`, `maximum`, `step`, `value`, and fires both
  `changed` (continuously, while dragging) and `released`.
- ✓ **`LineEdit` exposes no readable cursor offset** — only
  `set-selection-offsets()`. The builtin `TextInput` exposes
  `out property <int> cursor-position-byte-offset`
  (`internal/compiler/builtin_elements.rs:2298`). So any approach that destroys
  and recreates a text field can only restore the caret to the end of the
  string. Not needing to is a reason to prefer preserving the element.

## Cross-thread findings

1. **This slice adds no protocol and removes a renderer subset.** R-16 and R-57
   already type all five kinds; R-55 names the gap as a subset that must not
   narrow the contract. The tier comes from design size, not from canon.
2. **A-2 and typed input are in direct tension, and the spike resolves it.**
   A-2 depends on the rebuild; a caret depends on there being no rebuild. The
   guarded re-assert satisfies both, and is the only candidate measured to do
   so. Anything else the design considers must be held to the same two tests.
3. **The mechanism this slice depends on is invisible to the test tier that
   would naturally house it.** That is a planning input, not a detail: either a
   new one-test binary in the `event_loop_schedule` shape, or a stated gap
   verified by a person. Choosing silently is how a vacuous green gets written.
4. **`Edited::state_of`'s as-drawn default generalises badly.** `Checked(false)`
   is the protocol's answer for a boolean, but there is no protocol answer for
   *as-drawn* on a `number` with bounds, a `choice` with alternatives, or a
   `datetime`. R-58 forbids omitting the value and SPEC-001/OQ-2 (`field.value`)
   is unlanded, so the design must state a per-kind as-drawn value and own it.
   This is the sharpest *undesigned* question in the slice.
5. **The debounce has a submit race.** A 150 ms text debounce means the draft
   can lag the widget; a person who types and immediately clicks the option
   button could submit the previous text. The flush point — on `accepted`, on
   focus loss, or before building a `respond` — is a design decision with a data
   loss failure behind it.

## Design-input deltas

- **The re-present problem is one decision, not a second design surface.** It was
  framed as one at scoping and priced accordingly; the spike shows a measured,
  controlled mechanism. What survives is the *integration* work — where the
  epoch lives, when `glass` may write in place versus rebuild (the shape
  changing, a new view, a refused command), and how that is tested.
- **`datetime` is the least-determined kind**, and its cost was underestimated:
  no inline control, two popups, and R-57 requires an RFC 3339 instant **with an
  offset** while `DatePickerPopup` yields a bare `{year, month, day}`. Composing
  a date, a time and the local offset into a conforming string is host work with
  its own failure modes, and it is the kind most likely to force OQ-4 open.
- **`number` with absent bounds is undesigned.** R-16 makes `min`/`max`
  optional; a slider requires both. What an unbounded `number` draws — a
  `SpinBox`, a `LineEdit`, or a slider over an invented range, which would be
  the host inventing domain meaning — is a design question with an invariant
  behind it.

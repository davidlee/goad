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

First raised at `a698217` (six tests), restored and extended at `0813ee7` and
again at `4f93d41`. All green, each load-bearing claim negative-controlled or
injection-passed. The code is on disk in `spike-fields/` until the design
closes; after that, read those commits.

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

### The split channel (measured during design, slice 009 D-9)

- **M** **A flat value property indexed by a slot tracks, and replacing it
  wholesale destroys no element.** `rows` is repeated over and carries
  structure only; `values` is a bare `[FieldValue]` property nothing repeats
  over, read as `root.values[field.slot].checked` from inside the repeater.
  Replacing the entire `values` property and bumping the epoch converges a
  clicked widget with **inits unchanged** (2 → 2), and a second present that
  agrees writes nothing (`reasserts` stays 1).
  `split.rs::a_flat_value_model_tracks_and_a_wholesale_rewrite_costs_no_element`.

  Doubly negative-controlled, both confirmed to compile and run: deleting the
  re-assert leaves `reasserts` at 0 and the widget diverged; forcing a `rows`
  rebuild at the same point takes inits **2 → 4**, which is what shows the
  counter moves at all rather than being a number that never changes.

- **M** **A number drawn as a `LineEdit` must compare numerically, not as
  text.** A guard written `self.text != …number` writes `"0"` back over a
  person who cleared the field to retype; written
  `self.text.to-float() != …number` it stays quiet and the clear survives the
  present. `numeric_guard.rs::a_numeric_guard_does_not_fight_a_cleared_field`,
  negative-controlled by restoring the string comparison, which fails with
  `reasserts 1`. Slint's `to-float()` reads `""` as `0`, which is what makes
  the two agree.

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

### The guard's comparand, measured (round 3, `4f93d41`)

`guard_text.rs` runs seven slots against **one** guard — §5.2's, exception and
all — and varies only what the host puts in `text`.

- **M** **Re-formatting the `f64` corrupts ordinary typing.** Typing `1.05`
  yields `105`: the host holds `1` after `1.`, the guard overwrites the dot,
  and the `0` and `5` land on what is left. Typing `-3` yields `3` — `-` alone
  does not parse, so nothing is recorded and the guard replaces the sign with
  `0` before the digit arrives. This is F-30; the finding's own example (`15`)
  was wrong and the defect is worse than it stated.
- **M** **Holding the text the person typed is quiet through all of it** —
  `1.05`, `-3`, trailing zeros, and `1e400`, which `input-type: decimal` admits
  (Slint validates through an `f32` parse, where it is an infinity) and which
  `Finite` must refuse. The host keeps `1e400` on screen and `1e40` as the
  number it would submit, so nothing non-finite can reach the wire and nothing
  is written over the person.
- **M** **AC-6 still converges.** An edit that reaches nothing is corrected on
  the very next present, in one step.
- ✓ **The control admits exactly three texts no parse accepts** — `-`, the
  locale separator alone, and `-` followed by it. `accept_text_input` allows a
  candidate of length ≤ 2 only in those three shapes and otherwise requires
  `string_to_float` to succeed (`i-slint-core/items/text.rs:2202-2229`). `--` is
  **not** admissible, so the design's earlier edge example could not be typed.
- **M** **The cleared-field exception still earns its place**, in the race
  between a clear and its own debounce. `numeric_guard.rs`'s case survives the
  change of comparand.

### The revision trigger, measured (round 3, `4f93d41`)

`revision.rs` answers whether the host could drop the comparison altogether and
converge per field on its own say-so.

- **M** A present that changes no revision fires **nothing** — so the trigger
  does not write over a person on every present.
- **M** One bump converges that slot and leaves its neighbour mid-edit.
- **Not taken** (F-30), because `-` and `1e400` are edits the host cannot
  record, which is exactly when a revision guard converges. It is available if
  the cleared-field exception ever grows.

### A popup's lifetime — the fact that refuted F-31 (round 3, `4f93d41`)

- **M** **No `PopupWindow` state survives a close.** `show-popup` compiles to a
  fresh `#popup_window_id::new(...)` on every show
  (`i-slint-compiler/generator/rust.rs:3736-3763`) and the closed instance is
  dropped from `active_popups` (`i-slint-core/window.rs:1955-1990`). Measured:
  two untouched `datetime` fields seeded identically each open on their own
  seed, and so does the field whose pick would have leaked. A differing seed for
  the second field is the positive control.
- **Consequence:** seeding a picker needs no defence against the previous
  field's pick, and picks up a re-seed through a fresh **binding** rather than
  through `changed date` — so it is not a `changed` handler and not, on that
  ground, confined to the loop tier (F-36). Seeding is still required, because
  a *picked* field must reopen on its pick rather than on the widget's default
  of today.
- **M** **A `PopupWindow`'s properties cannot be assigned from an enclosing
  component's handler** — *"Cannot access property or callback 'picker.date'
  inside of a Window from enclosing component"*, a hard compiler error. They can
  be bound at the popup's own declaration site, and `show()` may be called from
  outside, which is why the first spike did not meet this. F-35.
- **M** **The date-picker chain needs no pointer event.** A calendar day cell is
  `accessible-role: button` with the day number as its label and a default
  action (`common/datepicker_base.slint:59-63`); the dialog's OK is a
  `StandardButton` labelled `OK`. Both drive through
  `invoke_accessible_default_action`, so neither depends on layout. A
  `ComboBox`'s `ListItem` has the role and the label but **no** default action
  (`fluent/components.slint:49-53`), so that one does need
  `mock_single_click` — F-33 reduced to that row alone.

## Thread 4 — mechanisms considered and not taken

Written so design does not rediscover these. **None of Thread 4 is measured**;
each is a reasoned argument from Thread 3's measurements and the cited code, and
design should verify anything it leans on.

### Rejected: skip the `set_vec` when the rows are unchanged

The obvious cheap fix — build the rows, compare against what the model already
holds, and skip the write when nothing differs — **breaks A-2 in exactly the
case A-2 exists for.** The dropped-click case is one where the *model data did
not change* (the draft still says `false`, because the host never heard the
click) while the *widget* diverged. The comparison sees equality and skips,
leaving the widget wrong permanently.

Worth stating plainly because it is the first thing a reader proposes and it
looks right: the diff is over the wrong pair. What diverged is widget-vs-draft,
and a host comparing draft-vs-draft cannot see it.

### Rejected: do not present after a successful `Edit`

Proposed at scoping, before the spike. `controller.rs:661` already notes an edit
"is not an exchange", and the widget already holds what the person just did, so
the present is arguably redundant. It would fix the per-keystroke rebuild.

It is not enough. It only suppresses the present that the *edit itself*
triggers; every other present — a `view: null` answer to a scheduled evaluate, a
back-pressure notice, diagnostics arriving — still runs the loop top and still
rebuilds the form under whatever the person is doing. A partial fix that looks
total is worse than none, because the residual case is rare enough to ship.

### Rejected: commit-only bindings

Bind `accepted` (Enter) and `released` rather than `edited` and `changed`, so no
command is raised mid-interaction. Cheapest of all, and it needs no re-assert.

It loses data: `LineEdit::accepted` fires on Enter only, focus-out does not fire
it, and a person who types and then clicks the option button submits the
*previous* value. Silent, and in the direction that matters. Note this is not
the same question as OQ-3's debounce, which narrows the same race to ~150 ms
rather than making it unbounded — but the flush point OQ-3 has to choose is
this failure in miniature.

### Superseded: 007's stated focus repair

`slice-007.md` §Follow-ups proposes "a focus identity that survives a rebuild —
the row reports focus back, the host retains the focused field id, the row's
`init` restores it". Thread 3 makes it unnecessary: an element that is never
destroyed never loses focus, so there is nothing to restore.

It also has a hole, which is why it should not be revived as a fallback:
`LineEdit` exposes no readable cursor offset, so a restored caret can only go to
the end of the string — right while appending, wrong the moment anyone edits
mid-word.

### Not taken, but available: rebuilding one row

A third mechanism, if the design ever needs a rebuild narrower than the whole
form. `Model::remove_row` then `insert_row` at the same index rebuilds **exactly
that row's element**: `row_removed` drains the instance from the repeater's
vector and `row_added` splices in `(Dirty, None)`, which `ensure_updated` then
constructs (`i-slint-core/model/repeater.rs`). `set_row_data` never does this —
a Dirty instance that still holds a component is updated, not recreated.

Read the source before leaning on it; this is inferred from the repeater's
implementation and is not covered by any spike test.

### A claim worth verifying, because it would simplify OQ-5

**Divergence may always be observable to the host.** A widget self-assigns and
the host fails to record it only when the command did not arrive or was refused
— `Refused::Full` on the one-slot channel, or an `edit()` refusal for a stale
view — and the host knows about both. `toggled` fires on every click, so there
is no silent third path.

If that holds, A-2 could be satisfied by *rebuild when a command was refused,
write in place otherwise* — with no epoch and no per-widget guard, which is
materially less markup than Thread 3's mechanism. It would also make the
re-assert testable in a tier that has no event loop, which would dissolve OQ-4.

**Not verified, and design kept the guard** (D-9, corrected at review F-12 and
F-23). Two of design's attempts to kill the claim were themselves wrong, and the
record says so rather than leaving a falsification nobody rechecked.

The first attempt argued from
`wire.rs:128-132` that the back-pressure signal is a level about the last send,
so a refused edit A followed by a landed edit B would leave a clean notice with
A permanently diverged. That trace is not reachable — `serve` handles
`Command::Edit` synchronously and presents before any UI callback can run again
(`controller.rs:667-676`, `:739`), and an exchange presents `busy = true` and
disables every control before it awaits (`:818-819`).

The second attempt was that the host, granted it knows *that* an edit was
refused, cannot know *which* widget diverged. It can. ✓ `TrySendError::Full(T)`
returns the command it
refused, and `wire.rs:127-133` binds it `_returned` and discards it
deliberately; a `Command::Edit` names the view, the option and the field.

What does kill the alternative is two things, neither of which is about
identity:

1. It rests on a **completed enumeration** of the ways an edit can be lost, and
   `docs/memory/enumerate-the-class-not-the-instances.md` is the standing
   warning about that shape of argument. This thread never completed the list;
   the measured guard depends on no list at all.
2. Even holding the field id, the only correction available without the epoch is
   a targeted row rebuild — *Not taken, but available*, above — and the row it
   would rebuild is the row the person is typing in, because that is where edits
   come from. Narrower than a whole-form rebuild, and fatal in the same way.

The measured guard stays.

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

## Upstream slint development docs — read during audit session 2

`~/.local/src/slint/docs/development/` is a slint source checkout's internals
documentation. Five of its seventeen files bear on this slice's findings.

**Read the version note first, and then the way round it.** The checkout is at
`v1.18.0` (released 2026-09-16); this workspace pins `=1.17.1`
(`Cargo.toml:54-56`). So the working tree of those docs describes a **later
release** than the one that ships here.

The skew costs nothing, because the checkout carries the `v1.17.1` tag. Read
the version-matched docs directly and the question does not arise:

```zsh
git -C ~/.local/src/slint show v1.17.1:docs/development/input-event-system.md
```

Every finding in `review-code.md` cites the **vendored** 1.17.1 source under
`~/.cargo/registry/`, never these files; the docs are read for mechanism and
intent.

### What they corroborate, and what they add

**`window-backend-integration.md` §Popup Management — bears on F-R1.** It
documents `PopupWindow`'s structure and `PopupClosePolicy`'s three values
(`CloseOnClick`, `CloseOnClickOutside`, `NoAutoClose`) and **names no
host-side API for closing a popup**. The only inspection route it offers is
`WindowInner::from_pub(&window).active_popups()`, which is `i-slint-core`
internals this workspace does not depend on. So the markup-side
`dismiss-pickers()` the injection pass settled on is not a workaround for a
missing API — it is the sanctioned road, and the finding's observation that
`close_all_popups` has exactly one caller is consistent with that.

**`input-event-system.md` §Key Event Processing — bears on F-R1.** The key
dispatch diagram reads *"If popup active → send to popup"* **before** *"Send to
focus item"*. That is the documented statement of the keyboard half of F-R1,
which the audit's loop-tier case measured independently.

**`property-binding-deep-dive.md` §ChangeTracker — bears on F-R5.** It states
the intended contract in as many words: a `PropertyTracker` is *"notified when
dependencies become dirty"*, a `ChangeTracker` *"when the evaluated value
actually changes"*. F-R5 is therefore a defect **against a documented
intent**, not a quirk to be lived with: `SharedImageBuffer::eq` comparing
`data.as_ptr()` makes two byte-identical icons compare unequal, so the tracker
fires on a value that did not actually change. It strengthens F-R5's
`fix-now` — rasterise the two icons once and hand out clones.

**`model-repeater-system.md` §Performance — bears on D8, F-S1, F-S5.** Its
Common Issues table names *"Recreating model on every change → Modify existing
model, don't replace"*, and its first performance rule is *"Prefer modify over
replace"*. That is D8's guard, arrived at independently and for the same
reason. Worth knowing that the repeated `options` model already follows it —
`present` writes it only where the `ViewId` changed — and that the per-present
`VecModel` for `values` is **not** an instance of the anti-pattern, because
`values` is read by `root.values[field.slot]` bindings rather than repeated
over, so replacing it destroys no element.

### What they do not answer

Nothing in these files documents what a **disabled** item does with input that
is delivered to it. `TextInput::key_event` returning `EventIgnored` on
`!enabled` (`items/text.rs:954`) and `TouchArea`'s disabled branch cancelling a
live grab (`items/input_items.rs:81-93`) remain read from the vendored source,
which is where F-A1 and F-R2 cite them.

### slint 1.18.0 — assessed against this slice, 2026-09-20

The release is four days old and the question was whether to take it now. **It
fixes none of the three defects under repair**, which is the answer:

- **F-A1 / F-R2** — `enabled` semantics are untouched. A disabled item still
  drops input rather than queueing it.
- **F-R1** — the three `PopupWindow` entries are an offset calculation, a
  base/derived shared-`is-open` bug, and `#12602`, *showing or closing a
  `PopupWindow` from a repeated or conditional element*. **None of them gives
  the host a way to close an open popup**, which is F-R1's actual gap, so the
  markup-side dismiss stays the repair.
- **F-R2 row 5** — the `ComboBox` popup item's missing `enabled` gate is not in
  the list. The one `ComboBox` entry is about writing `current-value`.

**One opportunity it does open**, recorded and not taken: with `#12602` fixed,
the pickers *could* be declared inside the `if root.mode == WindowMode.prompt`
block, so the mode switch would destroy them and F-R1 would close structurally
rather than by an explicit dismiss. That is a design change, it is not needed
once the dismiss lands, and it belongs to whatever slice takes the upgrade.

**Four migration hazards to price when that slice runs**, all of which bear on
the suite rather than on the product:

1. **"Redundant inner elements of compound widgets are now hidden from the
   accessibility tree."** The whole renderer tier selects by accessible role
   and description (`tests/renderer/tree.rs`, `fields.rs`). This is the one
   that could move a large number of the 592.
2. **"Accessibility: Exposed the content of text inputs to assistive
   technologies, including the text selection."** Every text case drives
   `set_accessible_value`; the semantics around it are being changed.
3. **`Slider`: the `pressed` property now follows the touch area** (`#13385`) —
   the property VH-1's drag failure turns on.
4. **`show()` / `hide()` on `SystemTrayIcon` components fixed** — this host has
   a tray, and `glass.rs` calls `show()` on every present.

**Sequencing.** Not during this audit: all twenty-one findings cite 1.17.1
sources, so a bump moves the citation base and changes the audit's subject
(`a698217..HEAD`) while repairs are in flight. It is its own slice, and the
hazard list above is what that slice starts from.
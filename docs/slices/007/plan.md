# Plan — Slice 007: the renderer grows a form

The executable phase plan. Read with `design.md` — the plan never overrides the
design or canon; if it seems to, the plan is wrong.

<!-- Phase ids (PHASE-NN) and criterion ids (EN-/EX-/VT-/VA-/VH-N) are
     immutable: edits append, never renumber, so the sequence goes
     non-monotonic after a split and that is expected. Criterion ids are local
     to their phase — cite another phase's phase-qualified (PHASE-03/EX-2).
     Verification modes — VT: automated test. VA: agent check. VH: human
     acceptance.
     Progress is NOT recorded here. Status lives in `notes.md`. -->

## Overview

Six phases, in strict dependency order, each one agent-session sized including
bookkeeping.

```
PHASE-01  the notice gets an owner              wire, controller, main, glass, 6 test files
PHASE-02  the window draws a form               build.rs, app.slint, glass, tree.rs
PHASE-03  the mapper and the draft              view_model, draft (new), diagnostics, lib, mapper, reception
PHASE-04  the draft is retained, and answered   reception, controller, wire, install, glass, wiring
PHASE-05  the form on the wire                  fields.rs (new), harness.rs, scheduling.rs (a lift)
PHASE-06  the demo, and the look                examples/, app.slint (bounded), audit Evidence
```

The middle of the slice is deliberately split by **stratum and by tier**, not by
feature: 02 is the markup and what a headless window can be asked on its own, 03
is the pure Rust that names no widget, 04 joins them in the controller, and 05
is the only phase that reads the wire. A defect found in one of them is
therefore a defect in one layer, which is the property a single diff of all four
would not have.

**Every phase sheet re-derives its ids from this file.** `docs/AGENTS.md`
§Phase plan has each agent expand its phase into a sheet in `notes.md`
immediately before executing, copying objective, criteria and reading list out of
this document — and nothing checks a sheet against the plan it was copied from.
That is the same renumber-shaped edit that produced three stale citations during
this plan's own repair round (`review-plan.md` F-7), performed six times by six
agents. So: **the first task in every phase sheet is to re-derive its criterion
ids from `plan.md`, not to trust the copy.** PHASE-04's sheet does it twice — it
grew from seven exit criteria to ten under repair and restates more claims in
prose than any other phase.

Nothing in the gate checks this document against itself. The check is mechanical
and cheap: every `PHASE-0N/<id>` cross-reference resolves to a criterion that
exists in that phase, no phase's id sequence has a gap or a duplicate, and every
count the prose asserts matches the list it describes.

**Not in any phase, and not a phase's to take:**

- **AC-8's promotion.** `canon-delta.md` becomes SPEC-001 R-57 and R-58 at
  **audit**, with explicit user endorsement (`docs/AGENTS.md`). What the phases
  owe it is the two verification vehicles (Coverage, AC-8) so the §7 rows have
  something to name. A slice does not close holding an unpromoted draft, so
  audit must reach it; no phase writes into `docs/specs/`.
- **The look beyond what drawing fields forces.** Typography, window sizing, the
  idle surface and the look of the controls are 008's (`slice-007.md`
  §Non-goals, AC-10).
- **The six open questions and D1–D13.** Closed by recorded user decision
  (`slice-007.md` §Open questions, `design.md` §6, §7). Re-opening one is a
  design change.
- **SPEC-002/OQ-4.** Deferred by D13. A scheduled firing can still replace a
  half-filled form, and that is the decided behaviour, not a defect to fix in
  passing.

## Sequencing & rationale

- **01 first, and it discharges no acceptance criterion.** That is correct
  rather than a gap: the notice repair arrived through design review, after
  `slice-007.md`'s criteria were written, and its exit criteria are
  `design.md` §5.1, §5.3 and §5.4 instead. It is first for two reasons. It is
  the only phase that depends on nothing — no field, no draft, no block — so
  placing it anywhere later risks its being read as optional. And it changes
  three signatures (`Controller::frame`, `serve`, `Wire::new`) across 111 measured
  call sites in six test files; doing that before PHASE-05 adds a seventh file is
  strictly cheaper than after.
- **02 before 04** because `glass.rs` cannot build a `FieldBlock` that the
  markup has not declared. 02 also front-loads the two risks that come from
  outside this workspace: the style default (`design.md` R-7, whose signal is
  `cargo test -p goad` going red immediately after the `build.rs` change) and
  the codegen and model-reset assumptions A-1 and A-2.
- **03 before 04** because `Prepared` cannot hold a `Draft` that does not exist,
  and `option_rows` cannot read `presentation.blocks`. 03 is pure Rust: no
  window, no platform, no runtime.
- **02 and 03 are independent of each other** and could be swapped, or run in
  parallel by two agents if the worktree rule allowed it — their surfaces are
  disjoint. 02 is placed first only to meet the outside-dependency risk earlier.
- **04 before 05** because nothing can be read off the wire until `answer()`
  fills `values`. 04 verifies at `answer()` and at the window; 05 verifies at
  the backend's own log. Neither substitutes for the other (`design.md` §9).
- **06 last** because AC-7 and AC-10 are a person's, and a person judging the
  legibility of a form needs the form finished.
- **Could be dropped:** nothing. `examples/typescript/backend.ts` is a
  deliberate non-surface — no acceptance criterion reaches it, and the gate
  typechecks it, so touching it is work without a criterion.

**STOP conditions**, at any phase. Stop and consult the user; do not improvise
past one.

- S-1 — an unresolved **design** issue surfaces. Go back to design and work
  forward (`docs/AGENTS.md` §Plan, §Phase plan). Do not repair it in a phase
  sheet.
- S-2 — an existing test's **assertion or fixture** must change to stay green,
  outside the four places this plan names one: PHASE-02's mechanical `blocks`
  member in `tree.rs`'s row builder; PHASE-01's inversion of `wiring.rs`'s
  back-pressure case, which is a deliverable and not a fixup; PHASE-04's
  **extension** of `reception.rs`'s `every_refused_variant_renders_one_line_with_the_failure_prefix`
  with a fourth variant, which keeps the claim its name makes rather than
  altering one; and **PHASE-02's two `wiring::busy` cases, which may set an
  explicit window size before the first present** — the fixture gains a
  viewport, no assertion changes. **AC-6 is the criterion this protects, and
  `design.md` §9 scopes it to `tree.rs`, `table.rs` and `wiring.rs`** — the
  option tests — not to every file in the target. A *signature* sweep — `frame(notice)`,
  `serve(.., notice, ..)`, `Wire::new` losing an argument — changes call shape
  and not what any test asserts, and is expressly allowed, as is a change to a
  test **helper's** implementation (PHASE-02/EX-7), which is neither an assertion
  nor a fixture.

  The **fourth** allowance was taken during execution, by user decision, when
  R-7's signal fired (`plan-log.md`, 2026-09-15). Under `material` a shown window
  clips its second option out of the `Flickable`'s rect, and `ElementQuery` skips
  what is clipped — `i-slint-backend-testing-1.17.1/search_api.rs:373-375` into
  `i-slint-core-1.17.1/item_tree.rs:399-408`, a geometric test against the
  nearest clipping ancestor. The two cases passed before only because the window
  happened to be 65px and fluent's `Button` happened to be 32px: an 18px margin
  on a layout nothing declares, which is a proxy of the shape the slice-004 scar
  is about. **The window's own size remains AC-10's and 008's.** This fixes what
  a test can see, not what the product does, and that distinction is held open by
  a finding rather than closed by a green test.
- S-3 — a new dependency, or a feature added to an existing one.
- S-4 — anything under `crates/goad-semantics/` must change. The specific
  temptation is deriving `Ord` on `OptionId`; `design.md` §5.2 shapes `Draft`
  precisely so that it is not needed, and the crate orders an id exactly where
  the protocol keys a serialized map by it.
- S-5 — an `Edited` variant beyond `Checked`, or a drawn kind beyond `boolean`,
  arriving without a change to the command channel or the commit event
  (`design.md` §3, R-3). The seam fixes the shape of a value, not the transport
  that carries it.
- S-6 — the draft is about to enter `Presentation`: a `checked` member on
  `PresentationField`, or any mutable member on a `view_model.rs` type (I-4,
  R-4). **No instrument in the gate reaches this** — `cargo test -p goad-semantics`
  and the three other ADR-001 instruments all stop short of stratum 3
  (POL-001 §Verification, `design.md` §3), so a green gate is not evidence
  against it.
- S-7 — a field test that neither reads an invocation log nor asserts something
  about the screen. That is the proxy the slice-004 scar is about
  (`docs/memory/a-green-test-can-assert-a-proxy.md`, `design.md` §9).
- S-8 — PHASE-06 wants to change anything beyond the block container and its
  separator, and the heading's own treatment. That is AC-10's bound; the rest is
  recorded verbatim and becomes 008's brief.
- S-9 — `Glass::present` is about to acquire a write-only-on-change path, or an
  exception for the field model (I-5). Weakening totality is not the repair for
  anything in this slice, and it breaks A-2.

## Coverage

| AC | discharged by |
|----|---------------|
| AC-1 | PHASE-05/VT-1 — N keys read off the wire, each a JSON boolean matching the screen |
| AC-2 | PHASE-03/VT-3 (the rule, as a pure `present()` test), PHASE-04/VT-7 (`option_rows` carrying blocks and their order from the presentation to the row model), PHASE-02/VT-4 (row model to screen, read from tree order). **Three links, because the chain has three** — a criterion over any two of them leaves the third unverified |
| AC-3 | PHASE-03/VT-4 (the **`Undrawn` value** naming option, field and kind), PHASE-03/VT-6 (it reaches the diagnostic surface), PHASE-05/VT-4 (the view is still shown and the option still answers, carrying only its boolean keys). The rendered **wording** is held by review, as every diagnostic wording in this project is |
| AC-4 | PHASE-05/VT-2 — two options sharing a field id; the request names one option and carries only its keys |
| AC-5 | PHASE-05/VT-3 — **both** halves, screen and wire. Neither implies the other |
| AC-6 | PHASE-02/EX-4 and VT-5 (a zero-field option adds no element), PHASE-02/VA-1 and PHASE-04/VA-1 (no existing assertion changed in the option tests). `design.md` §9 scopes AC-6's "no change to what they assert" to `tree.rs`, `table.rs` and `wiring.rs`; `reception.rs` gaining a fourth `Refused` variant assertion (PHASE-04/VT-6) is outside that scope and is named in S-2 |
| AC-7 | PHASE-06/VH-1 |
| AC-8 | PHASE-03/VT-2 (R-57's `boolean` clause at `draft.rs::submitted`), PHASE-04/VT-3 (R-58 at `answer()` over a two-option view). **Promotion into SPEC-001 is audit's**, with explicit user endorsement; no phase writes canon. R-57's `text`, `number`, `choice` and `datetime` clauses are review, not a test, until a renderer draws them |
| AC-9 | each phase's last exit criterion is `just check` exit 0 (PHASE-01/EX-9, 02/EX-8, 03/EX-8, 04/EX-10, 05/EX-5, 06/EX-7); PHASE-06/VA-1 restates it over the finished slice, with the two things the gate does not reach named |
| AC-10 | PHASE-06/VH-2 |

---

## PHASE-01 — the notice gets an owner

**Objective:** back-pressure is retained at the edge and written to the window
from the frame, so the notice explaining a dropped action survives the present
that corrects the action.

**Surfaces:** `crates/goad/src/{wire.rs, controller.rs, main.rs, glass.rs,
diagnostics.rs}`, `crates/goad/tests/renderer/{wiring.rs, scheduling.rs,
table.rs, ingress.rs}`, `crates/goad/tests/event_loop/closing.rs`,
`crates/goad/tests/event_loop_schedule/scheduling.rs`.

**Entry**
- EN-1 — `just check` exits 0 on a clean tree at the slice's base commit, re-run
  before anything is edited; transcript kept.

**Exit**
- EX-1 — `wire::Notice` exists, following `Cancel` edge for edge
  (`wire.rs:142-181`): a `tokio::sync::watch::<bool>` holding both halves,
  `Debug + Clone + Default`, a **synchronous** setter callable from a Slint
  callback, and a non-async read of the current value. `watch` is the vehicle
  because it retains its last sent value, which is the property the repair needs
  (`design.md` §5.3).
- EX-2 — `main` constructs it beside `Cancel`, clones it into `Wire`, and passes
  it to `serve` beside `cancel` (`main.rs:85-119`, `controller.rs:576-584`).
- EX-3 — `Wire::send` writes the signal: `true` on `Err(TrySendError::Full(_))`,
  `false` on `Ok(())`. `Closed` keeps its present behaviour of doing nothing, so
  the arm it shares with `Ok` splits and `clippy::match_same_arms` no longer
  reaches it.
- EX-4 — **`Wire` no longer writes the window.** Its `slint::Weak<PromptWindow>`
  field, the `window` parameter of `Wire::new`, and its import of
  `crate::generated` all go: with `send` no longer upgrading the handle the
  field is never read, and `dead_code` is an error under the gate's
  `-D warnings`. `Wire`'s hand-written `Debug` goes with it — every remaining
  field carries one — and so does the paragraph in its doc-comment explaining
  why the handle was weak. The six `Wire::new` call sites follow
  (`src/main.rs`, `src/wire.rs`'s own tests, `tests/renderer/wiring.rs`,
  `tests/event_loop/closing.rs`, `tests/event_loop_schedule/scheduling.rs`).
- EX-5 — `Frame` gains `notice: bool`; `Controller::frame` gains one parameter
  and stays `&self` and non-mutating; **`Controller` gains no field**. `serve`
  reads the signal at each of its three present sites — `controller.rs:611`
  (top of the loop), the `busy = true` present after `engage`, and the
  `ingress`-stopped present inside the inner loop — and passes it to
  `frame(..)`.
- EX-6 — `Glass::present` writes `notice` from the frame: `BUSY_NOTICE` when
  `true`, `""` when `false` (`glass.rs:107`). It is the **only** writer of the
  window's `notice` property, and `BUSY_NOTICE`'s import moves from `wire.rs` to
  `glass.rs`.
- EX-7 — **every live home of the old rule is corrected**, enumerated here so the
  sweep cannot stop early
  (`docs/memory/a-repair-sweep-misses-the-binding-site.md`); the first two are
  the binding ones and neither is prose a search for the repaired wording would
  match:
  - `controller.rs:92-93` — `Frame`'s doc loses its carve-out: every property,
    not "every property but `notice`". With the field present the "but" goes,
    and that is §5.1's stated reason the change is an improvement rather than a
    cost.
  - `glass.rs:23` — the trait doc's *"`notice` is written `""` here and set from
    nowhere else in this trait"* becomes the frame-carried rule.
  - `controller.rs:669` — the refusal site's comment says the `continue`
    "presents with `busy = false` and **clears `notice`** in the same call". It
    no longer does.
  - `wire.rs:115-118` — `send`'s doc describes writing `BUSY_NOTICE` through the
    weak handle.
  - `wire.rs:184-186` — the test module's comment about "`Full` writing `notice`
    through a live weak handle".
  - `diagnostics.rs:304-306` — `BUSY_NOTICE`'s doc says "`Wire::send` is its
    only writer". `Wire::send` now raises a signal; `Glass::present` writes the
    string.
  - `tests/renderer/wiring.rs:346-348` — `mod back_pressure`'s own doc states
    the cleared-on-next-present rule.
  - `ui/app.slint:18` — `notice` is described as "transient". Check it against
    the new lifetime and correct it or leave it deliberately; it is the one
    entry on this list that may need nothing.
  - `wire.rs:120-124` — *"`Closed` does nothing, deliberately … **It shares one
    arm with `Ok(())`** — `clippy::match_same_arms` — so the pattern stays named
    rather than swept into a wildcard."* EX-3 splits that arm, so the sentence is
    false. **This line contains no occurrence of "notice" and VA-2's instrument
    cannot reach it**, which is why it is on a list rather than left to a grep.
  - `glass.rs:36-38` — *"Hand-written for the same reason `Wire`'s is: the
    generated component handles carry no `Debug` …"*. EX-4 deletes the impl it
    points at, so the cross-reference goes nowhere. `SlintGlass`'s own `Debug`
    stays hand-written and the reason stays true of it; what must go is the
    "same reason `Wire`'s is" clause. Also invisible to VA-2.
- EX-8 — every existing test passes with **no assertion changed** except
  VT-1's, which is a deliverable. The rest of the diff in the six test files is
  call shape only (S-2's allowance).
- EX-9 — `just check` exits 0. A phase is not done until the gate is green
  (`docs/AGENTS.md` §Execute); the next phase's EN-1 confirms it rather than
  discovering it.

**Verification**
- VT-1 — `tests/renderer/wiring.rs`'s `a_full_channel_sets_notice_and_the_next_present_clears_it`
  **inverts**, and is the phase's headline deliverable rather than a fixup: a
  command sent into a full channel raises the notice; the next present writes
  `BUSY_NOTICE` to the window; **a further present leaves it there**; a
  subsequent successful send, followed by a present, clears it. Its name and its
  assertion messages change with it — the current message cites `design.md §5.3`
  for a clearing rule that section no longer holds.
- VT-2 — `Frame`'s half, with no channel, no window and no platform:
  `controller.frame(true)` carries `notice: true` and `frame(false)` carries
  `false` (`design.md` §5.3).
- VA-1 — `git diff` over the six test files shows nothing but call shape and
  VT-1's case: no other assertion, no fixture.
- VA-2 — `grep -rin notice crates/goad/src crates/goad/ui crates/goad/tests`
  after the sweep, read against EX-7's list, and the residue accounted for line
  by line.

**Notes for the implementer**

`Cancel` is the template and it is two screens long; read it whole
(`wire.rs:142-181`) before writing `Notice`. The one difference is direction:
`Cancel` is one-way and level-held forever, `Notice` is set both ways. Names
that keep the symmetry: `Notice::set(bool)` and `Notice::raised() -> bool`, the
read being `watch::Receiver::borrow` rather than an `async` wait — nothing ever
waits on this signal, `serve` samples it.

**Do the signature sweep with the compiler, not with a grep.** All three
signature changes are compile errors at every site: 69 `.frame()` calls, 36
`serve(..)` calls and 6 `Wire::new` calls, all measured. The `serve` figure is
the one to expect trouble from — it is thirteen sites in `ingress.rs` and
fourteen in `scheduling.rs`, neither of which this phase's Surfaces would
suggest carries the weight. What the compiler does *not* catch is
EX-7's ten prose sites, which is why they are enumerated rather than left to a
sweep.

`Wire` losing its window handle is a consequence the design did not state, and
it is forced rather than chosen: `design.md` §5.3 makes `Glass::present` the
only writer of the property, which leaves the field unread, and a never-read
private field is an error under the gate's `cargo clippy --workspace
--all-targets -- -D warnings`. Take it as an improvement — `Wire` afterwards
names no Slint type and no generated type at all — and not as licence to move
anything else. Its blast radius is bounded and enumerated: the field, the
`Wire::new` parameter, the `crate::generated` import at `wire.rs:17`, the
hand-written `Debug`, the doc paragraph explaining the weak handle, the six
`Wire::new` call sites, and EX-7's `glass.rs:36-38` cross-reference. Nothing
else.

This phase discharges no acceptance criterion. It is not thereby optional: §5.4
shows the defect it repairs, and A-4's whole mitigation is that a dropped edit
is "visible and self-correcting — the tick undoes itself and the notice, which
now survives the present that undoes it, says why".

---

## PHASE-02 — the window draws a form

**Objective:** the markup declares fields, a headless test can drive a checkbox
and address it by option, and the two Slint assumptions the design settled on
paper are pinned as regressions.

**Surfaces:** `crates/goad/build.rs`, `crates/goad/ui/app.slint`,
`crates/goad/src/glass.rs` (mechanically), `crates/goad/tests/renderer/tree.rs`.

**Entry**
- EN-1 — PHASE-01's exit criteria discharged; `just check` exits 0.

**Exit**
- EX-1 — `build.rs` selects the style as a **default**:
  `std::env::var("SLINT_STYLE").unwrap_or_else(|_| "material".into())`, passed to
  `with_style` on the same builder that keeps `with_debug_info(true)`. The
  explicit `var` read is load-bearing: `CompilerConfiguration::new()` already
  takes the variable and `with_style` overwrites it, so the one-line form
  silently removes an override that works today (D12, A-3).
- EX-2 — `app.slint` declares the three structs and the callback exactly as
  `design.md` §5.2 gives them: `FieldRow { option, id, label, checked }`,
  `FieldBlock { heading, fields }`, `OptionRow` **gaining** `blocks` and keeping
  its three members, and `callback edited(string, string, string, bool)` —
  view, option, field, checked.
- EX-3 — one container **per option**, inside the option's row and **guarded**
  — `if option.blocks.length > 0` — carrying `accessible-role: groupbox`,
  `accessible-description: option.id` and `accessible-label: option.label`; the
  `for` over `blocks` sits inside it; a heading drawn per block where
  `heading != ""`; a `CheckBox` per field binding `checked: field.checked`,
  carrying `accessible-description: field.id` and `enabled: !root.busy`, and
  firing `toggled => root.edited(option.view, option.id, field.id, self.checked)`.
  The role is not decoration — the compiler admits an accessibility property
  only on an element whose role is bound. The button's text stays
  `option.label`; the host authors no label (D14).
- EX-4 — an option with no fields has `blocks: []`, and **EX-3's guard** is
  what makes the container absent rather than empty: no container, no separator,
  no stray control (AC-6, and `slice-007.md`'s "no empty container" outright). A
  per-option container is not produced by the `for` over `blocks` and would
  otherwise exist whether `blocks` is empty or not; a container the `for`
  produces would be per-**block**, and would then carry `option.id` once per
  block. The guard is what reconciles the two, and it is stated here because
  neither criterion implies it.
- EX-5 — `glass.rs`'s `option_rows` (`:138-149`) and `tree.rs`'s `rows` builder
  (`:28-38`) compile again with a **mechanical** `blocks: ModelRc::default()`
  and nothing else. `OptionRow` is a generated struct built with an exhaustive
  literal, so a fourth member is `E0063` at both sites; that diff is not empty
  and cannot be (AC-6).
- EX-6 — `tree.rs` gains an **option-scoped** element query beside
  `element_described` (`:42-49`) rather than a second copy of it inlined at call
  sites: `match_predicate` carrying an owned `option.id`, then
  `match_descendants()`, then `match_predicate` carrying an owned `field.id`,
  then `find_first()`. `ElementQuery` has no accessible-description matcher, so
  the description is read through `ElementHandle::accessible_description` inside
  the predicate — the shape `element_described` already uses.
- EX-7 — **`element_described` itself is disambiguated**, because EX-3 gives the
  container the same `accessible-description: option.id` the option's `Button`
  already carries (`app.slint:51`). Two elements then answer to one description
  and `find_first()` takes whichever the walk reaches first, which no criterion
  pins. The helper gains a type filter — `match_inherits("Button")` before the
  predicate, the shape `tree.rs:81-83` already uses for `StyledText` — so it
  returns the control it is documented to return. This is a change to a
  **helper's implementation**, not to an assertion or a fixture (S-2), and it is
  what keeps `tree.rs:95-100` and `:124-130` green: the first asserts
  `accessible_item_index`, which the container does not declare, and the second
  invokes a default action, which a groupbox does not have. Two prose homes of
  the old uniqueness claim move with it and the compiler reaches neither:
  `tree.rs:40-41` (*"the only unambiguous one"*) and `app.slint:49-51` (*"the
  identity the tests select on, and the only unambiguous one"*). The **scoped**
  query of EX-6 is unaffected — `find_first` breaks only on a complete stack
  match, so a `Button` matching `option.id` with no matching descendant yields
  `Continue` and the walk reaches the container
  (`i-slint-backend-testing-1.17.1/search_api.rs:178-213`).
- EX-8 — `just check` exits 0.

**Verification**
- VT-1 — **A-1's pin.** A row carrying one block with two fields renders two
  checkboxes, each found by the option-scoped query. Its force is as much in
  compiling as in passing: `blocks` generates as `ModelRc<FieldBlock>`, and a
  future Slint that types an array member differently fails here rather than
  three phases later.
- VT-2 — **A-2's pin.** Find a checkbox, `invoke_accessible_default_action()` to
  assign `checked` imperatively, then replace the options model through the same
  `Rc<VecModel<OptionRow>>`-then-re-hand sequence `SlintGlass::present` uses
  (`glass.rs:87-90`), and assert the checkbox reads the **model's** value again.
  A test holding a fresh `ModelRc` per call does not exercise this; it must hold
  the `Rc<VecModel<_>>` and call `set_vec`.
- VT-3 — activating a checkbox fires `edited` with all four selectors: the view
  token, the option id, the field id, and the new `checked`.
- VT-4 — **AC-2's on-screen half.** With two blocks over five fields in declared
  order, collecting the option's field descriptions with `find_all()` — which
  returns matches in tree order, depth-first pre-order — yields the declared
  sequence. Not `accessible-item-index`: the markup declares none, and an index
  would assert a number the markup supplied rather than the order the tree is
  in. **`find_all()` is sound here despite `tree.rs:66-68`'s standing warning** —
  *"never `find_all().len()`, because the list virtualises"*. Virtualisation is
  the `ListView` path only: a plain repeater creates every instance
  (`i-slint-core-1.17.1/model/repeater.rs:143-154`, and `:156-158` scopes the
  lazy algorithm to a `ListView` viewport), and `app.slint:40-46` is a
  `ScrollView` around a `VerticalLayout` with a plain `for`. Correct
  `tree.rs:68`'s claim to say which markup it is true of; leaving it standing
  hands the next reader a contradiction.
- VT-5 — **AC-6.** A row with `blocks: []` adds no element: the option's button
  is present and the option-scoped query finds no groupbox and no checkbox under
  it.
- VA-1 — `git diff` over `tree.rs` shows the new cases, the scoped helper and
  the one mechanical `blocks:` member — no changed assertion in the five cases
  that exist today (AC-6, S-2).
- VA-2 — the style change is taken **first and alone**: apply `build.rs`, run
  `cargo test -p goad`, record it green, and only then write markup. That is
  R-7's named signal, and it is worthless once two changes are in the same diff.

**Notes for the implementer**

Both A-1 and A-2 are **settled from the pinned sources**, not open
(`design.md` §5.4, §5.5, §8). Neither is this phase's to discover, and neither
may be carried forward as a question. If either pin goes red, that is a finding
about a changed dependency, not a licence to redesign: `design.md` §8/R-1 keeps
the two-flat-models fallback named but unneeded, and S-1 is the road.

**Do not reach for `set_row_data` to keep keyboard focus.** It keeps focus and
silently detaches the checkbox from the model, which is the defect A-2 exists to
exclude (`design.md` §5.4). Focus loss on every present is a known, accepted
cost with a Follow-up in `slice-007.md`.

`element_described`'s unscoped `find_first` does **not** carry over to fields.
An option id is unique within a view (R-14), a field id only within an option
(R-52), and `crates/goad-semantics` has a fixture asserting two options may share
one. An unscoped query takes whichever comes first and reports no ambiguity —
which is exactly the case AC-4 exists to prove. Joining the two ids into one
description was rejected: ids are backend-supplied strings whose characters no
requirement constrains, so any separator can occur inside one.

Whether the container should also declare `accessible-role: list` with
`accessible-item-count`, so a reader announces a field's position, is **left open
deliberately** (`design.md` §5.2). Do not settle it by what a test happens to
need.

---

## PHASE-03 — the mapper and the draft

**Objective:** a canonical view becomes blocks of drawn fields with everything
else reported, and there is one pure place a widget's state becomes a submitted
value.

**Surfaces:** `crates/goad/src/{view_model.rs, draft.rs (new), diagnostics.rs,
lib.rs}`, `crates/goad/tests/renderer/{mapper.rs, reception.rs}`.

**Entry**
- EN-1 — PHASE-02's exit criteria discharged; `just check` exits 0. (PHASE-02 is
  not a code dependency of this phase; the ordering is the plan's, and the
  criterion is how the previous phase is proved done.)

**Exit**
- EX-1 — `draft.rs` exists, pure, naming no Slint type and no clock, with
  `Edited`, `Draft`, and `submitted` exactly as `design.md` §5.2 gives them.
  `Draft` is keyed by **(option id, field id)** over a `Vec` — not a `BTreeMap`,
  because `OptionId` carries no `Ord` and this host's storage is no reason to
  spend that distinction (S-4). Two methods, `state_of` and `record`, and
  **deliberately no way to enumerate what the draft holds**: that absence is what
  makes D6's walk a property of the type rather than a convention. No
  `PartialEq` — over a `Vec` it would be insertion-order sensitive; tests assert
  through `state_of`.
- EX-2 — `submitted(&Edited) -> serde_json::Value` is a **total match**, the
  single site where R-57 is applied — `pub(crate)`, because PHASE-04/EX-3 calls
  it from `controller.rs` while `slice-007.md` AC-8 names `draft.rs::submitted`
  as a SPEC-001 §7 vehicle a test must reach, and a `#[cfg(test)] mod tests`
  inside `draft.rs` reaches it either way. `design.md` §5.2 writes the signature
  bare; the visibility is what makes all three true. The doc-comment states what it does
  and does not guard: it catches the *host* growing a drawn kind without deciding
  what it submits; it does **not** catch the *protocol* growing a kind, because
  `Edited` is host-local and a sixth `FieldKind` leaves the match exhaustive.
  The site that breaks for that is `present()`'s mapper arm.
- EX-3 — `view_model.rs` grows `FieldBlock { heading: Option<String>, fields }`
  and `PresentationField { id, label }`; `PresentationOption` gains `blocks`.
  `Undrawn::OptionFields` is **removed** and replaced by
  `FieldForm { option, field, form }` and `GroupHint { option, field }`, with
  `FieldForm` naming the four kinds this renderer does not draw. Both new
  variants are excluded from `body_is_degraded`, as `OptionFields` already was
  (`view_model.rs:29-41`), and the doc-comment there naming `OptionFields` moves
  with them.
- EX-4 — grouping is by **runs over the drawn fields in declared order** (D8): a
  heading wherever the `group` value changes, no sorting and no merging. A
  non-string `group` is ungrouped, drawn in place, and reported `GroupHint`;
  `""` is an **untitled block** — `heading: None` — and is not reported. A block
  containing no drawn field is not emitted.
- EX-5 — `diagnostics.rs`'s `OptionFields` arm (`:191-199`) is replaced by the
  two lines `design.md` §5.2 states verbatim. No test asserts diagnostic wording
  in this project and none is added; the wording is stated in the design so it is
  reviewed once rather than discovered in a diff.
- EX-6 — `present()` stays pure, total and panic-free (I-1). It reads the `group`
  hint, which R-18 permits the renderer and only the renderer to do; `group` is
  read in `view_model.rs` and nowhere else. No host type or module is named for
  grouping — `FieldBlock` and `heading` are layout, not a concept (R-6).
- EX-7 — `lib.rs` declares `pub mod draft;`.
- EX-8 — `just check` exits 0.

**Verification**
- VT-1 — `Draft`, unit: absent means as-drawn (an unticked box); the same field
  id under two different options is two independent keys; `record` overwrites.
- VT-2 — **AC-8, R-57's `boolean` clause at its enforcement site.**
  `submitted(&Edited::Checked(true))` is `Value::Bool(true)`, and `false`
  likewise. This is the test SPEC-001 §7's new R-57 row will name.
- VT-3 — **AC-2's rule half**, over `present()`: a heading where the value
  changes; two `Morning` runs separated by an `Evening` produce two blocks, both
  titled; grouped fields separated only by an **undrawn** field produce one
  block; a group whose every member is undrawn produces no block and no heading;
  declared order is preserved throughout.
- VT-4 — **AC-3.** One `Undrawn::FieldForm` per undrawn field naming the option,
  the field and the kind, for each of `text`, `number`, `choice` and `datetime`;
  `GroupHint` for `"group": 7`; nothing reported for `"group": ""`;
  `body_is_degraded()` false with both present alongside a markdown body that
  parsed.
- VT-5 — existing `mapper.rs` coverage of the option path survives: a view whose
  options carry no fields maps as it does today.
- VT-6 — `tests/renderer/reception.rs`: a view carrying an undrawn field reaches
  the diagnostic surface through `receive`, on the precedent already in that
  file. `receive` remains the only path to a `Presentation` and cannot produce
  one without handing `undrawn` to `Diagnostics::of` (I-2).
- VA-1 — the total match in `submitted` is asserted by the **compiler**: adding a
  second `Edited` variant without a `submitted` arm must fail to build. Record
  that it was tried and reverted; that compile failure is the test
  (`design.md` §9).
- VA-2 — `mapper.rs:156-194`'s `an_option_with_fields_is_reported_undrawn_by_id_and_count`
  is the one existing case that must change, because the variant it names is
  gone. It is replaced by VT-4, not adjusted — and it is the **only** existing
  assertion this phase may touch (S-2).

**Notes for the implementer**

Everything the draft and the mapper need from `crates/goad-semantics` is already
`pub` and was verified accessor by accessor: `Field::{id,kind,label,hints}`,
`FieldKind`'s variants, `Fields::as_slice`, `Hints::as_map`, `FieldId: Ord + Clone`.
Id constructors are `pub(super)`, so the renderer clones an id and cannot mint
one. Nothing there changes (S-4).

**No new identifier in `crates/goad/src` may contain the word `resolve`** — no
`resolve_field`, no `resolve_draft`. This one fails the gate rather than review:
`crates/goad-boundary/tests/checks/structure.rs:306-314` matches by identifier
word over production lines.

`FieldForm` takes its name from SPEC-001 §6.2's own heading, *Field forms*, and
mirrors the existing `Undrawn::ContentForm { form: ContentForm }` exactly — same
shape, same `Display`-driven diagnostic line. Copy that pattern rather than
inventing a second.

**Reading a hint makes SPEC-001 §7's R-18 row stale, and that is
`canon-delta.md`'s to carry, not this phase's.** That row is *review, not a
test*, and its reasoning says `hints` is read in `src/` only by
`normalize.rs::normalize_field` and that "the renderer, the one component that
may, does not exist yet". Both are false after EX-6. The requirement is
unamended and the verdict is unchanged; only the evidence beneath it moves, and
it moves at audit. Do not edit `docs/specs/` here.

`Edited` carries `Checked(bool)` and one variant only. The seam exists so that
adding a kind later is a `submitted` arm rather than a reshaping of `Command`,
`Draft`, `install.rs` and every test that builds a command (D11) — it is not an
invitation to add the second variant now (S-5).

---

## PHASE-04 — the draft is retained, and the answer carries it

**Objective:** an edit travels from the window to retained state, the screen is
written back from it every present, and `answer()` submits a value for every
drawn field of the answered option and none for any other.

**Surfaces:** `crates/goad/src/{reception.rs, controller.rs, wire.rs, install.rs,
glass.rs, diagnostics.rs}`, `crates/goad/tests/renderer/{wiring.rs,
reception.rs}`. `diagnostics.rs` is here because `Refused` lives there and not
beside the controller — deliberately, and its own doc says why: *"it is a refusal
with a user-visible rendering, and every user-visible string in this renderer is
in one file"* (`diagnostics.rs:47-54`).

**Entry**
- EN-1 — PHASE-02 and PHASE-03's exit criteria discharged; `just check` exits 0.

**Exit**
- EX-1 — `Prepared` gains `draft: Draft`, defaulted in `receive`, which stays its
  only constructor — so a `Prepared` cannot exist without a draft. `absorb`'s
  three `Shift` arms need **no change at all**: `Replaced` installs a fresh
  `Prepared` with an empty draft, `Retained` leaves it, `Closed` drops it. A
  draft and the view it answers are one value, so `shown = None` cannot strand
  one (`design.md` §5.3).
- EX-2 — `Controller::edit(&mut self, view, option, field, value) -> Result<(), Refused>`
  makes the same two refusals `answer` already makes — `SupersededView`,
  `UnknownOption` — plus `UnknownField` when the option's blocks do not declare
  it, and records into the draft otherwise. `answer` stays `&self`; only `edit`
  needs `&mut self`.
- EX-3 — `answer()` fills `values` at `controller.rs:216` by walking the answered
  option's **blocks** and calling `submitted(draft.state_of(..))` for each
  declared field — never by walking the draft (D6). R-58 then holds in both
  directions with no check to forget, and a draft key that outlived its view
  cannot reach the wire.
- EX-4 — `Command::Edit { view, option, field, value: Edited }` in `wire.rs`,
  following `Command::Choose`'s shape: three opaque selectors matched against
  retained state and never parsed back into a value. `Eq` survives the derive.
  `dispatch` gains an arm returning `None` on success — there is nothing to
  exchange, so the loop continues to the top and presents — and
  `Some(Err(refused))` on refusal, which routes to the single existing refusal
  site (`controller.rs:669-684`). No new reporting path.
- EX-5 — `install.rs` grows a **seventh** installation with its own `Wire` clone,
  named `editing`, matching the file's one-clone-per-callback convention. The
  module's doc-comment says "six installations"; it moves with the change.
- EX-6 — `glass.rs`'s `option_rows` builds real blocks: one generated
  `FieldBlock` per presentation block **in declared order**, each block's
  `FieldRow`s in declared order, `heading: None` rendering as `""`, and each
  `FieldRow`'s `checked` read from `draft.state_of(option, field)` — a lookup,
  never stored in the row model as truth. **`FieldBlock` names two types here**
  — `design.md` §5.2 gives the Slint struct and the `view_model.rs` struct the
  same name, and `glass.rs` is the one file that holds both. Generated types keep
  their bare names, as `OptionRow` does today (`glass.rs:14`), and the mapper's
  is path-qualified `view_model::FieldBlock` at its use sites. Do not rename
  either type; the design names both. `present` stays total: every property, every call,
  including every `FieldRow`, with no exception for the field model (I-5, S-9).
- EX-7 — `Presentation` and `Draft` remain separate types in separate files and
  the draft does not enter `Presentation` (I-4). Nothing in the gate reaches
  this; it is stated here so review has something to match against (S-6).
- EX-8 — `Refused::UnknownField` is added to `diagnostics.rs`'s enum with a doc
  in the shape its siblings use, and `Diagnostics::refused` (`:148-163`) gains
  its arm rendering the line `design.md` §5.2 states verbatim. That match is
  exhaustive with no `_`, so the variant cannot be added without the line — which
  is the design's reason for `Refused` living in that file at all.
- EX-9 — `tests/renderer/reception.rs`'s
  `every_refused_variant_renders_one_line_with_the_failure_prefix` (`:624-652`)
  gains a fourth case. Its name claims every variant; extending it keeps that
  claim rather than altering an assertion, and S-2 names this as its third
  allowance. **It is not in AC-6's scope** — `design.md` §9 scopes AC-6's "no
  change to what they assert" to the option tests, `tree.rs`/`table.rs`/`wiring.rs`.
- EX-10 — `just check` exits 0.

**Verification**
- VT-1 — `edit`'s refusals, in `wiring.rs`'s existing `mod interaction` shape: a
  stale `view` token is `SupersededView`; nothing retained at all is
  `SupersededView`; an unknown option is `UnknownOption`; a field the option does
  not declare is `UnknownField`. Nothing is recorded in any of the four.
- VT-2 — an edit recorded, then `answer()` over that option: `values` carries a
  key for **every** drawn field, the edited one `true` and the untouched ones
  `false`. R-35 and P-3 in one case — a half-filled form goes out as it stands
  and the host fills no gap on the person's behalf.
- VT-3 — **AC-8, R-58's vehicle.** A **two-option** view whose options each carry
  fields and **share a field id**: `answer(view, "morning")` names `morning` and
  carries exactly `morning`'s drawn field keys — none of `evening`'s, and none
  for a field reported undrawn. This is the test SPEC-001 §7's new R-58 row will
  name.
- VT-4 — `dispatch(Command::Edit { .. })` returns `None` on success and
  `Some(Err(..))` on refusal; a refused edit reports through the existing
  refusal site and `refusal_re_arms` is `false` for it by construction, exactly
  as for `Choose`.
- VT-5 — the screen is written from the draft: after an `edit` and a `present`,
  the window's `FieldRow` for that field reads `checked`. Asserted on the row
  model or through the option-scoped query; either is the screen, not the draft.
- VT-6 — `Refused::UnknownField` renders its line, asserted by its exact string
  beside the three variants already there (EX-9).
- VT-7 — **AC-2's middle link**, over `option_rows` with no window: a
  `Presentation` carrying two blocks — one titled, one `heading: None` — over
  five fields in declared order yields row-model blocks in that order, each
  block's fields in that order, and `""` for the untitled heading. Without this,
  `View → present()` is verified by PHASE-03/VT-3 and `row model → screen` by
  PHASE-02/VT-4, and the link between them — the only host code that puts a
  backend's declared order and its headings on the screen — is verified by
  nothing.
- VA-1 — **AC-6.** `git diff` over `table.rs`, `wiring.rs`, `scheduling.rs` and
  `ingress.rs` shows no changed assertion and no changed fixture. `Prepared`
  gaining a defaulted field must be invisible to every existing case; if it is
  not, that is S-2. `reception.rs` is **excluded** from this check by EX-9,
  which extends one case there by design.
- VA-2 — `answer()` is read against D6 by eye: the walk is over
  `presentation.blocks` for the matched option, and `Draft` exposes no iterator
  for anyone to reach for. Confirm `Draft` still offers no key enumeration.

**Notes for the implementer**

The structural claim is the whole point and is worth re-reading before writing a
line: `answer()` walks what was **drawn**, so a value is submitted for every
drawn field of the answered option, no value is submitted for a field that was
not drawn, and a stale draft key cannot reach the wire — three properties with no
check anywhere. Combined with the `SupersededView` refusal `answer()` already
makes, every submitted key provably came from a field the currently-retained view
declared.

`wire.rs` depends on `draft.rs` and not the reverse: a value's meaning belongs
with what stores it, not with what carries it.

Two edits cannot race an exchange. `enabled: !root.busy` already disables the
option buttons during a call and PHASE-02 gives the checkboxes the same binding,
so the window is inert exactly while `serve`'s outer loop is not reading
commands. SPEC-002/R-9 is untouched; do not add a guard for it.

An edit arriving just after a new view landed is refused by `edit`'s own
`SupersededView` check rather than applied to the wrong draft. That is the
existing path, not a new one.

---

## PHASE-05 — the form on the wire

**Objective:** what the person ticked is shown to leave the host as JSON, under
the option they pressed, surviving a present that changed nothing.

**Surfaces:** `crates/goad/tests/renderer/fields.rs` (new),
`crates/goad/tests/renderer/main.rs` (one `mod` line and the module roll-call in
its doc), `crates/goad/tests/renderer/harness.rs`,
`crates/goad/tests/renderer/scheduling.rs` (a helper lift only).

**Entry**
- EN-1 — PHASE-04's exit criteria discharged; `just check` exits 0.

**Exit**
- EX-1 — `fields.rs` drives the **production `serve`** against a real child
  process, as every other case in this target does. There is no second loop and
  no test-only harness for it.
- EX-2 — `logging_scripted` moves from `scheduling.rs:101-109` to `harness.rs`,
  which is the stated home for what two or more modules in this target need. Its
  body does not change — it calls `logging_backend` from the shared
  `crate::scripting`, not from `scheduling.rs`, so it compiles unchanged in its
  new home — its callers in `scheduling.rs` follow the import, and **no
  assertion in `scheduling.rs` changes** (S-2). Duplicating it into `fields.rs`
  instead is the thing to avoid. `harness.rs:1-8`'s module doc **enumerates its
  consumers** — *"anything two or more of `wiring.rs`/`table.rs`/`scheduling.rs`
  need lives here"* — and `fields.rs` is a fourth the list does not admit; it
  moves with the lift, exactly as `renderer/main.rs`'s roll-call does in
  Surfaces.
- EX-3 — `fields.rs` carries a reader for the submitted values — the nth logged
  request's `["response"]["values"]` — beside `scheduling.rs`'s `request_kind`
  precedent. It stays in `fields.rs`: one consumer, one home.
- EX-4 — every case in the file either reads the invocation log or asserts
  something about the screen; VT-3 does both. No case reads the draft through
  `Controller` and stops there (S-7).
- EX-5 — `just check` exits 0.

**Verification**
- VT-1 — **AC-1.** A view with one option carrying three `boolean` fields draws
  three checkboxes and one button. Tick two through
  `invoke_accessible_default_action`, press the button, and the logged `respond`
  carries `values` with **exactly three keys**, each a JSON boolean matching what
  was on screen.
- VT-2 — **AC-4.** Two options, each carrying fields, **sharing a field id**.
  Tick the first option's box through the option-scoped query, press the first
  option's button: the request names that option and carries only that option's
  keys. The shared id is the point — with an unscoped selector this case is green
  both where the design is right and where the draft key's `option` half is
  ignored.
- VT-3 — **AC-5, both halves, and both required.** Tick a box; take a
  `Shift::Retained` fold (an `evaluate` answering `{"view":null}`); assert the
  box is **still ticked on screen**; then submit and assert the value is still
  `true` on the wire. The wire half alone stays green while the screen is wrong,
  because the wire value is built from the draft and no present writes the draft;
  the screen half alone stays green if the draft is dropped on `Retained`. A
  disjunction is the right rule for every other case in this file and the wrong
  one for this one.
- VT-4 — **AC-3's remaining half.** An option carrying one `boolean` and one
  `text` field: the view is still shown, the option still answers, the `respond`
  carries the boolean key and **no key for the text field** (R-55, R-58), and the
  undrawn report is on the diagnostic surface.
- VA-1 — each negative case has been seen to fail for its own reason: inject the
  defect it guards, run, read the message, revert
  (`docs/memory/a-green-test-can-assert-a-proxy.md`). VT-2's injection is the
  load-bearing one — ignore the draft key's option half and VT-2 must go red
  while VT-1 stays green.

**Notes for the implementer**

`logging_scripted` runs `tests/backends/logs-the-request-then-answers.sh`, which
writes each raw request to a log path passed as argv[2] — so a `respond`'s
`response.values` is readable straight off the log. `answers-as-instructed.sh`
never reads its own stdin and cannot report what it received.

**The invocation log proves an exchange *began*, not that it was absorbed.**
The script writes its line before it *answers* — `request="$(cat)"` at
`tests/backends/logs-the-request-then-answers.sh:19`, the log append at `:26`,
the answer at `:32` — and long before the host folds the outcome in. Where a
case needs the exchange to have been folded, poll for an observable the
production glass wrote; `scheduling.rs`'s own `next_check`-line helper is the
precedent. **`scheduling.rs:127-142` states this backwards** — "written by the
backend script *before* it reads the request" — and the script does the
opposite. Its conclusion still holds for the reason above. Correcting that
comment is not this phase's job; record it in `notes.md` Findings for the audit.

Use `harness::until` with `TIMEOUT` rather than a fixed sleep for anything the
driving code does not control directly.

The proxy rule is this phase's whole reason for existing: a test that reads the
draft through `Controller` and never reads an invocation log would pass with
`answer()` walking the draft's keys, which is D6 — the defect most worth
catching.

---

## PHASE-06 — the demo, and the look

**Objective:** a person fills a real multi-field form against a real backend,
sees every answer recorded from one exchange, and says whether the form is
legible.

**Surfaces:** `examples/shell/backend.sh`, `crates/goad/ui/app.slint` (bounded
by AC-10 — the block container and its separator, and the heading's treatment,
and nothing else), `docs/slices/007/audit.md` (Evidence),
`docs/slices/007/notes.md` (Findings, Harvest).

**Entry**
- EN-1 — PHASE-05's exit criteria discharged; `just check` exits 0.
- EN-2 — a display. This phase runs **outside the jail**; there is none in it.

**Exit**
- EX-1 — `examples/shell/backend.sh` sends a form: one option carrying several
  `boolean` fields under **at least two `group` values**, so a heading change is
  visible, and **at least one field of a kind this renderer does not draw**. That
  last clause is not decoration — it is what makes the artefact a backend author
  copies a *protocol-shaped* form rather than this renderer's subset, and it is
  R-55's "or produce the effect of" clause, which no other artefact in this slice
  discharges.
- EX-2 — the same script **records the submitted `values`** where a person can
  read them: appended to a file, or written to stderr, which the host reports on
  the diagnostics surface. Today it discards them and answers `view: null`, so
  AC-7's "the record shows every answer" has no vehicle without this.
- EX-3 — the script keeps its existing discipline: **no value the host carries
  opaquely reaches its control flow**, and any watcher-authored string it
  interpolates is escaped. Its header states that rule; a new branch must not
  quietly break it.
- EX-4 — AC-10's feedback is split at the moment it is given: what drawing fields
  forces is actioned here; everything else — typography, window sizing, the idle
  surface, the look of the controls — is recorded **verbatim and not actioned** in
  `notes.md`, and becomes 008's brief (S-8).
- EX-5 — `audit.md`'s Evidence carries AC-7 and AC-10 in the person's own
  account, naming what was observed. A green gate is not this evidence
  (`docs/AGENTS.md` §Tiers; slices 001–003 all closed green on a binary that
  could not open a window).
- EX-6 — `notes.md`'s Harvest is current: what exists, what was learned, what is
  open — including ADR-004's connection to the deferred hold, so the follow-up
  re-derives nothing.
- EX-7 — `just check` exits 0.

**Verification**
- VH-1 — **AC-7.** A person runs `just demo`, fills the multi-field form,
  submits **once**, and the record shows every answer from that one exchange.
  The undrawn field is visible doing its job on the diagnostic surface, which is
  also AC-3's human half.
- VH-2 — **AC-10.** A person runs the slice's output and iterates with the
  implementer until the form is legible: which fields belong to which option, and
  which heading covers which fields, both apparent without reading the protocol.
  The bound is on what may be *changed*, never on what may be *said*.
- VA-1 — **AC-9.** `just check` exits 0; the domain-vocabulary scan and the four
  ADR-001 instruments pass. Then the two things a green gate does **not** say:
  no host type or module is named for grouping — `group` is not on the scanned
  word list and never will be — and stratum 3's purity, `view_model.rs` staying
  pure and `Draft` never entering `Presentation`, is held by **review alone**
  (POL-001 §Verification, `design.md` §3). Do not write a criterion that implies
  an instrument checks either.
- VA-2 — every acceptance criterion in `slice-007.md` and every verification
  criterion above has a named passing test or a named argument, and the surfaces
  each phase declared are diffed against the paths actually touched. Undeclared
  paths are the strongest lead the audit has; leave none.

**Notes for the implementer**

Three of this slice's residues belong to **audit**, not here, and this phase
must leave them reachable rather than close over them. `canon-delta.md` is
promoted into SPEC-001 as R-57 and R-58 with a §7 row each, or abandoned in
writing (AC-8). R-57's text as drafted carries two `and`s — "…`choice` … as a
JSON string, **and** `datetime` an RFC 3339 `date-time` string" — a copy-edit
worth making before it is promoted verbatim. And SPEC-001 §7's **existing R-18
row** states that `hints` is read in `src/` only by `normalize.rs` and that the
renderer "does not exist yet"; PHASE-03 falsifies both, and `canon-delta.md`
CD-1 now carries the amendment.

The demo backend is `bash`, reads one JSON document on stdin and writes one on
stdout, and is deliberately readable without a parser. Match its existing style;
it is documentation as much as it is a fixture.

`just demo` runs on `examples/demo.toml`, whose socket lives in the checkout. If
`nix develop` is involved, use the bare git-input flake reference — a `path:` ref
breaks on the demo socket (`docs/memory/path-flake-ref-breaks-on-demo-socket.md`).

The keyboard-focus cost is known and accepted: every present rebuilds the rows,
so the focus ring is dropped once per tick and the form is filled by mouse. It is
not a defect to repair here (`slice-007.md` Follow-ups).

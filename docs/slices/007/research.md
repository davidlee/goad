# Research — Slice 007

**Producers:** `deepseek-flash`, one agent, both threads, per `research-brief.md`
**As of:** 2026-09-14 · `ab9ba3a` (goad); pinned Slint sources read from the
`v1.17.1` tag of the checkout at `/workspace/slint` (`88c5e6a32` on `master`)

**Environment note, first, because the brief's paths do not hold here.** This
session runs inside the bwrap jail (`jpi`, `flake.nix` §`jailEnvOptions`). The
brief's three path assumptions are all false inside it, and each was checked:

- `/home/david/dev/goad` does not exist; this repository is `/workspace/goad`
  (`git rev-parse --short HEAD` → `ab9ba3a`, matching the brief's "at the time of
  writing").
- `~/.cargo/registry/src/*/i-slint-compiler-1.17.1/` does not exist. The registry
  under `~/.cargo/registry/src/` holds no `slint` crate at all — its newest
  entries are from a different project. The only Slint source visible is the
  read-only bind `flake.nix` makes: `/home/david/.local/src/slint` →
  `/workspace/slint`.
- `~/.local/src/slint` is not a jail path; it is `/workspace/slint`, and it is at
  `88c5e6a32`, workspace version `1.18.0` — the drift the brief warns about,
  confirmed (`git describe --tags` → `v1.17.0-1435-g88c5e6a32`).

The pinned sources were reached anyway, without the drifted tree: `/workspace/slint`
is a **full clone** with the `v1.17.1` tag (`cf62c975c311e7036d599ed8ed0b7e6a8386a934`),
so every widget and testing-backend claim below is read with
`git show v1.17.1:<path>`. That is the tagged release, not the drifted working
tree, so the API facts are the pinned ones. They are cited as
`i-slint-compiler-1.17.1/<path>` and `i-slint-backend-testing-1.17.1/<path>` —
the names the consumer will find in a normal (unjailed) registry.

**There is no display in the jail** (`DISPLAY`, `WAYLAND_DISPLAY`, `XDG_RUNTIME_DIR`
all empty). Nothing about AC-7 can be observed from here, by anyone.

## Verification legend

- ✓ — independently verified by the *consuming* agent (a read or grep of the
  cited site).
- unmarked — researcher claim: cited, not checked.

Design and plan may only load-bear ✓ rows, or rows they verify at point of use.
Verify what you lean on, not everything.

Per the brief, **nothing below is marked ✓**; that column is the consuming
agent's. Every cited site in this file was opened by the researcher — the
unmarked rows are "a claim with its site named", not "a claim from memory".

## Citation forms

Canon claims cite the document id (`SPEC-003 §4`, `ADR-007`). Code claims cite
`path:line`. An uncited claim is unverifiable by definition.

## Thread 1 — governing canon

### Binding

**The response shape.** SPEC-001/R-8: a `respond` MUST carry "the `view_id` being
answered, the host's current instant, the chosen option id, and a map of field id
to submitted value". SPEC-001/R-8 is the only requirement that shapes a
submission; there is no R-N after it that names a value's JSON type.

**Opacity, and what it covers.** SPEC-001/R-9, verbatim: *"The host MUST NOT
interpret an event's data payload or a submitted field value. It carries both
verbatim."* §6.3 restates the same list from the ownership side: *"It uses,
without interpreting: event payloads, submitted values, hint keys and values, and
URI content."* §7's R-9 row is explicit that the requirement is one-directional:
*"the requirement is that host code never **reads** a payload, and a test can
only observe code that does."*

**Fields on an option.** SPEC-001/R-15: *"An option MAY carry fields. Each field
MUST carry an id, a kind and a label. Any key on the field object that this spec
does not name is a hint."* §6.2 *Field forms* names the field object's own keys —
`id`, `kind`, `label`, and where the kind calls for them `min`, `max`, `options`
— and states *"Every other key is a hint"* (§6.2, line 289-291). §6.2 also states
the asymmetry deliberately: a misspelled optional key becomes a hint; a
misspelled required key is an error.

**The five kinds, and their parameterisation.** SPEC-001/R-16: kinds are `text`,
`boolean`, `datetime`, `number` and `choice`; `number` MAY carry `min` and `max`;
a `choice` field MUST carry its own `options`; the other three carry no additional
protocol keys. R-17 bounds `number`: each bound finite, `min <= max`, else the
message is rejected. R-50 refuses a spec-named key on the wrong kind. R-52 requires
field ids unique within an option — because `values` is keyed by field id (R-8).
R-53 makes a `choice` field's option an **alternative** id, a value namespace
disjoint from an option id, and refuses `fields` on one.

**Hints are renderer-only.** SPEC-001/R-18: hints are an open flat map; *"Only the
renderer MAY branch on a hint key; nothing that normalizes, schedules, transports,
or manages interaction state may."* A nested `hints` object is refused; a nulled
one is omission (R-51). This is what lets `group` and `multiline` be read by the
renderer and by nothing else.

**The renderer subset rule, binding on this slice.** SPEC-001/R-55: *"A renderer
MUST NOT refuse a whole view because it cannot draw part of it. It draws what it
can and reports what it did not draw. A capability the protocol admits and a
renderer does not implement is a renderer subset, and MUST NOT be treated as, or
produce the effect of, a narrowing of the protocol."* §2's boundary sentence is
the one 007 rests on: *"Drawing stays out of scope; what a renderer owes the
protocol having received a view does not (R-55)."* R-55's §7 row cites the only
two tests that hold it today — `mapper.rs::rejected_markdown_degrades_to_plain_and_is_reported_undrawn`
and `reception.rs::a_view_with_rejected_markdown_yields_unclear_diagnostics_and_a_prepared_presentation`
— and names `Undrawn::OptionFields` as the mechanism through which *"option
fields, HTML and URI content are admitted and undrawn the same way."*

**Validation is the backend's.** SPEC-001/R-35: *"The host MUST NOT validate a
submitted answer beyond its `view_id`. Whether an answer is acceptable is the
backend's judgement."* §5's *Ambiguity is failure* and §3 P-B constrain what the
host may do with what it does not understand.

**One-way strata.** ADR-001 §Decision: three strata; a stratum may depend on a
lower-numbered one and never on a higher-numbered one; stratum 1 is pure (no I/O,
no async runtime). ADR-003 §Decision places `crates/goad-semantics` at stratum 1,
`crates/goad-shell` at stratum 2, `crates/goad` (the Slint renderer and host
binary) and `crates/goad-emit` at stratum 3, and `crates/goad-boundary` at no
stratum. `crates/goad/src/controller.rs`, `src/view_model.rs` and `src/glass.rs`
are therefore all **stratum 3**, and stratum 3 may name strata 1 and 2 and a
renderer dependency — but is itself held by nothing but review (ADR-003
§Consequences *Negative*; POL-001 §Verification, below).

**The four ADR-001 instruments, the vocabulary scan, and the residue.** POL-001
§Verification is the canonical statement, and its boundaries are load-bearing here:

| what is held | by what | what it does not reach |
|---|---|---|
| ADR-001's direction rule at **crate edges** | Cargo resolution | anything not expressed as a crate edge |
| a runtime-, renderer- or filesystem-shaped **dependency entry** in a stratum 1 or 2 manifest | the manifest allowlist test in `crates/goad-boundary` | versions, features, and what a permitted dependency does |
| a direct `std` reach for the filesystem, processes, sockets, threads, the environment or a clock, **in stratum 1's sources** | the stratum 1 purity scan in `crates/goad-boundary` | aliased or brace-grouped imports, and I/O performed on stratum 1's behalf by a permitted dependency |
| stratum 1 built with its **own** feature set | `cargo test -p goad-semantics` | which features those are — it rejects nothing |

plus the **domain-vocabulary scan** (a different invariant, over every member),
plus **one residue nothing enforces**: a feature switched on in a shared
dependency by stratum 2 or 3 unifies into stratum 1's build under `--workspace`.
POL-001 also states the consequence for this slice directly: *"no stratum-3 member
carries a manifest allowlist row at all, because POL-001 §Verification scopes that
instrument to 'a stratum 1 or 2 manifest'. The real residue is narrower and
sharper: **a stratum-3 manifest is checked by nothing but review**"* (ADR-003
§Consequences *Negative*).

**The domain-vocabulary scan, exactly.** `crates/goad-boundary/tests/checks/vocabulary.rs:18-26`
holds the seven-word list: `habit`, `streak`, `journal`, `site`, `goal`,
`reminder`, `compliance`. The walk (`crates/goad-boundary/src/scan.rs`,
`Scan::run`/`walk`/`inspect`, `crates/goad-boundary/tests/checks/vocabulary.rs:21-27`)
covers `.rs` and `.slint`, over every directory `workspace.members` names, with
`tests` and `target` excluded by directory *name* anywhere in the walk. Matching
is by word, case-insensitive, plural-tolerant, with camel-case segmentation
(`scan.rs::mentions`, `camel_segments`, `is_singular_or_plural_of`); comments are
stripped, **string literals are kept** (`scan.rs::code_of` → `strip(.., Literals::Kept)`).
The vacuity guard fails a walk that inspected no file. Consequences for design:

- `group` is not on the list and is not a substring of any listed word. Safe.
- `examples/` and `tests/` are outside the scan (the former is not a member; the
  latter is excluded by name), so a backend or fixture may say anything.
- A **string** in `crates/goad/src/*.rs` or `crates/goad/ui/*.slint` naming a listed
  word is caught (the fixture `vocabulary/component_name.slint` and
  `accessible_label.slint` prove it). Renderer-authored prose and markup are in
  scope; backend-supplied text is data and is not.

**The gate.** POL-001 §Compliance is six commands; `just check` mirrors them.
`crates/goad/build.rs`'s style line and everything 007 adds are held by that gate
plus review. The vocabulary scan's passing is *necessary and not sufficient* for
"`group` never becomes a host concept" — that is the slice card's own AC-9 wording
(`slice-007.md`, AC-9), and it is correct.

### Checked, not applicable

| authority | why it is not applicable |
|---|---|
| **SPEC-003 (host event ingress)** | The slice's surfaces (`slice-007.md` §Scope) add no stimulus, no envelope, no listener change, and no `evaluate` kind. `serve`'s ingress arms are untouched; the second anchor's one write site is untouched. SPEC-003/R-11 and R-13 (reserved source) are not reached. |
| **ADR-005 (the event envelope normalizes in stratum 2)** | Its entire subject is the ingress envelope's permissive type and its normalization site. This slice adds nothing to ingress. |
| **ADR-004 (scheduled firings are spaced from the previous scheduled firing)** | **Not applicable to the slice's changes, and applicable to OQ-3's design question.** The code changes touch no anchor and no firing. But OQ-3 is "a scheduled firing lands while the form is half filled", and SPEC-002/OQ-4 asks whether a host should suppress or defer a firing while a presentation is outstanding — a question about the firing policy ADR-004 decided. Listing ADR-004 as simply "not applicable" (`slice-007.md` §Governing canon, last paragraph) is true of the diff and false of the question OQ-3 keeps on the record. See *Design-input deltas*. |
| **ADR-002 (single crate until triggered)** | Superseded by ADR-003. The slice card does not list it; it should not be cited for anything. |
| **SPEC-001/R-17, R-50, R-52, R-53** | Not amended, and not touched — but they bind the *view* side this slice renders: bounds are finite and ordered (R-17), a named key on the wrong kind is refused (R-50), field ids are unique within an option (R-52), alternatives are not options (R-53). The renderer reads the already-normalized canonical types and must not re-derive any of these. |

### Amendment candidates

**CD-1 itself. Confirmed real, with two corrections.** See *Cross-thread findings*
F1 and F2.

**A second candidate: R-9 may need one clause about authorship.** R-9 forbids
*interpreting* a submitted value and says the host "carries both verbatim". Once
the host *authors* a value from a widget, a strict reader can ask whether choosing
`true` rather than `"true"` is an interpretation. It is not — interpretation is
reading, and §6.3 says the host "uses, without interpreting" a submitted value —
but R-9 says nothing about the write direction, and CD-1's type rule is what fills
that gap. If design wants the gap closed in R-9 as well as R-57, the minimal
wording is a single sentence distinguishing *carrying a value it received* from
*authors the value a widget produced*. Recommend folding it into CD-1's text
rather than a second delta entry: one document, one section change.

**CD-1 as one requirement or two.** See F2. In short: R-57 as drafted mixes a
**type** rule (`kind → JSON type`), which is a statement about one value, with a
**completeness** rule (a value for every field drawn; nothing for a field not
drawn), which is a statement about the shape of the map. §4's style is one
falsifiable claim per row, and R-52/R-53 are the precedent for splitting two rules
that travel together. Recommend **R-57 = type by kind**, **R-58 = completeness**.

**Verification cost.** Any new §4 row needs a §7 row. §7's conventions (fixtures
under `tests/fixtures/{protocol,protocol-text,schedule}/`, run by the three named
walkers) are stated in §7's preamble. R-57 is **outbound**, so its fixtures are
unit tests in `canonical.rs` beside `a_respond_serializes_to_the_spec_s_wire_form`
(`crates/goad-semantics/src/protocol/canonical.rs:752-771`), not the inbound
fixture corpora — the same way R-1/R-2/R-6/R-8 are verified. A per-kind fixture
"per drawn kind" (AC-8's wording) means one unit test per kind that submits that
kind's value and asserts the serialized JSON type.

## Thread 2 — code map

### Hotspots

| path | why |
|---|---|
| `crates/goad/ui/app.slint` | field rendering, per-option grouping, the submit path, the edit callback. 110 lines today. |
| `crates/goad/src/view_model.rs` | `Presentation` grows fields; `Undrawn` narrows. 165 lines. |
| `crates/goad/src/reception.rs` | `Prepared`'s home; `receive` is `Prepared`'s only constructor. 89 lines. |
| `crates/goad/src/controller.rs` | the draft and `answer()`; `serve`'s present sites. 848 lines. |
| `crates/goad/src/glass.rs` | a second model beside `options`; `present`'s totality. 149 lines. |
| `crates/goad/src/install.rs` | the callback table — a new edit callback is wired here. 47 lines. |
| `crates/goad/src/diagnostics.rs` | the narrowed undrawn line. 501 lines. |
| `crates/goad/build.rs` | the style selection (OQ-5). 10 lines. |
| `crates/goad/tests/renderer/mapper.rs` | the one test `Undrawn::OptionFields` narrowing breaks. |
| `examples/shell/backend.sh`, `examples/typescript/backend.ts`, `examples/demo.toml` | AC-7's vehicle. |

### Cited facts

**The draft's home — `Prepared` and `Controller`.**

- `Prepared` has exactly two fields, `pub view_id: ViewId` and
  `pub presentation: Presentation` (`crates/goad/src/reception.rs:25-28`). It is
  constructed in exactly one place: `receive`, at `reception.rs:52-62`, from
  `Presented { view_id, view }` inside an `Outcome`. `receive` has no other caller
  and is documented as the only destructuring site (`reception.rs:44-45`).
- `Controller`'s retained state (`crates/goad/src/controller.rs:108-124`):
  `shown: Option<Prepared>`, `diagnostics: Diagnostics`, `focus: Focus`,
  `engaged: bool`, `next_check: Option<Timestamp>`. The doc comment calls this
  *"the complete retained state"* and says it holds no Slint types.
- `absorb` (`controller.rs:160-186`) is the only writer of `shown` and `focus`:
  - `Shift::Replaced` → `self.shown = prepared; self.focus = Focus::Automatic;`
  - `Shift::Closed` → `self.shown = None;`
  - `Shift::Retained` → `{}` (both untouched)
  and then unconditionally `self.diagnostics = diagnostics; self.engaged = false;
  self.next_check = Some(next_check);`.
- `reduce` (`controller.rs:258-272`) is a total match on
  `(exchanged, has_view, refused)`, eight combinations, no `_` and no
  `unreachable!()`. `Shift::Replaced` iff `has_view`; `Shift::Retained` for the
  failed/empty cases except `(Answer, false, false)` → `Closed`.
- **Consequence for a draft inside `Prepared`.** Every `Replaced` fold installs a
  `Prepared` freshly built by `receive`, so a draft stored as a `Prepared` field
  is discarded on every replacement — correct, because a returned view takes a
  fresh `view_id` and the old interaction becomes stale (SPEC-001/R-33). A
  `Retained` fold leaves `shown` (and therefore the draft) alone, which is exactly
  the "same question, nothing further to show" case. A `Closed` fold drops it.
  Nothing in the fold makes a draft field impossible; the only new work is that
  `Prepared` gains a field `receive` must initialise, and a `&mut self` edit entry
  point must exist (today `answer` is `&self`, `controller.rs:200`).
- `answer` (`controller.rs:200-220`) is `&self`, makes exactly two refusals —
  `Refused::SupersededView` when nothing is retained or `view` mismatches, and
  `Refused::UnknownOption` when the option is not in the retained presentation —
  and constructs `values: BTreeMap::new()` at `controller.rs:216`. This is the
  only construction of a `UserResponse` in `crates/goad/src` (grep). Assembling
  `values` from the draft keeps the existing `SupersededView` check as the guard
  that every submitted key came from the retained view — the point OQ-1's
  recommendation rests on.
- The response is consumed by `dispatch`'s `Command::Choose` arm
  (`controller.rs:550-560`) → `Pending::Respond { now, view_id, answer }`
  (`controller.rs:308-314`) → `serve`'s `host.respond(now, view_id, answer)`
  (`controller.rs:705`). `UserResponse` is `canonical.rs`'s outbound type, with
  `pub values: BTreeMap<FieldId, serde_json::Value>` (`crates/goad-semantics/src/protocol/canonical.rs:499-503`).

**The present path, end to end.**

- `serve` presents at exactly three sites, all `glass.present(controller.frame())`:
  the top of the outer loop with `busy = false` (`controller.rs:611`), once
  `engage()` has run with `busy = true` (`controller.rs:692`), and the
  ingress-stopped branch inside the inner `select!` (`controller.rs:744`). The
  top-of-loop call therefore follows every `absorb` on the next iteration.
- `Controller::frame` (`controller.rs:239-247`) returns `Frame { surface, shown:
  self.shown.as_ref(), diagnostics, busy, next_check }`; `Frame<'a>` is `Debug,
  Clone, Copy` and borrows (`controller.rs:94-102`). Adding a draft to `Prepared`
  does not change `Frame`'s shape.
- `Glass` is a one-method trait: `fn present(&mut self, frame: Frame<'_>)`,
  documented *"Total and idempotent"* and *"Write **every** property from the
  frame, then show the window"* (`crates/goad/src/glass.rs:20-27`).
- `SlintGlass` holds `window`, `tray`, and `options: Rc<VecModel<OptionRow>>`
  (`glass.rs:30-34`). `present` (`glass.rs:72-119`) writes every window property,
  and **replaces the whole options model**: `self.options.set_vec(options);` then
  `self.window.set_options(ModelRc::from(Rc::clone(&self.options)));`
  (`glass.rs:103-108`). `option_rows` (`glass.rs:148-164`) rebuilds every
  `OptionRow` from `prepared` on each call. There is no write-only-on-change path,
  and no property survives a `present` other than the two models and the tray.
- **This is AC-5's mechanism.** Because `present` rewrites the model wholesale,
  any field state held only in the widget is lost on the next present; the draft
  must be written back from the controller each time. A second `VecModel` beside
  `options` (`slice-007.md` §Scope) is the natural place, and it must be filled
  from `frame.shown`'s draft on every call, exactly as `options` is.
- `main` creates the `VecModel` once (`crates/goad/src/main.rs:98-102`) and it
  lives for the process.

**The command path.**

- `Command` (`crates/goad/src/wire.rs:29-41`): `Evaluate(Stimulus)`,
  `Choose { view: String, option: String }`, `OpenDiagnostics`, `CloseDiagnostics`;
  derives `Debug, Clone, PartialEq, Eq`. The two `Choose` strings are opaque
  selectors, never parsed back into a value. `Stimulus` (`wire.rs:47-70`) is
  `Copy`, has the three `Event.kind` strings, and hard-codes `source: "host"`.
- The channel is `mpsc::channel::<Command>(1)` — **capacity one** — at
  `crates/goad/src/main.rs:86`.
- `Wire::send` (`wire.rs:115-137`) is `try_send`, never `send().await`, because a
  Slint callback is synchronous on the UI thread. `Full` writes
  `diagnostics::BUSY_NOTICE` to `notice` through the weak window handle and the
  command is **dropped** (the returned value is discarded as `_returned`).
  `Closed` does nothing.
- `serve` reads commands at exactly one place: the outer `select!`'s
  `commands.recv()` arm (`controller.rs:611-617`). The **inner** `select!` that
  waits on an in-flight exchange (`controller.rs:735-760`) watches cancel, the
  call, and `ingress.arrival()` — **not** `commands`. So commands queued during an
  exchange sit in the capacity-one channel, and a second one is dropped.
- `dispatch` (`controller.rs:529-563`): `OpenDiagnostics`/`CloseDiagnostics` →
  `None` (the loop `continue`s to the top and presents); `Evaluate` → `Some(stamp….)`;
  `Choose` → `Some(controller.answer(...).and_then(stamp…))`. `None` means *there is
  nothing to exchange*. A command resolving to no exchange costs one loop
  iteration and one `present`, nothing else.

**The mapper.**

- `present(&View)` (`crates/goad/src/view_model.rs:107-165`) is pure, total and
  panic-free; it is the only constructor of `Presentation`. `Undrawn::OptionFields
  { option: OptionId, count: usize }` is pushed at `view_model.rs:145-148` when
  `option.fields().as_slice().len() > 0`.
- `Undrawn` is consumed in three places and no others:
  - `view_model.rs:29-41`, `Presentation::body_is_degraded`, which matches only
    `MarkdownUnsupported | ContentForm` and *deliberately excludes* `OptionFields`;
  - `crates/goad/src/diagnostics.rs:191-206`, `undrawn_line`, a total match whose
    `OptionFields` arm renders `"not drawn: option {id} carries {count} field{s}; …"`
    (`diagnostics.rs:193-199`);
  - `crates/goad/src/reception.rs:64-74`, which takes `prepared.presentation.undrawn`
    as a slice and hands it to `Diagnostics::of` — generic over the variant, no
    match.
- **Tests that assert on the variant:** exactly one —
  `crates/goad/tests/renderer/mapper.rs:156-194`,
  `an_option_with_fields_is_reported_undrawn_by_id_and_count`, which asserts
  `presentation.undrawn == vec![Undrawn::OptionFields { option, count: 2 }]`.
  No test asserts the `diagnostics.rs` wording (a grep for `not drawn` over
  `crates/goad/tests/` returns nothing). Narrowing `OptionFields` therefore breaks
  one test and one match arm.

**The canonical field types — accessors a renderer needs, and whether semantics
must change.**

- `Field` (`crates/goad-semantics/src/protocol/canonical.rs:220-244`): private
  fields, `pub fn id()`, `kind()`, `label()`, `hints()`.
- `FieldKind` (`canonical.rs:246-255`): `pub enum` with `Text`, `Boolean`,
  `DateTime`, `Number(NumberRange)`, `Choice { alternatives: Alternatives }` — all
  variants public.
- `NumberRange` (`canonical.rs:411-458`): private `min`/`max`, `pub fn min(self)
  -> Option<f64>`, `pub fn max(self) -> Option<f64>`.
- `Alternatives` (`canonical.rs:350-378`): `pub fn as_slice() -> &[Alternative]`;
  `Alternative` (`canonical.rs:261-274`): `pub fn id()`, `label()`.
- `Fields` (`canonical.rs:380-402`): `pub fn as_slice() -> &[Field]`; `new` accepts
  empty (`Fields` is the one collection that does not reject empty; `Options` and
  `Alternatives` do — `canonical.rs:277-292` (the note), `:332-334` (Options),
  `:362-364` (Alternatives)).
- `Hints` (`canonical.rs:127-137`): `pub fn as_map() -> &BTreeMap<String, serde_json::Value>`.
- Id types: `FieldId`, `OptionId`, `AlternativeId` each expose `pub fn as_str()`;
  their constructors are `pub(super)` (`canonical.rs:52-96`). A renderer can read
  and clone them but cannot mint one — which is what a renderer wants (clone the
  id from the retained view, never invent one).
- **Conclusion: `crates/goad-semantics/` needs no change.** Every accessor the
  mapper and the draft need already exists and is `pub`. The slice card's
  "verify that, do not extend it" (`slice-007.md` §Scope) is satisfiable as
  written. **One caveat**: assembling `values` needs `FieldId: Ord + Clone` (both
  derived, `canonical.rs:83-84`) and `serde_json::Value: Eq` if the edit command
  carries a value (verified: `serde_json::Value` implements `Eq`, see the
  environment note's provenance and F9).

**The pinned Slint widget set.**

Styles `with_style` can select: `slint_build::CompilerConfiguration::with_style(self,
style: String)` (`slint-build-1.17.1/api/rs/build/lib.rs:152-158`; the `style`
field is `Option<String>`, so this is a one-line change to `build.rs`). The
resolver (`i-slint-compiler-1.17.1/internal/compiler/typeloader.rs:937-956`)
takes the configured style, resolves `"native"` separately, and accepts a style
whose directory holds a `std-widgets.slint`. `fileaccess::styles()` enumerates
those directories plus eight aliases (`i-slint-compiler-1.17.1/internal/compiler/fileaccess.rs:106-129`).
At `v1.17.1` those are: **cosmic, cupertino, fluent, material, qt**, plus
`cosmic-light`, `cosmic-dark`, `fluent-light`, `fluent-dark`, `material-light`,
`material-dark`, `cupertino-light`, `cupertino-dark`, plus `native`. **The default
with no style set is `fluent`** (`typeloader.rs:937`, `unwrap_or_else(|| "fluent".into())`).

Every builtin style exports the widgets the five kinds need. From each style's
`std-widgets.slint` at `v1.17.1`: `CheckBox`, `ComboBox`, `LineEdit`, `RadioGroup`,
`SpinBox`, `Slider`, `Switch`, `TextEdit`, `GroupBox`, `TimePickerPopup`, and
`DatePickerPopup` are exported by **all** of `fluent`, `material`, `cosmic`,
`cupertino` and `qt`. So the brief's "exists in `material` and not in the current
default" trap does **not** exist for these eleven names. What differs per style is
the **accessibility surface**, which is what the headless tier drives:

| kind | widget(s) | in/out, callbacks (all styles) | accessible surface | headlessly drivable? |
|---|---|---|---|---|
| `boolean` | `CheckBox` | `in-out checked: bool`, `in text: string`, `callback toggled` | `accessible-role: checkbox`, `accessible-checked <=> checked`, `accessible-action-default` toggles | **yes** — `invoke_accessible_default_action()`, read `accessible_checked()` |
| `boolean` | `Switch` | `in-out checked: bool`, `in text: string`, `callback toggled` | `accessible-role: switch`, `accessible-checked <=> checked`, `accessible-action-default` | **yes** (default action; no set-value action) |
| `text` (single line) | `LineEdit` | `in-out text: string`, `in placeholder-text`, `in read-only`, `callback edited(text)`, `callback accepted(text)` | `accessible-role: text-input`, `accessible-value <=> text`, **`accessible-action-set-value(v) => { text = v; edited(v); }`** | **yes** — `set_accessible_value(..)`, read `accessible_value()` |
| `text` (multiline, `multiline` hint) | `TextEdit` | `in-out text: string`, `in wrap`, `in read-only`, `callback edited` | `accessible-role: text-input`, `accessible-value <=> text`, **no `accessible-action-set-value` in any style** | **no** — value readable, not settable |
| `number` | `SpinBox` | `in-out value: **int**`, `in minimum: int`, `in maximum: int`, `in step-size: int`, `in read-only`, `callback edited(value: int)` | `accessible-role: spinbox`, `accessible-value`, `-minimum`, `-maximum`, `-step`, `accessible-action-set-value`, `-increment`, `-decrement` | **yes**, but **integer only** |
| `number` | `Slider` | `in-out value: **float**`, `in minimum/maximum/step: float`, `callback changed`/`released` | `accessible-role: slider`, same five accessible properties/actions | **yes**, float |
| `number` | `LineEdit` with `input-type: decimal` | as `LineEdit` | as `LineEdit` | **yes**, but the host parses the string |
| `choice` | `ComboBox` | `in model: [string]`, `in-out current-index: int`, `in-out current-value: string`, `callback selected(string)` | `accessible-role: combobox`, `accessible-value <=> current-value`, **only `accessible-action-expand`** | **no** — no set-value action, selection needs the popup |
| `choice` | `RadioGroup` | `out current-value: string`, `callback selected(string)`; children `RadioButton { in-out checked }` | `accessible-role: radio-group` on the group; **`RadioButtonImplBase` declares no `accessible-*` at all** | **no** — no per-item accessible action |
| `datetime` | `DatePickerPopup`, `TimePickerPopup` | `PopupWindow` subclasses; `in date`/`in time`, `callback accepted`, `callback canceled` | none declared on the popup roots | **no** — popups, not inline controls |

Sources for the table: `i-slint-compiler-1.17.1/internal/compiler/widgets/{fluent,material,cosmic,cupertino,qt}/{checkbox,lineedit,textedit,spinbox,slider,combobox,radiogroup,datepicker,time-picker,switch,groupbox}.slint`
at `v1.17.1`; the shared bases under `internal/compiler/widgets/common/`
(`spinbox-base.slint:5-21` for `int` `value`/`minimum`/`maximum`/`step-size`;
`radiogroup-base.slint:6-12` and `116-156`; `combobox-base.slint:5-20`);
`internal/compiler/builtins.slint:2449-2468` for the `RadioButton`/`RadioGroup`
builtins; `internal/common/enums.rs:331-345` for `InputType { Text, Password,
Number, Decimal, Search }`. The testing API is
`i-slint-backend-testing-1.17.1/internal/backends/testing/search_api.rs`:
`invoke_accessible_default_action` (:606), `accessible_value` (:616),
`set_accessible_value` (:637), `accessible_value_minimum/maximum/step`
(:647/:658/:669), `accessible_checked` (:721), `accessible_item_selected` (:743),
`invoke_accessible_increment_action` (:893), `invoke_accessible_decrement_action`
(:904), `invoke_accessible_expand_action` (:915), `match_accessible_role` (:275).

**The headless test tier.**

- `crates/goad/tests/renderer/main.rs` enumerates the target's modules and the two
  shared helpers (`tests/support/scripting.rs`, `driving.rs`, `waiting.rs`).
- `harness.rs` (`crates/goad/tests/renderer/harness.rs`): `window_and_tray` (:31)
  calls `init_no_event_loop()`, `glass_over` (:38), `current_view_token` (:43-51,
  reads the token off the real options model), `until` (:60-70, polls a predicate
  with a panic), `stub_clock` (:31).
- Driving a widget today: `tree.rs:44-49` (`ElementQuery::from_root(..).match_predicate(..).find_first()`),
  `tree.rs:125` (`invoke_accessible_default_action`), `tree.rs:93/98/99`
  (`accessible_item_count`/`item_index`/`label`), `wiring.rs:62-70`
  (`accessible_enabled` via a description predicate).
- Reading what the backend received: `tests/support/scripting.rs`
  (`logging_backend`, `scripted`, `invocations`) plus
  `crates/goad/tests/renderer/scheduling.rs:95-110` `logging_scripted`, which
  swaps `answers-as-instructed.sh` for `logs-the-request-then-answers.sh`, whose
  invocation log holds **each raw request** (see that script's header), and
  `scheduling.rs:113-125` `request_kind`, which parses a logged line as JSON and
  reads a field off it. This is the exact vehicle AC-1's "read `response.values`
  off the wire" needs — a `request["response"]["values"]` read instead of
  `request["event"]["kind"]`.

**The example backends.**

- `just demo` (`justfile`, `demo:` → `run examples/demo.toml`) starts goad on
  `examples/demo.toml`, whose `[backend].command` is
  `["bash", "examples/shell/backend.sh"]`.
- `examples/shell/backend.sh` answers `respond` with
  `{"view":null,"next_check":"45 minutes"}` and **never reads `values`**; its
  header states it "decides nothing". It also deliberately reads only `type`,
  `source` and `kind` and matches on those values rather than on the raw request.
- `examples/typescript/backend.ts` is a fuller backend: it types
  `values: Record<string, unknown>` (`:50`), has a `Field` union including a
  `text` field with `multiline: true` (`:114`, `:155`), and returns `view: null`
  until `event.data.minutes_since_entry >= 45` (`:141-165`). It is typechecked by
  the gate (`justfile` `typecheck:` → `deno check examples/typescript/backend.ts`;
  POL-001 §Compliance command 4).
- **A form-sending backend must either extend `examples/shell/backend.sh` (and so
  `just demo`) or add a third example joined to the demo config.** The gate
  constrains the TypeScript one only.

### Precedents

- **A new `Command` variant + callback wiring:** `wire.rs`'s `Command` and
  `install.rs`'s six-installation table (each callback owns its own `Wire` clone)
  are the shape. `Command::Choose { view, option }` is the closest precedent for an
  identity-carrying command.
- **A second `VecModel` on the glass:** `SliderGlass` holds one today
  (`glass.rs:30-34`, created in `main.rs:98-102`); `present` re-hands its
  `ModelRc` every call.
- **A mapper-only degradation with a diagnostic line:** `Undrawn::ContentForm` and
  `Undrawn::MarkdownUnsupported` are the precedent for a per-item `Undrawn`
  variant, its `diagnostics.rs` line, and its `body_is_degraded` exclusion.
- **A `#[cfg(test)] mod tests` for a private pure function:** `controller.rs`'s
  own bottom module (`controller.rs:767-848`) is the precedent for testing a
  private helper that `tests/` cannot reach.
- **A test that reads a request off the wire:** `scheduling.rs:95-125`.
- **A test that drives a widget by accessibility:** `tree.rs:107-131`.
- **A reducer table test with a total match:** `table.rs`'s `mod reducer`
  (`crates/goad/tests/renderer/table.rs:749-880`) — seven rows, eight
  combinations. If the draft adds a `Shift`-visible behaviour, that table is the
  precedent for how to test it.

## Cross-thread findings

**F1 — CD-1 cites the wrong section for the `respond` example.** CD-1's *Sections*
line says "§6.2 *Response messages* (the `respond` example and a companion
paragraph beside *Field forms*)". The `respond` example is in **§6.1 Request
messages** (`docs/specs/001-host-backend-protocol.md:248-255`); §6.2 begins at
line 260 and carries *Content forms* (line 278) and *Field forms* (line 289). The
companion paragraph beside *Field forms* does belong in §6.2. `slice-007.md`
§Purpose repeats the error twice ("§6.2 shows exactly one submitted value";
"the §6.2 respond example"). Fix the section line in CD-1 to "§6.1 (the `respond`
example) and §6.2 (a paragraph beside *Field forms*)".

**F2 — CD-1's gap is real, and nothing implies a type for any kind.** SPEC-001 was
searched exhaustively for a statement of a submitted value's JSON type: `grep`
for `values|submitted|opaque|verbatim|JSON type|field value` over the whole
document returns R-8, R-9, R-52, R-53, §6.1's `"minutes": 20`, §6.3's ownership
sentence, §7's R-9 row, and nothing else. There is **no rule, no fixture, and no
verification row** that fixes the type of a submitted value for any kind — the one
non-empty submission anywhere is the illustrative example in §6.1, and the only
non-empty `values` map in code is `canonical.rs:752-771`'s unit test, which is the
same example. `UserResponse.values` is `BTreeMap<FieldId, serde_json::Value>`
(`canonical.rs:499-503`), and the TypeScript example types it
`Record<string, unknown>` (`examples/typescript/backend.ts:50`). **The tier-2
argument stands unchanged.** But CD-1's three clauses are not one thing:
*type by kind* is a claim about a single value; *a value for every field drawn*
and *nothing for a field not drawn* are a claim about the map's shape. Recommend
splitting into R-57 (type) and R-58 (completeness) — see *Design-input deltas*.

**F3 — R-9 reaches reading, not authoring, and §7 says so.** R-9's text forbids
*interpreting* a payload or a submitted value and says the host *carries* both
verbatim; §6.3 lists submitted values under "uses, without interpreting"; §7's
R-9/R-19 row is explicit that "the requirement is that host code never *reads* a
payload". Choosing the JSON type of a value the host *authors* from a widget is not
reading a submitted value, so R-9 does not decide CD-1 — it only leaves the write
direction unstated. The slice card's sentence ("R-9's opacity is about the host not
*reading* a value; the host is nonetheless the only thing that can *write* one",
`slice-007.md` §Purpose) is accurate. A one-clause clarification in R-9 or R-57
(Thread 1, *Amendment candidates*) would close the read, but it is not load-bearing.

**F4 — a partial submission is not addressed by any requirement, in either
direction.** No requirement says what a response must do when it omits a field the
view declared. R-8 says `respond` MUST carry "a map of field id to submitted
value"; it does not say the map is total over the drawn fields. R-35 forbids the
host validating an answer beyond `view_id`, which is why the host must not refuse
an incomplete map, but it does not tell the host how to *produce* one. The only
text that reaches the question is CD-1 itself and §5's *Ambiguity is failure*
(P-B), which is about values the host reads. So CD-1's completeness clause is
genuinely new rule, not a restatement — which supports F2's split.

**F5 — "unanswered" versus "false" is not canon, and canon refuses to make it
canon.** SPEC-001/OQ-2 (`docs/specs/001-host-backend-protocol.md:428-433`):
*"Validation feedback. R-35 puts validation in the backend, but a rejection the
user can act on needs per-field errors and retained values on the re-presented
view. Those are additive fields, and honouring them is a version or capability
question — see OQ-1."* `docs/slices/001/design.md:1765-1773` adds the design
intent: `UserResponse.values` is opaque, the host "can retain and echo submitted
values without understanding any of them", and whether the host retains them or
the backend echoes them is "a mechanism choice for that slice". Nothing anywhere
authorises a tri-state or an omission-as-unanswered rule, and the domain boundary
(brief §3.1: the host may understand *fields* and *user responses*, not
*semantics*) puts the `false`-as-`not-yet` reading on the backend. The slice card's
*What the workaround costs* is correct and cited.

**F6 — ADR-001 and the draft: all three changed files are stratum 3, and only
review holds them.** `crates/goad/src/{controller,view_model,glass}.rs` are
stratum 3 under ADR-003. Stratum 3 may name strata 1 and 2 and its renderer
dependency; the four instruments hold stratum 1 (crate edges, manifest, purity
scan, own-feature build) and do not see stratum 3 at all. The vocabulary scan sees
stratum 3's `.rs` and `.slint` **sources**, but holds a different invariant.
`view_model.rs` is *documented* as "the pure half" of stratum 3
(`view_model.rs:1-9`) and already names `slint::StyledText`, so "pure" there means
"no clock, no file, no socket, no widget handle", not "no dependency". **Nothing
would catch a clock or a widget handle appearing in `view_model.rs` except
review** — that is the residue POL-001 names, applied one stratum down. Put the
"draft stays in `Prepared`/`Controller`, never in `Presentation`" rule in the
design and check it at review, because no instrument will.

**F7 — one identifier is banned in `crates/goad/src`, by test.**
`crates/goad-boundary/tests/checks/structure.rs` has
`no_production_line_in_the_renderer_names_the_identifier_resolve` (`structure.rs:308-318`),
scoped to `crates/goad/src`, over production lines only, using `mentions`'s word
matcher. Any new identifier whose camel/snake segments include `resolve` — e.g.
`resolve_field`, `resolve_draft`, `resolve_values` — fails the gate. The
observation is scoped to production lines (inline `#[cfg(test)]` items are
skipped), so a test helper may use it.

**F8 — `present`'s model replacement is the AC-5 trap, and it is structural.**
`glass.rs:103-108` calls `self.options.set_vec(..)` on every present and re-hands
the `ModelRc`. Any widget-local state (a checked box Slint itself flipped) is
overwritten on the next present by whatever `option_rows` produces. A draft that
lives only in the widget is therefore lost on the *next* present even without a new
view; a draft in the controller survives because `present` reads
`frame.shown`. This is why the distinction between OQ-1's (a) and (b) is testable
at all, and it is what AC-5 must assert.

**F9 — `serde_json::Value: Eq`, so an edit command may carry one.** Verified by
compiling `assert_eq_trait::<serde_json::Value>()` against the workspace's own
`libserde_json` rlib: exit 0. `Command` derives `Eq` (`wire.rs:29`), so a new
variant carrying a `serde_json::Value` keeps the derive. `Event`/`UserResponse`
in `canonical.rs` derive only `PartialEq`, but that is not evidence of a
constraint.

**F10 — collision between "one channel message per edit" and the widget's own
edit cadence.** OQ-1's recommendation accepts one channel message per widget edit
and calls it "nothing for fourteen checkboxes". That holds for a checkbox (one
message per click). It does **not** hold for a text field: `LineEdit`/`TextEdit`
expose `callback edited(text)`, which fires on every edit, and `Wire::send` is
`try_send` on a **capacity-one** channel (`main.rs:86`, `wire.rs:126`) that
`serve` drains only at the top of the outer loop (`controller.rs:611-617`); the
inner select does not read commands. Two edits in one loop iteration lose the
second, with only `BUSY_NOTICE` on the window. `LineEdit` also exposes
`callback accepted(text)`, which fires on commit/Enter — the cheap way to keep the
cadence human-paced. This is the argument OQ-1 says should be made with a
measurement, and it is the design's to resolve (per-keystroke + bigger channel, or
`accepted`-only, or both).

**F11 — a `number` field's widget does not match its canonical type.**
`FieldKind::Number(NumberRange)` carries `Option<f64>` bounds
(`canonical.rs:411-458`), and `min`/`max` are optional (R-16). `SpinBox` is
integer-only in every style (`common/spinbox-base.slint:5-17`: `int value`,
`int minimum`, `int maximum`, `int step-size`), and its `minimum`/`maximum` have
defaults (0 and 100) when unset. `Slider` is float but a poor control for an
unbounded number. `LineEdit` with `input-type: decimal` is float and unbounded but
puts a string-parse step in the host. **Drawing `number` is not "strongly implied"
by the spec, as OQ-2 claims** — the spec fixes the *bounds* and the *value's JSON
type* but says nothing about how to present an unbounded or fractional number.
This is the strongest single input to OQ-2.

**F12 — `choice` and `datetime` are not drivable in the headless tier, and
multiline `text` is only half-drivable.** From the widget table: `ComboBox`
exposes only `accessible-action-expand`; `RadioGroup`'s per-item
`RadioButtonImplBase` declares no `accessible-*` at all, and the builtin
`RadioButton` has none either (`builtins.slint:2449-2454`); `TextEdit` declares
`accessible-value` but no `accessible-action-set-value` in any of the five styles.
So a headless test can *read* those widgets but cannot *operate* them through the
testing API. AC-1..AC-6 do not require operating them (AC-1 is boolean; AC-2 is
structural; AC-3 is mapper/diagnostics; AC-4 is the wire log; AC-5 is present;
AC-6 is zero fields), so every AC remains expressible. But OQ-2's four-kind
recommendation would land `choice` (and `number`, per F11) as widgets whose
*values* no automated test exercises — only AC-7's human run would.

**F13 — AC-7's "the record shows every answer" has no vehicle today.**
`examples/shell/backend.sh` discards `values` and returns `view: null`. So "the
record" must be produced by a change to that script (append the answered
`values` to a file, or echo them to stderr, which the host reports on the
diagnostics surface via `Diagnostics::of`'s stderr line, `diagnostics.rs:136-138`).
`examples/` is in scope (`slice-007.md` §Scope) and is not scanned by the
vocabulary check. The evidence must also be gathered **outside the jail** — there
is no display in it (environment note).

**F14 — the domain scan's reach, restated for design.** `group` is safe; the scan
strips comments but keeps strings and markup; `examples/` and `tests/` are
outside it; and the scan passing is necessary and not sufficient for "`group`
never becomes a host concept" (AC-9's own wording). A host-side type named
`Group`/`Grouping`, or a module `grouping.rs`, would pass the scan and still be a
host concept — review holds that, per AC-9 and F6.

## Design-input deltas

1. **Fix CD-1's section citation** (§6.1 for the `respond` example, §6.2 for the
   paragraph beside *Field forms*) — F1. Update `slice-007.md` §Purpose's two
   mentions to match.
2. **Split CD-1 into R-57 (type by kind) and R-58 (completeness).** One
   falsifiable claim per row, per §4's style and R-52/R-53's precedent — F2, F4.
   State each §7 verification row as a `canonical.rs` unit test (outbound), not an
   inbound fixture.
3. **Consider one clarifying clause on R-9's write direction**, folded into CD-1
   rather than a second delta entry — F3.
4. **Decide `number`'s widget explicitly, and say what happens with absent
   bounds.** The spec gives bounds as `Option<f64>` and says nothing about
   presentation; `SpinBox` silently invents 0..100 when they are absent and
   truncates a fractional bound. If OQ-2 keeps `number`, the design must name the
   control and the absent-bounds behaviour — F11.
5. **Decide text-field commit timing.** `LineEdit`'s `edited` fires per keystroke
   on a capacity-one channel; `accepted` is the human-paced alternative. OQ-1's
   "one message per edit" cost is only true of checkboxes — F10.
6. **Note that `choice` (and `datetime`, and multiline `text`) are not drivable in
   the headless tier**, so any test for them is out of reach and any value check
   for them is a human run — F12. This does not block AC-1..AC-6; it bounds what
   "tests" can mean for the extra kinds OQ-2 recommends.
7. **Shape `Undrawn`'s new variant to name option + field + kind.** Exactly one
   test breaks (`mapper.rs:156-194`) and exactly one match arm changes
   (`diagnostics.rs:193-199`); `reception.rs` is generic. State the new
   `diagnostics.rs` wording in the design so the wording is reviewed once — the
   existing wording is asserted by no test.
8. **Keep the draft out of `view_model.rs`.** It is stratum 3's pure half and no
   instrument holds it — F6. The draft belongs in `Prepared` (constructed by
   `receive`, replaced only on `Shift::Replaced`) with a `&mut self` edit entry
   point on `Controller`; `answer`'s two existing refusals are the guard that
   every submitted key came from the retained view.
9. **Avoid any new `resolve*` identifier in `crates/goad/src`** — F7.
10. **Give AC-7 a vehicle.** The demo backend must record the submitted `values`
    (or write them to stderr) for "the record shows every answer" to be
    observable, and the observation must be made outside the jail — F13.
11. **Record OQ-3's relationship to ADR-004 honestly.** The slice card lists
    ADR-004 as not applicable; it is not applicable to the diff, and it is
    directly in the path of the suppression/deferral question OQ-3 leaves open
    (ADR-004's Verification section names SPEC-002/OQ-4 as the question it does
    not answer). Leave OQ-3 unanswered as recommended, but cite ADR-004 in the
    Harvest rather than filing it under "not applicable" — Thread 1.
12. **No change to `crates/goad-semantics/`.** Every accessor the renderer and the
    draft need is already `pub`; `FieldKind`'s variants are public; `FieldId` is
    `Ord + Clone`; constructors being `pub(super)` is what forces id-by-clone,
    which is what the design wants — *Cited facts*, canonical field types.

### Residual uncertainty

- The widget table's "headlessly drivable" column is derived from the
  **declarations** in the pinned style sources plus `search_api.rs`'s dispatch
  (`accessible_action`). It was not exercised by building and running a test —
  `just check` is not this assignment's to run. A design that leans on
  `set_accessible_value` for a `TextEdit` or on any action for a `ComboBox`
  should prove it in the first red test, not assume it.
- Whether a Rust-side `VecModel<FieldRow>` handed to an `in-out property
  <[FieldRow]>` supports Slint writing a field back through `set_row_data` was
  read from the model trait (`i-slint-core-1.17.1/internal/core/model.rs`, the
  `set_row_data` implementations) and from the documented two-way pattern
  (`.../reference/std-widgets/basic-widgets/radiogroup.mdx`, "checked <=>
  model.field") and the memory-game example (`memory_game_logic.slint:75-84`,
  which writes `tile.image_visible = …` in a `for`). It is the established
  pattern, but the design should confirm the exact binding shape in the first
  phase rather than in the last.
- `just demo` was not run and cannot be run here (no display); AC-7's evidence is
  necessarily a person's, outside this environment.

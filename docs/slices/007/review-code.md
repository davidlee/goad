# Review — implementation — Slice 007

**Subject:** implementation — `9447973..1969a27` on `main`, the six execution
phases of slice 007 *(the renderer grows a form)*. The code, not the documents:
`crates/goad/src/`, `crates/goad/ui/app.slint`, `crates/goad/build.rs`,
`crates/goad/tests/`, `examples/shell/backend.sh`.
**Reviewer:** a fresh agent per round — five of them (rounds 1-5)
**Opened:** 2026-09-15
**Resolved:** 2026-09-16
**State:** resolved

Structured, append-only findings ledger for one adversarial review. Everything
needed to drive it is in this file. Narrative history — what was decided and
why, round by round — stays in the matching `-log.md`; this file holds findings
and their fate.

## Protocol

**Roles.** The **raiser** finds and states; the **responder** disposes. One agent
may hold both roles, but must switch deliberately and say which it is acting as —
disposing a finding while still wearing the raiser's hat is how a review talks
itself into `aligned`.

**Append-only.** Findings are never edited or deleted once raised, and ids
(`F-1`, `F-2`, …) are immutable across rounds. A finding raised in error is
**withdrawn**, not removed. A second round appends `F-4` onward to this same
file; it does not start a new ledger.

**Severity** — set by the raiser at raise time, not negotiated afterwards:

| | |
|---|---|
| `blocker` | Must not proceed. The only severity that gates acceptance. |
| `major` | Real defect, unsound design, or breach of canon. Recorded, does not gate. |
| `minor` | Worth fixing, survivable. |
| `nit` | Style or taste. Costs nothing to note, nothing to ignore. |

**Disposition** — set by the responder, one per finding:

| | |
|---|---|
| `aligned` | The observation is correct but nothing needs to change. Say why. |
| `fix-now` | Fix inside the current unit of work, before it closes. |
| `doc-wrong` | The artefact under review is the defect, not the thing it describes. Amend the design / plan / spec. |
| `follow-up` | Owned future work. Must land in `slice-nnn.md` Follow-ups — a disposition is not a place to put things down. |
| `tolerated` | Knowingly accepted, with a written rationale. |
| `settle-in-code` | Real, unsettled, and cheaper to answer in code than in prose. Names the phase that settles it and the test that will. Design and plan reviews only. |

**Outcome** — set by the raiser, terminal:

| | |
|---|---|
| `verified` | Disposition accepted. Done. |
| `contested` | Disagree; hands back to the responder for re-disposition. Not terminal — the finding returns to open. |
| `withdrawn` | The finding was wrong. Terminal. |

**Done** = every finding `verified` or `withdrawn`, and no `blocker` outstanding.
A ledger with no findings at all is **not** done — it means the review has not
run yet.

**Guardrails.** Do not reach for `follow-up` because the fix is large. Do not
normalise `tolerated` without a real reason. Do not downgrade a `blocker` to get
past the gate. `settle-in-code` is not a way to end an argument you are losing:
it needs a named phase and a named test, it is unavailable to a `blocker`, and a
finding that survives its phase returns to the ledger `contested`. Reject a
finding on **evidence**, never on assertion. Confirm each disposition with the
user before acting on it. Fix the class, not the instance, and do not introduce
new defects repairing old ones.

## Brief

<!-- Written BEFORE the review, so it is not shaped by what turned out to be easy
     to find. What this review is probing, and the invariants it holds the
     subject to. Where the bodies are likely buried. -->

This review runs at **full strength**, at audit, with rounds unbounded until the
repairs are themselves reviewed. `just check` is green at `1969a27` — 535 tests
across 21 binaries, verified independently by the audit — so a green gate is the
starting condition, never a finding's answer. Two things the gate does **not**
reach are named here so no finding is rejected by pointing at it: **no ADR-001
instrument reaches stratum 3's purity** (POL-001 §Verification), and **`group`
is not on the vocabulary scan's word list**
(`crates/goad-boundary/tests/checks/vocabulary.rs:18-26`). Both are held by this
review and by reading, or by nothing.

**What this review is probing**, in descending order of where the defects are
likely to be:

1. **The wire contract the slice just wrote.** `canon-delta.md`'s R-57 (a
   submitted value's JSON type is fixed by the field's `kind`) and R-58 (a
   `respond` carries values for exactly the fields the host drew of the option
   answered — all of them, and no others). `draft.rs::submitted` is the single
   site R-57 is applied at; `answer()` is R-58's. Attack both: a field drawn but
   absent from the map, a field present that was never drawn, a value under the
   wrong option's id, a kind whose JSON type is not what R-57 says.
2. **The renderer subset must not narrow the protocol.** This is the failure the
   project exists to avoid. 007 draws `boolean` and reports four kinds undrawn.
   Find anywhere the `boolean`-only *renderer* has leaked into what the *wire*
   or the *normalizer* will accept — a type, a match arm, a validation, a
   fixture — such that a conforming backend sending `text` or `choice` is
   refused, silently dropped, or crashes rather than being drawn-minus-that-field
   and still answerable (R-55, R-35).
3. **Permissive parsing, canonical internals.** Past normalization nothing is
   unvalidated, and an *ambiguous* message fails rather than being guessed at.
   The new `group` hint is the live case: a non-string `group` is drawn
   ungrouped **in place** and reported; `""` is ungrouped and silent; grouping
   is by **runs** and **no field is ever reordered**. Check that the mapper does
   not sort, merge, or dedupe, and that a repeated group name draws its heading
   twice.
4. **Totality of `present`.** `Glass::present` is contracted to write every
   property, every time, and the draft-to-screen path depends on it: the wire
   value is built from the draft, which no present writes, so a present that
   stops writing `checked` leaves the wire green and the screen wrong. Look for
   any write-only-on-change path or field-model exception (plan S-9, design I-5).
5. **The draft's compartmentalisation.** The draft must not have entered
   `Presentation` — no `checked` member on `PresentationField`, no mutable
   member on any `view_model.rs` type (plan S-6, design I-4/R-4). **The gate
   does not reach this**; read it.
6. **A backend failure never takes the host down** and never leaves it unable to
   invoke the backend again. The new `Notice` back-pressure signal and the
   `Edit` command channel are the new failure surfaces: a full channel, a
   backend that dies mid-form, a `view: null` arriving over a half-filled form.
7. **Tests that assert a proxy.** This project has a scar
   (`docs/memory/a-green-test-can-assert-a-proxy.md`): in slice 004 four green
   tests each asserted something the regression they guarded would survive. The
   field tier is new and large — `tests/renderer/fields.rs` is 627 lines — and
   plan S-7 says a field test must read an invocation log or assert something
   about the screen. Find the cases that would stay green through the defect
   they name.

**Where the bodies are likely buried**, handed over rather than discovered, so
the review spends its effort past them:

- `draft.rs` (new, 203 lines), `view_model.rs` (+220) and `controller.rs` (+202)
  are the three files carrying the new semantics.
- `Draft` is deliberately a `Vec` with **no `BTreeMap`, no enumeration and no
  `PartialEq`**. If those absences are load-bearing they should be hard to
  violate; if they are merely unimplemented, say so.
- Keyboard focus is dropped on every present and is a **known, accepted**
  follow-up (`slice-007.md` §Follow-ups) — do not re-raise it.
- `serve` carries an `#[expect]` for the arity lint, and `build.rs` an
  `#[expect]` the plan did not anticipate. Both are on the record; whether they
  are the right shape is not.

**Four cheap repairs are already known and are in scope for this review** — they
are handed to you so you can judge the *class*, not so you can re-find them:
`wiring.rs:65-84`'s `accessible_enabled_of`, which collapses to
`element_described(window, description).and_then(|e| e.accessible_enabled())`
with no behaviour change and removes a second statement of the same filter rule;
the viewport sizer written **three times** in three files, whose three copies
differ in value and in why (consolidating needs a design decision, not a
collapse); `tree.rs`'s two `OptionRow` literals; and `scheduling.rs:415` /
`wiring.rs:1570` both calling `scripted("vt8", …)` in one binary — one pid, one
temp path, one shared invocation log, measured at one failure in six runs.

**Out of scope.** The *look* is slice 008's (`slice-007.md` §Non-goals):
typography, window sizing, the idle surface and the look of the controls. The
window clipping its own second option at its preferred size is a known open
defect (`notes.md` Harvest, F-6) owned by 008 — do not re-raise it, but **do**
raise anything that makes it worse or hides it further. Documentation defects in
`docs/` belong to the audit, not to this ledger; raise only what is wrong in the
code, including code comments.

**Round 1** — 2026-09-15 — the seven lines of attack above, over the whole
slice's code diff. Raised F-1 to F-4. The audit raised F-5 independently.

**Round 2** — 2026-09-15 — round 1's **repairs**, not the slice. Raised F-6 to
F-10: five defects in five repairs, including two false statements in this
ledger's own Response (F-7).

**Round 3** — 2026-09-15 — round 2's repairs. Raised F-11 to F-13.

**Round 1's out-of-band §4** — `review-code-r1` sent a six-item *"below the bar
for a finding"* list outside this ledger. Three were closed before this point (a
miscounted comment at `controller.rs:113`; `within_option`'s doc describing a
walk the query does not perform; and the `accessible_enabled_of` refactor, which
F-13's response closes). The other three are **F-14 to F-16**, raised into the
ledger by the audit rather than dispositioned in a chat message: two of them
changed the code, and every other code change in this slice is accounted for
here. The audit holds both roles for these three and says so — it raises them as
stated by r1, and round 4 is what checks the responses.

**Round 4** — 2026-09-15 — round 3's repairs, and F-14 to F-16's. Raised F-17 to
F-21: five defects in five repairs, three of them false counted claims in this
ledger's own Responses.

**Round 5** — 2026-09-16 — round 4's repairs. Raised F-22 to F-24. **No code
defect**, as in round 4: everything from here on is prose asserting more than it
holds.

**Where the rounds stopped, and why that is a decision rather than an ending.**
Round 5's repairs did not get a round 6. The trend is the argument and it is on
the record above: rounds 1 and 2 found nine code defects between them, round 3
found three, and rounds 4 and 5 found **none** — of twenty-four findings, the
last eight are all counts, enumerations and citations in prose. That residue is
*mechanically* checkable, and a script checks it better than an adversarial
reader because it cannot itself miscount, which is precisely how the last two
rounds failed. So the sixth pass is a verification script over this ledger and
the documents it repaired — every `path:line` resolves, every count matches the
tree — rather than a sixth reading. User decision, 2026-09-16, written here
rather than left for a reader to infer from a ledger that simply stops.

**Round 5** — 2026-09-16 — round 4's repairs, which are prose only. Raised
F-22 to F-24.

### Negative results, rounds 1 and 3

Recorded because an audit cannot write *"checked and sound"* on an absence of
findings, and because two of these were open questions a later agent would
otherwise re-derive.

**Round 1's mutation set** — seven applied at `1969a27`, each reverted with a
precise inverse edit. Four correctly red, three green and each became a finding:

| # | mutation | site | result |
|---|---|---|---|
| M1 | the glass ignores the draft (`checked = false`) | `glass.rs:196` | **red — 6 cases**, and it reddens *screen* assertions while the wire ones stay correct, which is exactly why AC-5 is a conjunction |
| M2 | `answer()` omits fields whose draft value is `Checked(false)` | `controller.rs:224-231` | **red — 5**, three of them reading the child process's own invocation log. This is P-3's named temptation — omitting untouched fields to manufacture an *unanswered* |
| M3 | `Draft::state_of` ignores the option half of the key | `draft.rs:55` | **red — 4, across three tiers**: unit, wire, `answer()` and screen. AC-4 is held, not asserted |
| M4 | an undrawn field dropped with no `Undrawn` report | `view_model.rs:257-261` | **red — 6**. The "silently dropped" failure mode R-20 and R-55 name |
| M5 | delete the element that draws the heading | `app.slint:111-114` | green → **F-1** |
| M6 | delete both `enabled: !root.busy` | `app.slint:58`, `:123` | green → **F-2** |
| M7 | write a constant into `FieldRow.option` | `glass.rs:198` | green → **F-3** |

Two attacks on R-57/R-58 were **structurally unreachable, and why was checked
rather than assumed**: *a field drawn but absent from the map* needs two drawn
fields of one option sharing a `FieldId`, which R-52 rejects at the normalizer;
*a kind whose JSON type is not R-57's* needs a non-`boolean` field to reach
`submitted`, and an undrawn field never enters a block, so `drawn_fields` cannot
reach one and `edit` refuses one with `Refused::UnknownField`.

Also sound, by reading rather than by mutation, since no instrument reaches
them: **I-4/S-6** — `PresentationField` is `{id, label}`; no `checked`, and no
`Cell`, `RefCell`, `Mutex` or mutable member anywhere in `view_model.rs`. The
draft lives in `Prepared` beside the presentation, never inside it. **I-5/S-9** —
all nine of `PromptWindow`'s `in property` declarations matched to unconditional
setters in `present`, no `if changed`, no `set_row_data`, and the nested block
and field models are fresh `VecModel`s each call, so the reset reaches the inner
repeaters. **R-18** — `"group"` is read at exactly one site workspace-wide
(`view_model.rs:188`) and `hints()` has no reader outside `normalize.rs`.

And end to end, which no mutation supplies: `examples/shell/backend.sh` was run
as a real child process against a real `evaluate` and its stdout pushed through
the production `read_response` and `present`. Zero discards; the `text` field
produced exactly one `FieldForm{option: yes, field: note, form: Text}`; the
option survived with its five boolean fields in three blocks. A conforming
backend sending a kind this renderer does not draw is drawn-minus-that-field and
still answerable — **measured, not argued.**

**Round 3's two open questions, both closed.**

*Can F-6's `expect` fire?* **No, not through any fixture.** Seven shapes were
driven through the real markup, printing every label under the container: zero
blocks `[]` (the `if option.blocks.length > 0` guard means no container exists,
so the case fails as a clean vector mismatch rather than a panic); an empty
block `[]`; a named empty block `[Some("Heading")]`; an empty field label
`[Some("H"), Some("")]`; both empty `[Some("")]`; a 4096-character label; two
empty blocks `[]`. Every `Text` that can be in scope is a heading or a
`CheckBox`'s inner label, and both report `Some`. Firing it would take a markup
change, not a fixture.

The role filter was also confirmed load-bearing rather than decorative:

```
without role filter: [None, Some("Before you go"), Some("Stretched"), Some("Read"), Some("Walked")]
with    role filter: [Some("Before you go"), Some("Stretched"), Some("Read"), Some("Walked")]
```

The `None` is the option `Button`'s inner `Text` — the dependency F-6 was raised
about, now excluded by scope rather than hidden by a filter.

*Does `claim`'s `(kind, name)` key miss a cross-kind collision?* **No.** Marker
paths in the renderer binary are `goad-invocations-*` (every caller reaches
`marker` through `logging_backend`'s prefix) and socket paths
`goad-serve-*.sock`. The adversarial case was constructed and checked:
`marker("serve-x")` mints `goad-serve-x-<pid>` and `socket_path("x")` mints
`goad-serve-x-<pid>.sock` — distinct. `claim` is also unreachable from any
non-test build.

**What no round reached, stated so the Verdict cannot borrow it.** Nobody forced
a genuine `Full` on the one-slot channel in a running `serve`, killed a backend
mid-form with a draft outstanding, or drove a real ingress arrival at a window
holding a half-filled form. All three are expressible with the existing harness,
so this is budget and not tooling, and it is the honest gap in this slice's
evidence. `choice`, `number` and `datetime` are covered as mapper values only;
nothing drives one through `serve` to a `respond`. No round had a display, so
AC-7 and AC-10 rest entirely on the user's own account.

**Round 4's checks.** Five findings came out of round 4 (F-17 to F-21); four of
its six lines of attack cleared, and the two most likely to hide a defect — the
new `claim` call sites and F-13's extraction — cleared on measurement rather than
on reading.

*The new `claim` call sites cannot panic a correct case.* This is the failure
mode a new assertion introduces and neither round 3 nor the responder checked it.
All **50** `socket_path` call sites were enumerated — 35 in
`crates/goad-shell/tests/integration/ingress.rs`, 13 in
`crates/goad/tests/renderer/ingress.rs`, 2 in `startup.rs` — and every one passes
a distinct string literal: 35, 13 and 2 distinct names respectively. Each sits
directly inside its own `#[test]` / `#[tokio::test]` body; none is in a loop, and
none is inside a helper that two cases reach, which is `table.rs`'s shape and the
one that would have produced a false positive. Two cases mint two paths each
(`:447-448`, `:810`/`:825`), under different names. Both socket claims and
`marker`'s run **before** the destructive `remove_file`, read at all four sites.
The direct measurement is the gate itself: `just check` exit 0 at 537 cases, and
a false positive here is a panic in a case that was correct.

*F-11 does not re-open F-10 for the new kinds.* `spelled_at_the_call_site` strips
only `invocations-`, so it contributes nothing for the three socket kinds — and
it does not need to, because those kinds claim the call site's own argument
unprefixed. Measured, by pointing `ingress.rs:344` at `"vt1"`:

```
two cases in one test binary claimed the serve socket name "vt1". Such a path is
qualified by pid, not by case, so the two share one file and whichever case starts
second clears it from under the first. Give one of them a name of its own.
```

`"vt1"` is exactly what `ingress.rs:256` spells. Reverted. The helper's doc
generalises a marker-only fact in its first sentence but hedges correctly in its
last (*"the one prefix that exists"*), so it is not raised; the kind rule beside
it is F-21.

*F-13's other counted claims all hold.* `grep -rn
'accessible_description().as_deref()' crates/goad/tests/` returns one line,
`harness.rs:121`, as stated. The mutation reddens 17 across three modules, as
stated — `fields` 4, `tree` 8, `wiring` 5 — and the five-consumer call graph in
the Response is accurate against the tree: `element_described`, `within_option`,
`field_described` and `tree::labels_in_option` call `described` directly, and
`wiring::accessible_enabled_of` reaches it through `element_described`. Only the
*before*-count and its attribution are wrong, which is F-19. Two sub-questions
also cleared: the `'static` bound is **required**, not over-strong —
`ElementQuery::match_predicate` is `impl Fn(&ElementHandle) -> bool + 'static`
(`search_api.rs:282`), so the per-call `to_string` is what makes a `&str`
argument possible at all, not a copy taken by choice; and `search_api.rs:232-287`
does list six matchers beside `from_root`.

*The collapse of `accessible_enabled_of` changed nothing it selects, and the
filter claim it inherited is still true.* `element_described`'s doc asserts *"No
case in this target goes red without the filter, measured rather than
assumed"*, and that claim was made before `wiring.rs` reached the helper —
`accessible_enabled_of` asserts `Some(true)`/`Some(false)` where a groupbox would
answer `None`, so it is exactly the consumer that could have falsified it.
Re-measured after the repair: deleting `.match_inherits("Button")` from
`harness.rs:103` leaves **184 passed; 0 failed**. Reverted. The two helpers'
reasons for the filter are the same mechanism with different consequences — a
groupbox has no default action or item index (`element_described`), and a
groupbox answers `None` for accessible-enabled (`accessible_enabled_of`) — and
the new coupling *is* stated where it matters: `wiring.rs:69-77` says the filter
is not this helper's to restate and names the helper that carries it.

*F-14's three claims each hold.* `PromptWindow` declares nine `in property`s
(`app.slint:16-30`) and `present` writes nine; `Tray` declares three (`:178-180`)
and `present` writes two, so `shown` is the **only** declared property with no
writer and *"one declared property"* is exact. `visible: root.shown`
(`:184`) is the binding, and the markup's own comment at `:175-177` states the
constant-folding reason the doc cites. `Frame` carries no value for it — the
`frame.shown` that `glass.rs:79` and `:116` read is the frame's own field of that
name, an `Option<&Prepared>`, not a tray visibility — and `SlintGlass` holds a
strong `Tray` clone for the life of the process (`glass.rs:38-39`).

*F-15's repair is exactly one input wide, and its `#[expect]` still fires.*
`std::env::var` fails only with `NotPresent` or `NotUnicode`, and both took the
default before the change (`unwrap_or_else(|_| …)`) and take it after (`.ok()` →
`None`); every non-empty `Ok` passes the filter unchanged. So `""` is the only
input whose behaviour moved. The `#[expect(clippy::disallowed_methods)]` is
**fulfilled**, measured rather than assumed: `cargo clippy --workspace
--all-targets` does lint build scripts, and an unfulfilled expectation there is
an error — a planted `#[expect(clippy::let_and_return)]` in `build.rs` produced
`error: this lint expectation is unfulfilled … -D unfulfilled-lint-expectations
implied by -D warnings`. Reverted. One residue, stated and not raised:
`SLINT_STYLE=" "` still fails with the same six-error cascade that names the
markup rather than the variable. That is the repair's third arm working as
written — *"an invalid style must still fail loudly"* — since a whitespace value
is a typo and not how a shell unsets a variable for one command.

*The tree is as it was found.* Every file touched by a mutation this round was
copied before the edit and restored from that copy; the twelve copies verify by
`md5sum -c` with no mismatch, `git diff --stat` is the same 20 files and
1083/125 lines as at the round's start, and `just check` is exit 0 at the end of
the round.

**Round 5's checks.** Round 4's repairs are entirely prose, so every check is a
statement read against the tree. Three findings came out of it (F-22 to F-24);
what follows cleared, on measurement.

*The four-mint table is exact, character for character.* `claim`'s doc and
F-21's Outcome both list `goad-<name>-<pid>`, `goad-serve-<name>-<pid>.sock`,
`goad-startup-<name>-<pid>.sock` and `goad-ingress-<name>-<pid>.sock`. Read from
the four format strings — `scripting.rs:130`, `renderer/ingress.rs:129`,
`renderer/startup.rs:426`, `shell/ingress.rs:49` — all four agree, including
both `.sock` suffixes and the pid's placement after the name rather than before
it. The four kinds are `"marker"`, `"serve socket"`, `"startup socket"` and
`"ingress socket"`: distinct, one per helper, so the key admits no false
positive. The aliasing scenario the doc draws out is real — a fifth helper
minting `goad-<k>-<name>-<pid>` under kind `"k"` does mint `marker("k-<name>")`'s
path, and `(kind, name)` reports no collision — and unreached today, though not
for the reason F-21's Response gives (F-23). One reading noted and not raised:
*"one kind per helper"* is the converse of the property that prevents a false
positive, which is one **helper** per kind; the clause beside it states the
needed direction outright — *"two helpers sharing a kind would report a collision
between paths that differ"* — so a reader cannot come away with the wrong rule.
Also not raised: `renderer/ingress.rs:123`'s *"the other socket helpers' are
`goad-startup-…` and `goad-ingress-…`"* omits `goad-emit`'s, where the sibling
doc at `shell/ingress.rs:43` names it as unheld; in a sentence whose whole
subject is **kinds**, the helper that claims no kind is defensibly out of scope,
and rewriting a defensible sentence at audit closure buys nothing.

*F-19's amendment to F-13's Outcome is true in every number, re-derived rather
than accepted.* The after-count, by mutation at `harness.rs:125`
(`accessible_description()` → `accessible_label()`): **17 failed**, `fields` 4,
`tree` 8, `wiring` 5 — so the impossibility argument holds as stated, the largest
two-module subset being `tree` + `wiring` = 13. The pre-repair tree was then
reconstructed on top of that mutation, each of the three non-`harness` consumers
given back its own correct predicate (`tree::labels_in_option`,
`wiring::accessible_enabled_of` with its own `Button` query, and
`harness::field_described`'s field half, its option scope still reaching the
mutated `within_option`):

```
test result: FAILED. 171 passed; 13 failed
    4 fields::…   7 tree::…   2 wiring::…
```

**13, in three modules**, as the amendment says. The two `wiring` cases still red
are the two `editing` ones that reach a field through `field_described`, so that
call site contributes **none** of the four new cases; the three that go green are
`transitions::dt3_…`, `busy::…after_a_success` and `busy::…after_a_failure`, all
three reaching the predicate only through `accessible_enabled_of`; and the fourth
new case is `tree::a_block_heading_reaches_the_screen_and_an_untitled_block_draws_none`,
whose only route to the predicate is `labels_in_option` — the call site the
finding named and the Response did not add. Every edit reverted.

*F-18's replacement sentence holds on both halves, and no third case doubles.*
Parsing `shell/ingress.rs` by enclosing function: **35** `socket_path` call
sites across **33** functions, so there are more call sites than cases; exactly
two functions mint two paths each — `a_symlink_to_a_live_socket_is_refused_…`
(`vt3b-symlink-target` / `vt3b-symlink`) and `unavailable_s_two_causes_carry_different_detail`
(`vt6b-clock` / `vt6b-shutdown`) — and every other function mints one. The names
quoted are the tree's; only the line numbers beside them are stale (F-24).

*F-20's repair lost nothing load-bearing, and its two counted claims hold.*
`logging_scripted`'s surviving list is correct: `grep -rn 'logging_scripted'
crates/goad/tests/renderer/` gives `harness.rs` and callers in `scheduling.rs`
(`:141`, `:350`) and `fields.rs` (`:279`) and nowhere else. `grep -rn 'because
two of them need it' crates/goad/tests/renderer/` returns exactly one line,
`harness.rs:178`. The two corrected helpers name no file at all — each now
carries *"because several need it"* plus the purposes, `element_described` three
and `field_described` two, which is what the deleted lists were reaching for; the
reason a reader needed from either list was *why the helper is in `harness.rs`*,
and that survives. `described`'s four scopes match its four callers
(`harness.rs:106`, `:143`, `:167`, `tree.rs:288`), the fourth spelled out because
F-19 turns on it.

*F-17's obstacle reproduces, and the follow-up landed.* Adding `#[path =
"../../../../tests/support/scripting.rs"] mod scripting;` to
`crates/goad-emit/tests/binary/main.rs` yields exactly the nine `dead_code`
warnings quoted, `claim` among them; `justfile:50` is `cargo clippy --workspace
--all-targets -- -D warnings`, so the gate does make them errors. Reverted. The
follow-up is in `slice-007.md` §Follow-ups with the obstacle written out — though
carrying the count F-22 corrects.

*The tree is as it was found.* Five files were mutated this round —
`harness.rs`, `tree.rs`, `wiring.rs`, `goad-emit/tests/binary/main.rs` and
`goad-emit/src/main.rs` — each copied before the edit and restored from that
copy; all five verify by `md5sum` against their copies, `git status` shows no
modified file outside `docs/`, and `just check` is exit 0 at **537 cases across
21 binaries** at the end of the round, the same as at its start.


## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | major | fix-now | verified |
| F-2 | minor | fix-now | verified |
| F-3 | minor | fix-now | verified |
| F-4 | minor | follow-up | verified |
| F-5 | major | fix-now | verified |
| F-6 | minor | fix-now | verified |
| F-7 | minor | fix-now | verified |
| F-8 | minor | fix-now | verified |
| F-9 | minor | fix-now | verified |
| F-10 | nit | fix-now | verified |
| F-11 | minor | fix-now | verified |
| F-12 | minor | fix-now | verified |
| F-13 | nit | fix-now | verified |
| F-14 | nit | fix-now | verified |
| F-15 | minor | fix-now | verified |
| F-16 | minor | follow-up | verified |
| F-17 | minor | fix-now | verified |
| F-18 | minor | fix-now | verified |
| F-19 | minor | fix-now | verified |
| F-20 | minor | fix-now | verified |
| F-21 | nit | fix-now | verified |
| F-22 | minor | fix-now | verified |
| F-23 | minor | fix-now | verified |
| F-24 | nit | fix-now | verified |

<!-- Round 1 findings appended below by the raiser. -->

### F-1 — the block heading never reaches an assertion at the screen: deleting the element that draws it leaves the whole gate green

**Severity:** major
**Location:** `crates/goad/ui/app.slint:111-114`; the third link of AC-2's
chain, `crates/goad/tests/renderer/tree.rs:250-257` and `:395-416`

**Expected:** `plan.md` §Coverage states AC-2's rule itself: *"PHASE-03/VT-3 (the
rule, as a pure `present()` test), PHASE-04/VT-7 (`option_rows` carrying blocks
and their order from the presentation to the row model), PHASE-02/VT-4 (row
model to screen, read from tree order). **Three links, because the chain has
three** — a criterion over any two of them leaves the third unverified."*
`design.md` §9/AC-2's observable is *"a heading is drawn where the `group` value
changes, **and** the drawn order equals the declared order"* — two clauses.
Grouping is this slice's one new *visible* output, and the heading is the whole
of what a person sees of it: `design.md` §5.5/I-6 and app.slint's own comment
say the heading is *"the answer to which heading covers which fields"*.

**Observed:** the heading traverses two of its three links and the third holds
nothing. Link 1 is `mapper.rs::blocks_of` (`:67-80`), which reads
`block.heading` as an `Option<String>` off the `Presentation`. Link 2 is
`wiring.rs::blocks_of_row` (`:1199-1216`), used by
`the_row_model_carries_the_blocks_and_their_fields_in_declared_order`
(`:1422-1446`), which reads `block.heading` as a `SharedString` off the **row
model**. Link 3 — PHASE-02/VT-4,
`tree.rs::a_fields_screen_order_is_its_declared_order_across_blocks` (`:395`) —
goes through `fields_in_option` (`:250-257`), which is
`within_option(..).match_inherits("CheckBox").find_all()` and collects only
checkbox descriptions. No query in this workspace ever looks for the heading's
`Text`, and no case asserts any string that a heading would have put on screen.
The row model is not the screen: the same distinction `wiring.rs:1381-1387`
makes for `checked` — *"Read off a shown window's element tree by the
option-scoped query, which is the screen and not the row model: a `checked` that
never reached a control would pass a model assertion"* — is not made for the
heading. Both of the heading's branches in the markup are unheld: the `Text`
itself, and the `if block.heading != ""` guard that distinguishes an untitled
block from a missing heading.

The contrast is the measure of the gap. The *checkbox* half of exactly the same
markup is held at the screen by six cases — injecting `checked = false` into
`glass.rs::field_block` reddens `wiring::editing::the_next_present_writes_every_control_back_from_the_draft`,
`wiring::editing::an_edit_starts_no_exchange_and_a_refused_one_reports_where_a_refused_click_does`
and all four of `fields.rs` (measured, below). The heading half of the same
element reddens nothing at all.

**Evidence:** delete lines 111-114 of `crates/goad/ui/app.slint` — the
`if block.heading != "": Text { text: block.heading; font-weight: 700; }` — and
the whole gate stays green:

```
$ cargo test -p goad
test result: ok. 22 passed; ...        (lib)
test result: ok. 1 passed; ...         (event_loop)
test result: ok. 1 passed; ...         (event_loop_schedule)
test result: ok. 182 passed; 0 failed  (renderer)
```

Identical to the 206-test baseline at `1969a27`. The control, run in the same
session: injecting `let checked = false;` in place of the draft lookup at
`crates/goad/src/glass.rs:196` gives

```
test result: FAILED. 176 passed; 6 failed
    fields::a_field_id_shared_by_two_options_is_two_keys_and_only_the_answered_ones_are_sent
    fields::a_present_that_changes_nothing_leaves_a_half_filled_form_on_the_screen_and_on_the_wire
    fields::a_view_carrying_an_undrawn_field_is_still_shown_and_still_answers_its_drawn_keys
    fields::every_drawn_field_of_the_pressed_option_reaches_the_wire_as_the_screen_showed_it
    wiring::editing::an_edit_starts_no_exchange_and_a_refused_one_reports_where_a_refused_click_does
    wiring::editing::the_next_present_writes_every_control_back_from_the_draft
```

Both edits were reverted; the tree is as found.

This is the shape `docs/memory/a-green-test-can-assert-a-proxy.md` records, seen
from the other side: not a test asserting the wrong thing, but a chain whose own
plan says it has three links and whose third link was built for the fields and
never extended to the heading they sit under. `tree.rs` shows the vehicle is
available and cheap — `within_option(option).match_inherits("Text")` over
`find_all()`, or `ElementHandle::find_by_accessible_label(&window, "Before you
go")`, in the same case that already declares two blocks with a heading on one
of them.

**Disposition:** fix-now — confirmed with the user, 2026-09-15.

**Response:** The finding is correct and it is this slice's own criterion that
it breaches. AC-2's Coverage entry does not merely list three vehicles, it
states the property — *"Three links, because the chain has three — a criterion
over any two of them leaves the third unverified"* — and then the third link
was built for the fields and never extended to the heading they sit under. That
is `docs/memory/a-green-test-can-assert-a-proxy.md` seen from the other side,
and the slice that inherited that scar is not the slice to close over it.

Repaired in `tree.rs`, at the screen and in the same case that already declares
the two blocks: `headings_in_option` beside the existing `fields_in_option`,
built from the same `within_option` scope with `match_inherits("Text")`, and
`a_block_heading_reaches_the_screen_and_an_untitled_block_draws_none` asserting
**both** of the markup's branches — that a named block puts its heading on
screen, and that an untitled block puts nothing there, which is the `if
block.heading != ""` guard the finding names as the second unheld branch. A
single positive assertion would have left that guard exactly as unheld as
before.

Verified by the same mutation that raised the finding: deleting
`app.slint:111-114` now reddens the new case, and removing only the `if` guard
reddens its second half. Both reverted.

**Outcome:** verified

### F-2 — `enabled: !root.busy` is held by nothing, on either control, and `mod busy`'s two cases cannot tell the binding from its absence

**Severity:** minor
**Location:** `crates/goad/ui/app.slint:58` (the option control) and `:123` (the
new checkbox); `crates/goad/tests/renderer/wiring.rs:75-84`, `:325`, `:490-491`,
`:517-518`

**Expected:** `design.md` §5.4's *No edit can race an exchange* rests on the new
binding by name: *"`enabled: !root.busy` already disables option buttons during
a call (`app.slint:47`); **the checkboxes carry the same binding**. So the window
is inert exactly while `serve`'s outer loop is not reading commands, and the
one-slot channel is never asked to hold two edits at once."* A-5 (`§5.5`,
*Assumptions*) leans on the same sentence. `controller.rs:820`'s own comment —
`// busy = true, controls disabled` — is the claim in the production loop.

**Observed:** deleting **both** occurrences of `enabled: !root.busy;` from the
markup leaves every one of the 206 tests green. Every assertion about a control's
enabled state in this workspace is in the `Some(true)` direction — `wiring.rs:325`,
`:490`, `:491`, `:517`, `:518` — and `Some(true)` is also what an unbound
`enabled` answers, because `CheckBox` and `Button` both default
`in property <bool> enabled: true`
(`i-slint-compiler-1.17.1/widgets/material/checkbox.slint:9`). So the two
`wiring::busy` cases are sound for the defect they were written for (F-21, an
`absorb` that leaves `engaged` set) and completely blind to the inverse one; and
`accessible_enabled_of` is never once called with a **field** id, so the binding
this slice added has no assertion pointed at it in either direction.

What a regression here costs is bounded and self-reporting rather than silent —
an edit or a second press landing in a full channel is dropped, the notice
explains it, and the next present writes the tick back off the screen — which is
why this is `minor` and not `major`. What is not bounded is that the sentence
§5.4 argues from is checked by nothing, and the site it names is new in this
slice.

**Evidence:**

```
$ python3 - <<'EOF'
p='crates/goad/ui/app.slint'
s=open(p).read(); assert s.count('enabled: !root.busy;')==2
open(p,'w').write(s.replace('enabled: !root.busy;',''))
EOF
$ cargo test -p goad
test result: ok. 22 passed; 0 failed
test result: ok. 1 passed; 0 failed
test result: ok. 1 passed; 0 failed
test result: ok. 182 passed; 0 failed
```

Reverted; the tree is as found. The gap is one assertion wide: either
`busy` case already has a `frame(busy = true)` present in hand
(`wiring.rs:479`, `:508`) and could read `accessible_enabled_of(.., "yes") ==
Some(false)` at that point, and the field half wants the same read through
`harness::field_described(..).accessible_enabled()` on a fixture that carries
one.

**Disposition:** fix-now — confirmed with the user, 2026-09-15.

**Response:** Accepted as stated, including the severity: the cost of a
regression here is bounded and self-reporting, and what is wrong is that
`design.md` §5.4's *No edit can race an exchange* argues from a binding nothing
checks, at a site this slice added.

The observation that made the gap invisible is the one the repair is built on —
`Some(true)` is what an unbound `enabled` answers, so **only the `false`
direction discriminates**.

> **Amended after F-7.** The first version of this paragraph recorded a change
> to `mod busy`'s *first* case that is not in the tree: it was written, it
> failed — that case engages before absorbing a view, so its window holds no
> options and there is no control to read — it was restructured, and this
> Response was not corrected. What follows is what the tree holds. The error is
> left visible rather than quietly overwritten, because a ledger that edits away
> its own wrong record is worth less than one that shows it.

Both bindings are held, in both directions, by a new case in `mod editing` —
`both_controls_are_disabled_while_an_exchange_is_in_flight_and_enabled_after_it`.
It reads the **checkbox** through `field_described(..).accessible_enabled()`,
which is the site the finding notes had never once been called with a field id,
and the **option control** through `accessible_enabled_of` beside it. It lives
there because the checkbox needs a view carrying fields and `mod busy`'s fixture
carries none.

`mod busy`'s **second** case now asserts the disabled direction too (F-7, which
measured that the control is readable there while the first case's is not), so
the option control's binding is held at two sites and that case's own name —
*re-enable* — is true of what it checks for the first time.

Verified by the finding's own mutation: deleting either `enabled: !root.busy;`
reddens the new case at its own assertion, and deleting both reddens both.
Reverted.

**Outcome:** verified

### F-3 — `FieldRow.option` is written on every present and read by nothing

**Severity:** minor
**Location:** `crates/goad/ui/app.slint:7`, `crates/goad/src/glass.rs:198`,
`crates/goad/tests/renderer/tree.rs:64`

**Expected:** `design.md` §5.3's ownership table and CLAUDE.md's
*carefully compartmentalise state*: the row model carries what the markup reads
and nothing else. `design.md` §5.2 settles a field's identity as **scoped, not
composite** — the container carries `option.id`, the control carries `field.id` —
so the option half of a field's identity lives on the container, not on the row.

**Observed:** `FieldRow` declares a fourth member, `option: string`
(`app.slint:7`). `glass.rs:198` fills it from `option.id.as_str()` on every
present, for every field of every option, and the test builder at `tree.rs:64`
fills it too. Nothing reads it. The markup's `edited` callback passes
`option.id` — the enclosing **`OptionRow`**'s id (`app.slint:126`), not
`field.option`; `field.option` appears nowhere in `app.slint`, and no Rust in
`src/` or `tests/` reads a `FieldRow`'s `option` member. It is a second copy of
an identity the row model already carries one level up, kept in step with the
first by nothing.

**Evidence:** replacing the write with a constant leaves the gate green —

```
$ sed -i 's/option: option.id.as_str().into(),/option: "WRONG".into(),/' crates/goad/src/glass.rs
$ cargo test -p goad
test result: ok. 22 / 1 / 1 / 182 passed; 0 failed
```

Reverted. Corroborating greps:

```
$ grep -n '\.option' crates/goad/ui/app.slint
53:          accessible-item-count: root.options.length;
55:          for option[index] in root.options: VerticalLayout {
(no `field.option` anywhere)
$ grep -rn 'FieldRow' crates/goad --include=*.rs --include=*.slint
crates/goad/ui/app.slint:7    (the declaration)
crates/goad/ui/app.slint:10   ([FieldRow] inside FieldBlock)
crates/goad/src/glass.rs:15   (the import)
crates/goad/src/glass.rs:197  (the literal)
crates/goad/tests/renderer/tree.rs:21,62,63,71  (the import and the builder)
```

Three literal sites to change and no behaviour to preserve. If the member is
meant to survive as the identity a future `edited` should send — which would be
a real improvement over reading it off the enclosing row — then that is the
change to make, and it wants the callback moved with it; carrying it unread is
the one option that buys nothing.

**Disposition:** fix-now — confirmed with the user, 2026-09-15.

**Response:** The finding's own closing sentence is the argument: of the three
available options, carrying the member unread is the one that buys nothing.
Moving `edited` to read it is a design change this slice did not take and has
no criterion for; deleting it is what `design.md` §5.2 already settled, where a
field's identity is **scoped, not composite** — the container carries the
option id and the control carries the field id, and a second copy of the option
id on the row is a third statement of an identity two things already hold.

`FieldRow.option` removed at all three sites it was written: the declaration
(`app.slint:7`), the production write (`glass.rs`) and the test builder
(`tree.rs`). Nothing read it, so nothing changed but the count of places an
identity has to be kept in step — from three to two.

**Outcome:** verified

### F-4 — the undrawn report went from one line per option to one line per field, and the list it grows is unbounded and rebuilt on every present

**Severity:** minor
**Location:** `crates/goad/src/diagnostics.rs:131-134`;
`crates/goad/src/glass.rs:97-105`; `crates/goad/src/controller.rs:737`, `:820`

**Expected:** CLAUDE.md's fourth invariant — *a backend failure never takes the
host down* — and `diagnostics.rs`'s own posture toward backend-chosen lengths:
the module carries `#![deny(clippy::arithmetic_side_effects)]` *"because both
halves compute over lengths a backend or a transport chose"* (`:10-13`), and
every individual line is escaped and clamped through `finish(.., LINE_LIMIT)`.
`STDERR_LIMIT`, `LINE_LIMIT` and `TOOLTIP_LIMIT` are the three bounds the module
states.

**Observed:** the removed `Undrawn::OptionFields { option, count }` produced
**one** line per option, whatever the field count — the count was a number inside
one line. `Undrawn::{FieldForm, GroupHint}` produce one line **per field**, and a
field carrying both defects produces two (`view_model.rs:243-273`, and
`mapper.rs::a_field_that_is_both_undrawn_and_badly_grouped_is_reported_twice`).
`Diagnostics::of`'s loop (`diagnostics.rs:131-134`) has no cap on the number of
lines, so an option of N fields this renderer does not draw yields up to 2N lines
of up to 1024 bytes each. R-15 places no bound on how many fields an option may
carry, so N is the backend's to choose, and a *conforming* backend reaches this
without misbehaving at all.

The cost is paid per present, not once: `glass.rs:97-105` re-collects the whole
list into fresh `SharedString`s and a fresh `VecModel` every call, and
`controller.rs:737` presents at the top of **every** loop iteration — which,
after this slice, includes one iteration per checkbox tick (`dispatch` returns
`None` for an `Edit` and the loop `continue`s to the top). Before 007 the same
view cost one line; now it costs 2N, rebuilt on every tick of a form the person
is filling in.

Nothing here crashes, and the class is not new — `discarded` (`:135-138`) is
unbounded in the same way — which is why this is `minor`. What is new is the
multiplier and the per-tick rebuild, and it is worth deciding rather than
inheriting: either the list takes a bound of its own (with an *"and N more"*
line, the shape `finish` already models for a single line), or the decision to
leave it unbounded is written down beside the three bounds that are.

**Evidence:** `diagnostics.rs:117-147` is the whole of `Diagnostics::of`; the
only `Vec` bound anywhere in it is per-line. The reachable multiplier is
demonstrated by a fixture already in the suite —
`mapper.rs::every_undrawn_kind_is_reported_by_option_field_and_form` asserts
`presentation.undrawn.len() == 4` for four fields in one option, and
`reception.rs::a_view_carrying_an_undrawn_field_reaches_the_diagnostic_surface_through_receive`
asserts `lines.len() == 2` for two fields, i.e. one line per field with nothing
between the mapper and the surface to collapse them. Scaling is linear and
uncapped by construction.

**Disposition:** follow-up — confirmed with the user, 2026-09-15. Landed in
`slice-007.md` §Follow-ups, which is what a `follow-up` disposition owes.

**Response:** The finding splits into two halves with different answers, and
saying so is most of the disposition.

**The per-present rebuild is not a defect.** `Glass::present` is contracted to
write every property every time, and `plan.md` S-9 and `design.md` I-5 forbid
acquiring a write-only-on-change path or a per-model exception — weakening
totality is what `A-2` exists to exclude, and it is the mechanism that
re-establishes `FieldRow.checked` from the draft. The rebuild is that contract
being honoured, and it is `aligned`.

**The unboundedness is real and is a class, not an instance.** It is a property
of every list `Diagnostics::of` builds — the finding names `discarded` itself as
unbounded in the same way, pre-existing. Bounding only the undrawn list would
fix the instance this slice made cheaper to reach and leave the rule it breaks
unstated, which is the opposite of what `CLAUDE.md` asks. What the class wants is
one decision — what bound, and what an *"and N more"* line means when the list
rather than the line is what overflowed — taken beside the three bounds the
module already states (`STDERR_LIMIT`, `LINE_LIMIT`, `TOOLTIP_LIMIT`) rather
than as a fourth added in passing.

What is genuinely new in 007 and is recorded with the follow-up so it is not
re-derived: the multiplier went from *one line per option* to *up to 2N lines
per option*, N is the backend's to choose under R-15, and a **conforming**
backend reaches it without misbehaving.

**Outcome:** verified

### F-5 — two cases in one test binary share one invocation-log path, and the convention that forbids it is held by nothing

**Severity:** major
**Location:** `tests/support/scripting.rs:36` (`marker`), `:55` (the convention
in prose); `crates/goad/tests/renderer/scheduling.rs:415` and
`wiring.rs:1636`; and `table.rs:555`

**Raised by:** the audit, not the round-1 reviewer. Handed to the review in the
Brief as one of four known-cheap repairs and not taken up there; raised here so
it has an id, evidence and a fate rather than living in a handover note.

**Expected:** `marker`'s own first sentence — *"A path in the temp directory
that **no other case will collide with**, cleared before it is handed out"* —
and `logging_backend`'s — *"**Each case names its own log**, so concurrent cases
cannot read each other's lines"* (`scripting.rs:30-33`, `:50-56`). `just check`
is the gate (`CLAUDE.md`, POL-001) and a gate is a thing that means the same on
every run.

**Observed:** the path is `/tmp/goad-{name}-{pid}` (`scripting.rs:36`). It is
qualified by **pid**, not by case, and every case in one target runs in one
process — so two cases naming one marker get one file. `marker` then calls
`clear` at **handout**, so whichever case starts second *deletes the other's log
mid-run*, and `cargo test` runs them on parallel threads.

Two such pairs exist, and the second was not known to anyone:

- `scheduling.rs:415` and `wiring.rs:1636` both asked for `"vt8"` — a criterion
  id, not a case name, which is how two files picked it independently. The
  `scheduling.rs` case asserts `invocations(&log) == 1`, so the deletion is
  observable: **one failure in six runs**, measured.
- `table.rs:555`'s `inert_answer` helper asked for `"table-inert-option"` **once
  per row** of a thirty-three-row loop. Nothing reads that log, so it was
  costing nothing today; it is the same breach of the same rule, and it was
  found only by the instrument.

Neither was introduced by this slice — `git log -S` puts the first at slice 003
PHASE-02 and slice 001 PHASE-10. That bears on whose regression it is and not on
whether 007 may close over it.

**Evidence:** the mechanism, read rather than inferred: `scripting.rs:36` builds
the path from `name` and `std::process::id()`; `:37` clears it; `:97`
(`logging_backend`) is the only caller and prefixes `invocations-`. The second
pair was surfaced by the repair itself — adding the registry turned the latent
collision into two named failures on the first run:

```
thread 'table::every_failure_in_the_taxonomy_is_read_off_one_retained_host' panicked at
  tests/support/scripting.rs:62:
two cases in one test binary asked for the marker "invocations-table-inert-option". ...

thread 'wiring::serving::serve_drives_one_exchange_through_the_production_loop' panicked at
  tests/support/scripting.rs:62:
two cases in one test binary asked for the marker "invocations-vt8". ...
```

**Disposition:** fix-now, with an instrument — confirmed with the user,
2026-09-15.

**Response:** A rename alone would have fixed the instance and left the rule
where it was: written in a doc comment, breached twice, and checked by nothing.
The user's decision was the rename **and** the instrument, and the instrument is
what found the second pair.

The instrument is not a lint. `marker` now keeps a per-process registry of the
names it has handed out and **panics** on the second request for one, naming the
marker and failing inside the case that asked. A source scan was the alternative
and is worse on every axis that matters here: it would have to parse call sites,
it cannot see a name built at runtime — which is exactly `inert_answer`'s
shape — and it would report a file where this reports a case. The registry is
keyed per process, which is the scope the collision actually has: two *targets*
may share a name freely, because the pid already separates them.

Both pairs repaired. `wiring.rs`'s case takes `"wiring-serve-one-exchange"` —
named for the case, not for a criterion id, which is how two files chose `"vt8"`
independently in the first place. `inert_answer` takes the row id it is called
for and mints `"table-inert-option-{row}"`, because a helper called once per row
needs a name per row.

Red before green: the registry was added first and the suite went red in two
places; the renames turned it green. The gate is green at 184 renderer cases,
and it is now green for the same reason on every run rather than on five runs in
six.

**Outcome:** verified

### F-6 — `labels_in_option`'s `None` filter silences the option **button's own internal `Text`**, not "the markup's structural elements"; the heading case depends on a `std-widgets` internal it does not name

**Severity:** minor
**Location:** `crates/goad/tests/renderer/tree.rs:257-280` (`labels_in_option`,
and its doc comment at `:269-273`); the case at `:478-489`

**Expected:** the doc comment states the filter's justification as fact and
says it was measured: *"Unlabelled `Text` elements are dropped and
empty-labelled ones are kept … so an untitled block that wrongly drew a heading
appears here as an empty string rather than vanishing into the same `None` as
the markup's **structural elements**. Measured, not assumed."* A test comment
that says *measured* is a claim about what was observed, and a helper whose
scope reaches outside the markup under test should say so — this file already
warns, twice, about queries that silently depend on something nothing pins
(`tree.rs:107-118`, `harness.rs:88-96`).

**Observed:** there is exactly **one** `None`-labelled element in the query's
result, it is **first**, and it is not a structural element of `app.slint`: it
is the `Text` inside the option's own `Button`. `within_option`
(`harness.rs:120-128`) matches *any* element whose accessible-description is
`option.id` and takes its descendants — and both the option's `Button`
(`app.slint:61`) and the field container (`app.slint:76`) carry that
description. So `labels_in_option` walks inside the control as well as inside
the fields, and the `filter_map` is what hides the control's label element.

The dependency is on `std-widgets`, undocumented, and asymmetric with the one
the comment *does* name: `CheckBox`'s internal `Text` **does** report an
accessible label (that is what puts `"Stretched"`, `"Read"`, `"Walked"` in the
vector), while `Button`'s does not. Nothing in this repository fixes that
asymmetry. A Slint release that labels `Button`'s inner `Text` the way it
already labels `CheckBox`'s turns the expected vector into
`["Yes", "Before you go", "Stretched", "Read", "Walked"]`, and the case fails
with a heading-shaped message — *"the heading is drawn once, above the two
fields it covers"* — for a reason that has nothing to do with headings. That is
the confusing breakage, and it is one dependency wider than the comment admits.

**Evidence:** replacing the filter with a total map and running the case —

```
$ # tree.rs: .filter_map(|e| e.accessible_label())
$ #       -> .map(|e| e.accessible_label().unwrap_or(SharedString::from("<NONE>")))
$ cargo test -p goad --test renderer a_block_heading_reaches
  left: ["<NONE>", "Before you go", "Stretched", "Read", "Walked"]
 right: ["Before you go", "Stretched", "Read", "Walked"]
```

and then, with the same total map, against an option carrying **no blocks at
all** — so the only element in scope is the option's `Button` and its
descendants:

```
$ # a probe case: window.set_options(one_option_with(vec![]))
$ #               panic!("no blocks: {:?}", labels_in_option(&window, AN_OPTION.0));
no blocks: ["<NONE>"]
```

One unlabelled `Text`, present when the field markup is absent entirely. Both
edits reverted; `just check` is green at exit 0, 537 tests.

The cheaper assertion the finding asks for is already in this file: `tree.rs:522`
uses `ElementQuery::match_accessible_role(AccessibleRole::Groupbox)`, which
scopes to the field container and excludes the option's control, so the query
would return only what the field markup drew. Under that scope the `None` filter
becomes provably unnecessary rather than load-bearing-for-an-unstated-reason,
and the vector asserted is unchanged. What is *not* wrong here: the interleaving
of headings and checkbox labels is the right claim for AC-2, and the
`Some("")`-versus-`None` distinction the comment calls load-bearing is real —
measured, removing only the `if block.heading != ""` guard yields
`["Before you go", "Stretched", "Read", "", "Walked"]`.

**Disposition:** fix-now — confirmed with the user, 2026-09-15.

**Response:** Correct, and the doc comment was the worse half: it said
*measured* about a claim that had not been measured, in a file that twice warns
about queries depending on something nothing pins. A comment asserting more than
it holds is the same defect as a test asserting more than it holds.

Taken the way the finding recommends, which is the option that removes the
dependency rather than documenting it. `labels_in_option` now scopes by **role**
— `match_accessible_role(AccessibleRole::Groupbox)`, this file's own idiom at
`:522` — so the query covers the option's field container and not its `Button`.
Every element then in scope was drawn by the field markup, the `filter_map`
becomes a total `map`, and the `None` question stops existing rather than being
answered: the helper `expect`s a label, so an unlabelled element in scope would
fail loudly instead of being silently dropped.

That is the difference between a query that is correct and one that is
accidentally correct. The Slint asymmetry the finding names — `CheckBox`'s inner
`Text` labelled, `Button`'s not — is depended on no longer, in either direction.

Re-verified after the change: both of F-1's mutations still redden the case, and
still each for its own reason. Deleting the heading `Text` gives
`["Stretched", "Read", "Walked"]`; removing only the `if` guard gives
`["Before you go", "Stretched", "Read", "", "Walked"]`. The asserted vector is
unchanged, which is the sign the repair changed the query's justification rather
than its claim.

**Outcome:** verified

### F-7 — F-2's Response records a repair to `mod busy` that is not in the tree, and the new case's stated reason for not living there is false for one of the two cases

**Severity:** minor
**Location:** `review-code.md` F-2 §Response; the doc comment at
`crates/goad/tests/renderer/wiring.rs:1429-1433`;
`crates/goad/tests/renderer/wiring.rs:495-518` (`mod busy`'s second case)

**Expected:** the ledger's Response is the record of what was changed — the
Protocol above makes a finding's fate the thing this file holds, and the
Guardrails require a disposition to be answered on evidence. F-2's Response
states two changes: *"Both halves now read it: **`mod busy`'s first case asserts
the option control is disabled at the `busy` present it already has in hand**,
before the assertion it already made that the control re-enables; and a new case
in `mod editing` asserts the same of a **checkbox**"*.

**Observed:** only the second half exists. `mod busy` is untouched — its two
cases still assert `accessible_enabled_of(.., "yes") == Some(true)` and nothing
else (`wiring.rs:489-490`, `:517-518`), which is exactly the blind direction F-2
identified. The option control's `false` direction is held by the **new
`mod editing` case**, not by `mod busy`, so the repair is sound and the record
of it is not.

The doc comment that explains the placement is separately wrong. It reads:
*"Here rather than in `mod busy`, whose two cases engage **before** a view is
absorbed: at their busy present the window holds no options, so there is no
control to read and `accessible_enabled_of` answers `None`."* That is true of
`busy_clears_and_controls_re_enable_after_a_success` (`:471`), which engages at
`:479` with nothing absorbed. It is **false** of
`busy_clears_and_controls_re_enable_after_a_failure` (`:495`), which absorbs a
view at `:504`, presents it at `:505`, and only then engages at `:507` — so at
its busy present (`:509`) the window holds two options, `with_room_for_every_control`
has already sized the viewport to reach them (`:501`), and the control is there
to read. The generalisation to "whose two cases" is the part that is not true,
and it is the comment's whole argument for where the case lives.

**Evidence:** inserting one assertion into `mod busy`'s second case, immediately
after its existing `assert!(window.get_busy())` at `wiring.rs:509`:

```rust
assert_eq!(accessible_enabled_of(&window, "yes"), Some(false));
```

```
$ cargo test -p goad --test renderer busy_clears_and_controls_re_enable_after_a_failure
test result: ok. 1 passed; 0 failed
```

The control is readable there and answers `Some(false)`. The same probe in the
**first** case, after its `assert!(window.get_busy(), "the exchange must be
shown in flight")`, passes against `None` — confirming the comment for that one:

```
$ # assert_eq!(accessible_enabled_of(&window, "yes"), None);
$ cargo test -p goad --test renderer busy_clears_and_controls_re_enable_after_a_success
test result: ok. 1 passed; 0 failed
```

Both probes reverted; the tree is as found.

Nothing about the production code is wrong and no coverage is missing — the two
mutations F-2 named both redden the new case, each for its own assertion
(`wiring.rs:1460` for the checkbox, `:1465` for the button). What is wrong is
that a reader of this ledger who went looking for the `mod busy` change would
not find it, and a reader of `wiring.rs:1429` would believe a second site was
unavailable when it was available and measured to be so.

**Disposition:** fix-now — confirmed with the user, 2026-09-15.

**Response:** Both statements were mine and both were false. The Response was
written describing a repair that was then restructured when it failed, and never
corrected — the exact failure this ledger exists to prevent, committed in the
ledger itself. F-2's Response above is amended to record what is actually in the
tree.

The doc comment's error was a generalisation from one case to two, and the
finding did the work of measuring which is which: `…after_a_success` engages
before absorbing anything, so its window holds no options and the control is
genuinely unreadable; `…after_a_failure` absorbs and presents first, so the
control is there and answers `Some(false)`. The comment now says that, names the
real reason the new case lives in `mod editing` — the **checkbox** needs a view
carrying fields, and `mod busy`'s fixture is `TWO_OPTIONS`, which carries none —
and claims nothing about the second case.

And the assertion went in where the finding proved it fits.
`busy_clears_and_controls_re_enable_after_a_failure` now reads the disabled
direction at its busy present, which makes that case's own name true: *re-enable*
presupposes they were disabled, and until now nothing in it checked the first
half. The binding is held at two sites rather than one, which is what F-2 asked
for.

**Outcome:** verified

### F-8 — the repair added the collision check and left the file that most depends on it saying "Nothing checks for a collision"

**Severity:** minor
**Location:** `crates/goad/tests/renderer/fields.rs:263-269`

**Expected:** F-5's disposition was *fix-now, with an instrument*, and its
Response says the point of the instrument over a rename was that *"a rename
alone would have fixed the instance and left the rule where it was: written in a
doc comment, breached twice, and checked by nothing."* The Guardrails say to fix
the class. A doc comment that states the opposite of what the code now does is
the same defect the repair was made to remove, one file over.

**Observed:** `rigged` — the helper every case in `fields.rs` gets its marker
through — still carries: *"**`case` is a path, not a label.** `scripting::marker`
turns it into `goad-invocations-<case>-<pid>` … so two cases given the same name
share one log, each truncating the other's, and fail intermittently under cargo's
parallelism. **Nothing checks for a collision**; the names here are prefixed with
this file's own, as `wiring.rs`'s and `table.rs`'s are."*

The final clause is now false in both of its halves. Something does check
(`tests/support/scripting.rs:64-71`), and the consequence is no longer an
intermittent failure — it is an immediate panic naming the marker, in the case
that asked. This is the most load-bearing statement of the old rule anywhere in
the workspace: it is the only place that explains *why* a case name is a path,
and it now tells a future author to hold by hand a rule the tooling holds. The
same paragraph's *"clears it on the way out"* is also inaccurate — `marker`
clears at **handout** (`scripting.rs:73`), which is precisely why the collision
was destructive — but that error predates this slice.

**Evidence:**

```
$ grep -n 'Nothing checks for a collision' crates/goad/tests/renderer/fields.rs
267:/// intermittently under cargo's parallelism. Nothing checks for a collision;
$ sed -n 64,71p tests/support/scripting.rs
  assert!(
    HANDED_OUT
      .lock()
      .expect("the marker registry must not be poisoned")
      .insert(name.to_owned()),
    "two cases in one test binary asked for the marker {name:?}. ...
```

**Disposition:** fix-now — confirmed with the user, 2026-09-15.

**Response:** The finding is right about why this one matters more than its
severity suggests: `rigged`'s comment is the only place in the workspace that
explains *why a case name is a path*, so leaving it saying "Nothing checks for a
collision" would have told the next author to hold by hand a rule the tooling
now holds — the same defect F-5 was raised to remove, one file over, and
introduced by F-5's own repair.

Rewritten to what is now true: `claim` checks and panics naming the case, so
prefixing the names with this file's own is a courtesy to the reader rather than
the only thing standing between the target and an intermittent failure. The
comment keeps the history, because it *was* the only thing until this audit and
two pairs had already slipped past it.

The finding's second, pre-existing error is repaired in the same pass: the
comment said `marker` clears the log *"on the way out"*, and it clears at
**handout** — which is precisely why the collision destroyed data rather than
merely confusing it. Fixing one and leaving the other would have been fixing the
instance.

**Outcome:** verified

### F-9 — `socket_path` makes `marker`'s exact promise, in the same test binary, with the same criterion-id names, and the new registry does not reach it

**Severity:** minor
**Location:** `crates/goad/tests/renderer/ingress.rs:105-113`; the registry at
`tests/support/scripting.rs:32-38`

**Expected:** F-5 was dispositioned *fix-now, **with an instrument***, and the
Guardrails say *fix the class, not the instance*. The class is: **a per-process
path helper that promises uniqueness a test binary must not breach, held by
nobody**. The Response frames the registry as closing it — *"so that the
uniqueness `marker` promises is held by something rather than by everyone
remembering."*

**Observed:** `ingress.rs` — a case file in the **same test binary** as
`wiring.rs`, `scheduling.rs` and `table.rs` — declares a second such helper, and
the registry does not see it:

```rust
/// A path no other case will collide with, cleared before it is handed out —
/// `std::env::temp_dir()` and the process id, because `tempfile` is not on the
/// manifest allowlist (`plan.md` PL-3) and this phase adds no dependency.
fn socket_path(case: &str) -> PathBuf {
  let path = std::env::temp_dir().join(format!("goad-serve-{case}-{}.sock", std::process::id()));
```

Its first sentence is `marker`'s first sentence, near-verbatim; the mechanism is
`marker`'s mechanism (`temp_dir` + name + pid, cleared at handout); the failure
mode is `marker`'s failure mode, and worse — two cases sharing a socket path
share a **listener**, and `cleanup` (`:117-127`) removes the lock file beside it,
so the second case to start unlinks the first case's lock mid-run. And the names
are drawn from the same well that produced the `"vt8"` collision: `socket_path`
is called with `"vt1"`, `"vt2"`, `"vt3"`, `"vt4"`, `"vt5"`, `"vt7"` (`:239`,
`:327`, `:399`, `:480`, `:591`, `:721`) — criterion ids, not case names, which
F-5's own Response identifies as *"how two files chose `"vt8"` independently in
the first place."*

Nothing collides today: all thirteen `socket_path` names are distinct, and the
`goad-serve-` prefix keeps them clear of `marker`'s `goad-` and
`goad-invocations-` paths. That is the position `table.rs`'s
`"table-inert-option"` was in — *"Nothing reads that log, so it was costing
nothing today; it is the same breach of the same rule"* — and F-5 repaired it
anyway.

**Evidence:**

```
$ grep -n 'socket_path(' crates/goad/tests/renderer/ingress.rs
108:fn socket_path(case: &str) -> PathBuf {
239:  let path = socket_path("vt1");
327:  let path = socket_path("vt2");
399:  let path = socket_path("vt3");
480:  let path = socket_path("vt4");
591:  let path = socket_path("vt5");
721:  let path = socket_path("vt7");
860:  let path = socket_path("p5vt1");
950:  let path = socket_path("p5vt2");
1052: let path = socket_path("p5vt3");
1160: let path = socket_path("p5vt4");
1248: let path = socket_path("p5vt5");
1378: let path = socket_path("f3-inner-stop");
1476: let path = socket_path("p5vt6");
$ grep -rn 'HANDED_OUT' crates tests
tests/support/scripting.rs:38:static HANDED_OUT: LazyLock<Mutex<BTreeSet<String>>> = ...
tests/support/scripting.rs:65:      HANDED_OUT
```

`socket_path` never calls `marker` and never touches `HANDED_OUT`. The cheap
shape is the one the registry already has: the uniqueness check is three lines
and is independent of what the path is *for*, so either `socket_path` mints its
suffix through a shared "claim this name" helper, or the registry is exposed as
one and both callers use it. The gap is not whether the registry works — it
does, and it found the second pair — but that it was scoped to one of the two
helpers that make the promise.

**Disposition:** fix-now — confirmed with the user, 2026-09-15.

**Response:** Accepted without qualification. F-5 was dispositioned *fix the
class* and was scoped to one of the two helpers making the claim. That
`socket_path` collides with nothing today is the position `table.rs` was in, and
F-5 repaired that anyway on the argument that it is the same breach of the same
rule — an argument this finding turns back on the repair, correctly. The stakes
are higher here than the severity implies: two cases sharing a socket path share
a **listener**, and `cleanup` unlinks the lock file beside it.

Taken the second way the finding offers, which is the one that fixes the class.
The registry is now `claim(kind, name)` in `tests/support/scripting.rs`, a
helper independent of what the path is *for*; `marker` calls it with `"marker"`,
and `ingress.rs`'s `socket_path` with `"socket"`. Keying by kind as well as name
is what keeps it honest in the other direction: the two mint different paths —
`goad-<name>-<pid>` and `goad-serve-<name>-<pid>.sock` — so two callers of
different kinds sharing a name are not a collision and must not be reported as
one.

Verified by mutation in **both** namespaces. Pointing two `socket_path` cases at
`"vt1"` panics with *two cases in one test binary claimed the socket name
"vt1"*; restoring the original `scripted("vt8", …)` in `wiring.rs` panics on the
marker name. Both reverted; `just check` exit 0, 537 cases.

**Outcome:** verified

### F-10 — the registry's panic names a marker no call site spells

**Severity:** nit
**Location:** `tests/support/scripting.rs:64-71`, and `:97`
(`logging_backend`'s `invocations-` prefix)

**Expected:** the instrument's stated advantage over a source scan is that it
*"reports the name, in the case that asked for it"* (`scripting.rs:59-62`), and
its message ends *"Give one of them a name of its own."* — an instruction to go
and edit a call site.

**Observed:** every caller but one reaches `marker` through `logging_backend`,
which prefixes the name (`:97`, `marker(&format!("invocations-{case}"))`). So
the panic reports `"invocations-vt8"` while the call site to edit reads
`scripted("vt8", …)`. F-5's own evidence block shows both messages in exactly
that form. Grepping the workspace for the string the panic prints finds nothing:

```
$ grep -rn 'invocations-vt8\|invocations-table-inert-option' crates tests
$ (no matches)
```

The message is still far better than a source scan's, and the pid-qualified path
it prints elsewhere is unambiguous — hence `nit`. Naming the caller's own
argument rather than (or as well as) the prefixed key would close the last step.

**Disposition:** fix-now — confirmed with the user, 2026-09-15.

**Response:** A nit worth taking because it costs four lines and because the
claim it undercuts is the instrument's whole advantage over a source scan: that
the panic *reports the name, in the case that asked for it*. A name that greps
to nothing is not that.

Taken as the finding's parenthesis suggests — **as well as**, not instead of.
The key is still printed, because it is what the registry holds and what the
path contains; where it carries the one prefix that exists, the message now also
prints the call site's own spelling:

```
two cases in one test binary claimed the marker name "invocations-vt8", which the
call site spells `scripted("vt8", …)` or `logging_backend(.., "vt8")`. …
```

Both spellings, because `logging_backend` has two doors and a reader should not
have to work out which one they came through.

**Outcome:** verified

### F-11 — the class F-9 was dispositioned to close is still open in the same test binary: `startup.rs` mints the same kind of path, unclaimed, and `claim`'s own doc says it does not

**Severity:** minor
**Location:** `tests/support/scripting.rs:46-47` (the `claim` doc's scope claim);
`crates/goad/tests/renderer/ingress.rs:109-110`; the unclaimed helpers at
`crates/goad/tests/renderer/startup.rs:410-417` and
`crates/goad-shell/tests/integration/ingress.rs:31-39`

**Expected:** the repair's own doc states its reach as fact — *"The instrument
behind **every** "a path no other case will collide with" promise in this
repository's test support. **Call it wherever such a path is minted**"*
(`scripting.rs:46-47`). F-9's argument against F-5's scope is the standard: the
class is *"a per-process path helper that promises uniqueness a test binary must
not breach, held by nobody"*, and F-9 was accepted on the ground that
`table.rs`'s non-colliding name was repaired anyway because it is *"the same
breach of the same rule"*. The Guardrails say fix the class, not the instance.

**Observed:** `startup.rs:410`'s `mod listener::socket_path` is `ingress.rs`'s
helper, line for line in mechanism — `temp_dir()` + case + pid, `.sock`, cleared
at handout — and its `cleanup` (`:421-428`) removes the lock file beside the
socket, which is precisely the stake F-9's Response cited as making this worse
than a shared log. It is in the **same test binary** as the helper that now
claims (`renderer`: `startup.rs`, `ingress.rs`, `wiring.rs`, `scheduling.rs`,
`table.rs`, `fields.rs`, `tree.rs` are one target), so `claim` is in scope at
zero cost. It does not call it.

`crates/goad-shell/tests/integration/ingress.rs:31-39` is a third, in the only
other target that `#[path]`-includes `tests/support/scripting.rs`
(`crates/goad-shell/tests/integration/main.rs:17`). Its doc makes the promise in
so many words — *"A unique path per case, under `std::env::temp_dir()`"* — over
**33** call sites named `vt1-reclaim`, `vt2-live`, `vt3b-symlink`,
`client-vt4-no-reply`, … : the criterion-id well F-5's Response names as *"how
two files chose `"vt8"` independently in the first place."*

Two of the repair's written claims are therefore false of the tree: `claim` is
not behind *every* such promise, and `ingress.rs:109`'s *"made by a second
helper in the same test binary"* undercounts — there is a third in that same
binary and a fourth next door.

**Evidence:** the collision is destructive and the instrument is silent for it.
Pointing `startup.rs`'s two cases at one name —

```
$ # startup.rs:460  socket_path("regular-file") -> socket_path("some")
$ for i in 1 2 3 4 5 6; do cargo test -p goad --test renderer listener:: ; done
test result: ok.     3 passed; 0 failed
test result: ok.     3 passed; 0 failed
test result: ok.     3 passed; 0 failed
test result: ok.     3 passed; 0 failed
test result: FAILED. 2 passed; 1 failed
  thread 'startup::listener::some_path_binds' panicked at startup.rs:445:5
test result: FAILED. 2 passed; 1 failed
  thread 'startup::listener::some_path_binds' panicked at startup.rs:445:5
```

Two failures in six, with no panic from `claim` and no name in the message — the
`"vt8"` shape exactly, in the binary the instrument was added to. The control is
the claimed helper in the same run: pointing two `ingress.rs` cases at `"vt1"`
gives

```
two cases in one test binary claimed the socket name "vt1". Such a path is
qualified by pid, not by case, so the two share one file and whichever case
starts second clears it from under the first. Give one of them a name of its own.
```

Both edits reverted; the tree is as found.

What is **not** wrong, measured rather than assumed, so the repair is not
re-litigated: the `(kind, name)` key is right for the two kinds that exist. Every
marker name in the `renderer` binary reaches `marker` through `logging_backend`'s
`invocations-` prefix (`scripting.rs:133`), so marker paths are `goad-invocations-*`
and socket paths `goad-serve-*.sock`; even the adversarial pair
`marker("serve-x")` / `socket_path("x")` mints `goad-serve-x-<pid>` and
`goad-serve-x-<pid>.sock`, which differ. `CLAIMED` is per process, which is the
scope pid-qualification gives a collision, and `claim` is unreachable from any
non-test build — `scripting.rs` is `#[path]`-included by test `main.rs` files
only. Both call sites claim **before** they clear, so the panic fires ahead of
the destructive `remove_file` rather than after it.

One thing the fix must not do: give `startup.rs` the kind `"socket"`. Its paths
are `goad-startup-…` and `ingress.rs`'s are `goad-serve-…`, so a shared kind
would report a collision where there is none — the exact error the `(kind, name)`
key exists to prevent. A kind is per **helper**, not per concept.

**Disposition:** `fix-now`. The finding is right on both halves, and the second
half is the one that matters: the instrument's own doc states a reach it does not
have, which is the defect class F-8 and F-12 name. `follow-up` was considered and
rejected against the Guardrails — the fix is three lines, and "the fix is large"
is the only reason that disposition exists to refuse.

**Response:** `claim` now holds all four helpers, and the two false statements
are corrected rather than narrowed away.

- `crates/goad/tests/renderer/startup.rs` — `mod listener::socket_path` claims,
  under the kind `"startup socket"`.
- `crates/goad-shell/tests/integration/ingress.rs` — `socket_path` claims, under
  `"ingress socket"`, reaching `claim` through `harness`'s re-export, which is
  how that target already names `marker` and `clear`.
- `crates/goad/tests/renderer/ingress.rs` — the kind `"socket"` is renamed
  `"serve socket"`. The finding's warning is the reason: a kind that names a
  *concept* is a kind two helpers will eventually share, and a shared kind
  reports a collision between paths that differ. Every kind now names the prefix
  its helper mints, so the key is 1:1 with the helper by construction and the
  rule is legible at the call site.
- `tests/support/scripting.rs` — the `claim` doc no longer says "every"; it
  names the four helpers it holds and states the kind rule, with the four
  prefixes written out so a fifth helper has a worked example rather than a
  principle. `ingress.rs:109`'s "a second helper" is corrected to name all four.

The alternative key was weighed and not taken: `claim` could take the **minted
path** and drop `kind` entirely, which removes the wrong-kind trap by
construction. It was rejected for this repair because it is a redesign of an
instrument during audit closure, and because it would undo F-10 — the call site
spells `socket_path("some")` and a path key would put `/tmp/goad-startup-some-…`
in the panic where F-10 asked for the string the call site spells. Recorded here
so the option is on the record rather than re-derived.

**Outcome:** `verified`. Discharged by making the defect on **both** targets and
watching the instrument catch it, deterministically rather than one run in six.

Renderer target — `startup.rs:472` `socket_path("regular-file")` → `("some")`:

```
$ cargo test -p goad --test renderer listener::
thread 'startup::listener::some_path_that_is_a_regular_file_names_the_path' panicked at
  tests/support/scripting.rs:82:3:
two cases in one test binary claimed the startup socket name "some". Such a path is
qualified by pid, not by case, so the two share one file and whichever case starts
second clears it from under the first. Give one of them a name of its own.
test result: FAILED. 2 passed; 1 failed
```

Shell integration target — `ingress.rs:340` `socket_path("vt2-live")` →
`("vt1-reclaim")`:

```
$ cargo test -p goad-shell --test integration ingress::
thread 'ingress::a_stale_socket_with_no_listener_is_reclaimed_and_the_new_one_serves'
  panicked at tests/support/scripting.rs:82:3:
two cases in one test binary claimed the ingress socket name "vt1-reclaim". …
test result: FAILED. 35 passed; 1 failed
```

Both reverted. The absence of a false positive is the other half and is carried
by the green run itself: `startup`'s `"some"` and `ingress`'s `"vt1-reclaim"` are
live in one binary under different kinds, and the renderer target is 184 green.

### F-12 — "once per row of a thirty-three-row loop": measured, it is twice, for rows S1 and S2

**Severity:** minor
**Location:** `tests/support/scripting.rs:60-62`;
`crates/goad/tests/renderer/table.rs:555-558`

**Expected:** `table.rs` does not state its row count in prose — it pins it with
an assertion, `the_array_has_exactly_the_rows_design_md_states` (`:615-619`,
`assert_eq!(CASES.len(), 33)`), because a count in a comment is a claim nothing
checks. A comment that states a *measured* frequency is the same kind of claim,
and F-8 established in this ledger that a comment asserting more than it holds is
the same defect as a test asserting more than it holds.

**Observed:** `inert_answer` is awaited in exactly one place — the loop's
`Turn::RespondFabricated` arm (`table.rs:653-660`) — and exactly two of the 33
rows carry that turn (`:231`, `:352`). It is called **twice**, not thirty-three
times. Both statements of the frequency are wrong in the same way:

- `table.rs:555` — *"Takes the row's id because it is called once **per row**"*;
- `scripting.rs:60-62` — *"A second, silent pair (`table.rs`'s
  `"table-inert-option"`, **once per row of a thirty-three-row loop**)"*, which
  contradicts *"pair"* in its own sentence.

Nothing about the repair is wrong: two calls are all a collision needs, the
`-{row}` suffix is the right fix, and `row` is unique per case. What is wrong is
that a reader is told the helper mints thirty-three paths per run when it mints
two, and the `claim` doc is the workspace's standing explanation of the
instrument.

**Evidence:**

```
$ # table.rs: println!("PROBE inert_answer called for row {row}"); at :559
$ cargo test -p goad --test renderer every_failure_in_the_taxonomy -- --nocapture
PROBE inert_answer called for row S1
PROBE inert_answer called for row S2
test result: ok. 1 passed; 0 failed
```

Reverted. Corroborating: `grep -n 'Turn::RespondFabricated' crates/goad/tests/renderer/table.rs`
gives `:231` and `:352` inside `CASES`, and `:653`/`:677` in the loop's match.

**Disposition:** `fix-now`. Comment-only, and the finding's own argument carries
it: `table.rs` pins its row count with an assertion rather than a sentence
because a count in a comment is a claim nothing checks, and these two sentences
are that exact thing. That the repair itself is sound is why this is `minor` and
not higher.

**Response:** both statements now say what was measured, and both say *why* it is
two rather than leaving a reader to count.

- `crates/goad/tests/renderer/table.rs:555` — "called once **per row**" becomes
  "**once per row that reaches it** — S1 and S2, the two of the thirty-three
  carrying a `RespondFabricated` turn". The doc's first line already said S1 and
  S2 are the refused rows; the frequency sentence now agrees with it.
- `tests/support/scripting.rs:60-62` — "once per row of a thirty-three-row loop"
  becomes "minted **twice** — by the two of the loop's thirty-three rows that
  carry a `RespondFabricated` turn", which no longer contradicts *"pair"* in its
  own sentence.

**Outcome:** `verified`. There is no test to redden for a comment, and inventing
one would be a test asserting a proxy. The discharge is the measurement already
in **Evidence** above — the `PROBE` run printing two lines, S1 and S2 — and the
two corrected sentences are quoted from the tree as it now stands. `just check`
is exit 0, 537 cases across 21 binaries, with the same count as before: a
comment repair that changed a case count would be a comment repair that was not
one.

### F-13 — F-6's repair writes the accessible-description predicate a third time

**Severity:** nit
**Location:** `crates/goad/tests/renderer/tree.rs:288-291`;
`crates/goad/tests/renderer/harness.rs:105-107` and `:129-131`

**Expected:** CLAUDE.md — *"No parallel implementation! Find potential
duplication before writing new code"* and *"DRY — find out whether existing code
can be adapted"*. This review's own Brief treats a second statement of a filter
rule as a repair worth making (`accessible_enabled_of`, which *"removes a second
statement of the same filter rule"*). `harness.rs`'s module rule is *"anything
two or more case files here need lives in this file"*, and `within_option`'s own
doc says why the predicate exists at all: `ElementQuery` has no
accessible-description matcher.

**Observed:** the line
`move |element| element.accessible_description().as_deref() == Some(option.as_str())`
now appears three times — `harness.rs:105-107` (`element_described`),
`harness.rs:129-131` (`within_option`), and `tree.rs:288-291`
(`labels_in_option`, new in F-6's repair).

The third could not reuse `within_option`, and that is the point: `within_option`
fuses the predicate with `.match_descendants()`, so a caller that needs the same
predicate under a *different* scope has no choice but to restate it. Splitting
the two apart is the whole fix and leaves one statement of the rule —

```rust
fn described(option: &str) -> impl Fn(&ElementHandle) -> bool + 'static
```

with `within_option` becoming `from_root(window).match_predicate(described(option)).match_descendants()`
and `labels_in_option` becoming
`from_root(window).match_predicate(described(option)).match_accessible_role(Groupbox).match_descendants()`.
`element_described` takes it too, since its argument is a description like the
others.

Not raised higher because nothing is wrong today and the three copies are
identical. It is a nit in the same sense F-10 was: it costs a few lines, and the
rule it restates is the one `within_option`'s doc says is only needed because the
query API lacks a matcher for it.

**Disposition:** `fix-now`, and **wider than the finding**. The diagnosis is
exactly right — `within_option` fuses the rule with one scope, so every caller
needing a different scope must restate it — but three is an undercount, and the
Guardrails say fix the class. `grep 'accessible_description().as_deref()'` over
`crates/goad/tests/` found **five**, not three: the two the finding names in
`harness.rs`, `tree.rs`'s new one, plus `harness.rs::field_described` (the same
rule against a *field* description) and `wiring.rs::accessible_enabled_of`.

The fifth is the one worth naming. `accessible_enabled_of` did not restate the
predicate alone — it restated the whole of `element_described`, type filter and
all, and its doc said so in as many words: *"`tree.rs`'s `element_described` took
the same filter for the same reason."* A comment noting that two functions are
the same function is a duplication that has already been seen and left. It is
also the helper this review's own Brief listed as a cheap refactor with no
criterion behind it, so it is closed here rather than carried.

**Response:** one statement of the rule, five consumers.

```
harness::described(description) -> impl Fn(&ElementHandle) -> bool
  ├── harness::element_described   .match_inherits("Button") + find_first
  │     └── wiring::accessible_enabled_of   … .and_then(accessible_enabled)
  ├── harness::within_option       .match_descendants
  │     └── harness::field_described        … .match_predicate(described(field))
  └── tree::labels_in_option       .match_accessible_role(Groupbox) + descendants
```

- `harness.rs` — `described` is new and carries the doc for *why the predicate
  exists at all* (`ElementQuery` has no accessible-description matcher;
  `search_api.rs:232-287` lists its six builders). `within_option`'s doc sheds
  that paragraph and keeps its own subject: it is the **scope**, `described` is
  the **rule**. `element_described` and `field_described` call it.
- `tree.rs` — `labels_in_option`'s inline closure becomes `described(option)`.
  Its doc is untouched: every word of it is about the *role* scope and why that
  scope rather than `within_option`, which is still true.
- `wiring.rs` — `accessible_enabled_of` collapses to
  `element_described(window, description).and_then(|e| e.accessible_enabled())`,
  and its doc now says the filter is not its to restate, pointing at the helper
  that carries it. The now-unused `ElementQuery` import goes with it.

**Outcome:** `verified`. Discharged the way a shared helper has to be — by
breaking the single statement and checking that every consumer feels it. Reading
the accessible **label** instead of the description:

```
$ # harness.rs:121  accessible_description() -> accessible_label()
$ cargo test -p goad --test renderer
test result: FAILED. 167 passed; 17 failed
      4 fields::…      8 tree::…      5 wiring::…
```

Seventeen cases across all three modules, including
`wiring::editing::both_controls_are_disabled_while_an_exchange_is_in_flight…` —
the case that reaches `described` through `accessible_enabled_of`, which is the
consumer the finding did not name. Reverted; 184 green.

The count is the discharge's point. Before the repair the same mutation reached
only the copies it was applied to; after it, one statement, so it reaches every
consumer.

> **Amended at F-19, and the original left visible rather than edited away, per
> F-7.** This paragraph first read: *"Before the repair the same mutation
> reddened **14** in two modules; after it, 17 in three. The three additional
> cases are the two call sites this response added to the finding's list."*
> Both halves are wrong. **14 in two modules is not a number this suite can
> produce** — F-19 settles it by arithmetic, without reconstructing anything:
> the post-repair set of 17 spans `fields` (4), `tree` (8) and `wiring` (5), the
> pre-repair set is a subset of it, and the largest subset spanning two modules
> is 13. Measured against a reconstruction of the pre-repair tree, the real
> before-count is **13, in three modules**. The attribution is wrong in the way
> that matters more: of the two call sites this Response added to the finding's
> list, `wiring::accessible_enabled_of` supplies three of the four new cases and
> `harness::field_described` supplies **none** — a field query's option scope
> already runs through `within_option`, so those cases redden whether
> `field_described` restates the rule or not. The fourth new case arrives
> through `labels_in_option`, which the *finding* named and this Response did
> not add. The repair is sound and the grep is one line; it was the record of
> how it was discharged that asserted what it had not measured.

`grep -rn 'accessible_description().as_deref()' crates/goad/tests/` now returns
one line, `harness.rs:121`.

### F-14 — `present`'s contract says it writes **every** property, and one declared property is never written

**Severity:** nit
**Location:** `crates/goad/src/glass.rs:19-28` (the `Glass::present` contract);
`crates/goad/ui/app.slint:180`

**Raised by:** `review-code-r1`, out of band, §4. Stated here as r1 stated it.

**Expected:** `Glass`'s doc is unusually strong and deliberately so — *"Total,
and the only method: writing every property, every call, is the design's answer
to a display server that fails partway through an update"* — and `present`'s own
line repeats it in bold: *"Write **every** property from the frame."* The audit's
own I-5/S-9 check reads that contract as a checklist, matching all nine of
`PromptWindow`'s `in property` declarations to unconditional setters.

**Observed:** `Tray` declares `in property <bool> shown: true` and binds
`visible: root.shown`. `present` writes `set_image` and `set_hover_text` and
never writes `shown`. It is pre-existing and almost certainly intended — `shown`
exists so `visible` is a *binding* rather than a literal the compiler can fold
into a constant, which is E-4's panic trap, closed by measurement at F-28 — but
the contract does not carve it out, so a reader applying the contract as written
finds a property with no writer and cannot tell an exemption from an omission.

**Evidence:** `grep -n 'set_shown\|\.shown' crates/goad/src/` returns
`glass.rs:79` and `:116`, both `frame.shown` — the frame's field of that name,
not the tray's property. No setter for `Tray::shown` exists anywhere in the
renderer.

**Disposition:** `fix-now`, doc only. Nothing is wrong at runtime and `shown`
must stay — deleting it and writing `visible: true` is what re-arms E-4. What is
wrong is a contract that asserts more than it holds, which is F-8's and F-12's
class exactly, and this slice has repaired that class twice already.

**Response:** `present`'s doc now says *"every property **the frame carries a
value for**"* and names the exception with its reason: `Tray::shown` exists so
`visible` is a binding rather than a foldable constant (E-4, F-28), the frame
carries no value for it because the tray is present for the life of the process,
and a writer for it would be a new frame field with nothing to put in it. The
trait-level sentence is unchanged — it is about totality over what is written,
and that is still true.

**Outcome:** `verified`. A doc repair, so the discharge is that the exemption is
now checkable against the markup: `Tray` declares three `in property`s, `present`
writes two, and the doc names the third and why. Gate exit 0, 537 cases.

### F-15 — `build.rs`'s `SLINT_STYLE` fallback covers unset but not empty, and `with_style("")` fails the build

**Severity:** minor
**Location:** `crates/goad/build.rs`

**Raised by:** `review-code-r1`, out of band, §4. r1 recorded it as untested —
*"nobody has tested what the compiler does with it"* — and the audit measured it
rather than promoting the guess.

**Expected:** the module doc states the read's whole purpose: the style is *"a
**default**, not a fixture: `SLINT_STYLE=fluent` still reverts it"*, and the
explicit read exists because the one-line form *"would silently remove an
override that works today."* The value of an override mechanism is that the
override arm and the default arm both work.

**Observed:** `std::env::var("SLINT_STYLE").unwrap_or_else(|_| "material".into())`
falls back on `Err` only. `SLINT_STYLE=` is `Ok("")`, so an empty string reaches
`with_style("")`.

**Evidence:** measured, not reasoned. `SLINT_STYLE= cargo build -p goad` before
the repair:

```
Error: CompileError(["Style  is not known. Use one of the builtin styles
  [cosmic, cupertino, qt, fluent, material, …] or make sure your custom style is
  found in the include directories",
 "…/app.slint:1: Cannot find requested import \"std-widgets.slint\"…",
 "…/app.slint:49: Unknown element 'ScrollView'",
 "…/app.slint:56: Unknown element 'Button'",
 "…/app.slint:116: Unknown element 'CheckBox'",
 "…/app.slint:137: Cannot access id 'Palette'"])
```

Not a silent fallback and not a graceful one: the style is rejected, and with it
the `std-widgets` import, so every widget in the markup is then an unknown
element. The first line of a six-line cascade is the real cause and the double
space in *"Style  is not known"* is the empty value.

**Disposition:** `fix-now`. One line, and the measurement settles the argument
about whether it matters. `SLINT_STYLE=` is how a shell unsets a variable for one
command without unsetting it — a reasonable thing to type, and it currently fails
the build with six errors that name the markup rather than the variable.

**Response:** `.ok().filter(|style| !style.is_empty()).unwrap_or_else(…)`. An
empty override is an override that was never made, so it now takes the default
the same way an absent one does. The module doc records the measurement, so the
next reader does not re-run it.

**Outcome:** `verified`. All three arms checked against the repaired tree:

```
$ SLINT_STYLE=  cargo build -p goad    Finished  (was: six compile errors)
$ SLINT_STYLE=fluent cargo build -p goad    Finished  (the override still reverts it)
$ SLINT_STYLE=nonsense cargo build -p goad  Style nonsense is not known
```

The third is the one that keeps the repair honest: an *invalid* style must still
fail loudly, and only the empty case is treated as unset.

### F-16 — a backend-authored `group` string reaches `Text` unescaped and unbounded

**Severity:** minor
**Location:** `crates/goad/ui/app.slint:111-114`; the class also at `title` and
`option.label`

**Raised by:** `review-code-r1`, out of band, §4.

**Expected:** every diagnostic line in this host is bounded before it is drawn —
`diagnostics.rs` passes six separate compositions through `finish(.., LINE_LIMIT)`
at a 1024-byte limit. A backend-authored string is a backend-authored string.

**Observed:** `block.heading`, which is a backend's `group` hint value carried
through `view_model.rs:188`, binds straight to a `Text` with no bound. The same
is already true of `title` and `option.label`, so this is a third member of a
pre-existing class of two, not a new hole. Nothing here is an injection risk —
Slint's `Text` renders a string as a string and interprets no markup — so the
exposure is layout: a backend can hand the window a heading of arbitrary length.

**Disposition:** `follow-up`, and the reason is the class rather than the cost.
Bounding `heading` alone would fix the instance and leave the two older members,
which is the move the Guardrails name. The real question is a rule — *does a
backend-authored string reaching the screen get the treatment a backend-authored
string reaching the diagnostics pane gets?* — and it is not this slice's to
settle: the window's own sizing is already a known open defect owned by 008
(F-6), and a heading bound is a decision about the look, which `slice-007.md`
§Non-goals puts there.

**Response:** carried to `slice-007.md` Follow-ups as a class — every
backend-authored string that reaches the screen — naming all three sites, not
just the new one. No code change in this slice.

**Outcome:** `verified`. Landed in Follow-ups rather than left in a disposition,
which is what the Protocol requires of `follow-up`.

### F-17 — the class F-11 closed is open one crate over: `goad-emit`'s `socket_path` is a fourth such helper, unclaimed, and three of the repair's docs assert a completeness that excludes it

**Severity:** minor
**Location:** `crates/goad-emit/tests/binary/exchange.rs:11-19`; the three
completeness claims at `tests/support/scripting.rs:46-51`,
`crates/goad-shell/tests/integration/ingress.rs:40-43` and
`crates/goad/tests/renderer/ingress.rs:108-111`

**Expected:** F-11's own standard, which is F-9's turned on F-5's repair: the
class is *"a per-process path helper that promises uniqueness a test binary must
not breach, held by nobody"*, and `table.rs`'s non-colliding name was repaired
anyway because it is *"the same breach of the same rule"*. F-11's Response then
states its reach as fact — *"`claim` now holds **all four** helpers"* — and
`claim`'s doc enumerates them by path so *"a fifth helper has a worked example
rather than a principle"*.

**Observed:** there is a fifth `fn socket_path` in this workspace's test code and
a **fourth** that makes the promise:
`crates/goad-emit/tests/binary/exchange.rs:11-19`, whose doc is *"A socket of
this case's own, unlinked first"* and whose mechanism is the other three's line
for line — `temp_dir()` + case + pid + `.sock`, cleared at handout. It has six
call sites in one test binary and it does not claim.

Three statements written by F-11's repair are therefore false of the tree as
statements about the repository, in the same way F-11 found `claim`'s *"every"*
false:

- `scripting.rs:46-51` — *"the **three** `socket_path` helpers"*, enumerated by
  path, under *"the … promises in this repository's test support"*. Four helpers
  make that promise; three are named. The enumeration was written so a fifth
  helper would have a worked example, and the fourth is missing from it.
- `crates/goad-shell/tests/integration/ingress.rs:40-41` — *"**Three** helpers
  in this repository mint a socket path and all three mint a different one"*.
  That is a counted claim about the repository, not about the registry, and the
  repository has four.
- `crates/goad/tests/renderer/ingress.rs:108-111` — *"made by three more helpers
  … and **all four** are now held by the same instrument"*.

**What is different about this one, and why the disposition may not be F-11's.**
The other three sit in targets that already `#[path]`-include
`tests/support/scripting.rs`, so `claim` was in scope at zero cost.
`goad-emit`'s `binary` target does not, and including it is not free —
**measured**, not assumed. Adding

```rust
#[cfg(test)]
#[path = "../../../../tests/support/scripting.rs"]
mod scripting;
```

to `crates/goad-emit/tests/binary/main.rs` compiles and yields nine warnings:

```
$ cargo test -p goad-emit --test binary --no-run
warning: function `backend` is never used
warning: static `CLAIMED` is never used
warning: function `claim` is never used
warning: function `spelled_at_the_call_site` is never used
warning: function `marker` is never used
warning: function `clear` is never used
warning: function `logging_backend` is never used
warning: function `invocations` is never used
warning: function `scripted` is never used
warning: `goad-emit` (test "binary") generated 9 warnings
```

`dead_code` is `warn`, promoted to an error by the gate's `-D warnings` — which
is the reason `scripting.rs` exists as a separate file at all, stated in its own
module doc. So the three-line repair that closed `startup.rs` does not close this
one: the instrument would have to move to a file of its own that a target can
include without dragging the scripted-backend helpers with it. Reverted.

**Evidence:**

```
$ grep -rn 'fn socket_path' crates/
crates/goad-emit/src/main.rs:128            (production, not a test helper)
crates/goad-emit/tests/binary/exchange.rs:13
crates/goad/tests/renderer/startup.rs:421   claims "startup socket"
crates/goad/tests/renderer/ingress.rs:124   claims "serve socket"
crates/goad-shell/tests/integration/ingress.rs:44  claims "ingress socket"
$ grep -n 'socket_path(' crates/goad-emit/tests/binary/exchange.rs
13: fn socket_path(case: &str) -> PathBuf {
103,127,154,182,217,249:  "accepted" "refused" "too-soon" "unreachable"
                          "non-conforming" "normalizes"
```

Six names, distinct, in one binary: nothing collides today. That is exactly
`table.rs`'s position when F-5 repaired it and `ingress.rs`'s when F-9 did.

Also unclaimed and recorded so they are not re-found, but **not** the same class
and not raised as one: `startup.rs:492`'s `goad-startup-none-<pid>` directory and
`crates/goad-shell/tests/integration/ingress.rs:590`/`:636`'s two inline
directories. Each is minted at a single call site inside the one case that uses
it, and none of them makes a uniqueness promise in prose that a second caller
could believe.

**Disposition:** `fix-now`, **docs only**, with the split carried to Follow-ups.
User decision, 2026-09-15.

The finding is right that the class is open and right that F-11's three counted
claims are false. It is also the first finding in this ledger whose fix has a
*measured* obstacle rather than a cost: the repair that closed `startup.rs` is
three lines because that target already includes `scripting.rs`, and
`goad-emit`'s does not. Closing it properly means moving `claim`, `CLAIMED` and
`spelled_at_the_call_site` into a support file of their own that a target can
include without dragging the scripted-backend helpers along — a new file, an
edit to every including target's `main.rs`, and a decision about
`spelled_at_the_call_site`, which strips `logging_backend`'s `invocations-`
prefix and so would couple the new file back to the one it left.

That is a small design question, and taking it at audit closure is how a fourth
round becomes a fifth. What is **not** deferred is the false prose: three
statements assert a completeness the tree does not have, and leaving those
standing is the defect F-11 itself was raised about.

**Response:** the three claims now say what is true, and say where the gap is
rather than eliding it.

- `tests/support/scripting.rs` — *"**Five helpers make that promise and this
  holds four**"*, the four named, and the fifth named with its reason: its
  target does not `#[path]`-include this file, and adding the include costs nine
  `dead_code` warnings, which the gate's `-D warnings` makes errors. The reason
  is mechanical and the doc says so, so a reader does not take it for an
  oversight and close it with a line that fails the gate.
- `crates/goad/tests/renderer/ingress.rs` — "three more helpers … all four"
  becomes the four named, "held by the same instrument for all but the last,
  whose target cannot include the instrument without failing the gate".
- `crates/goad-shell/tests/integration/ingress.rs` — "Three helpers in this
  repository mint a socket path" is dropped for the fact it was reaching for,
  with `goad-emit`'s marked unheld inline.

Carried to `slice-007.md` §Follow-ups as its own item: the instrument reaches
four of five, and the fifth needs `claim` extracted to a support file of its own.

**Outcome:** `verified`. The gap is now stated in the three places a reader would
look, and the finding's own measurement is why the cheap fix is unavailable:

```
$ # + #[path = "../../../../tests/support/scripting.rs"] mod scripting;
$ cargo test -p goad-emit --test binary --no-run
warning: `goad-emit` (test "binary") generated 9 warnings
```

`just check` exit 0, 537 cases across 21 binaries, unchanged — as a doc repair
must be.

### F-18 — "the thirty-three call sites below": measured, there are 35, and 33 is the number F-12 was raised about

**Severity:** minor
**Location:** `crates/goad-shell/tests/integration/ingress.rs:33-35`

**Expected:** the rule this ledger established at F-12 and applied again at F-14:
*"a count in a comment is a claim nothing checks"*, which is why `table.rs` pins
its thirty-three rows with `assert_eq!(CASES.len(), 33)` rather than a sentence.
F-12 was raised against one wrong count in a comment and repaired two rounds ago;
F-11's repair wrote a new one.

**Observed:** the new doc reads *"held by `claim` rather than by the
**thirty-three** call sites below agreeing"*. There are **35**. The number
written is `table.rs`'s row count — the number F-12 is about — and it entered
through F-11's own finding text (*"over **33** call sites"*), which the Response
carried into the tree without checking.

Nothing is wrong with the repair: `claim` holds all 35 and the gate is green.
What is wrong is that the sentence arguing for the instrument states the size of
the problem it solves, and states it wrong, in the file where that argument is
strongest.

**Evidence:**

```
$ grep -c 'socket_path("' crates/goad-shell/tests/integration/ingress.rs
35
$ grep -o 'socket_path("[^"]*")' crates/goad-shell/tests/integration/ingress.rs \
    | sort -u | wc -l
35
```

Thirty-five sites, thirty-five distinct names. It is not thirty-five *cases*: two
cases mint two paths each — `vt3b-symlink-target` / `vt3b-symlink` (`:447-448`)
and `vt6b-clock` / `vt6b-shutdown` (`:810`, `:825`) — which is a fact the
sentence could carry instead of a number that drifts.

**Disposition:** `fix-now`. The finding also diagnoses how it happened, and the
diagnosis is the useful part: the number came out of **F-11's own finding text**
and was carried into the tree by the Response without being checked. A count
that arrives from a document rather than from the code is a count nobody
measured, whoever wrote it down first.

**Response:** the sentence now carries the fact rather than the number, which is
the finding's own suggestion and the only version that cannot drift as cases are
added:

> *"held by `claim` rather than by every call site below agreeing. There are
> more of them than there are cases, which is the point: two cases mint two
> paths each (`vt3b-symlink-target` / `vt3b-symlink`, and `vt6b-clock` /
> `vt6b-shutdown`), so "one per case" is not a rule anyone could check by
> eye."*

That is strictly stronger than "35". The argument for the instrument was never
*there are many call sites* — it is *the obvious invariant is false*, and a
reader who believes one path per case is a reader who would not look for a
collision.

**Outcome:** `verified`. No count remains that can be wrong: `grep -c
'socket_path("'` is 35 today and the sentence no longer depends on it. The two
double-minting cases are `vt3b-symlink-target` / `vt3b-symlink` and `vt6b-clock`
/ `vt6b-shutdown`, named rather than numbered — this Outcome first cited them at
`:447-448` and `:810`/`:825`, which was already wrong when written, because
this round's own repairs had moved them (F-24). A line number in an Outcome is
the very thing this finding was raised about; the symbol is what survives an
edit.

### F-19 — F-13's Outcome reports a before-count that cannot be true of any tree, and credits three cases to a call site that contributes none

**Severity:** minor
**Location:** `review-code.md` F-13 §Outcome; the code it describes at
`crates/goad/tests/renderer/harness.rs:119-122`, `:156-164` and
`crates/goad/tests/renderer/wiring.rs:78-80`

**Expected:** the Protocol makes this file the record of a finding's fate, and
F-7 is this ledger's own precedent for the defect: *"a ledger that edits away its
own wrong record is worth less than one that shows it"* — raised because a
Response described a repair that was not in the tree. A discharge stated in
numbers is a claim in numbers.

**Observed:** F-13's Outcome states two counts and one attribution:

> Before the repair the same mutation reddened **14** in two modules; after it,
> 17 in three. The three additional cases are the two call sites this response
> added to the finding's list …

The after-count is right — measured below, 17 across `fields` (4), `tree` (8),
`wiring` (5). The before-count cannot be right, and the arithmetic settles it
without reconstructing anything: post-repair the predicate is one statement, so
the mutation reaches **every** consumer; pre-repair it reached whichever copies
were edited, a subset. The reddened set can therefore only have grown, so the
before-set is a subset of those same 17 cases — and the largest subset of them
spanning **two** modules is `tree` + `wiring` = 13. **14 in two modules is not a
number this suite can produce.**

Reconstructed and measured, the before-count is **13, in three modules**, and the
attribution is wrong in a way that matters more than the number. Of the two call
sites the Response added to the finding's list, `wiring::accessible_enabled_of`
supplies three cases and `harness::field_described` supplies **none** — a field
query's option scope already runs through `within_option`, so mutating the shared
predicate reddens those cases whether `field_described` restates the rule or not.
The fourth new case is `tree::a_block_heading_reaches_the_screen_and_an_untitled_block_draws_none`,
which comes through `labels_in_option` — a call site the **finding** named, not
one the Response added.

**Evidence:** the after-count, at the tree as it stands:

```
$ # harness.rs:121  accessible_description() -> accessible_label()
$ cargo test -p goad --test renderer
test result: FAILED. 167 passed; 17 failed
    4 fields::…   8 tree::…   5 wiring::…
```

Then the pre-repair tree reconstructed — each of the five consumers carrying its
own copy of the predicate again, the two in `harness.rs` mutated and the three
elsewhere correct, which is the only reading under which "the same mutation"
means anything before a shared statement existed:

```
$ # harness::field_described        -> its own inline predicate (correct)
$ # tree::labels_in_option          -> its own inline predicate (correct)
$ # wiring::accessible_enabled_of   -> its own query, Button filter and
$ #                                    predicate (correct), not element_described
$ cargo test -p goad --test renderer
test result: FAILED. 171 passed; 13 failed
    4 fields::…   7 tree::…   2 wiring::…
```

The two `wiring` cases still red are the two that reach a field through
`field_described` — `editing::the_next_present_writes_every_control_back_from_the_draft`
and `editing::both_controls_are_disabled_while_an_exchange_is_in_flight_and_enabled_after_it`.
The three that go green are `transitions::dt3_a_failure_under_prompt_mode_leaves_the_question_in_place`,
`busy::busy_clears_and_controls_re_enable_after_a_success` and
`busy::busy_clears_and_controls_re_enable_after_a_failure`: all three reach the
predicate only through `accessible_enabled_of`. Every edit reverted; `just check`
exit 0 afterwards.

One honest limit on the second measurement: the reconstruction is this round's
rendering of the pre-repair tree, so **13** is a measurement of that
reconstruction and not of a tree anyone still has. The impossibility of *"14 in
two modules"* does not depend on it.

Nothing about F-13's repair is wrong — one grep line, 17 reds, every consumer
felt. What is wrong is the ledger's record of how it was discharged, which is the
third time in this review a Response has asserted something untrue of the tree.

**Disposition:** `fix-now`, and the correction is recorded rather than applied
silently. F-7 is this ledger's precedent and it is explicit: *"a ledger that
edits away its own wrong record is worth less than one that shows it."*

The finding's arithmetic is the part worth keeping. It disproves the number
**without** reconstructing anything — post-repair the predicate is one
statement, so the mutation reaches every consumer and the pre-repair set must be
a subset of the post-repair 17; the largest subset of `fields` (4) + `tree` (8) +
`wiring` (5) spanning two modules is 13; so "14 in two modules" is not a number
this suite can produce. That is a check the responder could have run against
their own claim in the time it took to write it.

**Response:** F-13's Outcome now carries the true account **and** the original,
quoted, marked amended, citing F-19. Both errors are named there: the impossible
before-count, and the attribution — `harness::field_described` contributes
**none** of the four new cases, because a field query's option scope already runs
through `within_option`, so those cases redden whether `field_described` restates
the rule or not. Three of the four come from `wiring::accessible_enabled_of` and
the fourth from `labels_in_option`, which the finding named rather than the
Response.

The substantive point stands and is not weakened by the correction: the repair is
sound, `grep` returns one line, and 17 cases across three modules feel a break in
the single statement. What failed was the record of it.

**Outcome:** `verified`. The finding states its own limit — the 13 is measured
against *its* reconstruction of the pre-repair tree, not a tree anyone still
has — and that limit is carried into the amendment rather than dropped, because
the impossibility argument does not need it and the measurement does.

### F-20 — `described`'s new doc counts three callers where there are four, and `element_described`'s doc still names a closed list of two case files that F-13's repair made three

**Severity:** minor
**Location:** `crates/goad/tests/renderer/harness.rs:110-113` (`described`'s
doc), `:98-100` (`element_described`'s), `:153-155` (`field_described`'s);
against the module rule at `:11-13`

**Expected:** this file's own module doc, which is a warning against exactly this:
*"The rule is *two or more*, and the set that satisfies it is **not fixed**:
`tree.rs` became a caller when `wiring.rs` needed the scoped field query too,
**which is why nothing here names a closed list of case files**."* And the class
F-8, F-12 and F-14 each name: a comment asserting more than it holds.

**Observed:** the repair that consolidated the predicate wrote a new closed list
and left a stale one, in the file that says not to.

- `described`'s doc — *"the **three** callers scope it three different ways — a
  type filter, a descendant walk, and a role filter then a descendant walk"*.
  There are **four** direct callers (`harness.rs:104`, `:139`, `:162`,
  `tree.rs:288`), and the omitted one is `field_described`, whose scope is a
  fourth shape: the predicate applied to a query `within_option` has **already**
  scoped, rather than to a fresh root. That fourth shape is the one this finding
  turns on in F-19, so it is not a decorative omission.
- the same sentence — *"a helper that fused the rule with one scope is what
  forced **the other two** to restate it"*. F-13's own Disposition counted the
  restatements and found **five**, not three, and said so in this ledger.
- `element_described`'s doc — *"Here rather than in one case file because two of
  them need it: `tree.rs` … and `fields.rs`"*. `wiring.rs` became the third in
  **this same repair**: `accessible_enabled_of` (`wiring.rs:79`) now calls it,
  which is the whole of F-13's fifth consumer.

Pre-existing and the same defect, named so the repair fixes the class rather than
the instance it created: `field_described`'s doc names `tree.rs` and `wiring.rs`;
`fields.rs` calls it at `:172` and `:228`.

**Evidence:**

```
$ grep -rn 'described(' crates/goad/tests/renderer/harness.rs
104:    .match_predicate(described(description))     # element_described
139:    .match_predicate(described(option))          # within_option
162:    .match_predicate(described(field))           # field_described
$ grep -n 'described(option)' crates/goad/tests/renderer/tree.rs
288:    .match_predicate(described(option))          # labels_in_option
$ grep -l 'element_described' crates/goad/tests/renderer/*.rs
harness.rs  fields.rs  tree.rs  wiring.rs
$ grep -l 'field_described' crates/goad/tests/renderer/*.rs
harness.rs  fields.rs  tree.rs  wiring.rs
```

Three case files each, both docs naming two. The fix wants to be the module
doc's rule rather than a corrected list: a helper in `harness.rs` is there
*because two or more case files need it*, and which two is not a fact worth
pinning in prose that nothing rereads.

**Disposition:** `fix-now`, and **as the finding prescribes** — obey the module
rule rather than refresh the counts. A corrected list is a list that goes stale
again the next time a case file needs the helper, which is what the module doc
says and why it says it.

**Response:** three docs lose their closed lists; the fourth keeps its own.

- `described` — "the three callers scope it three different ways" becomes "its
  callers scope it differently", followed by the **four** shapes, the fourth
  spelled out because F-19 turns on it: *the predicate applied to a query that
  `within_option` has already scoped*. "the other two" becomes the count F-13's
  own Disposition measured: *"the rule stood written five times before it was
  pulled out here."*
- `element_described` and `field_described` — each drops its named pair of case
  files for what the helper is *for*, with a pointer to the module rule.

Checked and left, so it is not re-found: `logging_scripted` (`harness.rs:178`)
also names two case files, and its list is **correct** — `scheduling.rs` and
`fields.rs` are its only callers. Same shape of prose, not raised, and true
today: rewriting correct text at audit closure buys nothing. If the module rule
is worth enforcing rather than stating, that is an instrument, and it has the
same shape as the `docs/memory/` citation follow-up already recorded.

**Outcome:** `verified`. `grep -rn 'because two of them need it'
crates/goad/tests/renderer/` returns one line — `harness.rs:178`, the one whose
list is true. The two corrected helpers name no file at all, so no future caller
can falsify them.

### F-21 — the kind rule the repair states four times is false of the one kind it was stated on first, and markers are separated from sockets by the `.sock` suffix rather than by the prefix the rule names

**Severity:** nit
**Location:** `tests/support/scripting.rs:54-58`;
`crates/goad/tests/renderer/ingress.rs:119-122`;
`crates/goad/tests/renderer/startup.rs:414-416`;
`crates/goad-shell/tests/integration/ingress.rs:40-43`

**Expected:** F-11's Response makes the rule the repair's load-bearing idea —
*"Every kind now names the prefix its helper mints, so the key is 1:1 with the
helper by construction"* — and writes it into all four docs, because a kind that
names a concept is *"a kind two helpers will eventually share"*.

**Observed:** `marker`'s kind is `"marker"` and the prefix it mints is bare
`goad-` (`scripting.rs:116`). There is no `goad-marker-`. The rule holds for the
three socket kinds and not for the fourth, which is the one `claim`'s own doc
lists first. The property that actually keeps the key exact is the sentence
beside it — *"One kind per helper"* — which is true of all four.

Not purely cosmetic, which is why it is stated rather than left: a **bare**
prefix is one any future kind can alias. A fifth helper minting
`goad-<k>-<name>-<pid>` under the kind `"k"` would mint the same path as
`marker("k-<name>")`, and the registry, keyed on `(kind, name)`, would report no
collision — the failure mode the key exists to trade against, reachable because
one of the four kinds names nothing. Today nothing reaches it, and the reason is
not the rule: every marker name in the two claiming binaries arrives through
`logging_backend`'s `invocations-` prefix (`scripting.rs:133`), and all three
socket kinds mint paths ending `.sock` while no marker does. Markers and sockets
are held apart by the **suffix**.

**Evidence:** the four mints, read from the tree —

```
scripting.rs:116   goad-{name}-{pid}                  kind "marker"
renderer/ingress.rs:126   goad-serve-{case}-{pid}.sock     kind "serve socket"
renderer/startup.rs:424   goad-startup-{case}-{pid}.sock   kind "startup socket"
shell/ingress.rs:46       goad-ingress-{case}-{pid}.sock   kind "ingress socket"
```

Three of the four kinds name their prefix; `"marker"` names its helper. The
adversarial pair F-11 checked — `marker("serve-x")` against `socket_path("x")` —
comes out distinct for the suffix, not for the prefix: `goad-serve-x-<pid>` and
`goad-serve-x-<pid>.sock`.

**Disposition:** `fix-now`, doc only. A `nit` by severity and not by kind: the
sentence is the repair's stated reason for existing, it was written into four
files, and it is false of the kind listed first in the instrument's own doc.
F-11's Response argued the rule was what made the key *"1:1 with the helper by
construction"* — the construction is real, but it is not the one the sentence
names.

**Response:** all four copies now state the property that is actually true and
load-bearing — **one kind per helper** — and `claim`'s doc, the workspace's
standing explanation of the instrument, carries the rest of the finding: that
`marker`'s prefix is the bare `goad-` every path shares, that what separates a
marker's paths from a socket's is the **`.sock` suffix**, and the consequence the
finding draws out —

> *"A fifth kind must therefore be checked against the paths the others mint, not
> assumed distinct because its name is."*

That last sentence is the finding's real content and is why this is written out
rather than quietly corrected. The failure it describes is reachable: a helper
minting `goad-<k>-<name>-<pid>` under kind `"k"` mints the same path as
`marker("k-<name>")`, and the registry, keyed on `(kind, name)`, reports no
collision. Nothing reaches it today — but a reader following the old rule would
have concluded it was impossible rather than merely unreached.

> **Amended at F-23, original shown rather than edited away, per F-7 and F-19.**
> The sentence above ended: *"Nothing reaches it today — every marker name in the
> two claiming binaries arrives through `logging_backend`'s `invocations-`
> prefix"*. False. `crates/goad-shell/tests/integration/transport.rs:316` calls
> `harness::marker("broken-pipe")` directly, in a claiming binary, minting a bare
> `goad-broken-pipe-<pid>` with no `invocations-` in it. F-11's version of the
> sentence was scoped to *the `renderer` binary* and is true; widening it to
> *"the two claiming binaries"* is what made it false — the fourth time in this
> review a Response has asserted something untrue of the tree, and the second
> where the falsehood was introduced by generalising a true statement.
>
> **The conclusion survives and the argument for it does not,** which is F-23's
> own phrasing and the reason it is `minor`. What holds markers apart from
> sockets is not the prefix: `marker` appends the pid **after** the name, so a
> marker path always ends in digits and can never end `.sock`, whatever it is
> named. That reason is stronger than the one it replaces because it does not
> depend on any call site's naming, and `"broken-pipe"` is the standing proof
> that the `invocations-` prefix was never load-bearing. The code doc was
> unaffected — it already said the `.sock` suffix — and now carries the
> pid-last reason instead.

**Outcome:** `verified`. The four mints are read from the tree into `claim`'s doc,
so the next reader checks a list rather than a principle:

```
scripting.rs           goad-{name}-{pid}                 kind "marker"
renderer/ingress.rs    goad-serve-{case}-{pid}.sock      kind "serve socket"
renderer/startup.rs    goad-startup-{case}-{pid}.sock    kind "startup socket"
shell/ingress.rs       goad-ingress-{case}-{pid}.sock    kind "ingress socket"
```

Gate exit 0, 537 cases across 21 binaries.

### F-22 — "Five helpers make that promise": measured, six do, and the sixth collides destructively at one run in six

**Severity:** minor
**Location:** `tests/support/scripting.rs:51-56`;
`crates/goad/tests/renderer/ingress.rs:109-113`; the unheld helper at
`crates/goad-emit/src/main.rs:178-193`

**Expected:** F-17's own standard, which is F-11's turned on F-11's repair: a
counted claim about the repository is a claim nothing checks, and `claim`'s doc
enumerates its helpers by path expressly *"so a fifth helper has a worked
example rather than a principle"*. F-17 found `claim`'s *"every"* false by
grepping `fn socket_path`; the class it was raised on is not a name, it is *"a
per-process path helper that promises uniqueness a test binary must not breach,
held by nobody"*. F-17 also states the two grounds on which it excluded three
other unclaimed paths: each *"is minted at a single call site inside the one case
that uses it, and none of them makes a uniqueness promise in prose that a second
caller could believe."*

**Observed:** a helper of this class need not be called `socket_path` and need
not mint a socket. `crates/goad-emit/src/main.rs:181`'s `config_home` mints
`std::env::temp_dir()/goad-emit-{case}-{pid}`, is called from **five** cases in
one test binary (`:215`, `:246`, `:256`, `:266`, `:276`), and makes the promise
in the same words F-17 accepted from `exchange.rs` — *"A configuration directory
of this case's **own**, under `temp_dir()`"* against *"A socket of this case's
**own**, unlinked first"*. Neither of F-17's two exclusion grounds reaches it.

It is also destructive in `marker`'s exact way rather than merely untidy: the
helper **writes or removes** `config.toml` under the minted directory at
handout, so two cases sharing a name have one of them clear or overwrite the
other's fixture mid-run. That is `clear`-at-handout, one crate over.

So three statements are short by one, and the third is the one the follow-up
rests on:

- `scripting.rs:51` — *"**Five** helpers make that promise and this holds four"*.
  Six make it.
- `crates/goad/tests/renderer/ingress.rs:109` — *"made by **three more**
  helpers"*, enumerated. Four more.
- `slice-007.md` §Follow-ups — *"`claim` reaches four of the **five** helpers
  that promise a unique temp path"*, which is the item carrying the extraction
  work forward. Raised here because F-17's Response put the number there; the
  artefact is the audit's to edit, not this ledger's.

Its target is `goad-emit`'s **bin unit-test** binary, so the obstacle F-17
measured applies to it and then some: `src/main.rs` cannot `#[path]`-include
`tests/support/scripting.rs` at all without the same nine `dead_code` warnings
and a `#[cfg(test)]` module to hang them on. Nothing about the follow-up's
*work* changes; what changes is that the count in it, and in the two docs, is
wrong, and the enumeration written so a future author has a worked example omits
the one helper that is not a socket.

**Evidence:** the full set of pid-qualified temp paths in the workspace, so the
sixth is found by the class rather than by the name:

```
$ grep -rn 'process::id()' --include=*.rs . | grep -v '^./target'
crates/goad-emit/tests/binary/exchange.rs:14   goad-emit-{case}-{pid}.sock   unheld, named at F-17
crates/goad-emit/src/main.rs:182               goad-emit-{case}-{pid}        unheld, NOT named
crates/goad-shell/tests/integration/ingress.rs:49   claims "ingress socket"
crates/goad/tests/renderer/startup.rs:426           claims "startup socket"
crates/goad/tests/renderer/ingress.rs:129           claims "serve socket"
tests/support/scripting.rs:130                      claims "marker"
  … plus four single-call-site inline paths and one argv marker, each excluded
    on F-17's own two grounds
$ grep -n 'config_home(' crates/goad-emit/src/main.rs
181: fn config_home(case: &str, contents: Option<&str>) -> PathBuf {
215,246,256,266,276:  "socket-wins" "absent" "unparseable" "no-ingress" "configured"
```

Five names, distinct, in one binary: nothing collides today — `table.rs`'s
position at F-5, `ingress.rs`'s at F-9 and `exchange.rs`'s at F-17.

The collision is destructive and the instrument is silent for it. Pointing
`an_absent_configuration_file_is_a_named_fault` at `no-ingress`'s name —
`src/main.rs:246` `config_home("absent", None)` → `config_home("no-ingress", None)`:

```
$ for i in 1 2 3 4 5 6; do cargo test -p goad-emit --bins; done
test result: FAILED. 33 passed; 1 failed
test result: ok. 34 passed; 0 failed      (×5)

thread 'tests::a_configuration_with_no_ingress_section_is_a_named_fault' panicked at
  crates/goad-emit/src/main.rs:269:5:
Unreadable { path: "/tmp/goad-emit-no-ingress-1823075/goad/config.toml",
             fault: Os { code: 2, kind: NotFound, … } }
```

One failure in six, no panic from `claim`, no name in the message, and either
case can be the one that loses — the `"vt8"` shape exactly, in the one binary
whose helper nothing has named. Reverted; `md5sum` against the pre-mutation copy
matches and `just check` is exit 0 at 537 cases across 21 binaries.

**Disposition:** `fix-now`, docs, **and the class rather than the count**. User
decision, 2026-09-16.

The finding is right three times over, and the third is the one that matters. It
is right that six make the promise; right that `config_home` is destructive in
`clear`'s exact way, which makes it the same stake as F-5 and not a tidiness
point; and right that F-17's enumeration was the defect rather than its count.
F-17 grepped `fn socket_path` and wrote down what it found. That is enumerating
instances, and a list of instances is short the moment a helper of the class
does not look like the others — which is exactly how this one, in a `src/`
file, minting a directory rather than a socket, went unseen by a round that had
just been raised about an incomplete enumeration.

Not repaired in code, for F-17's reason and then some: `goad-emit`'s bin
unit-test binary cannot `#[path]`-include `scripting.rs` without the nine
`dead_code` warnings *and* a `#[cfg(test)]` module to hang them on. The
extraction is the follow-up; what is not deferred is prose that is wrong.

**Response:** `claim`'s doc leads with the **class** and the grep that
enumerates it, and the list follows as an instance of it rather than as the
definition:

> *A helper of this class mints a path under `std::env::temp_dir()` qualified by
> the process id, promises in prose that it is the caller's own, and clears or
> overwrites what is there when it hands the path out. It need not be called
> `socket_path`, need not mint a socket, and need not live under `tests/` — one
> is a `#[cfg(test)]` helper inside a `src/` file.*
>
> ```text
> grep -rn 'process::id()' --include=*.rs . | grep -v '^./target'
> ```

with each hit falling into one of three buckets — a helper of this class, an
inline path minted at the single call site inside the one case that uses it, or
a production path — so the exclusion F-17 applied by judgement is now stated as
a rule anyone can apply.

The counts follow: **six make the promise, four are held**, both unheld ones
named, `config_home`'s destructiveness spelled out, and the reason for both
given as mechanical. `renderer/ingress.rs` goes from "three more helpers" to
four and points at `claim`'s doc for the class. `slice-007.md` §Follow-ups —
which F-22 correctly says is the audit's to edit and not this ledger's — is
rewritten around the same point: *"whoever does the work should start from the
grep, not from the six named here."*

**Outcome:** `verified`. The finding's own measurement is the discharge and is
not re-run: a deliberate collision in `goad-emit`'s bins failed one run in six,
with no panic from `claim` and no name in the message — `"vt8"`'s shape, in the
binary nothing had named. Two things checked against the repaired tree: the grep
in the doc is the one that finds all six (it returns both `goad-emit` helpers,
which `fn socket_path` does not), and the pid-last argument the doc now carries
is demonstrated by this very finding — `goad-emit`'s two helpers mint
`goad-emit-<case>-<pid>` and `goad-emit-<case>-<pid>.sock` off **one** prefix,
which is what a prefix rule would have called a collision. `just check` exit 0,
537 cases across 21 binaries.

### F-23 — F-21's Response says every marker name in the two claiming binaries carries `logging_backend`'s prefix; one does not, and it is a bare `goad-` path

**Severity:** minor
**Location:** `review-code.md` F-21 §Observed and §Response; the call site at
`crates/goad-shell/tests/integration/transport.rs:316`

**Expected:** the Protocol makes this file the record of a finding's fate, and
F-7 and F-19 are this ledger's own precedents for the defect — a Response
asserting something untrue of the tree. F-21's own subject is a rule that was
*"false of the one kind it was stated on first"*, so the reason it gives for the
replacement rule being safe today is load-bearing prose, not colour.

**Observed:** F-21's Response closes with

> *"Nothing reaches it today — every marker name in the two claiming binaries
> arrives through `logging_backend`'s `invocations-` prefix"*

and its Observed says the same. The two claiming binaries are `goad`'s
`renderer` and `goad-shell`'s `integration`. In the second,
`transport.rs:316` calls `harness::marker("broken-pipe")` **directly** —
`harness.rs:32` re-exports `marker` beside `claim` and `clear`, and
`transport.rs` is a module of that target (`main.rs:39`). It mints
`goad-broken-pipe-<pid>`: a bare `goad-` path with no `invocations-` in it.

F-11's version of the same sentence is scoped to *"the `renderer` binary"* and
is true. F-21's Response widened it to *"the two claiming binaries"*, and the
widening is what makes it false.

**The conclusion survives and the argument for it does not.** Nothing reaches
the aliasing hole today, but the reason is the one F-21 itself supplies two
sentences earlier and then does not rely on: `marker` appends the pid *after*
the name, so a marker path always ends in digits and can never end `.sock`,
whatever the name. The `invocations-` prefix is not what holds markers apart
from sockets, and `"broken-pipe"` is the standing proof that it was never
load-bearing.

**Evidence:**

```
$ grep -rn 'marker(' --include=*.rs crates tests | grep -v '^tests/support/scripting.rs'
crates/goad-shell/tests/integration/transport.rs:316:  let marker = harness::marker("broken-pipe");
$ grep -n 'pub(crate) use' crates/goad-shell/tests/integration/harness.rs
32:pub(crate) use crate::scripting::{backend, claim, clear, marker};
$ sed -n '39p' crates/goad-shell/tests/integration/main.rs
mod transport;
```

One direct caller, in a claiming binary. Corroborating, from the same sentence:
`logging_backend`'s prefix is cited as `scripting.rs:133` in F-21's Observed and
Response and is at `:156` — see F-24.

**Disposition:** `fix-now`. The fourth Response in this review to assert
something untrue of the tree, and the second where the falsehood was made by
*generalising a true statement* — F-11's sentence was scoped to the `renderer`
binary and correct; widening it to "the two claiming binaries" is the whole of
the error. Worth stating as a pattern rather than a fourth instance: the edits
that introduced falsehoods here were not careless transcriptions, they were
small strengthenings of claims that were already true.

**Response:** F-21's Response is amended in place with the original quoted and
marked, per F-7 and F-19 — the ledger shows the wrong record rather than editing
it away. The amendment carries the finding's own distinction, which is the part
worth keeping: **the conclusion survives and the argument for it does not.**

The replacement argument is stronger than what it replaces, and is the one F-21
supplied two sentences earlier without relying on: `marker` appends the pid
**after** the name, so a marker path always ends in digits and can never end
`.sock`, whatever a caller names it. That holds for every call site rather than
for the ones that happen to route through `logging_backend`, and
`transport.rs:316`'s bare `marker("broken-pipe")` is the standing proof that the
`invocations-` prefix was never doing the work.

The code doc needed no correction — it already said the `.sock` suffix — but it
now carries the pid-last reason instead, since "the suffix" is a fact about the
paths and "the pid comes last" is the reason the fact holds.

**Outcome:** `verified`. `grep -rn 'marker(' --include=*.rs crates tests` returns
one direct caller outside `scripting.rs` — `transport.rs:316` — and
`goad-broken-pipe-<pid>` contains no `invocations-`, which is the finding's
evidence and reproduces. Gate exit 0.

### F-24 — three `path:line` citations written by round 4 do not resolve, one of them false at the moment it was written, all three moved by round 4's own repairs

**Severity:** nit
**Location:** `review-code.md` F-18 §Outcome, F-19 §Location, F-21 §Observed and
§Response

**Expected:** F-18's own repair is the rule — *"the sentence now carries the fact
rather than the number, which is … the only version that cannot drift"* — and
F-12 established the class this ledger keeps finding: *"a count in a comment is a
claim nothing checks."* A `path:line` is a count. An Outcome is written **after**
its own repair and is the record a later reader follows.

**Observed:** round 4's repairs moved three lines its own prose cites, and the
prose was not re-measured against the tree the repairs produced.

- **F-18's Outcome** — *"The two double-minting cases are quoted from the tree at
  `:447-448` and `:810`/`:825`."* They are at `:450-451` and `:813`/`:828`. This
  one was **false when written**: `:447`/`:810`/`:825` is where the raiser found
  them *before* F-18's and F-21's repairs added three lines to the doc above
  them, and the Outcome carried the raiser's numbers across its own edit. That
  is the mechanism F-18 itself diagnoses — *"a count that arrives from a document
  rather than from the code is a count nobody measured"* — one section down.
- **F-19's Location** — `harness.rs:119-122` (`described`) and `:156-164`
  (`field_described`). They are at `:123-126` and `:161-169`, moved by **F-20's**
  repair, which added four lines to `element_described`'s doc above them in the
  same round. `wiring.rs:78-80` still resolves. F-13's Outcome's
  `harness.rs:121` — the line F-19's Evidence names as the mutation site — is
  `:125` for the same reason.
- **F-21's Observed and Response** — `scripting.rs:133` for `logging_backend`'s
  `invocations-` prefix. It is `:156`, moved by **F-17's** repair, which grew
  `claim`'s doc above it in the same round. (The sentence is separately false;
  that is F-23.)

**Evidence:**

```
$ grep -n 'vt3b-symlink-target\|vt6b-clock\|vt6b-shutdown' crates/goad-shell/tests/integration/ingress.rs
450:  let target = socket_path("vt3b-symlink-target");
813:  let clock_path = socket_path("vt6b-clock");
828:  let shutdown_path = socket_path("vt6b-shutdown");
$ git show 1969a27:crates/goad-shell/tests/integration/ingress.rs | grep -n 'vt3b-symlink-target'
434:  let target = socket_path("vt3b-symlink-target");
```

434 at the pre-review tree, `:447` after round 3's thirteen-line `claim` doc —
which is what round 4 measured — and `:450` after round 4's own three. Likewise
`grep -n 'accessible_description().as_deref()' crates/goad/tests/` returns
`harness.rs:125`, and `grep -n 'invocations-' tests/support/scripting.rs` returns
`:111` and `:156`.

**Why a `nit` and not higher.** An append-only ledger cites a moving tree, so
some drift is the format and not a defect; a reader landing three lines off lands
inside the function named. What is not drift is the first one: it was wrong
before the round closed, for the reason the same round raised a finding about. If
the repair is anything, it is F-18's own — an Outcome cites the symbol it is
about, and quotes a line number only where it also quotes the line.

**Disposition:** `fix-now`, and the repair is F-18's own rule applied to this
ledger rather than three corrected numbers. The finding says so itself: *"an
Outcome cites the symbol it is about, and quotes a line number only where it
also quotes the line."*

The `nit` is right and the reason is worth keeping: an append-only ledger cites
a moving tree, so some drift is the format rather than a defect. What is not
drift is F-18's — wrong **before the round closed**, because the Outcome carried
the raiser's line numbers across the very edit that moved them, which is the
mechanism F-18 diagnoses one section above. A round that repairs a stale count
by writing a stale count has not understood its own finding.

**Response:** the citations written by a responder no longer carry line numbers;
the raiser's text is not touched.

- **F-18's Outcome** — `:447-448` and `:810`/`:825` are gone. The two
  double-minting cases are now named (`vt3b-symlink-target` / `vt3b-symlink`,
  `vt6b-clock` / `vt6b-shutdown`), with a sentence recording that the numbers
  were already wrong when written and why.
- **F-21's Response** — the citation went out with the sentence that carried it.
  F-23's amendment replaced the whole *"every marker name … `invocations-`
  prefix (`scripting.rs:133`)"* clause, because the claim was false and not
  merely mis-cited, so there was nothing left to re-cite.

  > **Corrected after the fact.** This Response first said the citation *"becomes
  > `scripting.rs::logging_backend`"*. It does not — it is gone. A responder
  > describing a repair they had not re-read, which is the defect this very
  > finding is about and the fifth instance of it in this ledger. Caught by the
  > mechanical pass, which is the argument for that pass in one line: a script
  > asking *does this string still exist* does not care how plausible the
  > sentence containing it was.

**Not** amended, deliberately: F-19's §Location, F-21's §Observed and F-11's
§Evidence all carry line numbers that have since moved. Those are the **raiser's**
text, and the Protocol is unambiguous — findings are never edited once raised.
They were true at the tree the raiser measured, and a responder rewriting them
to today's lines would be doing to the ledger what F-18 was raised about, in the
one direction the Protocol forbids outright. This finding's existence is the
record that they moved.

**Outcome:** `verified`. Every `path:line` written by a responder in this ledger
resolves, checked by the mechanical pass that stands in for round 6 (Brief,
*Where the rounds stopped*); the raiser-written ones are left as raised and are
named above so a reader is not surprised by them.

## Synthesis

**State: resolved.** Twenty-four findings over five rounds, every one `verified`;
twenty-two `fix-now`, two `follow-up`, both landed in `slice-007.md`. No
`blocker` was raised and no severity was downgraded to clear the gate. `just
check` is exit 0 at 537 cases across 21 binaries — 535 when the phases finished,
the two additions being the cases F-1's and F-2's repairs required.

### The two defects worth carrying out of this slice

**A claim held at the row model is not held at the screen.** The whole of
`app.slint`'s heading markup could be deleted with all 206 tests green (F-1).
AC-2's chain was declared to have three links, and its third held the *fields*
and not the heading they sit under — so the criterion was satisfied by a test
that could not tell a heading from its absence. The checkbox half of the very
same element was held by six cases. The asymmetry is the lesson: a model
assertion and a screen assertion look equally like evidence in a test file, and
only one of them survives the markup being deleted.

**An assertion in the direction an unbound property already answers
discriminates nothing.** Every `accessible_enabled_of` assertion in the
workspace was `== Some(true)` (F-2) — which is also what an *unbound* `enabled`
answers. So `enabled: !root.busy` could be deleted from both controls with the
suite green, and the two cases named for busy could not tell the binding from
its absence. The repair is the general form: assert in the direction that a
missing binding cannot produce.

Both are lifted to `docs/memory/`, because neither is about checkboxes.

### What the review found in itself

This is the ledger's own result and it is worth more than any single finding.

| round | subject | raised | code defects | ceiling |
|---|---|---|---|---|
| 1 | the slice | F-1–F-4 | 4 | major |
| 2 | round 1's repairs | F-6–F-10 | 5 | major |
| 3 | round 2's repairs | F-11–F-13 | 3 | minor |
| 4 | round 3's repairs | F-17–F-21 | **0** | minor |
| 5 | round 4's repairs | F-22–F-24 | **0** | minor |
| — | round 5's repairs | mechanical pass | **0** | one false Response, caught |

**Round 2 found five defects in five of round 1's repairs, two of them false
statements in this ledger's own Response** (F-7). That is the argument for
unbounded rounds, demonstrated in one slice rather than asserted: the repairs
were made carefully, by the agent holding the whole audit, immediately after
reading the standard they failed to meet. A review that stops when the first
round's findings are repaired ships those five.

It did not stop there. Round 3 found three more; round 4 found five, **three of
them false counted claims** in Responses — including one (F-19) that arithmetic
alone shows no tree could produce, and which cost nothing to check. Round 5
found three more, one of which (F-23) was a true sentence made false by being
generalised.

The pattern across rounds 2, 4 and 5 is sharper than "be careful": **the
falsehoods were not careless transcriptions, they were small strengthenings of
claims that were already true.** "the `renderer` binary" became "the two
claiming binaries". "three copies" became a list. A count that arrived from a
finding's text was carried into the tree without being re-measured (F-18). Each
edit made the prose better and made it wrong.

And the class beneath all of it is one this slice kept re-finding in two
disguises: **a comment asserting more than it holds is the same defect as a test
asserting more than it holds.** `table.rs` pins its thirty-three rows with
`assert_eq!` rather than a sentence for exactly that reason; F-8, F-12, F-14,
F-18, F-20 and F-22 are all the sentence version.

### What the review confirmed, rather than changed

Recorded because an audit cannot write *"checked and sound"* on an absence of
findings. Four of round 1's seven mutations were correctly red, and two attacks
on R-57/R-58 were **structurally unreachable, with why checked rather than
assumed**. `PresentationField` carries no `checked` and no interior mutability —
the draft never entered `Presentation` (I-4/S-6), which no instrument in the
gate reaches. All nine of `PromptWindow`'s properties have unconditional setters
(I-5/S-9). `"group"` is read at exactly one site in `src/` (R-18). Round 3
closed both of its open questions by measurement: F-6's `expect` cannot fire
through any fixture, and `claim`'s `(kind, name)` key misses no real collision.
Round 4 proved the new assertions cannot panic a correct case, across all 50
call sites. Round 5's own numbers were then verified mechanically.

End to end, and supplied by no mutation: `examples/shell/backend.sh` was run as
a real child process through the production `read_response` and `present` — a
conforming backend sending a kind this renderer does not draw is
drawn-minus-that-field and still answerable, **measured, not argued.** That is
the invariant the project exists to protect.

### The gate was not deterministic while everyone treated it as the arbiter

Two cases shared one invocation-log path — one pid, one file, cleared at
handout — failing **one run in six** (F-5). `just check` is the gate, and a gate
green five times in six is not one. It is held by an instrument now rather than
by a naming convention, and the instrument earned itself immediately: it found a
second collision on its first run (F-5), then a third and fourth helper making
the same unheld promise (F-9, F-11), then a fifth and sixth (F-17, F-22).

That sequence is the finding. Four rounds each wrote down a list of the helpers
they had found, and **three of those lists were short within one round.** The
last one was short because a helper of the class need not be called
`socket_path`, need not mint a socket, and need not live under `tests/` — one is
a `#[cfg(test)]` helper in a `src/` file, and it clobbers a `config.toml` rather
than a log. `claim`'s doc now leads with the *class* and the grep that
enumerates it, and the list follows as an instance. Enumerating instances was
the defect; the count was only its symptom.

### Risks knowingly left standing

- **`claim` holds four of six.** Both stragglers are in `goad-emit`, and the
  obstacle is measured rather than assumed: neither target can include
  `scripting.rs` without nine `dead_code` warnings, which `-D warnings` makes
  errors. Nothing collides today. The extraction is a follow-up with a design
  question in it (F-17, F-22).
- **Three states no round reached** — a real `Full` on the one-slot channel in a
  running `serve`, a backend killed mid-form with a draft outstanding, an
  ingress arrival at a half-filled form. All three are expressible with the
  existing harness, so this is budget and not tooling. `choice`, `number` and
  `datetime` are covered as mapper values only. Stated here and in the Verdict
  so that no line above can borrow from a check that was never run.
- **Unbounded strings at the screen** — `title`, `option.label` and now
  `block.heading`, while every diagnostic line passes through `finish(..,
  LINE_LIMIT)`. A class question owned by 008 (F-16).
- **Unbounded diagnostic lists** (F-4), and the window's absent preferred size
  (F-6) — both 008's, both with their mechanism identified rather than their
  symptom.
- **No round had a display.** AC-7 and AC-10 rest entirely on the user's own
  account, which `audit.md` §Evidence records in their words.

### Why there is no round 6

Rounds 4 and 5 found **no code defect**. Of twenty-four findings the last eight
are counts, enumerations and citations in prose — a residue that is
*mechanically* checkable, and that a script checks better than an adversarial
reader because it cannot itself miscount, which is exactly how rounds 4 and 5
each failed. So the sixth pass was a verification script over this ledger rather
than a sixth reading: every `path:line` a responder wrote resolves (163 of them;
4 external, 2 shorthand in raiser text left as raised), every finding id cited
exists, every table row has a section and a disposition, and the six-of-six /
four-held counts were re-derived from the tree with the doc's own grep.

**The pass earned itself on its first run,** which is the strongest thing that
can be said for choosing it over a sixth reading. It caught a fifth Response
asserting something untrue of the tree: F-24's own, which described F-21's stale
citation as having been *converted to a symbol* when F-23's amendment had
**deleted** it along with the false sentence carrying it. A responder describing
a repair they had not re-read — inside the finding about citations that do not
resolve. No adversarial reader was needed for that; a script asking *does this
string still exist* does not care how plausible the sentence containing it was,
and that indifference is exactly what the last two rounds lacked.

The stopping rule is a **decision, taken by the user on 2026-09-16 and written
down**, not a ledger that quietly ran out. A reader who disagrees with it has
the trend table above and can reopen the argument with the evidence in hand.

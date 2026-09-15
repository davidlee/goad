# Review — design — Slice 007

**Subject:** design — `docs/slices/007/design.md`, and the draft canon it rests
on, `docs/slices/007/canon-delta.md` (CD-1 → R-57, R-58)
**Reviewer:** fresh agent, unjailed or jailed — see `review-design-brief.md`
**Opened:** 2026-09-14
**State:** open

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

Written before round 1 ran, by the author of the design, so it names where the
author thinks the bodies are rather than where they turned out to be easy to
find. The full hand-over is `review-design-brief.md`; this is the standing
statement of what the review is for.

**The invariants this design is held to**, in the order a breach would matter:

1. **A renderer subset is not a narrowing** (SPEC-001/R-55, `CLAUDE.md`'s third
   invariant). The design draws one field kind of five and writes a wire contract
   covering four. Either half could be the failure this project exists to avoid,
   argued in the other direction.
2. **Internal representations are canonical; the host does not understand the
   domain.** `group` is read in one file; the host reorders nothing, withholds
   nothing, reads no value.
3. **A backend failure never takes the host down**, and every refusal says which
   side was wrong.
4. **Strata run one way** (ADR-001), and stratum 3's purity is reached by **no
   instrument** — so anything holding it here is prose, and prose is what a
   review is for.

**Where the author expects defects:**

- **A-2** (§5.4) is the one assumption that can invalidate a section. If a
  `CheckBox` that assigned its own `checked` stays detached from the model,
  `present` stops writing the draft back and AC-5 fails. It may be settleable on
  paper from the pinned Slint sources.
- **D6**'s claim that walking declared fields yields R-58 "in both directions
  with no check to forget" is the load-bearing structural argument. One path by
  which an undrawn field enters a block, or a draft key reaches the wire, and it
  degrades from a property to a convention.
- **R-57 and R-58 as canon**: falsifiability, §7 conventions for an outbound
  requirement, and whether two rows is right where scoping wrote one.
- **The claim that no ADR is needed**, which rests entirely on SPEC-001 §1
  already carrying D1's rationale.

**Round 1** — 2026-09-14 — the whole design, both halves of the wire contract,
and the four expectations above.

**Round 2** — 2026-09-15 — job 1: the seventeen artefact-changing repairs from
F-1 through F-19, including their interactions and the round-1 outcomes.

## Findings

**Environment note.** Run unjailed; `git rev-parse --show-toplevel` is
`/home/david/dev/goad`, so the jail half of `review-design-brief.md` does not
apply and Slint facts below are cited from the pinned crates at
`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/i-slint-{core,compiler}-1.17.1/`.
One path in the brief does not exist: it cites `docs/memory/tests-asserting-proxies.md`;
the file is `docs/memory/a-green-test-can-assert-a-proxy.md`, and it is cited
below under that name. `design.md` itself names no filename there and is not
wrong.

**Outcome, and who sets it.** Every outcome below is unset. The protocol gives
the outcome to the **raiser**, and round 1's raiser does not return; the
responder cannot verify its own repairs without wearing both hats at once.
**User ruling, 2026-09-15: round 2's codex review stands as the verification of
round 1.** Round 2 therefore inherits the raiser's role over F-1..F-19 in
addition to raising its own findings, and at its close each outcome is set from
what it actually found:

- a repair round 2 checked and accepts → `verified`, the raiser being round 2.
- a repair round 2 finds wrong → the round-1 finding goes `contested` and
  returns to open, and its substance is raised as a new finding from `F-20`
  onward; the contested finding is disposed again against that one.
- a round-1 finding round 2 judges was never a defect → `withdrawn`.
- a repair round 2 did not reach → the outcome **stays unset**, and *Depth of
  the round* names it, so the gap is visible rather than implied by a blank
  cell.

This is a ruling about who holds the pen, not a licence to mark an outcome round
2 did not form a view on. The ledger is `Done` only when the column is full.

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | minor | doc-wrong | |
| F-2 | major | doc-wrong | |
| F-3 | major | doc-wrong | |
| F-4 | blocker | doc-wrong | |
| F-5 | major | doc-wrong | |
| F-6 | major | doc-wrong | |
| F-7 | minor | doc-wrong | |
| F-8 | major | doc-wrong | |
| F-9 | major | doc-wrong | |
| F-10 | major | doc-wrong | |
| F-11 | minor | doc-wrong | |
| F-12 | minor | doc-wrong | |
| F-13 | minor | doc-wrong | |
| F-14 | major | follow-up | |
| F-15 | minor | doc-wrong | |
| F-16 | minor | doc-wrong | |
| F-17 | minor | doc-wrong | |
| F-18 | nit | doc-wrong | |
| F-19 | minor | doc-wrong | |

### F-1 — A-2 is settled on paper, affirmatively: a model reset destroys and re-creates the repeater's elements, so `FieldRow.checked`'s binding is re-established

**Severity:** minor
**Location:** `design.md` §5.4 *The assumption this section rests on*, §5.5 A-2, §8 R-2, §9 *Proven by running code in phase 1*

**Expected:** §5.4 — "whether the model reset then rebuilds the repeater's
elements (restoring the binding) or updates them in place (leaving it detached)
is the open half", and A-2 is "**not proven in this workspace**", to be settled by
running code in phase 1.

**Observed:** it is settleable from the pinned sources, and it settles in the
design's favour. `set_vec` resets; a reset **clears the instance vector**; the
next `ensure_updated` constructs a fresh element per row, which installs the
declarative binding again. The `CheckBox` that imperatively assigned `checked` no
longer exists.

**Evidence** — the chain, each link cited:

1. `VecModel::set_vec` replaces the vector and calls `self.notify.reset()` —
   `i-slint-core-1.17.1/model.rs:404-407`.
2. `ModelNotify::reset` marks the row-count and row-data properties dirty and
   forwards `reset()` to every peer — `model/model_peer.rs:87-97`.
3. The repeater's peer is `RepeaterTracker`, whose `reset` is
   `self.is_dirty.set(true); self.inner.borrow_mut().instances.clear();` —
   `model/repeater.rs:506-509`. **The instances are destroyed.**
4. `Repeater::ensure_updated` sees `is_dirty`, builds `RustRepeaterOps` and calls
   `update_all_instances` — `model/repeater.rs:578-593`.
5. `update_all_instances` splices the instance vector back up to `count` with
   `(Dirty, None)` entries and calls `ops.ensure_updated(i, …)` for each —
   `model/repeater.rs:144-154`, `:354-359`.
6. `RustRepeaterOps::ensure_updated` finds `c.1.is_none()`, so `created` is true
   and it runs `c.1 = Some((self.init)())` — a **new** `ItemTree`, then
   `instance.init()` — `model/repeater.rs:361-381`.

A new element instance evaluates its own binding set, so `checked: field.checked`
is live again on the new `CheckBox`. The contrasting path confirms the mechanism
rather than a coincidence: `RepeaterTracker::row_changed` calls `comp.update(..)`
on the **existing** instance and never re-inits it (`model/repeater.rs:427-441`),
so a `set_row_data`-shaped present is the one that *would* leave the property
detached. The premise is real — every stock `CheckBox` assigns its own property
in its default action: `widgets/material/checkbox.slint:24-29`
(`root.checked = !root.checked; root.toggled();`), and identically in `fluent`,
`cosmic`, `cupertino` and `qt` (`:25`, `:25`, `:26`, `:13` respectively).

**Two consequences the design should absorb rather than carry:**

- Phase 1 no longer has to *discover* this; a test that pins it is still worth
  writing, but it is a regression pin, not an experiment, and the phase's risk
  budget is freed.
- **The fallback named in §5.4 and §8/R-2 would not have worked.**
  `toggled => { self.checked = field.checked; … }` is itself an imperative
  assignment to the same property, so under the refuted branch of A-2 it would
  have left the property detached *and* pinned to a stale value — the tick would
  revert and then never follow the draft. R-2's mitigation is not a mitigation;
  the real fallback, had one been needed, is the one this finding shows is
  already in force (destroy and re-create), not a line in the callback.

**Disposition:** doc-wrong
**Response:** Upheld, and the more valuable half is the second one. The chain was
re-walked in the pinned sources and holds link by link, so the design carried a
question it did not need to carry — and, after F-2's repair asserted the rebuild
as fact at the top of §5.4, the stale subsection twenty lines below contradicted
it outright and still sent phase 1 to discover a settled thing.

**Changed:** `design.md` at all five sites that treated A-2 as open — §5.4's
subsection, §5.5's A-2 entry, §6's "not questions for the user" paragraph, §8's
R-2 row, and §9's *Proven by running code in phase 1*.

§5.4's subsection is now *The mechanism this section rests on, and why its
fallback was not one*: the five-link chain with its citations, and the contrast
path (`row_changed` → `comp.update`) named as what confirms the mechanism rather
than a coincidence. AC-5 is restated as a **regression pin against a future
Slint**, not phase 1's experiment.

**The refuted fallback is kept, labelled.** `self.checked = field.checked` in
`toggled` is itself an imperative assignment to the property in question: under
the branch it was written to rescue it would have left the property detached
*and* pinned to a stale value — reverting the tick and then never following the
draft. It stays in the design marked `// NOT a mitigation`, because a line that
looks like a rescue and is not needs to be refuted where a reader will look for
it, not deleted. §8/R-2 is marked **discharged** on the destroy-and-re-create
mechanism and says explicitly that it is not discharged on that line.

**A-1 was left exactly as it stood.** §6 and §9 named A-1 and A-2 in one breath;
those sentences are now split so A-2 moves out and A-1's status is untouched. F-18
settles A-1 and is its own finding — the two were not folded together.

Endorsed by the user, 2026-09-15.

**Outcome:** verified | contested | withdrawn

### F-2 — every present destroys and re-creates every element under `options`, so keyboard focus is dropped on every edit round trip; §5.4's "same value, no visible change" is false

**Severity:** major
**Location:** `design.md` §5.4 *An edit is a round trip*, §5.3 *No focus per field*, I-5, AC-10

**Expected:** §5.4 — "the next present rewrites every row from the draft. When the
command arrived, that write is the same value and nothing moves." §5.3 — "**No
focus per field.** `Focus` stays what it is today". The design treats a present
that changes no value as observationally inert.

**Observed:** the present does not write the same value into the same widget. It
**destroys every widget under `options` and builds new ones** (F-1's chain,
steps 3 and 6). The window's focus item is a weak reference
(`i-slint-core-1.17.1/window.rs:519`, `pub focus_item: RefCell<ItemWeak>`), so
when the focused `FocusScope` inside a repeated `CheckBox` is dropped the upgrade
at `:1241-1242` fails and focus is nowhere. Each `CheckBox` forwards focus into
its own inner scope (`widgets/material/checkbox.slint:30`, `forward-focus:
i-focus-scope`), which is inside the repeated element.

The loop makes this reachable once per tick: `Command::Edit` → `dispatch`
returns `None` → `serve` continues to the top → `glass.present` at
`controller.rs:611` → `set_vec` at `glass.rs:87`. So a person filling a fourteen-item
form from the keyboard loses the focus ring on every box and must tab from the
start again. Today the same reset happens, but a present during a prompt follows a
fold or a refusal and a click ends the interaction, so nothing was there to lose.
This slice is what makes it a per-interaction cost.

**Evidence:** `model/repeater.rs:506-509` and `:361-381` (destroy / re-create);
`window.rs:519`, `:1241-1242`, `:1269-1280` (focus held weakly, taken when it
cannot be upgraded); `widgets/material/checkbox.slint:30`;
`crates/goad/src/glass.rs:87-90`; `crates/goad/src/controller.rs:611`.

The design's own structure forecloses the obvious repair. I-5 forbids a
write-only-on-change path, and the only Slint mechanism that updates a row without
destroying its element is `row_changed` → `comp.update(..)`
(`model/repeater.rs:427-441`) — which is exactly the path that leaves an
imperatively-assigned `checked` detached (F-1). So **the mechanism that makes A-2
true is the mechanism that loses focus**, and the design states neither side of
that trade. AC-10 cannot absorb it either: its bound is "legibility", and its own
sentence says feedback beyond that is "recorded verbatim and not actioned".

**Disposition:** doc-wrong
**Response:** Upheld, and the citations re-checked rather than taken on trust:
`focus_item` is `RefCell<ItemWeak>` (`i-slint-core-1.17.1/window.rs:519`) and is
upgraded before use (`:1241-1242`), so a destroyed focus item leaves focus
nowhere; `glass.present(controller.frame())` is the first statement of every
`'serving` loop iteration (`controller.rs:611`) and `self.options.set_vec(..)`
(`glass.rs:87`) is unconditional. An edit returns `None` from `dispatch`, so the
rebuild is once per tick.

The finding's sharpest observation is the one carried into the design: **the
mechanism that makes A-2 hold is the mechanism that loses focus.** `set_vec` →
reset → `instances.clear()` re-establishes the declarative `checked` binding
*because* it destroys the element that assigned its own; `set_row_data` →
`row_changed` → `comp.update` keeps the element, keeps focus, and leaves `checked`
detached forever. With stock widgets there is no third path — the cause is
`CheckBox`'s own default action (`checkbox.slint:24-29`).

**Two candidate repairs were chased and both are architectural, which is why this
is not `fix-now`:**

- *Do not present on an edit.* The widget already shows the tick, so the present
  looks redundant — but it requires the loop's top-of-iteration present to become
  conditional, and it makes the screen's correctness after a tick depend on the
  widget's self-assignment rather than on present's output, which is the coupling
  A-2 exists to police.
- *Update the draft from the toggle callback, never entering the loop.* Requires
  the draft shared between the UI thread and the controller, contradicting §5.3's
  ownership table outright.

**Changed:** `design.md` §5.4, §5.3 and I-5; `slice-007.md` Follow-ups.

§5.4's false sentence is gone. "That write is the same value and nothing moves"
now reads that it is the same *value* but not the same *element*, followed by the
mechanism, the cost, and its frequency, and by a blockquote warning in the
imperative: **do not repair this with `set_row_data`** — the obvious fix for focus
silently reintroduces the defect F-1 spent its budget excluding. That warning is
the most valuable line in the repair; without it the trap has no tripwire.
§5.3's *No focus per field* now reads as a decision with a cost rather than an
absence of one. I-5 gains a sentence distinguishing what present writes when it
runs from the cost of running, so that nobody weakens I-5 to chase focus and
breaks A-2 by another route.

**Owned, not fixed.** `slice-007.md` Follow-ups carries the work — a focus
identity that survives a rebuild: the row reports focus back, the host retains
the focused field id, the row's `init` restores it. That is a design surface of
its own, which is the principled reason for a follow-up rather than a large fix
being dodged: no acceptance criterion becomes unmeetable, no requirement is
breached (SPEC-001 addresses focus nowhere), and the form is fully usable by
mouse. The cost is stated in the card rather than softened: ticking the Nth box
from the keyboard costs N tabs.

The raiser is right that AC-10 cannot absorb this — its bound is legibility and
its own sentence says feedback beyond that is recorded verbatim and not actioned
— and AC-10 was left alone.

Endorsed by the user, 2026-09-15, over the stated alternative of taking the focus
mechanism into this slice as `fix-now`.

**Outcome:** verified | contested | withdrawn

### F-3 — `OptionId` does not implement `Ord`, so `Draft`'s `BTreeMap<(OptionId, FieldId), Edited>` will not compile; `crates/goad-semantics/` must change

**Severity:** major
**Location:** `design.md` §2 *Nothing in `crates/goad-semantics/` needs to change*, §5.2 *The draft*, §5.2 final paragraph, §10 ADR-001/ADR-003 row; `slice-007.md` §Scope *Untouched*

**Expected:** §2 — "**Nothing in `crates/goad-semantics/` needs to change.**
`Field::{id,kind,label,hints}`, `FieldKind`'s public variants, `Fields::as_slice`,
`Hints::as_map` are all `pub`; `FieldId` is `Ord + Clone`". §5.2 — "**Nothing in
`crates/goad-semantics/` changes.** Verified accessor by accessor". `slice-007.md`
lists the crate under *Untouched*: "already models everything this slice needs —
verify that, do not extend it".

**Observed:** the audit checked `FieldId` and not `OptionId`. `OptionId` derives
`Debug, Clone, PartialEq, Eq, Serialize` and **no ordering traits**, so
`(OptionId, FieldId)` is not `Ord` and `BTreeMap<(OptionId, FieldId), Edited>`
does not compile. There is no manual `impl Ord for OptionId` anywhere in the
crate.

**Evidence:** `crates/goad-semantics/src/protocol/canonical.rs:55-57`:

```rust
/// Names a *view's* option — what `UserResponse.option` selects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OptionId(String);
```

against `:84-86`, where the contrast is deliberate and documented:

```rust
/// Keys `UserResponse.values`, hence `Ord`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct FieldId(String);
```

`grep -rn "Ord" crates/goad-semantics/src | grep OptionId` returns nothing.

Why this is more than a missing derive: the `Ord` on `FieldId` carries a comment
saying *why* it is there, so the ordering traits on these ids are a deliberate,
minimal surface in stratum 1 rather than boilerplate. Adding `PartialOrd, Ord` to
`OptionId` is a stratum-1 edit inside a slice that declares stratum 1 untouched,
which under `docs/AGENTS.md` §Execute ("Stay inside the phase's declared
surfaces… stop and ask") sends the first phase to a STOP the design created. The
alternatives — keying the draft by `String`, or a host-local newtype, or a `Vec`
with a linear scan — are all design changes to §5.2's stated signature.

**Disposition:** doc-wrong
**Response:** Upheld, confirmed at the source: `OptionId` derives
`Debug, Clone, PartialEq, Eq, Serialize` and no ordering or hashing
(`canonical.rs:55-57`), with no manual impl, so neither a `BTreeMap` nor a
`HashMap` on `(OptionId, FieldId)` compiles. The raiser is also right that this is
more than a missing derive: in that file an id is ordered exactly where the
protocol keys a serialized map by it — `FieldId` is and says so in a comment,
`OptionId` and `AlternativeId` are not — so the derive set is documentation, and
adding to it for a host's convenience would drain the pattern of meaning.

The composite key itself is necessary and stays: R-52 makes a field id unique only
within an option, and `R-52-the-same-field-id-in-different-options` is a fixture
asserting two options may share one, so a draft keyed by `FieldId` alone collides
exactly there.

**Changed:** `design.md` §5.2's draft sketch, and the two places the crate audit
is claimed (§2 and §5.2). The key moves out of stratum 1 rather than stratum 1
gaining a trait:

```rust
#[derive(Debug, Default, Clone)]
pub struct Draft(Vec<(OptionId, FieldId, Edited)>);
```

`Draft`'s public surface is unchanged — the same two methods, the same absence of
enumeration — so §5.2's argument and D6 are untouched; the representation was
always private and carried none of that weight. A view's options and a form's
fields are both handfuls, so the scan is free. `PartialEq, Eq` are **dropped**:
over a `Vec` derived equality is insertion-order sensitive, and a draft's
behaviour is `state_of`, not its representation — tests assert through the
accessor, which is this project's own rule and
`docs/memory/a-green-test-can-assert-a-proxy.md` applied before the test exists.
Blast radius checked: `Prepared`, which holds the draft, derives only `Debug`
(`reception.rs:24`).

Both audit sentences now record the check that was missed rather than only its
result — §2 states that `OptionId` is deliberately not `Ord` and that the design
is shaped to that fact, and §5.2 adds that the audit is trait by trait and not
only accessor by accessor, which is the half that let this through.

**Why not the one-line derive.** Considered and rejected on two grounds: it is a
stratum-1 edit inside a slice whose card declares the crate Untouched, so phase 1
would open on the STOP `docs/AGENTS.md` §Execute requires; and its justifying
comment could only say that a renderer keys a map by it, which is stratum 1
documenting a stratum-3 need. `slice-007.md` needed no amendment — its Untouched
claim is true as it stands.

Endorsed by the user, 2026-09-15 (option 2 of two put to them).

**Outcome:** verified | contested | withdrawn

### F-4 — R-58 as worded is breached by this design's own behaviour, and conflicts with R-8 and R-52

**Severity:** blocker
**Location:** `canon-delta.md` CD-1 *The change as it will be stated*; `design.md` §5.2, I-3, D6, §5.5 edge-case table last row

**Expected:** I-3 — "A response carries exactly the drawn fields of exactly the
answered option. Held structurally… **R-58 in both directions, with no check to
forget.**" D6 says the same. R-58 is to be promoted verbatim into SPEC-001 §4 by
AC-8.

**Observed:** R-58's first clause is *"A host MUST submit a value for every field
it drew"*. The design draws **every option's** fields at once — D7, "a block per
option, each with its own button", and §5.5's edge case "a view mixing options
with and without fields". It then submits values for the answered option only.
The design states the breach itself, in the last row of §5.5:

> ticks under one option, then a different option's button pressed | `{}`
> submitted under the pressed option; **the ticks discarded** without a prompt

Those fields were drawn. No value is submitted for them. Under R-58 as worded
that is a MUST violation, and a conforming host would have to do the opposite.

The wording does not merely under-specify — it points the wrong way. A host that
*did* satisfy R-58 literally would put both options' fields in one flat map,
which R-8 forbids ("the chosen option id, and a map of field id to submitted
value") and which R-52 makes ambiguous, since two options may legally share a
field id — a fixture asserts that exact case is legal
(`R-52-the-same-field-id-in-different-options`, SPEC-001 §7 R-52 row). So R-58 as
drafted is satisfiable only by breaching two requirements already in the spec.

**Evidence:** `canon-delta.md:54-59` (R-58's text); SPEC-001 §4 R-8
(`docs/specs/001-host-backend-protocol.md:89`); R-52 (`:103`); SPEC-001 §7's R-52
row naming the same-field-id-in-two-options fixture as "the last legal and
accepted, which is what shows the scope is right" (`:386`); `design.md` §5.5 edge
case table, last row; `design.md` §5.1 "the walk is over what was drawn".

The scope is exactly the thing SPEC-001 is elsewhere fastidious about: R-52
spends a paragraph saying which ids are unique within which scope, and R-53
another saying which namespace an id belongs to. R-58 leaves its scope to be
inferred from a different requirement, in a document whose §5 says "Ambiguity is
failure". The missing words are small — *of the option being answered* — but the
design's claim that R-58 holds "in both directions with no check to forget" is
false of the text as written, and the text as written is what AC-8 promotes.

**Disposition:** doc-wrong
**Response:** Upheld as raised, at the severity raised. The finding's two halves
separate cleanly, and only one of them is a defect:

- **D6's mechanism is sound.** The raiser's own *Depth of the round* records
  three attempted break paths and none exists, and I-3's sentence — "exactly the
  drawn fields of exactly the answered option" — already states the scoped
  property. The design describes the right behaviour.
- **R-58's text stated a different, self-contradicting property**, and it is that
  text AC-8 promotes into SPEC-001 verbatim. Satisfying it literally would
  require one flat map over both options' drawn fields, which R-8
  (`docs/specs/001-host-backend-protocol.md:89`) does not admit and which R-52
  (`:103`) makes ambiguous, its §7 row (`:386`) naming
  `R-52-the-same-field-id-in-different-options` as legal and accepted. A MUST
  satisfiable only by breaching two shipped requirements is the ambiguity SPEC-001
  §5 calls failure.

**Changed:** `canon-delta.md` CD-1, R-58's wording, scope added:

> A `respond` carries values for exactly the fields the host drew **of the option
> being answered**: a host MUST submit a value for each of them, and MUST NOT
> submit a value for any other field — **neither a field it did not draw, nor a
> field of an option it is not answering**.

The remaining three sentences are unchanged. CD-1's *Why two requirements and not
one* restated the old unscoped form one paragraph below and was brought into step
with it, and no other paragraph of CD-1 needed amending.

**Two residues found later, while answering a question about D3, and repaired
then:** `design.md` §5.2 quotes R-57 and R-58 **verbatim**, and both quotes still
carried the pre-amendment wording — the drift that would have been transcribed
into SPEC-001 at reconciliation. Both now match `canon-delta.md` exactly, checked
by diffing the two blocks rather than by eye. And D3's rationale row in §7 ended
"R-57's last clause makes its absence a defined state rather than a hole", which
is the claim F-5 falsified; the row now states what actually closes the hole and
says why reversing it looks like tidying in either direction (see F-14).

In `design.md`, I-3 was already correct and is unchanged — it becomes true as
written once the rule it cites says what I-3 says. One sentence there did need
the scope, found while reading §5.1 for F-3 and folded in here rather than raised
separately, since it is this repair's own consequence: the first bullet under
*The one structural idea worth stating on its own* read "A value is submitted for
**every** drawn field, ticked or not (R-58, first half)", and now reads "every
drawn field **of the answered option**", with the walk described as over that
option's blocks only. The behaviour it describes was already scoped (§5.2 walks
the answered option's blocks); only the wording trailed the amended rule.

**Why not the alternatives.** Amending R-8 instead was considered and rejected:
R-8 is shipped canon carrying its own fixtures and verification row, and CD-1's
stated stance is one document, one change — the missing words belong to the new
row. `follow-up` is unavailable to a blocker in the current unit of work, and
`tolerated` would knowingly ship a self-contradicting MUST.

**What this disposition does not settle**, left to their own findings rather than
folded in: **F-5** — scoping does not close `datetime`'s hole, since a host that
drew one still owes a value it has no defined form for; **F-6** — R-58 still needs
a §7 verification row whose site can actually hold it.

Endorsed by the user, 2026-09-15.

**Outcome:** verified | contested | withdrawn

### F-5 — `datetime` having "no defined submitted form" is a hole a conforming host falls into, not a defined state

**Severity:** major
**Location:** `canon-delta.md` CD-1, R-57's last sentence and *Deliberately out*; `design.md` D3

**Expected:** D3 — "R-57's last clause makes its absence a defined state rather
than a hole." `canon-delta.md` *Deliberately out* — "R-57's last sentence is
written so its absence is a defined state rather than a hole."

**Observed:** read R-57 and R-58 together over a host that draws a `datetime`
field. R-58: it MUST submit a value for that field. R-57: `datetime` "has no
defined submitted form in this version". So the host is *required* to submit a
value whose JSON type and shape nothing in the spec constrains. Two conforming
hosts can submit `"2026-09-14T10:00:00+10:00"`, `1789000000`, or
`{"date":"2026-09-14"}` and both are conformant; a backend cannot be written
against any of them.

That is `canon-delta.md`'s own definition of the gap it exists to close: *"Drawing
fields makes it a contract two backends could disagree about, which is the
definition of one"* (`canon-delta.md:44-45`). R-57 closes that definition for four
kinds and re-opens it for the fifth, in the same breath as claiming not to.

Nothing else shuts the door. R-55 explicitly forbids treating a renderer subset as
a narrowing, so the spec cannot be read as saying "no conforming host may draw
`datetime`" — and if it could, that reading would itself be the narrowing this
project exists to avoid.

**Evidence:** `canon-delta.md:49-59` (R-57 and R-58 as they will be stated),
`:96-102` (*Deliberately out*), `:42-45` (the definition of a contract gap);
SPEC-001/R-55 (`docs/specs/001-host-backend-protocol.md:121`); `design.md` D3
and the §1 boundary table row that reports `datetime` undrawn.

The state that would be *defined* is a different sentence — one that says what a
host that draws a `datetime` field does (submit nothing, and report it; or hold
the kind undrawable until OQ-1's capability mechanism lands). "No defined form"
plus a MUST is not that.

**Disposition:** doc-wrong
**Response:** Upheld. Read against amended R-58 (F-4) the hole is exactly as
raised: a value is required, and neither its JSON type nor its shape is
constrained. R-55 closes the escape route the same way the raiser says — the spec
cannot be read as "no conforming host may draw `datetime`", because that reading
is the narrowing this project exists to avoid.

**Changed:** `canon-delta.md` CD-1. R-57's last sentence now closes the door
rather than leaving it ajar:

> `datetime` has no defined submitted form in this version: a host MUST NOT
> submit a value for a `datetime` field, and MUST report such a field undrawn
> under R-55. Defining its form is OQ-4.

CD-1's *Sections* line gains SPEC-001 §8, and *Deliberately out* now carries
**OQ-4** in full — the three degrees of freedom (offset, precision, date-only)
that were the reason for leaving the form undefined, moved from a slice document
into canon where a later host can find them.

**Why this and not a defined form.** Writing RFC 3339 into R-57 now would close
the hole by inventing a wire form no evidence asks for, in the tier-2 document
hardest to change later. Naming the hole and shutting the door costs one sentence
and one open question.

**Why this is not a narrowing**, since it is the obvious objection: the
*protocol* declines to define a form, uniformly for every host, and reports the
consequence through the mechanism R-55 already provides. R-55 guards against one
renderer's subset becoming the contract; this is the contract stating its own
undefined region identically for everyone. A backend may still send a `datetime`
field (R-16) and learns from the undrawn report what became of it.

**Composes with F-4.** A `datetime` field is never drawn, so amended R-58 — which
reaches only the drawn fields of the answered option — never requires a value for
one. The two rows stop contradicting each other.

Endorsed by the user, 2026-09-15.

**Outcome:** verified | contested | withdrawn

### F-6 — §7's proposed vehicle cannot hold R-57: `canonical.rs` is stratum 1 and knows no field kind, while R-57's enforcement site is `draft.rs::submitted` in stratum 3

**Severity:** major
**Location:** `design.md` §9 AC-8, §10 SPEC-001 §7 row; `slice-007.md` AC-8

**Expected:** AC-8's observable — "R-57 and R-58 in SPEC-001 with a §7 row, and
**one unit test per typed kind asserting the serialized JSON type**", vehicle
"`canonical.rs`, beside `a_respond_serializes_to_the_spec_s_wire_form` (`:752-771`)".
§10 states the same and calls it the outbound convention.

**Observed:** `canonical.rs` is `crates/goad-semantics/src/protocol/canonical.rs`,
stratum 1. The value it serializes is `UserResponse`:

```rust
pub struct UserResponse {
  pub option: OptionId,
  /// Opaque to the host — R-9.
  pub values: BTreeMap<FieldId, serde_json::Value>,
}
```

(`canonical.rs:499-503`). A submitted value arrives there as a bare
`serde_json::Value`, carrying **no field kind**. A test written there can assert
only that `Value::Bool(true)` serializes as `true` — a fact about `serde_json`,
not about the host — and it would stay green under every defect R-57 exists to
exclude, including `submitted()` returning `Value::String("true".into())`,
because the test, not `submitted`, chose the value.

The host behaviour R-57 constrains is the kind→JSON mapping, and the design names
its one site itself: "`submitted` … **the single site where R-57 is applied**"
(§5.2) and "`answer` fills `values` … by walking the answered option's blocks and
calling `submitted(draft.state_of(..))`". Both live in `crates/goad/src`, stratum
3. `goad-semantics` cannot reach them; the dependency runs the other way (ADR-001,
ADR-003).

**Evidence:** `crates/goad-semantics/src/protocol/canonical.rs:499-503` and
`:751-771`; `design.md` §5.2 *The draft* and *The controller*; `research.md` F2,
which already records that `canonical.rs:752-771`'s test *is* §6.1's illustrative
example and is "the only non-empty `values` map in code".

This is the shape `docs/memory/a-green-test-can-assert-a-proxy.md` names, one
level up, in its own words: *"Watch for the same shape in a **Verification row**:
a row citing a case that cannot hold the claim is this defect one level up."* AC-8
is a closure criterion; as written the slice can close green on a test that could
not fail for the reason it exists.

**Disposition:** doc-wrong
**Response:** Upheld, and confirmed at the source before disposing.
`UserResponse.values` is `BTreeMap<FieldId, serde_json::Value>`
(`canonical.rs:499-503`) — the kind does not survive into stratum 1 — and the
test AC-8 wanted to sit beside constructs the value it then asserts
(`values.insert(FieldId::new("minutes"), json("20"))`, `:755`). It would stay
green under `submitted()` returning `Value::String("true")`, which is the one
defect R-57 exists to exclude. The raiser is right that this is
`docs/memory/a-green-test-can-assert-a-proxy.md` one level up, in a closure
criterion.

**Changed:** `design.md` §9's AC-8 row and §10's SPEC-001 §7 row, and
`slice-007.md`'s AC-8. Two parts, the second more consequential than the vehicle
move:

- **Vehicle** → `draft.rs::submitted` for R-57 and `answer()` for R-58, both
  `crates/goad/src`, stratum 3, where the kind still exists. R-58's test is
  specified over a **two-option** view, the case that would actually catch a key
  from the unanswered option.
- **"One unit test per typed kind" was not achievable and is no longer
  promised.** R-57 types four kinds; 007 draws one. For `text`, `number` and
  `choice` there is no host code to observe, and a test constructing values by
  hand would reproduce this very defect. Those three clauses are now **review,
  not a test** — SPEC-001 §7's existing convention for claims no host test can
  observe (R-9, R-18, R-20, R-56's tolerance clause), not a precedent invented
  here. The total match in `submitted` is what carries the kinds forward: adding
  one without a decided type is a compile error, which is a stronger guarantee
  than the absent tests would have been.

**Related.** F-7 finds §10's account of the R-8 precedent wrong in the same
paragraph; this repair rewrote the sentence F-7 cites, and F-7 should be read
against the new text rather than the old.

Endorsed by the user, 2026-09-15.

**Outcome:** verified | contested | withdrawn

### F-7 — §10's characterisation of the §7 precedent is wrong for R-8: its row points at unit tests in `canonical.rs`, not at the inbound fixture corpora

**Severity:** minor
**Location:** `design.md` §10, SPEC-001 §7 row

**Expected:** "Both are **outbound**, so the vehicle is unit tests in
`canonical.rs` … — not the inbound fixture corpora, **which is where the R-8 and
R-2 rows already point**."

**Observed:** SPEC-001 §7's row for R-6/R-7/R-8 reads, in full: *"unit: the three
`canonical.rs` serialization tests above, each against the literal JSON of §6.1
parsed to a `serde_json::Value`, so key order is not asserted and a missing
`protocol` or `type` is"* — unit tests in `canonical.rs`, and no fixture at all.
R-2's row (R-1/R-2/R-3) points at both `canonical.rs` unit tests *and* fixtures
*and* an integration test.

**Evidence:** `docs/specs/001-host-backend-protocol.md:379` (the R-6/R-7/R-8 row)
and `:376` (the R-1/R-2/R-3 row).

The design's conclusion — outbound requirements are held by `canonical.rs` unit
tests — has a real precedent and is the right family of vehicle; it is the
sentence justifying it that is false, and it is false in the direction that makes
the precedent look weaker than it is. It matters because the sentence will be
transcribed into SPEC-001 §7 at reconciliation, where a wrong claim about a
neighbouring row is cheap to make and expensive to find. (F-6 is the separate
question of whether that vehicle can hold *this* requirement.)

**Disposition:** doc-wrong
**Response:** Upheld, and **already repaired by F-6** before this finding was taken: that disposition rewrote §10's SPEC-001 §7 row wholesale, and the false clause — "which is where the R-8 and R-2 rows already point" — went with it, as did §9's matching "not the inbound fixture corpora". Nothing in the design now characterises a neighbouring row.

The raiser's underlying point was taken seriously rather than assumed away: the replacement text makes its own precedent claim — that "review, not a test" is §7's existing convention where no host test can observe a requirement — and that claim was checked against the rows themselves before being written. R-9/R-19, R-18 and R-20 each carry it in those words. F-7 is the reason it was checked instead of asserted.

Endorsed by the user, 2026-09-15 (batched with the other mechanical findings).

**Outcome:** verified | contested | withdrawn

### F-8 — AC-5 ranks its two assertions backwards: the wire assertion cannot see the regression AC-5 names

**Severity:** major
**Location:** `design.md` §9 AC-5 row, §8 R-5; `slice-007.md` AC-5

**Expected:** AC-5's row — "tick a box, take a `Shift::Retained` fold …, then
submit — **the value is still `true` on the wire** … Asserting the widget is still
ticked is the weaker half and is kept as well; **the wire assertion is the one a
regression cannot survive**."

**Observed:** the regression AC-5 exists for is named in `slice-007.md`: *"`serve`
calls `glass.present` at the top of every loop iteration and `Glass::present` is
contracted to write every property, every time"* — i.e. a present clobbering the
form on screen. Under this design the wire value is built from the **draft**
(`answer()` → `submitted(draft.state_of(..))`, §5.2), and no present writes the
draft. So inject the regression AC-5 names — `option_rows` stops reading the draft,
or writes `checked: false` unconditionally, or the new `FieldRow` model is never
handed to the window — and the draft is untouched, `answer()` still returns
`true`, and **the wire assertion is green while the screen is wrong**.

The two assertions catch different defects and neither implies the other:

| injected defect | wire assertion | widget assertion |
|---|---|---|
| present stops writing `checked` from the draft | **green** | red |
| the draft is dropped on `Shift::Retained` | red | red |
| `answer()` walks the draft's keys rather than declared fields (D6) | red | green |

So for AC-5 specifically the *widget* assertion is the load-bearing one, and the
design demotes it. The stronger claim — "the one a regression cannot survive" — is
true of AC-1 and AC-4, where the wire is the only place the defect shows, and the
design has generalised it to a row where it does not hold.

**Evidence:** `design.md` §5.2 *The controller* (values built from the draft, not
from the widgets); §5.3 (`FieldRow.checked` — "from `draft.state_of(option,
field)`, a lookup"); `slice-007.md` AC-5's own statement of the regression;
`crates/goad/src/glass.rs:87-90`, `crates/goad/src/controller.rs:611`.
`docs/memory/a-green-test-can-assert-a-proxy.md`, *How to apply*: "Inject the
regression the case is written against, run it, read the message, revert."

§9's closing paragraph makes the same generalisation — "Every field test either
reads the wire or asserts something about the screen" — which is a disjunction
where AC-5 needs a conjunction, and says so.

**Disposition:** doc-wrong
**Response:** Upheld, and the raiser's injection table is the argument: the wire value is built from the draft (`answer()` → `submitted(draft.state_of(..))`) and no present writes the draft, so the regression AC-5 exists for — a present clobbering the form — leaves the wire green and the screen wrong. The design had generalised a rule true of AC-1 and AC-4 to a row where it inverts.

**Changed:** §9's AC-5 row and `slice-007.md`'s AC-5 now require **both** assertions and say which is load-bearing *here* — the on-screen one — with the wire assertion named as guarding a different defect (the draft dropped on `Retained`). §9's closing paragraph said "Every field test either reads the wire or asserts something about the screen"; it now carries the exception, with the two injected defects that make it a conjunction rather than a disjunction for this row. §8/R-5 needed no change: it is about proxies generally and remains true.

Endorsed by the user, 2026-09-15 (batched with the other mechanical findings).

**Outcome:** verified | contested | withdrawn

### F-9 — `accessible-description: field.id` is not a unique selector: field ids repeat across options, and `element_described` silently takes the first

**Severity:** major
**Location:** `design.md` §5.2 *The window*, §9 AC-1 and AC-4 rows

**Expected:** §5.2 — a `CheckBox`'s "`accessible-description` is the field id,
mirroring how an option button already carries its option id (`app.slint:49`),
**because a label may repeat and an id may not**."

**Observed:** the premise is false for fields, and the design establishes that
itself two sections earlier: §3 — "**Two options can use the same field id.** R-52
requires field ids to be unique *within an option*, not across the view", with
§5.5's edge case "the same field id in **two** options | two distinct draft keys,
two independent boxes". So a view with a `morning` and an `evening` option, each
carrying `done`, draws two checkboxes with `accessible-description == "done"`.

The existing selector resolves that by taking the first match and reporting no
ambiguity:

```rust
fn element_described(window: &PromptWindow, description: &str) -> Option<ElementHandle> {
  ElementQuery::from_root(window)
    .match_predicate(move |element| {
      element.accessible_description().as_deref() == Some(description.as_str())
    })
    .find_first()
}
```

(`crates/goad/tests/renderer/tree.rs:42-49`.) AC-4's test is exactly this case —
"two options each carrying fields" — so the criterion that exists to prove values
never cross options is the one whose test cannot address the second option's box.
A test that ticks the wrong box and then asserts on the wire would be green in the
world where the design is right *and* in a world where the `option` half of the
draft key is ignored, which is AC-4's whole subject.

**Evidence:** `design.md` §3 *Two options can use the same field id*; §5.5 edge
case table; `crates/goad/tests/renderer/tree.rs:42-49` (the selector);
SPEC-001/R-52 and its `R-52-the-same-field-id-in-different-options` fixture
(`docs/specs/001-host-backend-protocol.md:103`, `:386`). For contrast, the
selector's doc comment at `tree.rs:38-41` records why option ids *are* safe:
"never by label, which two options may share" — the property field ids do not
have.

The design's sentence is right about labels and wrong about ids, and the fix it
implies (a selector composed of both ids, or an item index — see F-10) is a change
to §5.2's stated markup, not to a test helper alone.

**Disposition:** doc-wrong
**Response:** Upheld. The design's premise — "a label may repeat and an id may not" — is true of option ids (R-14) and false of field ids, which R-52 makes unique only *within* an option; `R-52-the-same-field-id-in-different-options` is a fixture asserting the sharing case is legal. `element_described`'s `find_first()` (`tree.rs:42-49`) would take whichever box came first and report no ambiguity, so AC-4's own case — two options each carrying fields — had an unaddressable second box.

**Changed:** `design.md` §5.2 *The window*, and the AC-1, AC-2 and AC-4 rows in §9.

The repair is a **scoped identity, not a composite one**. The `CheckBox` keeps `accessible-description: field.id`; the per-option container carries `accessible-description: option.id`; a field is addressed by a query scoped to its option — `match_accessible_description(option)` → `match_descendants()` → `match_accessible_description(field)`, which the testing API supports (`i-slint-backend-testing-1.17.1/search_api.rs:239`).

**Joining the ids into one description was rejected**, and the reason is worth recording because it is not obvious: option and field ids are backend-supplied strings whose *characters* no requirement constrains. Any separator can appear inside an id, so `"a/b" + "/" + "c"` and `"a" + "/" + "b/c"` collide. A composite description invents a string grammar over data the host does not control — P-B, with a delayed fuse.

AC-4's row now also **requires the two options to share a field id**. That is the case the criterion exists for, and without it the test is green both where the design is right and where the draft key's `option` half is ignored.

*Left open deliberately*, and recorded in §5.2 as such: whether the per-option container should also declare `accessible-role: list` and `accessible-item-count`, as the options container does. That is a question about what a screen reader announces to a person, not about what a test can reach, and it should not be settled by what a test happens to need.

Endorsed by the user, 2026-09-15 (batched with the other mechanical findings).

**Outcome:** verified | contested | withdrawn

### F-10 — AC-6's "the diff for those files is empty" is not achievable: `tree.rs` builds `OptionRow` with an exhaustive struct literal

**Severity:** major
**Location:** `design.md` §9 AC-6 row, §5.2 *The window*; `slice-007.md` AC-6

**Expected:** AC-6's observable — "the existing option tests pass **unmodified**,
and a zero-field option adds no element to the tree", vehicle "`tree.rs`,
`table.rs`, `wiring.rs` as they stand — **the diff for those files is empty**".
§5.2 — "`OptionRow` grows one member and keeps the three it has, **so every
existing option test stands.**"

**Observed:** the generated `OptionRow` is a plain Rust struct with a field per
member (`i-slint-compiler-1.17.1/generator/rust.rs:706-737`), and `tree.rs`
constructs it with an exhaustive struct literal:

```rust
fn rows(entries: &[(&str, &str, &str)]) -> ModelRc<OptionRow> {
  let rows: Vec<OptionRow> = entries
    .iter()
    .map(|(id, label, view)| OptionRow {
      id: SharedString::from(*id),
      label: SharedString::from(*label),
      view: SharedString::from(*view),
    })
    .collect();
  ModelRc::new(VecModel::from(rows))
}
```

(`crates/goad/tests/renderer/tree.rs:28-38`.) Adding a fourth member is `E0063`,
missing field `blocks`. The same is true of the production constructor at
`crates/goad/src/glass.rs:138-150`, which the design expects to change anyway.
`table.rs` and `wiring.rs` do not name `OptionRow` and are unaffected; `tree.rs`
is, and `tree.rs` is the file the row names.

**Evidence:** `crates/goad/tests/renderer/tree.rs:28-38`;
`i-slint-compiler-1.17.1/generator/rust.rs:706-737` (`generate_struct` emits one
`pub` field per member, `#[derive(Default, PartialEq, Debug, Clone)]`);
`grep -rn "OptionRow" crates/goad/tests` shows `tree.rs` as the only test
constructing one.

The criterion's *intent* survives — the assertions in those tests need not change,
and `..Default::default()` keeps the helper's diff to one line — but "the diff for
those files is empty" is a stronger claim than the design can make, and an
acceptance criterion stated as an empty diff is one an implementer will be tempted
to satisfy by not touching the file. Restating it as "no assertion in the existing
option tests changes" is the claim that is both true and load-bearing.

**Disposition:** doc-wrong
**Response:** Upheld. `OptionRow` is a generated struct with a field per member, and `tree.rs:28-38` builds it with an exhaustive literal, so a fourth member is `E0063` — as it is at the production site `glass.rs:138-149`, which the design expects to change anyway. "The diff for those files is empty" was unachievable.

**Changed:** the criterion moves from the diff to the assertions, in §9's AC-6 row and on the card. The existing option tests must pass with **no change to what they assert**, the mechanical `blocks: ModelRc::default()` in the builder being the only edit permitted. The raiser also checked the other two files: `table.rs` and `wiring.rs` never name `OptionRow`, so their diff genuinely is empty, and the row now says which claim applies to which file.

Endorsed by the user, 2026-09-15 (batched with the other mechanical findings).

**Outcome:** verified | contested | withdrawn

### F-11 — D11's compile-error device names the wrong function: `submitted` matches `Edited`, not `FieldKind`, so a kind added to R-16 breaks nothing there

**Severity:** minor
**Location:** `design.md` §5.2 *The draft* (the doc comment on `submitted`), §7 D11; `slice-007.md` §Open questions closing paragraph

**Expected:** §5.2's comment on `submitted` — "SPEC-001/R-57, in one total match.
**A kind added to R-16 without a decided submitted type becomes a compile error
naming this function** — the device `present()` already uses for `View`'s
variants (`view_model.rs:103-106`)." D11 gives that as the reason to land the
`Edited` seam now, and `slice-007.md` restates it as "R-57 has a single
enforcement site the compiler guards".

**Observed:** `submitted` matches on `Edited`, a host-local enum declared in
`crates/goad/src/draft.rs`. `FieldKind` is declared in
`crates/goad-semantics/src/protocol/canonical.rs:245-256` and has no relationship
to `Edited`. Adding a sixth variant to `FieldKind` leaves `submitted` exhaustive
and compiling; it breaks only whatever matches `FieldKind` exhaustively. Under
this design that is the new mapper arm in `present()` that sorts a kind into drawn
or `Undrawn::FieldForm` — `view_model.rs`, not `draft.rs`.

The analogy to `present()`'s existing device is what makes the slip easy to miss:
`present` matches `View`, a canonical enum, so it genuinely is the compile error
when the protocol grows a view kind (`view_model.rs:107-109`). `submitted` matches
a host enum, so it is a compile error when *the host* grows a drawn kind — which
is the person who was already writing the mapping.

**Evidence:** `design.md` §5.2 (`fn submitted(edited: &Edited)`, and the `Edited`
declaration above it); `crates/goad-semantics/src/protocol/canonical.rs:245-256`
(`FieldKind`); `crates/goad/src/view_model.rs:107-109` (`present`'s match on
`View`).

The seam may still be worth landing — D11's second sentence ("Without it, adding a
kind reshapes `Command`, `Draft`, `install.rs` and every test that builds a
command") is untouched by this. What is wrong is the first sentence, the one the
slice card promoted into a standing claim about R-57 being compiler-guarded.

**Disposition:** doc-wrong
**Response:** Upheld. `submitted` matches `Edited`, declared in `crates/goad/src/draft.rs`; `FieldKind` is canonical (`canonical.rs:245-256`) and unrelated. A sixth `FieldKind` leaves the match exhaustive. The analogy to `present()` is what made the slip easy to miss — `present` matches `View`, a *canonical* enum, so it really is the compile error when the protocol grows a view kind.

**Changed:** `submitted`'s doc comment in §5.2 now states what the match does guard (the *host* growing a drawn kind without deciding what it submits), states explicitly that it is **not** the guard against the protocol growing a kind, and names the site that does break — `present()`'s mapper arm, which must sort a new kind into drawn or `Undrawn::FieldForm`. D11 carries the same correction and now leads with its load-bearing reason, which F-11 leaves untouched: without the seam, adding a kind reshapes `Command`, `Draft`, `install.rs` and every test that builds a command. `slice-007.md`'s closing paragraph had promoted the wrong claim into a standing one and is corrected there too.

Endorsed by the user, 2026-09-15 (batched with the other mechanical findings).

**Outcome:** verified | contested | withdrawn

### F-12 — AC-2's on-screen half names a mechanism §5.2 does not provide

**Severity:** minor
**Location:** `design.md` §9 AC-2 row, §5.2 *The window*

**Expected:** AC-2's observable — "a heading is drawn where the `group` value
changes, and the drawn order equals the declared order", held by "the rule as a
pure `present()` test (`mapper.rs`); **and** the order as it reaches the screen,
by `accessible-item-index` over the tree (`tree.rs:93-99`)".

**Observed:** the screen half has nothing to read. `accessible-item-index` reaches
the tree today because `app.slint` sets it explicitly on each option button,
inside a container that declares `accessible-role: list`,
`accessible-label: "options"` and `accessible-item-count`
(`crates/goad/ui/app.slint:42-52`). §5.2's markup sketch for fields sets neither:
the `CheckBox` carries `checked`, `toggled` and `accessible-description`, and no
container for `FieldBlock`/`FieldRow` is described at all.

**Evidence:** `crates/goad/ui/app.slint:42-52` (the existing mechanism, including
`accessible-item-count: root.options.length` and `accessible-item-index: index`);
`crates/goad/tests/renderer/tree.rs:91-100` (the assertions that consume them);
`design.md` §5.2 *The window* (the proposed markup, which adds neither).

Either the markup gains the two accessibility properties per field — which also
gives F-9 its unambiguous selector — or AC-2's second half needs a different
observable and should say which. As it stands the row names a vehicle that will
not exist, which is the AC-8 shape in F-6 at smaller scale.

**Disposition:** doc-wrong
**Response:** Upheld: `accessible-item-index` reaches the tree today only because `app.slint:42-52` sets it, inside a container declaring `accessible-role: list` and `accessible-item-count`, and §5.2's field markup declared neither. The row named a vehicle that would not exist — the AC-8 shape from F-6 at smaller scale, as the raiser says.

**Changed:** §9's AC-2 row. The on-screen half is now the option's field descriptions collected with `find_all()` and compared as a sequence to the declared order.

**This is a better observable than the one the finding asked for**, which is why the markup did not gain the two properties. `find_all()` returns matches in **tree order** — depth-first pre-order over children in index order (`i-slint-core-1.17.1/item_tree.rs:926-948`, reached via `search_api.rs:304`) — so the assertion reads the order the tree is actually in. `accessible-item-index` asserts a number the markup supplied, which stays correct under a reordering that sets it consistently: a proxy, and the shape `docs/memory/a-green-test-can-assert-a-proxy.md` names. The finding's alternative would have worked; it would also have been the weaker of the two.

Disposed together with F-9, which supplies the scoping the collection needs: the sequence is gathered per option, so two options' fields cannot interleave in the assertion.

Endorsed by the user, 2026-09-15 (batched with the other mechanical findings).

**Outcome:** verified | contested | withdrawn

### F-13 — P-1 as stated is not the rule the design applied, and `datetime`'s exclusion has no stated reason at all

**Severity:** minor
**Location:** `design.md` §4 P-1, §3 *What the tools cannot do* and *the command channel*, §7 D4

**Expected:** P-1 — "The contract is typed for the protocol. **The renderer is
subset by what can be tested.** … What this renderer draws is a fact about this
renderer, and it is decided by whether a test can drive the widget end to end."
P-1 is declared to settle "why R-57 has four rows and the renderer has one kind".

**Observed:** three different criteria did the deciding, and testability is only
one of them.

| kind | the reason the design actually gives | is that "can a test drive it"? |
|---|---|---|
| `text` | §3: `edited` fires per keystroke into a one-slot channel — "**This is the constraint that decided the drawn set**" | no; a transport constraint |
| `number` | D4: `SpinBox` is `int` against an `f64` protocol bound and invents `maximum: 100` | no; a fidelity constraint |
| `choice` | D4: `ComboBox` has no set-value action | yes |
| `datetime` | nothing, anywhere in the design | — |

§3 and P-1 each claim to be the constraint that decided the set, and §3 tries to
reconcile them in one clause — "which is the same reason, from the other side" —
which is true of `choice` and not of `text` or `number`. `datetime` is excluded in
the §1 boundary table and in D4's "the card's four kinds" without a reason being
given for it once.

**Evidence:** `design.md` §4 P-1; §3 *What the host's own design requires*, final
paragraph ("This is the constraint that decided the drawn set"); §3 *What the
tools cannot do*; §7 D4; §1 boundary table row 1.

The cost is not academic. P-1 is the rule §4 says settles conflicts, and R-3's
mitigation rests on §3's channel argument, not on P-1's testability argument. A
later slice reading P-1 alone could conclude that finding a drivable text control
discharges the objection to `text` — which is precisely the relaxation R-3 exists
to prevent. Stating the rule as the design applies it ("a kind is drawn only when
the widget is faithful to the protocol's type, the transport can carry its edit
cadence, **and** a test can drive it end to end") costs one sentence and closes
that reading.

**Disposition:** doc-wrong
**Response:** Upheld on both halves. P-1 claimed testability decided the drawn set; the design actually used three different obstacles, and `datetime` had no stated reason anywhere.

**Changed:** P-1's second sentence is restated as the rule the design applied — "the renderer draws a kind only when nothing about this host stands in the way of drawing it well" — followed by a table giving each kind its actual obstacle and the *kind* of obstacle it is: `text` transport, `number` fidelity, `choice` testability, `datetime` contract. Any one is sufficient.

`datetime`'s missing reason is now the strongest of the four, and it came free from F-5: R-57 gives it no submitted form, so there is nothing a drawn control could send. The gap the raiser found had already been half-closed by another repair.

The consequence the raiser named is kept explicitly, because it is why this is not cosmetic: R-3's mitigation rests on the transport argument, so a later slice that finds a drivable text control has **not** discharged the objection to `text`.

Endorsed by the user, 2026-09-15 (batched with the other mechanical findings).

**Outcome:** verified | contested | withdrawn

### F-14 — the no-ADR argument answers D1 and does not reach D3, the decision most likely to be reversed by accident

**Severity:** major
**Location:** `design.md` §10 *No ADR*, D3; `canon-delta.md` *Deliberately out*

**Expected:** §10 — "**No ADR.** Considered for D1 … and found unnecessary,
because SPEC-001 §1 already states that rationale as the spec's own purpose… D5
… is recorded in `slice-007.md` OQ-1 and this design; D11 … is enforced by the
compiler. Neither is reversible by accident in the way an ADR exists to prevent."

**Observed:** the paragraph considers D1, D5 and D11 and never considers **D3**,
the decision that `datetime` has no defined submitted form. D3 is the one decision
in the set that *presents as an omission*: a later reader meets an R-57 that types
four kinds and leaves the fifth blank, and the two repairs available — delete the
clause as an oversight, or fill it in with a plausible string format — are both
one-line edits that read as tidying. Neither leaves a trace of having reversed
anything. `docs/AGENTS.md` §Canon: "**Any decision that shaped the draft and could
later be reversed by accident gets its own ADR.**" D3 shaped R-57's text directly.

The argument offered for D1 does not transfer. SPEC-001 §1 records why a contract
must not track a renderer, which is D1's rationale; it records nothing about why
one kind is deliberately left unspecified while four are fixed. `canon-delta.md`'s
*Deliberately out* holds that reasoning today — and `canon-delta.md` is a slice
artefact that is consumed at promotion, so after reconciliation the reasoning
exists nowhere in canon.

**Evidence:** `design.md` §10 final paragraph; `design.md` §7 D3;
`canon-delta.md:96-102`; `docs/AGENTS.md`, *Canon that does not exist yet, or
must change*, final line of the Promote paragraph.

D1 itself is the weaker case and the design's argument for it is reasonable. D3 is
the strong one, and it is also the decision F-5 says is not yet correctly stated —
so if F-5 changes R-57's last sentence, the decision behind the new sentence is
the one needing a record.

**Disposition:** follow-up
**Response:** The finding is upheld on its substance and declined on its remedy.
Upheld: D3 is the decision that presents as an omission, D1's argument does not
transfer to it, and the reasoning lived only in `canon-delta.md`, which is
consumed at promotion — so after reconciliation it would have existed nowhere.
The raiser is also right that F-5 sharpens this, since the sentence D3 produced
has now been rewritten once.

**Declined: no ADR.** User decision, 2026-09-15 — "ADR feels wrong; find a home
for it in the planned next slices". The judgement is that an ADR is the wrong
*shelf*, not that the record is unnecessary: an ADR records why the code is
shaped as it is, and this is a question about what a future slice must decide.
The document a slice-planner actually reads is the roadmap.

**Where it landed**, three places with three jobs:

- `docs/roadmap.md` §Open decisions — the rationale itself, as a sibling of the
  existing OQ-1/OQ-2 entry: why four kinds were typed and the fifth was not, what
  R-57 does instead of leaving a hole, the trigger (the slice that first *draws*
  a `datetime` field, which is what produces evidence about which freedoms
  matter), why not 010 (the roadmap's own rule — canon-changing work folded into
  a documentation slice blows its tier), and the warning that reversal reads as
  tidying in either direction.
- `slice-007.md` Follow-ups — the pointer, because the ledger's Protocol requires
  a `follow-up` to land there rather than resting in a disposition.
- `design.md` §10 — the *No ADR* paragraph now considers D3 rather than passing
  over it, states why D1's argument does not transfer, and names the two records
  that carry it. That was the raiser's actual complaint about that paragraph.

**Note on scope.** Part of what F-14 asked for already landed under F-5: SPEC-001
gains **OQ-4**, which carries the degrees of freedom themselves into canon. So the
gap at disposition time was narrower than at raise time — what was still missing
was the record that this was *decided* rather than overlooked, which is what the
roadmap entry supplies.

**Outcome:** verified | contested | withdrawn

### F-15 — AC-10's bound is not separable from 008's non-goals: the instruments of legibility are exactly the things AC-10 forbids changing

**Severity:** minor
**Location:** `design.md` §9 AC-10 row, §1 boundary table; `slice-007.md` AC-10, §Non-goals

**Expected:** AC-10 — "iterating with the implementer **until the form is
legible**: which fields belong to which option, and which heading covers which
fields… The bound is on what may be *changed*, never on what may be *said* —
feedback beyond legibility is recorded **verbatim and not actioned**."

**Observed:** the two questions AC-10 makes binding are questions about visual
grouping, and the means of answering them are the four things §1 and
`slice-007.md` §Non-goals assign to 008: "typography, spacing, window sizing, the
idle surface". Indentation, gap, heading weight and container border are how a
person comes to see which heading covers which fields. So a legibility complaint
and a spacing complaint are the same complaint seen from two sides, and the rule
"legibility may be actioned, look may not" has no operative test to apply at the
moment it is needed — which is a person saying "I can't tell where the second
group starts".

**Evidence:** `design.md` §9 AC-10; `design.md` §1 boundary table, row 2
("grouping by the `group` hint | typography, spacing, window sizing, the idle
surface — **008**"); `slice-007.md` §Non-goals, *The look — slice 008*
("007 does only what drawing fields *forces* — a container per group").

`slice-007.md`'s own formulation is the better bound and is testable: **what
drawing fields forces**. AC-10 does not use it. Naming the specific surfaces
AC-10 may move (the block container and its separator; the heading's own
treatment) and declaring everything else 008's would make the bound checkable
rather than a judgement made under pressure at the end of the slice.

**Disposition:** doc-wrong
**Response:** Upheld, and the raiser's own alternative is what was adopted. The two questions AC-10 makes binding are questions about visual grouping, and the means of answering them — indentation, gap, heading weight, container border — are the four surfaces §1 and §Non-goals assign to 008. So "legibility may be actioned, look may not" had no operative test at the moment it is needed, which is a person saying "I can't see where the second group starts".

**Changed:** §9's AC-10 row and the card's AC-10. The bound is now `slice-007.md`'s own formulation — **what drawing fields forces** — made checkable by naming the surfaces that may move: the block container and its separator, and the heading's own treatment. Typography, window sizing, the idle surface and the look of the controls are named as 008's. The verbatim-and-not-actioned rule for everything else is unchanged.

Applied on the responder's recommendation as stated to the user, within a batch they asked to be wrapped up; reversible if they prefer the original bound.

Endorsed by the user, 2026-09-15 (batched with the other mechanical findings).

**Outcome:** verified | contested | withdrawn

### F-16 — nothing in the design says what the demo backend must send, so R-55's "produce the effect of" clause is discharged by no artefact

**Severity:** minor
**Location:** `design.md` §9 AC-7 row, §3 *What the protocol requires*; `slice-007.md` §Scope (`examples/`)

**Expected:** §3 — R-55 says a renderer subset "MUST NOT be treated as, **or
produce the effect of**, a narrowing of the protocol… Drawing only `boolean` is
therefore fine." AC-7 requires "a demo backend that records `values` — the current
`examples/shell/backend.sh` discards them (research F13)".

**Observed:** the design settles the *reporting* half of R-55 thoroughly — per
field, per kind, with a named diagnostic line and an `Undrawn` variant. It says
nothing about the one artefact through which the subset propagates outward. The
example backend is the document a backend author copies, and `examples/` is in
scope for this slice. If it sends only `boolean` fields, the reference
implementation of a goad backend encodes this renderer's subset as the shape of a
form, and the next backend author never learns that `text`, `number` and `choice`
are admitted at all — which is "producing the effect of" a narrowing by the most
ordinary route available, without any host type or spec sentence changing.

**Evidence:** SPEC-001/R-55 (`docs/specs/001-host-backend-protocol.md:121`), the
"produce the effect of" clause; SPEC-001 §1's decay paragraph (`:21-26`);
`slice-007.md` §Scope, "`examples/` — a backend that sends a form, so `just demo`
can show one"; `design.md` §9 AC-7, which specifies only that the backend records
`values`; `research.md` F13.

One sentence closes it: the demo form carries at least one field of a kind this
renderer does not draw, so what a backend author copies is a protocol-shaped form
and what they see on screen is the undrawn report doing its job. That also gives
AC-3's human half a vehicle it does not currently have.

**Disposition:** doc-wrong
**Response:** Upheld. The design settles R-55's *reporting* half thoroughly — per field, per kind, with a named diagnostic and an `Undrawn` variant — and said nothing about the one artefact through which a subset propagates outward. `examples/` is in this slice's scope, and the example backend is what the next backend author copies: if its form carries only `boolean`, the reference implementation of a goad backend encodes this renderer's subset as the shape of a form, and the next author never learns that `text`, `number` and `choice` are admitted. That is "producing the effect of" a narrowing by the most ordinary route available, with no host type and no spec sentence changing.

**Changed:** §9's AC-7 row. The demo backend's form must carry **at least one field of a kind this renderer does not draw**, so what a backend author copies is a protocol-shaped form and what they see on screen is the undrawn report doing its job. As the raiser notes, it also gives AC-3's human half a vehicle it did not have.

One sentence, and it is the only artefact in the slice that discharges R-55's second clause.

Applied on the responder's recommendation as stated to the user, within a batch they asked to be wrapped up; reversible if they prefer AC-7 unchanged.

Endorsed by the user, 2026-09-15 (batched with the other mechanical findings).

**Outcome:** verified | contested | withdrawn

### F-17 — `design.md` §2 says every cited line was re-opened at its site; two `glass.rs` citations were not

**Severity:** minor
**Location:** `design.md` §2 preamble and *`Glass::present` is total by contract*, §3, §5.3; `research.md` F8

**Expected:** §2's preamble — "Cited from `research.md` rather than restated;
**every line below was re-opened at its site.**" The lines cited for the AC-5
mechanism are `glass.rs:103-108` (the `set_vec` and the re-handed `ModelRc`) and
`glass.rs:148-164` (`option_rows`).

**Observed:** `glass.rs:87` is `self.options.set_vec(options);` and `:88-90` the
re-hand; `:103-104` is `set_diagnostic_lines`, a different property. `fn
option_rows` begins at `:138` and ends at `:150`; `:148-164` runs off the end of
the file's option-row code. The file has not changed since `21811b7`, so the drift
is not staleness introduced after the design was written — `research.md` F8
carries the same `glass.rs:103-108`, so the citation was inherited rather than
re-opened.

**Evidence:** `grep -n "set_vec\|set_options\|fn option_rows" crates/goad/src/glass.rs`
→ `87`, `90`, `138`; `git log --oneline -- crates/goad/src/glass.rs` → last touch
`21811b7`; `research.md` F8; `design.md` §2, §3 and §5.3, each citing
`glass.rs:103-108`.

Raised at `minor` rather than `nit` only because these are the citations under the
design's single highest-impact risk (R-2 / AC-5), and because §2's preamble makes
a claim about the whole section that these two falsify. The other citations spot-
checked hold or are within two lines: `view_model.rs:29-41`, `:107-165`,
`:145-148`; `controller.rs:216`, `:611`; `diagnostics.rs:191-199`;
`reception.rs:25-28`; `wire.rs:125-134`; `install.rs` (six clones today, so
`editing` is correctly the seventh); `canonical.rs:752-771`.

**Disposition:** doc-wrong
**Response:** Upheld; both citations re-opened at their sites and both wrong. `glass.rs:87` is the `set_vec` with the re-hand at `:88-90`; `:102-104` is `set_diagnostic_lines`, a different property. `fn option_rows` runs `138`–`149`, so `:148-164` ran off the end of it.

**Changed:** every occurrence of `glass.rs:103-108` → `glass.rs:87-90` and `glass.rs:148-164` → `glass.rs:138-149`, across §2, §3 and §5.3.

The raiser's framing belongs on the record: these were the citations under the design's highest-impact risk (R-2 / AC-5), and `research.md` F8 carries the same wrong lines — so they were inherited rather than re-opened, which is exactly what §2's preamble claims did not happen. The preamble's claim is true again now, and the finding's spot-check list records which other citations were verified.

Endorsed by the user, 2026-09-15 (batched with the other mechanical findings).

**Outcome:** verified | contested | withdrawn

### F-18 — A-1 is settled affirmatively, and the member's generated type is `ModelRc<FieldBlock>`, not a `Vec`

**Severity:** nit
**Location:** `design.md` §5.5 A-1, §3 *The `[FieldRow]`-inside-a-struct binding*, §8 R-1

**Expected:** A-1 — "An array-typed struct member survives Rust codegen …
**nobody has compiled one here**", with a two-flat-models fallback and R-1 naming
`build.rs` failing in phase 1 as the signal.

**Observed:** it survives, and the pinned compiler says how. `generate_struct`
emits one `pub` field per member, typed by `rust_primitive_type`
(`i-slint-compiler-1.17.1/generator/rust.rs:706-737`), and `rust_primitive_type`
maps `Type::Array(o)` to `sp::ModelRc<#inner>` (`:110-113`) with no restriction on
where the array appears. The struct's derive list is
`#[derive(Default, PartialEq, Debug, Clone)]` (`:732`), and `ModelRc<T>`
implements all four for any `T`: `Debug` at `i-slint-core-1.17.1/model.rs:700-704`,
`Clone` at `:706-710`, `Default` at `:712-717`, `PartialEq` at `:719-728`. So
`blocks: [FieldBlock]` compiles, and R-1's fallback is not needed.

**Evidence:** the five citations above. `[[StandardListViewItem]]` (e.g.
`i-slint-compiler-1.17.1/widgets/material/tableview.slint:120`) is, as the design
suspects, a different thing — an array-typed *property*, not a struct member — and
is not what settles this; `generate_struct` is.

Two details worth carrying into the plan rather than discovering:

- the member is a `ModelRc<FieldBlock>`, so `option_rows` builds a nested model
  per option (`ModelRc::new(VecModel::from(..))`), not a `Vec`;
- `ModelRc`'s `PartialEq` is **pointer identity** (`model.rs:719-728`), so
  `OptionRow`'s derived `PartialEq` stops being structural the moment `blocks` is
  added. Nothing in the tree today compares `OptionRow` values, so this costs
  nothing now; it is the kind of thing a later equality assertion would trip over
  silently.

**Disposition:** doc-wrong
**Response:** Upheld. A-1 is settled from the pinned compiler exactly as A-2 was from the pinned core: an array-typed struct member generates as `ModelRc<FieldBlock>` (`i-slint-compiler-1.17.1/generator/rust.rs:706-737`, `:110-113`), with `PartialEq` by pointer.

**Changed:** §5.5's A-1 entry is marked discharged with the citation, its fallback kept but labelled unneeded; §6 and §9 now say both assumptions are settled from the pinned sources rather than left to phase 1. Those two sentences were split under F-1 precisely so this could land without touching A-2's half again.

Raised as a `nit` and dispositioned with the same weight as F-1, because together they take phase 1's entire discovery budget off the table: the plan opened the first phase to prove A-1 and A-2, and neither needs proving. Both are now regression pins against a future Slint.

Endorsed by the user, 2026-09-15 (batched with the other mechanical findings).

**Outcome:** verified | contested | withdrawn

### F-19 — AC-3's first observable is the diagnostic line's wording, which §5.2 declares will not be asserted by a test

**Severity:** minor
**Location:** `design.md` §9 AC-3 row, §5.2 *The diagnostic lines*

**Expected:** AC-3's observable — "**the diagnostic line names option, field and
kind**; the view is still shown; the option still answers, carrying only its
boolean keys", vehicle "`mapper.rs` for the `Undrawn` value, `reception.rs`'s
precedent for the diagnostics, and one wire test that a view with a `text` field
still submits".

**Observed:** §5.2 gives the two line templates and then says why they are in the
design at all: "No test asserts the current wording, so these are stated here **to
be reviewed once rather than discovered in a diff** (research delta 7)." That is a
statement that the wording is held by review, not by a test — and it is true of
today's code: `undrawn_line` (`crates/goad/src/diagnostics.rs:191-207`) is
unasserted, and `grep -rn "not drawn" crates/goad` returns only the format string
itself.

So AC-3's three clauses divide unevenly: the second and third have vehicles named
and real, and the first — the one about naming the option, the field and the kind
— has a vehicle of "review", which is not what an acceptance criterion's
*observable* column is for. The two readings are materially different work: either
the slice adds the first assertion of a diagnostic wording this project has (and
takes on the brittleness that `reception.rs`'s precedent was set up to avoid), or
AC-3's first clause is satisfied by asserting the `Undrawn` **value** carries the
option, the field and the form — which `mapper.rs` can do and which is what the
row's first-named vehicle actually reaches.

**Evidence:** `design.md` §9 AC-3; `design.md` §5.2 *The diagnostic lines*;
`crates/goad/src/diagnostics.rs:191-207`; `crates/goad/src/view_model.rs:56-60` as
the design describes the same fact ("no test asserts the diagnostic wording at
all").

The second reading is almost certainly the intended one and is the better test —
it asserts the mapper's output rather than a string — but the row as written says
"the diagnostic line", and the difference decides whether a wording change is a
red test or a silent one.

**Disposition:** doc-wrong
**Response:** Upheld, and the raiser's second reading is the intended one. §5.2 states that no test asserts the diagnostic wording — true of the code today (`undrawn_line`, `diagnostics.rs:191-207`, is unasserted) — so AC-3's first clause named an observable its own vehicle could not reach, and the two readings are materially different work.

**Changed:** AC-3's first observable in §9 and on the card is now the **`Undrawn` value** carrying option, field and kind, asserted in `mapper.rs`. The rendered line stays held by review, as every diagnostic wording in this project is, and the row says so rather than leaving a reader to infer it.

This is the better test on its merits and not merely the achievable one: it asserts the mapper's output rather than a string, so a wording change stays a silent diff while a *semantic* loss — the option, the field or the kind going missing — is red.

Endorsed by the user, 2026-09-15 (batched with the other mechanical findings).

**Outcome:** verified | contested | withdrawn

### F-20 — F-9's option-scoped selector names a nonexistent `ElementQuery` method

**Severity:** major
**Location:** `design.md` §5.2 *The window* and §9 AC-1/AC-4; F-9 Response

**Expected:** F-9's repair replaces the ambiguous unscoped field lookup with a
query the implementation can write against the pinned testing API: select the
option container by its accessible description, descend from it, then select the
field by its accessible description. The design spells that as
`match_accessible_description(option)` → `match_descendants()` →
`match_accessible_description(field)` and cites `search_api.rs:239`.

**Observed:** `ElementQuery` has no `match_accessible_description` method in the
pinned `i-slint-backend-testing-1.17.1`. Its public match builders are
`match_id`, `match_type_name`, `match_inherits`, `match_accessible_role`, and
`match_predicate`; line 239 is `match_descendants`, not an accessible-description
matcher. `accessible_description()` exists only as an `ElementHandle` reader, so
the intended query must express both description matches through
`match_predicate` (with owned captures) or name another real construction.

**Evidence:**
`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/i-slint-backend-testing-1.17.1/search_api.rs:232-287`
(the complete `ElementQuery` builder surface), `:700-708`
(`ElementHandle::accessible_description`), against `design.md:406-408`.

This is not a cosmetic API-name slip in explanatory prose. AC-1 and AC-4 make
the scoped query their widget-driving vehicle, and the named expression does not
compile. The tree structure F-9 chose can still provide an unambiguous selector;
the repair has not yet specified it using an API this pinned version provides.

**Disposition:**
**Response:**
**Outcome:**

### F-21 — F-5 turns this renderer's `datetime` subset into a protocol-wide prohibition

**Severity:** blocker
**Location:** `canon-delta.md` CD-1 R-57 and *Deliberately out*;
`design.md` §3, P-1, D3 and §10; F-5 Response

**Expected:** F-5's repair must close the contradiction with R-58 without
narrowing a capability R-16 admits merely because this renderer does not
implement it. R-55's second sentence guards the effect, not only whether the
restriction is phrased per renderer.

**Observed:** amended R-57 requires **every host** to submit no value for a
`datetime` field and to report it undrawn. That does not merely leave this
renderer as a subset: it makes a conforming renderer that draws and answers the
admitted `datetime` kind impossible. The defence that the protocol "declin[es]
to define a form uniformly for every host" establishes uniformity, not
non-narrowing. Uniformly converting an admitted field kind into an always-undrawn
kind is exactly the protocol producing the effect of today's renderer subset.

The repaired text demonstrates the loss directly. R-16 still admits a
`datetime` field; R-57 then forbids any host from submitting its answer, while
OQ-4 defers the only action that would make the field answerable. A backend may
send the field only to be told that no conforming host can draw it. That is no
longer the separation §3 says the design preserves between the protocol's typed
set and this renderer's drawn set.

**Evidence:** `docs/specs/001-host-backend-protocol.md:102-105` (R-15/R-16 admit
fields and name `datetime`) and `:112-121` (R-55 says an admitted capability a
renderer lacks must not produce the effect of a protocol narrowing), against
`canon-delta.md:51-56` (the protocol-wide MUST NOT), `:104-124` (the uniformity
argument and deferred form), and `CLAUDE.md`'s third invariant.

The degrees of freedom are a sound reason not to guess a format. They do not
authorize promoting one renderer's inability to answer the kind into the wire
contract. Until the contract defines an answer, the design needs a disposition
that preserves the distinction between "this renderer reports it undrawn" and
"the protocol forbids every renderer from drawing it"; the current repair erases
that distinction.

**Disposition:**
**Response:**
**Outcome:**

### F-22 — F-2's false "no visible change" claim remains in the lifecycle diagram

**Severity:** minor
**Location:** `design.md` §5.4 sequence diagram; F-2 Response

**Expected:** F-2's repair removes the claim that the post-edit present is
observationally inert and records that rebuilding the elements drops keyboard
focus once per tick.

**Observed:** the prose below the diagram now states the cost correctly, but the
diagram still ends the post-edit present with `same value, no visible change`.
The same section then says the focus ring disappears and the person must tab
from the top. Those are mutually exclusive accounts of the lifecycle, and the
diagram is the compact account an implementer is most likely to follow.

**Evidence:** `design.md:680-690` (the edit → present sequence and its terminal
note), against `design.md:704-718` (the rebuild drops focus once per tick) and
F-2's Response ("That write is the same *value* but not the same *element*,
followed by the mechanism, the cost, and its frequency").

**Disposition:**
**Response:**
**Outcome:**

### F-23 — F-17 did not replace every stale `glass.rs` citation it claimed to replace

**Severity:** minor
**Location:** `design.md` §2 *`Glass::present` is total by contract*; F-17 Response

**Expected:** F-17's Response says every occurrence of the invalid
`glass.rs:148-164` citation was changed to `glass.rs:138-149`, restoring §2's
claim that every cited line was re-opened at its site.

**Observed:** §2 still says `option_rows` rebuilds every `OptionRow` at
`glass.rs:148-164`. The function is at `:138-149`; the retained citation starts
on its penultimate line and continues beyond the function. Later §5.3 uses the
correct range, so the repaired design now contains both versions.

**Evidence:** `design.md:66-73`, `design.md:645-648`, and
`crates/goad/src/glass.rs:138-149`; F-17's Response states "every occurrence of
`glass.rs:148-164` → `glass.rs:138-149`, across §2, §3 and §5.3."

**Disposition:**
**Response:**
**Outcome:**

### F-24 — F-18 leaves A-1 open in §3 and R-1 after declaring it discharged elsewhere

**Severity:** minor
**Location:** `design.md` §3 *What the tools cannot do* and §8 R-1; F-18 Response

**Expected:** once F-18 settles A-1 from the pinned compiler, the design treats
the nested model as established everywhere and phase 1 pins it as a regression;
it no longer instructs phase 1 to discover whether it compiles or presents the
fallback as an active mitigation.

**Observed:** §5.5 and §9 say A-1 is discharged, but §3 still says the binding is
"established but unproven here", that nobody has compiled one in this workspace,
and that the plan proves it in the first phase. R-1 likewise retains the risk as
`low / medium`, gives the two-flat-model fallback as its mitigation, and says a
phase-1 `build.rs` failure is why A-1 is proven first. The repaired design
therefore gives phase 1 opposite instructions about whether this is an
experiment or a regression pin.

**Evidence:** `design.md:184-188` and `:939` against `design.md:828-835` and
`:981-984`. The affirmative compiler evidence is
`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/i-slint-compiler-1.17.1/generator/rust.rs:110-113,706-737`,
as F-18 established.

**Disposition:**
**Response:**
**Outcome:**

### F-25 — F-16 closes R-55 only in design validation, not in the slice's acceptance criterion

**Severity:** minor
**Location:** `design.md` §9 AC-7; `slice-007.md` AC-7; F-16 Response

**Expected:** F-16's repair makes the demo backend carry at least one field kind
this renderer cannot draw, so the reference example cannot encode the renderer's
`boolean` subset as if it were the protocol's field set. The slice's closure
criterion must require that observable, because audit walks the card's acceptance
criteria.

**Observed:** design §9 now requires the demo form to carry an undrawn kind, but
`slice-007.md` AC-7 still requires only a multi-field form whose submitted
answers are recorded. A form containing boolean fields alone satisfies the card
and can close the slice while reproducing the exact "effect of a narrowing" F-16
was raised to prevent. The two current-truth artefacts assign different work to
the same AC id.

**Evidence:** `design.md:965` (the added undrawn-kind clause),
`slice-007.md:166-170` (AC-7 without it), and `docs/AGENTS.md` *Audit & reconcile*
(audit walks every acceptance criterion in `slice-nnn.md`). F-16's Response calls
the added sentence "the only artefact in the slice that discharges R-55's second
clause", making the omission from the closure card load-bearing rather than
editorial.

**Disposition:**
**Response:**
**Outcome:**

## Depth of the round, and what was examined without a finding

Stated because `review-design-brief.md` requires it: a thread looked at and left
clean should say so, so a later round does not re-spend the budget.

**D6, attacked directly and it holds — against the code, not against R-58's
text.** Three paths were tried and none exists:

- *an undrawn field entering a `FieldBlock`* — `present()` is the only constructor
  of a `Presentation` (`view_model.rs:107-165`), `receive` is the only caller
  (`reception.rs:64-81`), and §5.2 puts the drawn/undrawn decision inside the same
  match that builds the block. There is no second door.
- *a draft key reaching the wire* — `Draft`'s inner map is a private tuple field,
  so outside `draft.rs` there is no iterator, no `keys()`, no `IntoIterator`, and
  `answer()` is outside `draft.rs`. Worth one qualification the design does not
  make: privacy makes this a property of the **type boundary**, enforced against
  every caller, and a convention *inside* `draft.rs`, where a future `impl` could
  add an iterator without the compiler objecting. It holds where D6 needs it.
- *a drawn field the walk would miss* — the walk and the glass read the same
  `blocks`/`fields` structure (§5.2, §5.3), so a field drawn but unwalked would
  have to be drawn from somewhere other than the presentation.

F-4 is therefore not a defect in D6's mechanism. It is that R-58's **text** states
a different property from the one D6 establishes, and I-3 asserts they are the
same.

**Examined and clean:**

- *The vocabulary scan's reach.* `DOMAIN` is the seven words the design names
  (`crates/goad-boundary/tests/checks/vocabulary.rs:18-26`), `group` is not among
  them, and the scan covers `.rs` and `.slint` over every workspace member
  excluding `tests/` and `target/` (`:29-34`) — so `crates/goad/ui/app.slint` is
  in and `examples/` is out, as AC-9 and research F14 say. `FieldBlock`,
  `FieldRow`, `heading`, `Draft`, `Edited`, `submitted`, `FieldForm` and
  `GroupHint` are layout and protocol-report names, not domain names; none is a
  concept the host branches on beyond the one branch R-18 permits the renderer.
- *The `resolve` ban.* Confirmed live and confirmed strict enough to catch what
  the design warns about: `mentions` splits on every non-alphanumeric character
  (`crates/goad-boundary/src/scan.rs:225-234`), so `resolve_field` and
  `resolve_draft` both trip
  `structure.rs::no_production_line_in_the_renderer_names_the_identifier_resolve`
  over `crates/goad/src` (`structure.rs:25`, `:306-315`). Nothing in the design
  proposes such an identifier.
- *`Undrawn::OptionFields`' blast radius.* Exactly as §2 states: one definition
  (`view_model.rs:70`), one construction (`:145-148`), one match arm
  (`diagnostics.rs:193-199`), one test (`mapper.rs:151-194`), and one doc-comment
  mention that will also need the rename (`view_model.rs:29`). No test asserts the
  wording.
- *`material` versus `fluent`.* The accessible surfaces are identical, and not
  only for those two. `CheckBox` declares `accessible-enabled`,
  `accessible-checkable`, `accessible-label`, `accessible-checked`,
  `accessible-role: checkbox` and `accessible-action-default` in all five stock
  styles (`widgets/{fluent,material,cosmic,cupertino,qt}/checkbox.slint:20-25`,
  `:19-24`, `:20-25`, `:21-26`, `:8-13`). `Button` likewise
  (`widgets/fluent/button.slint:29-34`, `widgets/material/button.slint:90-97`).
  R-7's premise holds.
- *The `build.rs` style citations.* `i-slint-compiler-1.17.1/lib.rs:264` is
  `let style = std::env::var("SLINT_STYLE").ok();`, so
  `CompilerConfiguration::new()` does read the environment, and
  `slint-build-1.17.1/lib.rs:152-157` is `with_style` overwriting `config.style`
  unconditionally. D12's reasoning and the three-line form in §5.2 are both
  correct.
- *§10's canon rows.* SPEC-002/OQ-4 does stand open and its §5 paragraph still
  says so (`docs/specs/002-host-scheduling-behaviour.md:115-119`, `:253`).
  ADR-003 does state that a stratum-3 manifest is checked by nothing
  (`docs/adr/003-...:161-166`), and POL-001 does carry the
  four-instruments-plus-scan-plus-residue structure the design cites
  (`docs/policy/001-the-phase-gate.md:117-120`, `:153`). Research F6's claim that
  no ADR-001 instrument reaches stratum 3 is consistent with both.
- *`Command::Edit`'s derives.* `Edited::Checked(bool)` keeps `Eq`; the claim needs
  no `serde_json::Value` and so does not depend on research F9.
- *`install.rs`.* Six `wire.clone()` installations today (`:19`, `:27`, `:33`,
  `:39`, `:42`, `:45`), so `editing` is correctly the seventh.

**Looked at shallowly, and left for a later round or for `review-plan`:**

- **The grouping rule against §5.5's edge cases.** Read once and found internally
  consistent — including the `Some(name) → None` sub-rule that only
  `design-log.md` states, which §5.2's "`heading: ""` is an **untitled block**, not
  a missing one" covers. Not traced against a worked example of a mixed
  grouped/ungrouped/undrawn sequence, which is where a run-boundary rule usually
  fails.
- **`examples/`** beyond F-16: `examples/shell/backend.sh` was not opened, and
  what AC-7's "the record" should be was not designed against.
- **The plan.** `plan.md` was not reviewed; this ledger's subject is `design`, and
  the slice is tier 2, so the plan carries its own ledger.
- **`design-log.md`** was read last, as the brief directs. It was checked for
  claims the design drops and found to add one worth carrying into a repair: its
  entry for the `Edited` seam states F-11's compile-error claim in a stronger form
  still — "Adding a kind then touches **five places the compiler finds**" — of
  which the compiler finds one. A repair to F-11 should not assume the design is
  the only place that claim now lives.

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

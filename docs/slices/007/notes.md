# Notes — Slice 007: the renderer grows a form

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 — the notice gets an owner | done | 2026-09-15 |
| PHASE-02 — the window draws a form | pending | |
| PHASE-03 — the mapper and the draft | pending | |
| PHASE-04 — the draft is retained, and the answer carries it | pending | |
| PHASE-05 — the form on the wire | pending | |
| PHASE-06 — the demo, and the look | pending | |

## Phase sheets

<!-- One block per phase, written at phase-plan time, immediately before
     execution. Disposable detail — it exists to get one agent through one
     phase. -->

### PHASE-01 — the notice gets an owner

**Objective:** back-pressure is retained at the edge and written to the window
from the frame, so the notice explaining a dropped action survives the present
that corrects the action (`plan.md:162-165`).

**Criterion ids, re-derived from `plan.md` — T-0 below, not copied from any
brief.** `plan.md:160-303`, PHASE-01 entire:

| kind | ids | count |
|---|---|---|
| entry | EN-1 | 1 |
| exit | EX-1 … EX-9 | 9 |
| verification (test) | VT-1, VT-2 | 2 |
| verification (agent) | VA-1, VA-2 | 2 |

Contiguous from 1 in each kind, no gap and no duplicate; PHASE-01 makes no
`PHASE-0N/<id>` cross-reference, so there is none to resolve. EX-7's enumerated
prose homes number **ten**, matching the count its own sentence asserts. The
re-derivation is the rule at `plan.md:36-45` and `review-plan.md` F-7 is why it
exists.

**Reading list**

- `docs/slices/007/plan.md:160-303` — PHASE-01 whole; `:104-152` — S-1..S-9.
  `:36-45` — the re-derivation rule. `:70-79` — why 01 is first and discharges
  no acceptance criterion.
- `docs/slices/007/design.md:284-362` (§5.1, system model — `Frame` gains
  exactly one field and becomes *more* total for it), `:654-759` (§5.3, state and
  ownership — the two tables that own `notice`'s route), `:760-864` (§5.4,
  *When it did not arrive, the correction is the feedback*).
- `docs/slices/007/slice-007.md:46-78` — §Scope; the audit diffs actual paths
  against it.
- Prior art, and the template for the whole phase: `crates/goad/src/wire.rs:142-181`
  — `Cancel`, read whole before writing `Notice`.
- The code under change: `crates/goad/src/wire.rs:72-140` (`Wire`),
  `controller.rs:92-102` (`Frame`), `:237-247` (`Controller::frame`),
  `:576-584` (`serve`'s signature), `:611`, `:692`, `:744` (the three present
  sites), `glass.rs:19-26` (the trait doc), `:107` (`set_notice`),
  `diagnostics.rs:304-306` (`BUSY_NOTICE`), `main.rs:85-119`.
- The test this phase inverts: `crates/goad/tests/renderer/wiring.rs:346-388`,
  `mod back_pressure`.
- `docs/memory/a-repair-sweep-misses-the-binding-site.md` — the class EX-7 is
  built against. `docs/memory/cite-requirements-not-finding-ids.md` — a comment
  in `src/` or `tests/` cites a spec section or an ADR, never a slice-local id.

**Assumptions & STOP conditions**

Assumed, and checked rather than taken on faith where the check was cheap:

- A-a — the three signature changes are compile errors at every site, so the
  compiler drives the sweep. Counts re-measured before starting: 69 `.frame()`,
  36 `serve(`, 6 `Wire::new`. All three match `plan.md`.
- A-b — `Wire` losing its window handle is forced by `-D warnings`, not chosen
  (EX-4, `plan.md:288-297`). Its blast radius is the enumerated one and nothing
  else.
- A-c — EN-1's "clean tree" is literal. The slice's base commit is `9447973`
  ("007: design resolved, plan accepted — the slice opens execute") and
  `git status` is empty at it, so everything the diff shows from here is this
  phase's, which is what VA-1 reads.

STOP, per `plan.md:106-152`, and do not improvise past one. Live for this phase:

- S-1 — an unresolved **design** issue. Back to design; never repaired in this
  sheet.
- S-2 — an existing test's assertion or fixture changing, outside VT-1's
  inversion, which is this phase's deliverable. A *signature* sweep is call
  shape and is expressly allowed.
- S-3 — a new dependency, or a feature added to an existing one. `Notice` uses
  `tokio::sync::watch`, which `Cancel` already uses, so none is needed.
- S-4 — anything under `crates/goad-semantics/`. Nothing here reaches it.
- S-9 — `Glass::present` acquiring a write-only-on-change path or an exception.
  This phase moves in the opposite direction, and must be seen to.

**Tasks**

<!-- [ ] todo · [~] in progress · [x] done · [!] blocked -->
- [x] T-0 — re-derive the criterion ids from `plan.md`. Table above.
- [x] T-1 — EN-1: `just check` on the unmodified tree. Exit 0; transcript kept.
- [x] T-2 — EX-1: `Notice` in `wire.rs:170-208`, following `Cancel` edge for
      edge. Two unit tests: retention across reads, and one clone's raise read
      from another (the edge sets, the loop reads).
- [x] T-3 — VT-2 red, then EX-5's `Frame`/`frame(notice)` half green.
- [x] T-4 — EX-6: `Glass::present` writes `notice` from the frame, and is the
      only writer anywhere.
- [x] T-5 — EX-3 + EX-4: `Wire::send` raises the signal; `Wire` loses the window
      handle, the `crate::generated` import, its hand-written `Debug` and the
      weak-handle paragraph. `Wire` now derives `Debug` and names no Slint type.
- [x] T-6 — EX-2: `main` constructs `Notice`, clones it into `Wire`, passes it to
      `serve`; EX-5's `serve` half — all three present sites sample it
      (`controller.rs:620`, `:702`, `:754`).
- [x] T-7 — the signature sweep across the six test files (S-2's allowance).
      Measured against `plan.md`: 69 `.frame()`, 36 `serve(`, 6 `Wire::new`.
- [x] T-8 — VT-1: `mod back_pressure` inverts. Three negative controls run, D-3.
- [x] T-9 — EX-7: **twelve** live homes corrected, against the ten the criterion
      enumerates. The ten: nine as listed, plus `app.slint:18` under D-2. The
      eleventh, `controller.rs:113`, is F-5 — the plan's list was short.
      The twelfth is this phase's own: `controller.rs:580` read *"An ordinary
      `async fn`, carrying **no** attribute at all"*, and D-4 put an attribute
      directly beneath it. The `future_not_send` reasoning is true and stays;
      the absence claim goes. Not a Finding — a defect in this phase's own work
      is fixed (`docs/AGENTS.md` §Where it goes). Note its shape: like the two
      homes `review-plan.md` had to add by hand, it contains no occurrence of
      "notice" and VA-2's grep could not reach it.
- [x] T-10 — refactor. D-1 and the `glass.rs` tidy; two prose sites the sweep
      falsified, caught at VA-1 and repaired (F-2).
- [x] T-11 — VA-1 (`git diff` over the six test files), VA-2
      (`grep -rin notice crates/goad/src crates/goad/ui crates/goad/tests`, read
      against EX-7's list, residue accounted for line by line). **130 lines, in
      seven classes that partition them** — each class defined by a rule an
      auditor can run, so the accounting can be checked rather than taken. The
      first cut of this table had five fuzzy classes and a count that
      contradicted its own list; the classes below are re-derived from the grep
      output, not reconciled to the total.

      | # | class | how to count it | n |
      |---|---|---|---|
      | 1 | import lines | `grep -cE ':[0-9]+: *use '` (13) + the two continuation lines of a multi-line `use` (`glass.rs:12`, `wiring.rs:352`) | 15 |
      | 2 | a `serve(..)` argument | `grep -cE ':[0-9]+: *Notice::new\(\),$'` (33) + the three `notice,` argument lines (`main.rs:117`, `event_loop/closing.rs:90`, `event_loop_schedule/scheduling.rs:118`) | 36 |
      | 3 | a `Notice` made, or cloned into a `Wire` | `let … = Notice::new();` and `Wire::new(.., notice.clone() \| Notice::new())` | 13 |
      | 4 | the mechanism itself, in `src/` and the markup | the struct, its `Default`/`impl`, `Wire`'s field, `Frame.notice`, `frame(notice)`, `serve`'s parameter and its three `notice.raised()` sites, `set_notice`, `app.slint:81` | 18 |
      | 5 | prose this phase wrote or repaired | 13 `//`-prefixed lines, plus `controller.rs:595` (inside the `#[expect]` reason string) and `app.slint:18` (a trailing comment on a declaration) | 15 |
      | 6 | test bodies — names, assertions, messages | `wire.rs`'s three unit tests (10) and `wiring.rs`'s `mod back_pressure` (16) | 26 |
      | 7 | pre-existing `BUSY_NOTICE` residue, untouched | `tests/renderer/reception.rs:31,723,728,730`; `tests/renderer/table.rs:532` ("pass un**notice**d", a substring hit and not a home); `src/diagnostics.rs:5,308` | 7 |

      15+36+13+18+15+26+7 = **130**, matching the grep. **The claim the classes
      support:** no line in any of them states the old rule. Classes 1-4 and 6 are
      code; class 5 is the twelve repaired homes plus the three doc-comments this
      phase wrote; class 7 names the const, its publicness and its exact string,
      and says nothing about a writer or a lifetime.
- [x] T-12 — EX-8, EX-9: `just check` exit 0; Harvest updated in place.

**Decisions taken during execution**

- **D-1 — `Notice` is its own type, not `Cancel` generalised.** The two are the
  same vehicle (`watch::<bool>`, both halves held, constructed in `main`, cloned
  into `Wire`, read by `serve`) and the duplication is real and visible. They
  stay separate because their *invariants* differ and a shared type would admit
  the violation: `Cancel` is level-held — once tripped it stays tripped, which is
  the failure mode a bare `Notify` has — and a common `set(bool)` would hand every
  caller the `set(false)` that breaks it. `Notice` in exchange has no async wait,
  because nothing ever waits on it; `serve` samples it. Within `design.md` §5.3's
  "follows `Cancel`'s route edge for edge", which is a statement about the route,
  not about the type.
- **D-2 — `ui/app.slint:18`'s comment is corrected, not left.** EX-7's tenth home,
  the one the plan says "may need nothing". `// transient` was a statement of the
  old lifetime: the property was cleared by the next present. It is now written
  from the frame every present and stays up until a successful send, so the word
  was a live home of the repaired rule. It reads `// back-pressure,
  host-authored; "" = none`. See F-1 for the surface it touches.
- **D-3 — VT-1 is checked with negative controls, not just run.** Three, each
  reverted immediately: `Glass::present` clearing `notice` unconditionally (the
  pre-slice defect); `Wire::send` lowering the signal on `Full` instead of
  raising it; and a `Notice` that does not retain — `raised()` lowering what it
  reads. All three turn VT-1 red, and the third fails on the *survival*
  assertion specifically, which is the one the phase exists for. The first draft
  did not have that property: an extra `notice.raised()` read, taken for an
  unrelated `diagnostics.is_clear()` assertion, consumed the signal and moved the
  failure to an earlier line. That assertion now sits at the end of the case,
  where it reads a value nothing else depends on
  (`docs/memory/a-green-test-can-assert-a-proxy.md`).
- **D-4 — `serve` carries `#[expect(clippy::too_many_arguments)]`.** Referred up
  rather than taken in phase, and **approved**. `serve`'s eighth parameter trips
  `clippy::too_many_arguments` (8/7), and `clippy::pedantic` is `deny` with the
  gate at `-D warnings`. The design settled the *shape* — §5.3 puts `notice`
  "alongside `cancel`", EX-2 "beside `cancel`" — and did not settle how to record
  the exemption. Applied provisionally so the rest of the phase could be verified:
  `#[expect(clippy::too_many_arguments, reason = …)]` on `serve`
  (`controller.rs:584-592`), the narrowest of the three instruments and the only
  one that self-clears if the arity ever drops. The alternatives were
  `too-many-arguments-threshold = 8` in `clippy.toml`, which weakens the lint
  workspace-wide for one function, and grouping `cancel` and `notice` into an
  edge-signals type, which is a design change and therefore S-1. The approval
  verified the three claims independently and widened the precedent: five
  `#[expect(clippy::…)]` sites already exist across the workspace
  (`goad-shell/src/backend/process.rs:58`, `ingress/mod.rs:598`,
  `ingress/client.rs:268`, `goad-semantics/src/protocol/wire.rs:163`,
  `goad/tests/renderer/harness.rs:33`), so this is the sixth and not a new kind
  of thing. The attribute's own doc-comment neighbour was stale as a result; see
  T-9's twelfth home.

**Findings**

- **F-1 — PHASE-01's Surfaces line omits a file its own EX-7 requires it to
  edit.** `plan.md:166-169` lists five `src/` files and six test files; EX-7's
  tenth home is `ui/app.slint:18`, which the criterion directs the implementer to
  "correct it or leave it deliberately". The finding is against `plan.md`, not
  against the diff: **the audit's surface diff is read against `slice-007.md`
  §Scope**, which does name `crates/goad/ui/app.slint`, so no undeclared path is
  in play and the audit needs no lead here. What this records is a plan-internal
  bookkeeping gap of exactly the class `review-plan.md` catalogued — a phase
  entry inconsistent with itself, in a document nothing checks against itself.
- **F-2 — a mechanical sweep falsified two doc comments, and only VA-1 saw it.**
  `.frame()` → `.frame(false)` is call shape at 64 call sites and *prose* at two
  more: `tests/renderer/scheduling.rs:138` and `tests/renderer/ingress.rs:1236`
  both describe what `serve` does, and `serve` passes `notice.raised()`, never
  `false`. Repaired to the real call. The same class EX-7 is built against
  (`docs/memory/a-repair-sweep-misses-the-binding-site.md`), arriving from the
  opposite direction: not a claim left behind by a sweep, but a claim *broken* by
  one. Worth an instrument — a sweep over a call shape should exclude comment
  lines and have them read.
- **F-3 — rustfmt reflowed thirteen one-line `serve(..)` calls in
  `tests/renderer/ingress.rs` into nine-line ones.** The eighth argument pushes
  the call past the width. It inflates that file's diff to ~164 lines of pure
  call shape, which is worth knowing before reading the slice diff cold. No
  assertion in the file changes (VA-1).
- **F-4 — `Wire` is strictly better after EX-4 than the design anticipated.** It
  now holds three channel halves and names no Slint type, no generated type, and
  no `slint::Weak`; its hand-written `Debug` is gone and it derives. All three of
  `send`'s arms are now synchronous and platform-free, so `Full` is unit-testable
  in `wire.rs` for the first time — what still needs a window is the notice
  *reaching the screen and staying there*, which is VT-1's.

- **F-5 — EX-7 enumerates ten live homes and there are twelve.** Both extras are
  in `controller.rs`, both were repaired in phase, and **neither contains an
  occurrence of "notice", so VA-2's grep reaches neither** — the same blind spot
  that forced `review-plan.md` to add two homes to the list by hand. What makes
  the pair worth recording together is that they were falsified by *different
  mechanisms*:

  - `controller.rs:113` — *"Nothing else is retained anywhere in the renderer"*,
    falsified by **the repair the phase set out to make**: `notice` is now
    retained, at the edge. `design.md` §5.3 anticipates this precisely and scopes
    its own heading against it ("`notice` below *is* retained, and is retained
    somewhere else"), so the design was right and only the code was stale. The
    doc now names the one retained thing that is not in the struct.
  - `controller.rs:575` — *"An ordinary `async fn`, carrying **no** attribute at
    all"*, falsified by **the fix taken to land the phase**: D-4's `#[expect]`
    sits nine lines below it. The `clippy::future_not_send` reasoning that is the
    sentence's actual subject is true and survives intact; only the absence claim
    goes, and the doc now points at the attribute that exists and says it is
    about arity.

  **The lesson is one turn sharper than "the list was short."** An enumerated
  list of live homes is a floor, not a ceiling — and *the fix itself creates new
  homes*, so the sweep is not finished when the enumerated sites are. Neither of
  these was found by an instrument; both were found by reading the file being
  changed. The plan review's own lesson, a third and fourth time
  (`review-plan.md` §Synthesis).

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-09-15 · PHASE-01, done · base commit `9447973`

### Produced

- `wire::Notice` — the back-pressure signal, `Cancel`'s route with a two-way
  setter and a non-async read (`plan.md` EX-1).
- `Frame.notice`, and `Controller::frame(notice)` — the frame is total again;
  `Controller` gained no field (EX-5).
- `Glass::present` as the **only** writer of the window's `notice` property
  (EX-6), and `Wire` reduced to three channel halves, naming no Slint or
  generated type (EX-4).
- VT-1 inverted: `a_full_channel_raises_the_notice_and_only_a_successful_send_lowers_it`.
  VT-2 beside it. Gate at 506 tests, from 503.

### Learned

- **A two-way `watch` is a different type from a one-way one, even with an
  identical body.** `Cancel`'s level-held invariant is enforced by the *absence*
  of `set(false)`; generalising the two into one signal type hands every caller
  the operation that breaks it. D-1.
- **A signal a test reads more than once must not be read for anything else.**
  Sampling `notice.raised()` for an unrelated assertion made VT-1's failure
  attribute to the wrong line under a non-retaining-signal control. D-3.
- **A mechanical call-shape sweep edits comments too.** `.frame()` →
  `.frame(false)` was correct at 64 call sites and wrong at the two that were
  prose about `serve`. F-2, and the mirror image of
  `docs/memory/a-repair-sweep-misses-the-binding-site.md`.
- **A repair sweep is not finished when the enumerated sites are, because the
  fix creates homes of its own.** EX-7 listed ten; there were twelve, and the
  twelfth was falsified by the lint exemption taken to land the phase rather than
  by the repair the phase is about. F-5 carries both.
- **And the record of the sweep is itself a home.** T-11's first VA-2 accounting
  labelled a class `2` and listed five sites under it; the classes did not sum to
  the grep's total. The instrument that discharges EX-7 had the defect EX-7 is
  about — a count that does not match the list it describes
  (`plan.md` §Overview), in the closing bookkeeping of the phase whose subject is
  claims going stale where nothing checks them. **Third mechanism, same shape:
  the repair, the fix that landed it, then the record of both.** The repair was
  to re-cut the classes so each has a rule an auditor can run, and to derive the
  counts from the grep rather than reconcile them to the total — reconciling
  produces a number that adds up and still is not true.
- **An enumerated list of live homes is a floor, not a ceiling.** EX-7 named ten
  and there were eleven; the extra one was found by reading the file being
  changed, and no instrument in the gate or in VA-2 could have reached it. F-5.

### Open

- **F-1** — `plan.md`'s PHASE-01 Surfaces line omits `ui/app.slint`, which its
  own EX-7 sends the phase to edit. A plan-internal gap; no surface breach, and
  nothing for the audit's path diff to chase.
- **F-2** — no instrument separates a call-shape sweep from the comments it
  rewrites. Candidate for `docs/memory/`.
- **`serve` is one parameter over the arity lint** and now carries an
  `#[expect]` for it (D-4). The next phase that adds a parameter inherits the
  argument, not a free pass.
- Unchanged and not this phase's: keyboard focus dropped on every present
  (`design.md` §5.4, `slice-007.md` Follow-ups).

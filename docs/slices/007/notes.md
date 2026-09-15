# Notes — Slice 007: the renderer grows a form

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 — the notice gets an owner | done | 2026-09-15 |
| PHASE-02 — the window draws a form | done | 2026-09-15 |
| PHASE-03 — the mapper and the draft | done | 2026-09-15 |
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
  (`controller.rs:591-600` — the attribute moved when the repair below rewrote the doc-comment above it), the narrowest of the three instruments and the only
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

### PHASE-02 — the window draws a form

**Objective:** the markup declares fields, a headless test can drive a checkbox
and address it **by option**, and the two Slint assumptions the design settled on
paper are pinned as regressions (`plan.md:306-308`).

**Criterion ids, re-derived from `plan.md` — T-0 below, not copied from any
brief.** `plan.md:304-442`, PHASE-02 entire:

| kind | ids | count |
|---|---|---|
| entry | EN-1 | 1 |
| exit | EX-1 … EX-8 | 8 |
| verification (test) | VT-1 … VT-5 | 5 |
| verification (agent) | VA-1, VA-2 | 2 |

Contiguous from 1 in each kind, no gap and no duplicate. PHASE-02 makes no
`PHASE-0N/<id>` cross-reference outward; four references point *in* — S-2 cites
`PHASE-02/EX-7`, and Coverage cites `PHASE-02/VT-4`, `EX-4`, `VT-5` and `VA-1`.
All five resolve. The one count PHASE-02's prose asserts is EX-7's **two** prose
homes of the uniqueness claim, and the list under it has two — but PHASE-01's
lesson is that an enumerated list is a floor, so T-9 sweeps rather than trusts
it. The re-derivation is the rule at `plan.md:36-45`; `review-plan.md` F-7 is
why it exists.

**Reading list**

- `docs/slices/007/plan.md:304-442` — PHASE-02 whole, including *Notes for the
  implementer*; `:104-152` — S-1..S-9; `:36-45` — the re-derivation rule;
  `:70-79` — why 02 is second (the outside-dependency risk, R-7, A-1, A-2).
- `docs/slices/007/design.md:409-468` (§5.2, *The window* — the three structs,
  the callback, the scoped query, the groupbox role and the deliberately open
  question), `:631-645` (§5.2, *The build* — the explicit `var` read and why),
  `:184-192` (why A-1 is settled and not phase work), `:796-816` (§5.4, the
  model-reset chain that makes A-2 hold and the `set_row_data` trap),
  `:937-952` (§5.5, A-1/A-2/A-3 as discharged assumptions),
  `:1050`, `:1056` (§8 — R-1's named-but-unneeded fallback, R-7's signal).
- `docs/slices/007/slice-007.md:46-78` — §Scope; the audit diffs actual paths
  against it, and it names `build.rs` and `ui/app.slint` explicitly.
- The pinned sources, read **unjailed** (the jail does not reach
  `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/`):
  `i-slint-core-1.17.1/model/repeater.rs:143-154` (`update_all_instances`
  creates every instance) and `:156-158` (the doc scoping the virtualisation
  algorithm to a `ListView` viewport) — VT-4's warrant;
  `i-slint-backend-testing-1.17.1/search_api.rs:232-287` (the six builders, none
  of them an accessible-description matcher), `:298-307` (`find_all` passes
  `ControlFlow::Continue` and returns in walk order), `:178-213` (a
  non-matching `MatchSingleElement` yields `Continue`, which is why EX-7's type
  filter does not break EX-6's scoped query), `:701` (`accessible_description`),
  `:721` (`accessible_checked`), `:606` (`invoke_accessible_default_action`);
  `i-slint-compiler-1.17.1/lib.rs:264` (`with_style` overwrites what
  `CompilerConfiguration::new()` read from the environment) and
  `passes/lower_accessibility.rs:41-59` (an accessibility property needs a bound
  role).
- The code under change: `crates/goad/build.rs` (whole, 11 lines),
  `crates/goad/ui/app.slint:1-57` (the prompt branch) and `:85-110` (untouched,
  read so the diff is understood), `crates/goad/src/glass.rs`'s `option_rows`
  — **`plan.md` EX-5 cites `:138-149` and PHASE-01 moved it to `:140-151`; found
  by name** — and `crates/goad/tests/renderer/tree.rs` whole (196 lines):
  `:28-38` the rows builder, `:40-49` `element_described` and its prose home,
  `:66-68` the virtualisation warning, `:81-83` the `match_inherits` shape to
  copy, `:95-100` and `:124-130` the two cases EX-7 exists to keep green.
- `docs/memory/a-repair-sweep-misses-the-binding-site.md`,
  `docs/memory/cite-requirements-not-finding-ids.md` (a comment in `src/`, `ui/`
  or `tests/` cites SPEC-001, a spec section or an ADR — never `F-N`/`D-N`),
  `docs/memory/a-green-test-can-assert-a-proxy.md`.

**Assumptions & STOP conditions**

Assumed, and checked rather than taken on faith where the check was cheap:

- A-a — **A-1 and A-2 are settled, not open** (`design.md` §5.4, §5.5, §8/R-1).
  Neither is this phase's to discover; neither may be carried forward as a
  question. A red pin is a finding about a changed dependency and S-1 is the
  road — it is not a licence to reach for the two-flat-models fallback.
- A-b — `EN-1`'s entry is literal and was measured, not assumed: `just check`
  exit 0 at `fd4b162` on a tree clean but for this slice's own untracked brief.
  506 tests across 21 binaries, matching PHASE-01's record.
- A-c — the pinned registry sources named above are present at
  `index.crates.io-1949cf8c6b5b557f`, so every citation in the design that this
  phase depends on can be re-read rather than believed. Checked first.
- A-d — `OptionRow` is built with an exhaustive struct literal at exactly two
  sites (`glass.rs`'s `option_rows`, `tree.rs`'s `rows`), so EX-2's fourth
  member is `E0063` at both and the compiler drives EX-5. Re-measured before
  starting.

STOP, per `plan.md:104-152`. Live for this phase:

- S-1 — an unresolved **design** issue. Back to design; never repaired here.
  **`accessible-role: list` + `accessible-item-count` on the field container is
  the live instance**: left open deliberately (`design.md` §5.2,
  `plan.md:436-441`), and not to be settled by what a test happens to need. If a
  test wants it, that is S-1, not a decision.
- S-2 — an existing test's assertion or fixture changing. This phase has exactly
  two allowances, both named in `plan.md:110-121`: `tree.rs`'s mechanical
  `blocks:` member in the row builder (a fixture *shape*, forced by codegen), and
  EX-7's change to `element_described`'s **implementation**, which is a helper
  and neither an assertion nor a fixture. VA-1 is the check that nothing else
  moved.
- S-3 — a new dependency, or a feature added to an existing one. None expected:
  `slint`, `i-slint-backend-testing` and `slint-build` are all already in.
- S-4 — anything under `crates/goad-semantics/`. Nothing here reaches it.
- S-5 — a drawn kind beyond `boolean`. This phase draws `CheckBox` only.
- S-7 — a field test that neither reads an invocation log nor asserts something
  about the screen. Every VT here asserts about the screen or a fired callback.
- S-8 — the look. Layout, spacing and typography beyond what a container and a
  heading force are 008's, and PHASE-06's when it comes.
- S-9 — `Glass::present` acquiring a write-only-on-change path. This phase does
  not touch `present`; VT-2 *depends* on the totality it already has.

**Tasks**

<!-- [ ] todo · [~] in progress · [x] done · [!] blocked -->
- [x] T-0 — re-derive the criterion ids from `plan.md`. Table above.
- [x] T-1 — EN-1: `just check` on the unmodified tree at `fd4b162`. Exit 0, 506
      tests across 21 binaries, matching PHASE-01's record. Transcript kept.
      PHASE-01's exit criteria spot-checked in the code rather than read off its
      sheet: `Notice` at `wire.rs:180-215` (EX-1), `Frame.notice` at
      `controller.rs:105` and `frame(notice)` at `:252` (EX-5), `set_notice`
      called from exactly one site, `glass.rs:109` (EX-6), `wire.rs` naming no
      Slint or generated type (EX-4).
- [x] T-2 — **VA-2, first and alone.** EX-1's `build.rs` change, no markup, no
      test. R-7's signal is that run going red *immediately after this change
      and before any other*, and it is worthless once two changes share a diff.
      **The signal fired: `cargo test -p goad` exit 101, two failures, both in
      `wiring::busy`.** F-1 carries the mechanism and F-2 the STOP it raises.
      The diff at the moment of the run was `build.rs` and `notes.md` alone
      (`git diff --stat`), so the attribution is clean. Recorded, not repaired.
- [x] T-3 — EX-2: the three structs and the callback in `app.slint`, exactly as
      `design.md` §5.2 gives them. **A-1's pin fired at compile time on the way
      past**: `blocks: [FieldBlock]` inside a struct generated as
      `ModelRc<FieldBlock>` and `OptionRow`'s fourth member was `E0063` at both
      literal sites, which is what A-d predicted.
- [x] T-4 — EX-5: `blocks: ModelRc::default()` at `glass.rs`'s `option_rows`
      (found by name at `:140-151`, not at the plan's `:138-149`) and at
      `tree.rs`'s `rows`. Mechanical, and nothing else — see D-1. Prose around
      both read: `option_rows`'s doc-comment claims the row carries the option's
      id, label and view token and makes no claim about fields, so nothing was
      falsified.
- [x] T-5 — EX-3 + EX-4: the guarded per-option container, the `for` over
      blocks, the per-block heading, the `CheckBox` and `edited` (D-4).
- [x] T-6 — EX-7: `element_described` gains `match_inherits("Button")`, and both
      prose homes move with it. **The criterion's stated warrant does not hold —
      F-3** — and the change is still right for the reason the criterion gives
      first.
- [x] T-7 — EX-6: `field_described`, with EX-6's query shape kept verbatim, over
      a shared `within_option` scope (D-3).
- [x] T-8 — VT-1 … VT-5. VT-1 written red before the markup and recorded red for
      the right reason ("no control described \"stretched\" under \"opt-a\"").
      The other four arrived green, so each was checked with a negative control
      rather than believed — D-5, and one of them was vacuous and was rewritten.
- [x] T-9 — the sweep. `grep -rn "unambiguous\|accessible.description\|by
      label\|selects on\|select on"` over `crates/goad/{src,ui,tests}` and
      `tests/support`: 13 lines, and each read rather than counted.
      **Two are live homes of the claim EX-7 changes** — `tree.rs:40-41` and
      `app.slint:49-51`, the two the criterion names, both repaired.
      `tree.rs:141` ("selection is by `accessible_description` and never by
      label") is about label-versus-description and is still true.
      `wiring.rs:62-70` is a **code** home of the same assumption and is F-4 —
      not prose, not this phase's surface, and not yet broken.
      The remaining eight are the property declarations themselves and one
      error-message string. VT-4's second half — `tree.rs:68`'s virtualisation
      warning — is corrected in place, and the correction says which markup the
      old claim was true of and what actually bounds an exhaustive query here.
- [x] T-10 — refactor. `within_option` factored out of the two field queries
      (D-3); `EditedArgs` named rather than repeated; VT-2 rewritten after its
      own negative control passed (D-5).
- [x] T-11 — VA-1. `git diff crates/goad/tests/renderer/tree.rs`, read for
      changed assertions. **Five removed lines in the whole file**: two `use`
      lines (widened), and three of prose — EX-7's home and VT-4's warning.
      **No assertion, no expected value and no fixture value was removed or
      changed.** The one addition inside an existing case is EX-5's `blocks:`
      member in `rows`. AC-6 holds.
- [x] T-12 — F-2's STOP referred up and **decided: A**, the test-side fix as a
      named fourth S-2 allowance (`plan-log.md` 2026-09-15, `plan.md:106-122`).
      Applied under D-7.
- [x] T-13 — EX-8: **`just check` exit 0. 511 tests across 21 binaries**, from
      506. Clippy `-D warnings` clean, `cargo fmt --all --check` clean.

**Decisions taken during execution**

- **D-1 — `rows` keeps its own `OptionRow` literal rather than sharing a builder
  with `one_option_with`.** The duplication is one struct literal and it is
  visible. It stays because S-2's allowance in `tree.rs` is exactly one
  mechanical `blocks:` member in that builder (`plan.md:110-121`); rewriting the
  builder to delegate produces identical rows but is a larger edit than the
  criterion names, in the file VA-1 reads for exactly that. Recorded here rather
  than taken, and a candidate for the audit's refactor pass.
- **D-2 — `build.rs` carries `#[expect(clippy::disallowed_methods, reason = …)]`.
  Referred up and confirmed.** EX-1 mandates `std::env::var`, and
  `clippy.toml` disallows it — "use typed configuration loading instead" — with
  the gate at `-D warnings`. Neither `design.md` §5.2's *The build* nor EX-1
  anticipated it. The reason is about **run-time** configuration reached through
  the environment; this runs at build time, in cargo's own environment, and
  reads the one variable `slint-build` already reads and already declares
  `cargo:rerun-if-env-changed` for (`slint-build-1.17.1/lib.rs:532`). The
  alternatives are worse in both directions: an exception in `clippy.toml`
  weakens the rule for the whole workspace, and `std::env::var_os` passes the
  lint while evading it. The **class** is settled — PHASE-01's D-4 approved a
  narrow `#[expect]` over a config change and named five prior sites; this is
  the seventh. The instance was new, which is why it was referred; the approval
  re-read the load-bearing half rather than taking it — `slint-build-1.17.1/lib.rs:532`
  is `println!("cargo:rerun-if-env-changed=SLINT_STYLE")`, the first of seven —
  so the crate does declare the dependency the read relies on and the reason
  stands as written.
- **D-3 — EX-6's query shape is kept verbatim, over a shared scope.**
  `field_described` is `match_predicate(option)` → `match_descendants()` →
  `match_predicate(field)` → `find_first()`, exactly as the criterion gives it.
  VT-4 needs the same scope with a different terminal (`match_inherits("CheckBox")`
  → `find_all()`), so the first two steps live in `within_option` and both
  queries read from there. One scope, two questions, no second copy of the
  option predicate.
- **D-4 — the per-block `VerticalLayout` is forced, not chosen.** A Slint `for`
  body is a single element, and a block is a heading plus N fields, so the `for`
  over `blocks` must produce a container per block. That is not the look decision
  S-8 reserves: EX-4's warning is about the per-**option** container, which is
  guarded and sits outside this `for`, and it holds.
- **D-5 — every verification criterion is checked with a negative control, not
  just run.** Four of the five arrived green because the markup was already in
  when they were written, and a test that has never been red has not been shown
  to be about anything. Each control was reverted immediately:

  | criterion | control | result |
  |---|---|---|
  | VT-1 | `checked: field.checked` removed from the markup | red — `Some(false)` where `Some(true)` was declared |
  | VT-2 | (a) same binding removed; (b) the model reset skipped | red on both |
  | VT-3 | the `toggled =>` handler emptied | red — nothing captured |
  | VT-4 | two entries transposed in the expected order | red |
  | VT-5 | `if option.blocks.length > 0` replaced by `if true` | red — two containers where one was expected |

  **VT-2's first form passed its own control** and was rewritten. It started the
  field unchecked, clicked it to `true`, reset the model to `false` and asserted
  `false` — which a `CheckBox` with **no** `checked:` binding at all also
  answers, because a rebuilt widget defaults to unchecked. It now starts the
  field *checked*, so every assertion reads a value the widget's own default
  cannot supply (`docs/memory/a-green-test-can-assert-a-proxy.md`; the same shape
  as PHASE-01's D-3, found by running the control rather than by reasoning).
- **D-7 — the two `wiring::busy` cases size their window, under S-2's fourth
  allowance.** F-2's STOP, decided by the user as option A: the style stays
  `material`, and the fixture gains a viewport. One helper, `with_room_for_every_control`,
  shared by both cases and carrying the reason once — that a shown window clips,
  that this states what the test can see and not what the window should be, and
  that the 18px margin the cases used to rest on was a proxy. No assertion
  changed; re-checked with a control (`enabled: false` on the option's button
  turns both red, so they still test what they are named for).

  Bookkeeping, and the same class as PHASE-01's F-1: **PHASE-02's Surfaces line
  does not list `crates/goad/tests/renderer/wiring.rs`**, which this allowance
  sends the phase to edit. No surface breach — the audit reads paths against
  `slice-007.md` §Scope, which names `crates/goad/tests/renderer/` — and nothing
  for the path diff to chase.
- **D-6 — the container declares `groupbox` and nothing more.** Whether it should
  also declare `accessible-role: list` with `accessible-item-count` is left open
  deliberately (`design.md` §5.2). VT-4 reads order off the tree walk rather than
  off an index precisely so that nothing here settles it by what a test needed.
  Not settled, not raised: no test wanted it.

**Findings**

- **F-1 — R-7 fired, and the mechanism is geometry, not the accessible
  surface.** `design.md` §8/R-7 mitigated the style change by verifying that
  `material`'s `Button` and `CheckBox` carry the same accessible properties as
  `fluent`'s. That verification is **correct** — re-read at
  `i-slint-compiler-1.17.1/widgets/material/button.slint:90-95` and
  `checkbox.slint:19-24` against `fluent/button.slint:29-34` and
  `checkbox.slint:20-25`, `accessible-enabled` included — and it is not what
  broke. What broke is that **`ElementQuery` does not see an element clipped out
  of view**: `visit_descendants_impl` skips any item where `ItemRc::is_visible()`
  is false (`i-slint-backend-testing-1.17.1/search_api.rs:373-375`,
  `i-slint-core-1.17.1/item_tree.rs:399-408`), which is a geometric test against
  the nearest clipping ancestor — here the `ScrollView`'s `Flickable`.

  Measured, both styles, in `wiring::busy::busy_clears_and_controls_re_enable_after_a_success`:

  | | window | Flickable (the clip) | Button height | second button | found |
  |---|---|---|---|---|---|
  | fluent | 54 × 65 | 54 × 50 at y=15, so y 15–65 | 32 | y 47–79 | yes, by 18px |
  | material | 50 × 65 | 38 × 38 at y=15, so y 15–53 | 40 | y 55–95 | **no** |

  Material loses on both terms at once: its `ScrollView` reserves the scrollbar
  *inside* the viewport (38 rather than 50) and its `Button` is 8px taller. The
  second option leaves the clip rect and the query cannot reach it, so
  `accessible_enabled_of(&window, "no")` is `None` rather than `Some(true)`.

- **F-2 — the two failing tests find their element only because the window
  happens to be 65px tall, and that is the STOP.** The blast radius is exactly
  two tests, measured over `cargo test --workspace`:
  `wiring::busy::busy_clears_and_controls_re_enable_after_a_{success,failure}`.
  It is **not** every test that draws options — the clip only exists once the
  window is *shown*, and `Glass::present` is what shows it (`glass.rs:118`). An
  unshown window clips nothing: `tree.rs` never shows, and a probe there found
  **10 of 10** option buttons under material. So **PHASE-02's own tests are
  unaffected** and VT-1, VT-4 and VT-5 can be written as planned; what is
  affected is the tier that presents through the glass.

  Three things make this a consult rather than a repair I may take:

  1. **S-2.** Fixing it means changing an existing test's fixture, and this
     phase's two allowances (`plan.md:110-121`) are the `tree.rs` `blocks:`
     member and EX-7's helper. This is neither.
  2. **S-1 / S-8.** The window is 65px tall because nothing declares a size and
     the `ScrollView` demands almost none, so the shown window takes its
     preferred size. Whether the answer is a window size in the markup is the
     *look*, which is 008's and PHASE-06's under AC-10.
  3. **`design.md` §8/R-7 names this trigger and does not say what follows it** —
     "AC-7 and AC-10 are the real observations, and 008 owns the outcome either
     way."

  What is established and cheap, so the question is decidable: **a test may set
  the size** — `slint::ComponentHandle::window(&window).set_size(PhysicalSize::new(400, 400))`
  before the first `present` — the headless backend honours it, and the failing
  test goes green with that one line and no assertion touched. Verified, then
  reverted.

  **Resolved.** Referred up and decided by the user as option A — the test-side
  fix, `material` standing (`plan-log.md` 2026-09-15). `plan.md` S-2 now names a
  fourth allowance; D-7 is how it was taken. **That closes the test question
  only. F-6 is the half it does not close, and is deliberately a separate
  finding.**

- **F-3 — EX-7's warrant is false as written, and the change it asks for is
  still right.** The criterion says the type filter "is what keeps
  `tree.rs:95-100` and `:124-130` green". Measured, it is not, twice over:

  1. Both cases use options built by `rows`, whose `blocks` is empty, and
     **EX-4's own guard** means an option with no fields produces no container.
     There is no second element answering to `option.id` in either case.
  2. Even where an option *does* carry fields, the walk reaches the `Button`
     before the container, because the markup declares it first. Run as a
     control: a case asserting `element_described` returns the control for an
     option with fields **passes with `match_inherits("Button")` removed**. That
     case was written, seen to be vacuous, and deleted rather than kept green
     (`docs/memory/tests-asserting-proxies.md`).

  So after EX-3 the filter changes no observable behaviour anywhere today, and
  **no test can pin it**. It stays because of the reason the criterion gives
  first and does not rest on: `find_first()` would otherwise take whichever
  element the walk reached first, which is declaration order and which no
  criterion pins. The filter makes the helper's contract independent of the
  order the markup happens to be written in. That is worth having and is not
  worth a test that would pass either way.

- **F-4 — `wiring.rs`'s `accessible_enabled_of` is the same unscoped query, in
  code rather than in prose, and PHASE-04 is where it breaks.**
  `wiring.rs:62-70` selects an option's control by `accessible_description`
  alone, with no type filter — `element_described` before EX-7. It is correct
  today because no option in that file carries blocks. The moment one does, the
  option's field container answers to the same description, and the helper will
  return whichever the walk reaches first and read `accessible_enabled` off it;
  a `groupbox` declares none, so the answer is `None` — the same shape as the
  two failures in F-1, from a different cause. Not repaired here:
  `crates/goad/tests/renderer/wiring.rs` is not a PHASE-02 surface. **This is
  the home EX-7's enumeration does not reach**, and it contains no occurrence of
  "unambiguous" or of the repaired wording — the class
  `docs/memory/a-repair-sweep-misses-the-binding-site.md` names, arriving for
  the third time in this slice.

- **F-5 — EX-1's mandated form does not pass the workspace lint, and neither the
  design nor the plan noticed.** `clippy.toml` disallows `std::env::var`, the
  gate is `-D warnings`, and `design.md` §5.2's *The build* gives the code block
  verbatim with that call in it. The design's reasoning for the explicit read is
  sound and unaffected; what was missed is that the workspace already had a rule
  about it. **Closed as an instance** — D-2, referred up and confirmed. The
  **class** goes to the audit, and it is design drift rather than a code defect:
  a design that quotes a code block has asserted it compiles **and** lints, and
  nothing checks the second half until a phase runs. `design.md` is a record of
  intent and is not retro-fitted for it (`docs/AGENTS.md` §Audit & reconcile,
  *Design drift not reconciled*).

- **F-6 — under `material`, a shown window at its preferred size clips its own
  second option. This is a finding about the product, and no test here reports
  it.** It is the observation that made F-2's decision defensible rather than
  convenient, and it must not be read as fixed.

  **The two `wiring::busy` cases are green because the fixture now declares a
  viewport, not because the clipping stopped.** `with_room_for_every_control`
  sets 400×400 before the first present; nothing in `app.slint`, `glass.rs` or
  `build.rs` changed to make the second option reachable. Run the product at its
  own preferred size and the second option is still outside the `Flickable`'s
  rect — invisible to a query, and to a person, below the fold of a window
  nothing has sized.

  **The measurement, so PHASE-06 does not re-derive it.** Both styles, same
  case, window at its preferred size (50–54 × 65), clip = the options
  `ScrollView`'s `Flickable`:

  | | Flickable (the clip) | Button height | 1st button | 2nd button | reachable |
  |---|---|---|---|---|---|
  | `fluent` | y 15–65 | 32 | y 15–47 | y 47–79 | yes, by 18px of overlap |
  | `material` | y **15–53** | 40 | y 15–55 | y 55–95 | **no** |

  Material loses on both terms at once: its `ScrollView` reserves the scrollbar
  *inside* the viewport (38 rather than 50) and its `Button` is 8px taller.
  Note the fluent row as well — 18px of overlap on a layout nothing declares is
  not a margin anyone chose, so this is not a `material` defect so much as a
  window that has never been sized meeting a taller control.

  **Whose it is.** AC-7 (a person can read and answer the prompt) and AC-10 (the
  look, bounded) are the criteria that observe it, and both are **PHASE-06/VH-1
  and VH-2** — a person running the software, which `docs/AGENTS.md` requires
  before the slice closes and which a green gate is explicitly not. PHASE-06
  inherits this finding; slice 008 owns the repair if the answer is a declared
  window size, because that is AC-10's bound and S-8's.

  The fact has one home in the code — `tree.rs`'s module comment, which says why
  an exhaustive query is sound *there* and names the clipping that bounds it
  elsewhere — and one in the fixture, `with_room_for_every_control`'s doc, which
  says in as many words that sizing the viewport states what the test can see
  and not what the window should be.

### PHASE-03 — the mapper and the draft

**Objective:** a canonical view becomes blocks of drawn fields with everything
else reported, and there is one pure place a widget's state becomes a submitted
value (`plan.md:459-461`).

**Criterion ids, re-derived from `plan.md` — T-0 below, not copied from any
brief.** `plan.md:457-578`, PHASE-03 entire:

| kind | ids | count |
|---|---|---|
| entry | EN-1 | 1 |
| exit | EX-1 … EX-8 | 8 |
| verification (test) | VT-1 … VT-6 | 6 |
| verification (agent) | VA-1, VA-2 | 2 |

Contiguous from 1 in each kind, no gap and no duplicate. PHASE-03 makes exactly
one `PHASE-0N/<id>` cross-reference **outward** — EX-2 cites `PHASE-04/EX-3`,
which exists (`plan.md`'s PHASE-04 runs EX-1 … EX-10). Four point **in**, all
from Coverage: `PHASE-03/VT-2`, `VT-3`, `VT-4`, `VT-6`, and all four exist.
`review-plan.md` cites `PHASE-03/VA-2`, `EX-5`, `EX-6`, `EX-2`, `EX-3` — all
exist. The counts PHASE-03's own prose asserts: EX-1's "two methods" (`state_of`,
`record` — two), EX-3's "the four kinds this renderer does not draw" (`Text`,
`DateTime`, `Number`, `Choice` — four), EX-5's "the two lines" (two), VT-4's
four kinds (four). Each matches the list beneath it. The re-derivation is the
rule at `plan.md:36-45`; `review-plan.md` F-7 is why it exists.

**Reading list**

- `docs/slices/007/plan.md:457-578` — PHASE-03 whole, including *Notes for the
  implementer*; `:104-152` — S-1..S-9; `:36-45` — the re-derivation rule;
  `:80-84` — why 03 is third and independent of 02; `:160-170` — Coverage, for
  what AC-2, AC-3 and AC-8 hang on this phase.
- `docs/slices/007/design.md:469-557` (§5.2, *The mapper* and *The draft* — the
  types, `submitted`, and the three deliberate absences), `:606-631` (§5.2, the
  two diagnostic lines verbatim), `:654-759` (§5.3 — the ownership rule I-4 and
  what is derived), `:901-935` (§5.5, I-1 … I-6), `:961-980` (§5.5's edge-case
  table, which is the fixture list for VT-3 and VT-4), `:331-362` (§5.1 — why
  `answer()` walks the presentation and never the draft, which is what the
  absence of enumeration on `Draft` buys).
- `docs/slices/007/canon-delta.md` — R-57 (the rule `submitted` applies) and
  R-58 (PHASE-04's, and the reason `Draft` cannot be enumerated). The R-18
  amendment is **context, not an action**: `docs/specs/` is untouched here.
- `docs/slices/007/slice-007.md:46-78` — §Scope, which the audit diffs actual
  paths against; AC-2, AC-3, AC-8.
- The code under change: `crates/goad/src/view_model.rs` whole (165 lines) —
  `:29-41` `body_is_degraded` and the doc-comment naming `OptionFields` that no
  compiler reaches, `:66-101` `Undrawn` and `ContentForm` with its `Display`,
  `:138-155` the option map and the push site; `diagnostics.rs:190-206`
  (`undrawn_line`, the arm and its two neighbours, the shape to copy);
  `lib.rs` (12 lines); `crates/goad/tests/renderer/mapper.rs:151-196` (VA-2's
  one case) and `:1-50` (the fixture helpers); `tests/renderer/reception.rs:1-70`
  (the same helpers again, and the module's own statement of what it tests),
  `:660-681` (the `receive`-reaches-the-surface precedent VT-6 copies).
- The types the mapper reads, none of which change (S-4):
  `crates/goad-semantics/src/protocol/canonical.rs:56-95` (`OptionId` has no
  `Ord`, `FieldId` has and says why), `:126-137` (`Hints::as_map`),
  `:188-207` (`Opt::fields`), `:219-256` (`Field` and `FieldKind`'s five
  variants), `:380-401` (`Fields::as_slice`).
- `Cargo.toml:83-108` — `dead_code` is `warn` **by decision**, with
  `expect(dead_code, reason = …)` named as the mechanism for a type that lands
  one phase before its caller. That is D-2 below.
- `crates/goad-boundary/tests/checks/structure.rs:25`, `:306-314` — the
  `resolve` instrument and its subject dir; `checks/vocabulary.rs:16-34` — the
  seven domain words, matched case-insensitively and by word over `.rs` and
  `.slint`, comments and string literals included.
- `docs/memory/a-repair-sweep-misses-the-binding-site.md`,
  `docs/memory/a-green-test-can-assert-a-proxy.md`,
  `docs/memory/cite-requirements-not-finding-ids.md`.

**Assumptions & STOP conditions**

Assumed, and checked rather than taken on faith where the check was cheap:

- A-a — **EN-1 is literal and was measured.** `just check` exit 0 on the tree at
  `36c13ad` (clean but for this slice's own untracked briefs): **511 tests across
  21 binaries**, matching PHASE-02's record exactly. PHASE-02's exit criteria
  spot-checked in the code rather than read off its sheet — see T-1.
- A-b — **the compiler drives most of the sweep.** `Undrawn` is a `pub enum`
  matched exhaustively in `diagnostics.rs::undrawn_line` and asserted in one test,
  so removing `OptionFields` is a compile error at both. What it does **not**
  reach is prose: `view_model.rs:29` and `mapper.rs:151` name the variant in
  doc-comments. Measured before starting — six live homes in the code, and
  `docs/roadmap.md:313` outside it (F-1).
- A-c — `present()` is the only constructor of `Presentation` and `receive` the
  only path to one, so I-2 is held by the types rather than by this phase
  remembering it. Re-read at `reception.rs:60-81` before starting.
- A-d — nothing in `crates/goad/src` calls `submitted` until PHASE-04, so
  `pub(crate)` alone is a `dead_code` warning and the gate is `-D warnings`.
  Anticipated, and D-2 is the recorded mechanism.

STOP, per `plan.md:104-152`. Live for this phase:

- S-1 — an unresolved **design** issue. Back to design; never repaired here.
  **One fired: F-2**, the treatment of a `group` hint on a field that is itself
  undrawn. Referred up, work continued around it.
- S-2 — an existing test's assertion or fixture changing. This phase has exactly
  **one** allowance and it is named: `mapper.rs:156-194`'s
  `an_option_with_fields_is_reported_undrawn_by_id_and_count`, **replaced** by
  VT-4 rather than adjusted (VA-2). Anything else stops.
- S-3 — a new dependency, or a feature on an existing one. None expected:
  `serde_json` is already in, and `Value` is already named in `controller.rs`.
- S-4 — anything under `crates/goad-semantics/`. The specific temptation is
  deriving `Ord` on `OptionId` to key `Draft` by a `BTreeMap`. `Draft` is a `Vec`
  precisely so that it is not needed.
- S-5 — an `Edited` variant beyond `Checked`, or a drawn kind beyond `boolean`.
  The seam exists so the *next* slice is a `submitted` arm; it is not an
  invitation to write the second variant now. VA-1 adds one for the length of one
  `cargo check` and reverts it.
- S-6 — the draft entering `Presentation`: a `checked` on `PresentationField`, or
  any mutable member on a `view_model.rs` type (I-4). **No instrument in the gate
  reaches this**, so a green gate is not evidence against it.
- S-7 — a field test that neither reads an invocation log nor asserts something
  about the screen. Read narrowly, this phase draws nothing — every VT here is
  over a pure function, which is what `plan.md` asks for by name (VT-3 is "the
  rule, as a pure `present()` test"). The proxy risk it guards is met the other
  way, by D-3's negative controls.
- S-9 — `Glass::present` acquiring a write-only-on-change path. Not a surface
  here; nothing in this phase touches `glass.rs`.

**Tasks**

<!-- [ ] todo · [~] in progress · [x] done · [!] blocked -->
- [x] T-0 — re-derive the criterion ids from `plan.md`. Table above.
- [x] T-1 — EN-1: `just check` on the unmodified tree at `36c13ad`. **Exit 0,
      511 tests across 21 binaries**, matching PHASE-02's record. Transcript
      kept. PHASE-02's exit criteria spot-checked in the code rather than read
      off its sheet: `build.rs`'s `SLINT_STYLE`-defaulted `with_style` beside
      `with_debug_info(true)`, under the `#[expect]` D-2 recorded (EX-1); the
      three structs and `callback edited(string, string, string, bool)` at
      `app.slint:7-10`, `:31` (EX-2); the guarded per-option `groupbox` at
      `:72-79` with the `CheckBox` and its `toggled` at `:87-101` (EX-3, EX-4);
      `blocks: ModelRc::default()` at `glass.rs:149` and `tree.rs:49` (EX-5);
      `within_option`/`field_described` and `match_inherits("Button")` in
      `tree.rs` (EX-6, EX-7).
- [x] T-2 — EX-1, EX-7, VT-1, VT-2: `draft.rs`, **red first**. The two accessors
      and `submitted` stubbed to answer a constant, the four tests run: three
      red for the right reason (`Checked(false)` where `Checked(true)` was
      recorded; `Null` where `Bool(true)` was expected), the fourth — VT-1's
      as-drawn clause — green under the stub, because the stub *is* the
      as-drawn answer. That one is controlled the other way in D-3.
      `record` is a `retain`-then-`push` (D-4). `lib.rs` gains `pub mod draft;`
      and its own module-count comment is corrected with it (T-11).
      **`submitted` needed D-2** to pass `-D warnings`.
- [x] T-3 — EX-3: `FieldBlock`, `PresentationField`, `PresentationOption.blocks`,
      `Undrawn::{FieldForm, GroupHint}` and `FieldForm` with its `Display`.
      `Undrawn::OptionFields` removed; the compiler found `diagnostics.rs` and
      `mapper.rs`, and `body_is_degraded`'s doc-comment — which it does not
      reach — was moved by hand with them.
- [x] T-4 — EX-4, EX-6: `present()`'s field walk, **red first** — the types
      landed with `blocks: Vec::new()` and no field report, and seven of the
      nine new mapper cases went red before the walk was written. `group` is
      read at `view_model.rs:182` and **nowhere else** in any crate's `src`
      (grepped, EX-6).
- [x] T-5 — EX-5: `diagnostics.rs`'s arm replaced by the two lines `design.md`
      §5.2 states verbatim. Taken early: it is what the `OptionFields` removal
      breaks, so the compiler asked for it during T-3.
- [x] T-6 — VT-6: `reception.rs` gains 13m. Asserts one line per field and that
      each names its field; **not** the wording, which is review's (EX-5).
- [x] T-7 — VA-1. `Edited::Counted(u8)` added with no `submitted` arm:
      `error[E0004]: non-exhaustive patterns: \`&Edited::Counted(_)\` not
      covered`, at `draft.rs`'s `match edited` and pointing at the enum.
      Reverted; `cargo check` clean again. **That compile failure is the test**
      — a total match asserted only by reading the code is not asserted.
- [x] T-8 — VA-2. Every one of `mapper.rs`'s ten existing cases compared
      against its pre-phase text: **nine verbatim**, and the tenth is
      `an_option_with_fields_is_reported_undrawn_by_id_and_count`, the one the
      criterion names — removed whole and replaced by VT-4, not adjusted. The
      only other removed line in the file is the widened `use`. `reception.rs`'s
      diff removes **nothing**. S-2 holds.
- [x] T-9 — negative controls, D-3. Four tests arrived green; each was
      controlled rather than believed, and all four went red.
- [x] T-10 — refactor, D-4. One 45-line `fields_of` became `sift` +
      `blocks_from` over a `Drawn` — which is what makes "a run is over the
      drawn fields" structural rather than remembered. The hint-reading enum
      was renamed `Grouping` → `Run` under D-5.
- [x] T-11 — the prose sweep. Homes of the claim this phase falsifies, found by
      grepping `crates/goad/{src,ui,tests}` for field/draws/hint prose rather
      than by trusting the criterion's list: `view_model.rs:29`'s
      `body_is_degraded` exclusion (named by EX-3, moved), `mapper.rs:151`'s
      doc-comment (removed with its test), and **`lib.rs:1-3`, which no
      criterion names** — its "nine after 005" module count went stale the
      moment `draft` landed, and the compiler reaches none of it. Corrected in
      place. `glass.rs`'s `option_rows` doc and `controller.rs`'s `answer` were
      read and make no claim this phase falsifies. One home is **outside every
      phase's surfaces** and is left alone: F-1.
- [x] T-12 — EX-8: **`just check` exit 0. 524 tests across 21 binaries**, from
      511. Clippy `-D warnings` clean, `cargo fmt --all --check` clean, the
      boundary checks (43) green including the `resolve` instrument and the
      domain-vocabulary scan.
- [x] T-13 — **F-2's decision, pinned.** The intersection was left unasserted
      while the answer was open, which was right then and is a gap now: a
      decided behaviour with no test is the shape this slice keeps finding.
      `a_field_that_is_both_undrawn_and_badly_grouped_is_reported_twice` asserts
      both reports over one `text` field carrying `"group": 7`, each naming the
      option and the field and only the kind report naming the kind.
      **Controlled by implementing the rejected reading**, which is the sharpest
      control available here: with the hint read only for drawn fields, this
      case goes red and **every other case stays green** — so the test is about
      the decision and nothing else. Reverted. Gate re-run: exit 0, **525 tests
      across 21 binaries**.

**Decisions taken during execution**

- **D-1 — an absent `group` and `"group": ""` are two runs, not one.** Both draw
  no heading, so on today's markup they are indistinguishable; a separator or a
  block margin (06's) would make them visible. The rule EX-4 states is "a
  heading wherever the `group` value changes", and absent is not the value `""`,
  so the run changes. The alternative — merging every headingless field into one
  run — would merge two values the backend distinguished, which is the merging
  EX-4 forbids in the same sentence. Pinned by
  `an_absent_group_and_an_empty_one_are_blocks_with_no_heading`, so 06 finds it
  rather than discovering it in a screenshot. **What is *not* split**: a `group`
  that is not a string joins the *same* run as an absent one, because a hint
  that could not be read is not a different group, it is no group
  (`Grouping::run`'s own doc says so).
- **D-2 — `submitted` carries `#[cfg_attr(not(test), expect(dead_code, …))]`.**
  Nothing in `crates/goad/src` calls it until PHASE-04/EX-3, and `pub(crate)`
  plus `-D warnings` is a failed gate. **The class is already settled on the
  page**: `Cargo.toml:83-90` holds `dead_code` at `warn` precisely because "a
  phased plan lands a type one phase before its caller", and names
  `expect(dead_code, reason = …)` as the mechanism "for the cases worth
  *recording*". Not referred up, because the config decided it — what was
  measured rather than assumed is the `cfg_attr`: a bare `#[expect]` is
  **unfulfilled** in the `cfg(test)` build, where the unit tests below are
  already a caller, and `unfulfilled_lint_expectations` then fails the gate from
  the other side. Scoping it to `not(test)` satisfies both, and it self-clears
  the moment `controller.rs` calls it.
- **D-3 — every test that arrived green was controlled, not believed**
  (PHASE-02's D-5, and `docs/memory/a-green-test-can-assert-a-proxy.md`). Nine
  of thirteen were written red first. The other four:

  | criterion | control | result |
  |---|---|---|
  | VT-1, as-drawn clause | `state_of`'s absent answer flipped to `Checked(true)` | red |
  | VT-5 | `sift` opens one empty block per option | red — a bare option drew a container |
  | `FieldForm` `Display` | two arms transposed | red |
  | VT-6 | `receive` hands `Diagnostics::of` an empty `undrawn` | red |
  | VT-4, F-2's pin (T-13) | the **rejected reading** implemented — the hint read only for drawn fields | red, and alone |

  VT-1's as-drawn clause is the one worth naming: it **passed under the stub**
  that returns a constant, because the constant it returns is the claim. Its
  control has to move the default rather than remove the lookup, and no control
  that breaks the *lookup* can reach it.
- **D-5 — the private enum was called `Grouping`, and `design.md` §8/R-6 names
  that exact type as the thing not to write.** R-6's signal column reads *"Any
  host type or module named for grouping rather than for layout"* and its risk
  column names *"a `Grouping` type, a `grouping.rs`"* in as many words. The
  first implementation wrote one — privately, over the hint rather than the
  screen, and arguably inside EX-6's meaning, since EX-3 mandates a public
  `Undrawn::GroupHint` and the criterion cannot mean the token is banned. It was
  renamed anyway: leaving a type that the risk register names verbatim costs a
  reviewer at audit the whole argument above to get back to the same place. It
  is now `Run`, which is EX-4's own word — *"runs over the drawn fields"* — and
  is layout. Its accessor is `key`, and `Drawn.key` is what `blocks_from`
  compares, so "a run is a maximal sequence of equal keys" is one sentence with
  one vocabulary. **Found by re-reading R-6 at the end of the phase, not by any
  instrument**: the vocabulary scan checks seven words and `group` is not one of
  them, which R-6's own risk column says.
- **D-4 — the mapper's field walk is two functions over a third type, not one
  loop.** `sift` decides drawn-or-reported; `blocks_from` cuts runs; `Drawn`
  carries one drawn field and the run it joins, and is the **only** thing
  `blocks_from` can see. That is the gain: "a run is over the drawn fields in
  declared order" stops being a rule the reader must hold and becomes a fact
  about what the second function is handed. The single-loop version was written
  first, was green, and was replaced in the refactor step.

**Findings**

- **F-1 — two documents outside the phases' Surfaces name `Undrawn::OptionFields`,
  which no longer exists.** `docs/roadmap.md:313` (*"The renderer does not draw
  them — `Undrawn::OptionFields`, `crates/goad/src/view_model.rs`"*) and
  `slice-007.md:14` (*"Every option's fields become `Undrawn::OptionFields`
  (`view_model.rs:69`)"*). **Recorded, not repaired, and it is not obvious that
  either should be**: both are the *statement of the problem 007 opens on*, in a
  roadmap entry and a §Purpose, and a statement of what was true when the work
  was scoped is not the same kind of thing as a claim about the code. Neither is
  canon, neither is a declared surface, and `slice-007.md`'s own §Summary is
  written at close. **For the audit to decide**, which is where the
  document-stale-versus-code-wrong call belongs (`docs/AGENTS.md` §Audit &
  reconcile). Flagged because the `view_model.rs:69` line reference in one of
  them is now a different line of a different type, and that is the half that
  will mislead a reader whichever way the call goes.
- **F-2 — S-1 fired, and is closed by user decision: reading (a), two reports.**
  EX-4's sentence is scoped to the drawn — *"grouping is by runs over the
  **drawn** fields … A non-string `group` is ungrouped, **drawn in place**, and
  reported `GroupHint`"* — and §5.5's edge table said the same, so neither
  settled what `{"kind":"text","group":7}` produces. **Referred up before the
  mapper was written**, with the recommendation implemented provisionally and
  the cost of the other answer held at one `if` and no test. Decided by the
  user: **both** reports. `GroupHint` names a *hint*, and the value is malformed
  whether or not anything was drawn; under the other reading a backend fixes the
  kind, re-sends, and only then learns the hint was wrong — the extra round trip
  this slice exists to remove. `design.md` §5.5 now carries it as its own edge
  row and `design-log.md` the decision (both the lead's, written there and not
  here). **Now pinned**, by
  `a_field_that_is_both_undrawn_and_badly_grouped_is_reported_twice` — T-13.

- **F-3 — a domain word inside a `reason = "…"` string fails the gate, and
  "call site" is the live trap.** The vocabulary scan cuts comments but **not
  string literals** (`vocabulary.rs:195-228` asserts both halves), and `site` is
  one of the seven domain words. `#[expect(…, reason = "…the call site
  arrives")]` is therefore a breach while the identical sentence three lines
  above it in a doc-comment is not. **Measured, not reasoned**: the phrase was
  put in deliberately and
  `no_workspace_member_names_the_users_domain` failed naming `draft.rs:94:
  forbidden token \`site\`` — then reworded. This is a live hazard for every
  future `reason =` and every test-name string, and it is invisible to a reader
  who knows only that "comments are not scanned". Candidate for
  `docs/memory/`.
- **F-4 — `lib.rs`'s module-count comment is a claim about the file it sits in,
  and nothing checks it.** *"ten at PHASE-08, nine after 005 lifted `clock`"* —
  `draft` makes it ten again, and no criterion, no compiler and no grep for
  "draft" reaches it. Found by reading the file being changed, which is the only
  instrument that reaches this class (PHASE-01's F-5, PHASE-02's T-9). Corrected
  in place; recorded because the **class** keeps recurring and this is the third
  phase in a row to hit it.

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-09-15 · PHASE-03, done · base commit `36c13ad`

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
- The markup declares fields: `FieldRow`, `FieldBlock`, `OptionRow.blocks` and
  `callback edited(view, option, field, checked)`; one **guarded** container per
  option, `groupbox`, described by `option.id` (PHASE-02 EX-2, EX-3, EX-4).
- `build.rs` selects `material` as a **default**, `SLINT_STYLE` still overriding
  (EX-1) — with an `#[expect]` the plan did not anticipate, D-2.
- `tree.rs` gains `within_option`, `field_described` (the option-scoped query,
  EX-6) and `fields_in_option`; `element_described` gains a type filter (EX-7).
  Five cases: A-1's pin, A-2's pin, `edited`'s four selectors, declared order
  across blocks, and AC-6's absent-not-empty. Gate at **511 tests, from 506**.
- `draft.rs` — the one new file: `Edited`, `Draft` (a `Vec`, no `BTreeMap`, no
  enumeration, no `PartialEq`) and `submitted`, the single application of
  `SPEC-001/R-57`. Four unit tests inside it, because `pub(crate)` is out of
  reach of `tests/`.
- `view_model.rs` grows `FieldBlock`, `PresentationField`,
  `PresentationOption.blocks`, `Undrawn::{FieldForm, GroupHint}` and `FieldForm`
  with its `Display`; `Undrawn::OptionFields` is gone. The field walk is `sift`
  + `blocks_from` over a private `Drawn`, keyed by a private `Run` (D-4, D-5).
  `group` is read here and
  nowhere else in any crate (EX-6, `SPEC-001/R-18`).
- `diagnostics.rs` carries the two lines `design.md` §5.2 states verbatim; no
  test asserts either wording, as no test in this project asserts any.
- Ten new `mapper.rs` cases and one in `reception.rs`; `mapper.rs`'s
  `an_option_with_fields_is_reported_undrawn_by_id_and_count` removed whole.
  Gate at **525 tests, from 511**.

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
- **The headless element query cannot see what is clipped out of view, and only
  a *shown* window clips.** `ElementQuery` skips any element failing
  `ItemRc::is_visible()`, a geometric test against the nearest clipping
  ancestor. `Glass::present` shows the window, which gives it its preferred size
  — 65px tall — and the options `ScrollView` a viewport that fits **one**
  material control. `tree.rs` never shows, so an exhaustive query is sound
  there; the tier that presents through the glass is reading the screen through
  a viewport. PHASE-02 F-1, F-2.
- **A style change can break a test through layout while every accessible
  property it touches is identical.** R-7's mitigation compared material's and
  fluent's accessible surfaces and was right about them. What broke was 8px of
  button height and a scrollbar drawn inside the viewport rather than beside it.
  A mitigation that checks the API a test calls has not checked the geometry the
  query walks. F-1.
- **A criterion's stated warrant is a claim, and it can be false while the
  criterion is right.** EX-7 justified its type filter by two cases it does not
  in fact keep green — EX-4's own guard means those options have no container,
  and even with one the walk reaches the control first. The filter still belongs;
  the case written to pin it was vacuous and was deleted rather than kept. F-3.
- **A test that has never been red has not been shown to be about anything.**
  Four of PHASE-02's five verification criteria arrived green because the markup
  landed first. One of them — A-2's model-reset pin — passed with the binding it
  exists to verify **deleted from the markup**, because it asserted a value the
  widget's default also supplies. Start from the value the default cannot give.
  D-5, and PHASE-01's D-3 a second time.
- **The vocabulary scan cuts comments but not string literals, and "call site"
  is the trap.** `site` is one of the seven domain words. The identical sentence
  passes in a doc-comment and fails inside a `reason = "…"`, a test name string
  or an assertion message — measured, deliberately, by writing the breach and
  watching `no_workspace_member_names_the_users_domain` name the line. A reader
  who knows only "comments are not scanned" will write this. PHASE-03 F-3.
- **`dead_code` at `warn` plus a gate at `-D warnings` still fails, and the
  `#[expect]` that fixes it must be `cfg_attr(not(test), …)`.** A type landing
  one phase before its caller is the case `Cargo.toml` names and licenses; what
  it does not say is that the unit tests below the function are *already* a
  caller, so a bare `#[expect]` is unfulfilled in the `cfg(test)` build and
  fails from the other side. Both builds are in `--all-targets`. PHASE-03 D-2.
- **Where the claim *is* the default, no control that breaks the mechanism can
  reach it.** That is the corollary of PHASE-02's D-5 — *"assert a value the
  default cannot supply"* — and it had not been stated. VT-1's as-drawn clause
  passed against a `state_of` stubbed to a constant, because the constant **is**
  the claim; no control that removes the lookup can turn it red. The control
  that reaches it moves the **default** (`Checked(false)` → `Checked(true)`),
  not the mechanism. Before reaching for a control, ask which of the two kinds
  of claim the test makes. PHASE-03 D-3.
- **The sharpest control for a decided fork is the rejected reading itself.**
  F-2's pin was controlled by implementing option B faithfully: the new case
  went red and **every other case stayed green**, which says in one run both
  that the test is about the decision and that nothing else in the suite
  depends on it. Cheaper and more exact than inventing a way to break the code.
  PHASE-03 T-13.
- **A risk register names types, and one of them got written.** `design.md`
  §8/R-6 says *"a `Grouping` type, a `grouping.rs`"* is the shape to avoid, and
  the first cut of the mapper had a private `Grouping`. Nothing caught it: the
  vocabulary scan checks seven words and `group` is not one, which R-6 itself
  says. §8 is a checklist to re-read at the **end** of a phase, not only at the
  start. PHASE-03 D-5.
- **Read the criterion's scope before deciding what it settles.** EX-4 reads as
  a complete rule for the `group` hint until you notice every clause in it is
  scoped to the *drawn* fields — which leaves the hint on an undrawn field
  unsettled, and it is not a case the edge table covers either. The tell is a
  sentence whose subject was fixed two clauses earlier. PHASE-03 F-2.
- **A count comment is a claim about the file it sits in, and this is the third
  phase in a row to falsify one.** `lib.rs`'s "nine after 005" went stale the
  moment `draft` landed; no compiler, no criterion and no grep for the new name
  reaches it. Reading the file you are changing is still the only instrument for
  this class. PHASE-03 F-4, PHASE-01 F-5, PHASE-02 T-11.

### Open

- **F-1** — `plan.md`'s PHASE-01 Surfaces line omits `ui/app.slint`, which its
  own EX-7 sends the phase to edit. A plan-internal gap; no surface breach, and
  nothing for the audit's path diff to chase.
- **F-2** — no instrument separates a call-shape sweep from the comments it
  rewrites. Candidate for `docs/memory/`.
- **`serve` is one parameter over the arity lint** and now carries an
  `#[expect]` for it (D-4). The next phase that adds a parameter inherits the
  argument, not a free pass.
- **F-1 / F-2 — R-7 fired; resolved by user decision as S-2's fourth allowance**
  (`plan-log.md` 2026-09-15). Test question closed.
- **F-6 — the product question is open, and is PHASE-06's.** Under `material` a
  shown window at its preferred size clips its own second option; the two tests
  are green because the fixture declares a viewport, not because the clipping
  stopped. AC-7 and AC-10 observe it, at PHASE-06/VH-1 and VH-2. The measurement
  is in the finding so it is not re-derived.
- **PHASE-04 inherits the same edge.** It verifies "at the window" over a form of
  several checkboxes, and a shown window's viewport today fits **one** material
  control. `wiring::busy`'s `with_room_for_every_control` is the precedent, and
  F-4 is the second thing that bites there.
- **F-4 — `wiring.rs:62-70` will break the same way in PHASE-04**, when an
  option first carries blocks: an unscoped description query, no type filter,
  reading `accessible_enabled` off whatever the walk reaches first. Not yet
  broken, not a PHASE-02 surface, and invisible to any grep for the repaired
  wording.
- **F-5 — a design that quotes a code block has asserted that block lints, and
  nothing checks that until a phase runs.** `design.md` §5.2's *The build*
  quotes `std::env::var("SLINT_STYLE")` verbatim; `clippy.toml` disallows the
  call and the gate is `-D warnings`. The design's *reasoning* for the explicit
  read is sound and survives untouched — only the claim that the block compiles
  clean was never true. **This is design drift, not a code defect:** `design.md`
  is a record of intent at a point in time and is not to be retro-fitted, and
  `audit.md` has a section for exactly this (`docs/AGENTS.md` §Audit &
  reconcile, *Design drift not reconciled*). The instance is closed —
  D-2, confirmed — and it is the **class** the audit should hold.
- **D-1 — `tree.rs` holds two `OptionRow` literals** rather than one builder,
  deliberately, to stay inside S-2's named allowance. For the audit's refactor
  pass.
- Unchanged and not this phase's: keyboard focus dropped on every present
  (`design.md` §5.4, `slice-007.md` Follow-ups).
- **PHASE-03 F-2 — closed, not open.** The `group` hint on an undrawn field is
  two reports, by user decision; `design.md` §5.5 carries the edge row,
  `design-log.md` the decision, and `mapper.rs` the pin. Listed here only so an
  auditor reading the Findings above does not go looking for it.
- **PHASE-03 F-1 — `docs/roadmap.md:313` and `slice-007.md:14` name
  `Undrawn::OptionFields`**, which is gone, and one of them cites a line number
  that now holds a different type. Both are statements of the problem 007 opened
  on rather than claims about today's code, so whether they want repairing is
  audit's call, not a phase's.
- **PHASE-03 F-3 — the vocabulary scan's string-literal reach** is a hazard for
  every future `reason = "…"`. Candidate for `docs/memory/`.
- **`Draft` is what PHASE-04 inherits, and its three absences are load-bearing.**
  No `BTreeMap` (`OptionId` has no `Ord`, and S-4 forbids giving it one), no way
  to enumerate (so `answer()` *must* walk the presentation's blocks — the walk
  is not a convention it can drop), no `PartialEq` (assert through `state_of`).
  A PHASE-04 test that wants any of the three is the test to rewrite.

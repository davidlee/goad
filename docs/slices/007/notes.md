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
| PHASE-04 — the draft is retained, and the answer carries it | done | 2026-09-15 |
| PHASE-05 — the form on the wire | done | 2026-09-15 |
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

### PHASE-04 — the draft is retained, and the answer carries it

**Objective:** an edit travels from the window to retained state, the screen is
written back from it every present, and `answer()` submits a value for every
drawn field of the answered option and none for any other (`plan.md:581-583`).

**Criterion ids, re-derived from `plan.md` — T-0 below, and re-derived a second
time by a different method, because `plan.md`'s own Overview singles this phase
out (`:42-45`).** `plan.md:579-712`, PHASE-04 entire:

| kind | ids | count |
|---|---|---|
| entry | EN-1 | 1 |
| exit | EX-1 … EX-10 | 10 |
| verification (test) | VT-1 … VT-7 | 7 |
| verification (agent) | VA-1, VA-2 | 2 |

**Pass 1** — every `^- <ID> —` bullet in the phase, in order: `EN-1`, `EX-1 …
EX-10`, `VT-1 … VT-7`, `VA-1`, `VA-2`. Contiguous from 1 in each kind, no gap
and no duplicate. **Pass 2** — the same bullets bucketed by the `**Entry**` /
`**Exit**` / `**Verification**` heading each falls under, counted independently:
entry 1, exit 10, verification 9 (7 VT + 2 VA). The two passes agree, which is
what the doubling is for.

**Cross-references.** Outward, PHASE-04 cites `PHASE-02/VT-4` and
`PHASE-03/VT-3`; both resolve (`plan.md:406`, `:521`). Inward, Coverage cites
`PHASE-04/VT-3` (AC-8), `VT-7` (AC-2), `VA-1` (AC-6), `VT-6` (AC-6's exclusion)
and `04/EX-10` (AC-9); all five exist. The counts the phase's prose asserts:
EX-5's **seventh** installation (`install.rs` has six today — counted:
`chosen`, `closing`, `quitting`, `checking`, `showing`, `stopping`); EX-9's
**fourth** case in `every_refused_variant_renders_one_line_with_the_failure_prefix`
(it has three today — counted: `SupersededView`, `UnknownOption`, `NoClock`);
S-2's **third** allowance (S-2 lists four, and PHASE-04's `reception.rs`
extension is the third of them in declaration order). All three hold as counts.
**One of them rests on a false premise — F-1 below.** The re-derivation is the
rule at `plan.md:36-45`; `review-plan.md` F-7 is why it exists.

**Reading list**

- `docs/slices/007/plan.md:579-712` — PHASE-04 whole, including *Notes for the
  implementer*; `:102-142` — S-1..S-9; `:36-45` — the re-derivation rule;
  `:85-88` — why 04 is after 02 and 03 and before 05.
- `docs/slices/007/design.md:363-654` (§5.2 — the controller, the command, the
  diagnostic lines, the draft, the retained value, and the two `FieldBlock`s),
  `:654-760` (§5.3 — state and ownership; the draft's whole lifetime is
  `absorb`'s three existing arms, and I-4's named rule), `:760-901` (§5.4 —
  lifecycle, the model-reset mechanism §5.4 rests on, and why no guard against a
  racing edit is owed), `:901-981` (§5.5 — I-4, I-5, the edge table).
- `docs/slices/007/canon-delta.md` — R-58, which VT-3 is the vehicle for.
- `docs/slices/007/slice-007.md:46-` §Scope; AC-2, AC-5, AC-6, AC-8.
- The code: `controller.rs:211-230` (`answer`, and `values` at `:227`),
  `:543-574` (`dispatch`), `:693-713` (the single refusal site),
  `reception.rs:25-28` (`Prepared`), `:50-89` (`receive`), `wire.rs:16-32`
  (`Command`), `install.rs` (one clone per callback), `glass.rs:276-288`
  (`option_rows`, and `blocks: ModelRc::default()` at `:285`), `draft.rs`
  entire, `diagnostics.rs:47-68` (`Refused`) and `:143-168`
  (`Diagnostics::refused`), `tests/renderer/wiring.rs:62-70`
  (`accessible_enabled_of` — F-4 of PHASE-02) and `:829-1106` (`mod
  interaction`), `tests/renderer/tree.rs:274-310` (`within_option`,
  `field_described`, `fields_in_option`), `:453-455`
  (`with_room_for_every_control`, in `wiring.rs`).

**Assumptions**

- **A-a — a two-option view whose options each carry drawn fields normalizes.**
  `mapper.rs`'s field cases all use a *one*-option fixture
  (`one_option_with_fields`), so the two-option-with-fields document VT-3 needs
  has never been parsed in this workspace. Cheap to falsify; falsified early.
- **A-b — `Prepared` gaining a defaulted field is invisible to every existing
  case.** `receive` is its only constructor and no test builds one by literal.
  VA-1 is the check.
- **A-c — `submitted`'s `cfg_attr(not(test), expect(dead_code))` self-clears the
  moment `controller.rs` calls it**, and the gate says so via
  `unfulfilled_lint_expectations`. Removing the attribute is part of EX-3.

**STOP conditions** — `plan.md:102-142`, S-1..S-9 entire. The three live ones
for this phase:

- **S-1** — wanting an iterator over `Draft`. Its absence is load-bearing
  (`design.md` §5.2, PHASE-03's Harvest entry); `answer()` walks the answered
  option's **blocks**. Wanting to walk the draft is a design issue, not a local
  repair.
- **S-2** — any changed assertion or fixture in `table.rs`, `wiring.rs`,
  `scheduling.rs` or `ingress.rs`. `reception.rs` is the one allowance (EX-9),
  and it is an *extension*, not an alteration.
- **S-4** — anything under `crates/goad-semantics/`. The named temptation is
  `Ord` on `OptionId`; `Draft` is shaped so it is not needed.

Also S-6 (the draft entering `Presentation`) and S-9 (`present` acquiring a
write-only-on-change path): **no instrument in the gate reaches either**, nor
EX-7 or VA-2. They are discharged by reading, and T-13 says so.

**Tasks**

- [x] T-0 — re-derive the criterion ids from `plan.md`, twice, by two methods.
      Table above. The two passes agree; all seven cross-references resolve;
      all three prose counts hold as counts, and **one of them rests on a false
      premise (F-1)**.
- [x] T-1 — EN-1. **`just check` exit 0 on the unmodified tree at `246ed93`,
      525 tests across 21 binaries**, clippy `-D warnings` clean, `cargo fmt
      --all --check` clean, `deno check` clean. PHASE-02's and PHASE-03's exit
      criteria spot-checked in the **code** rather than read off their sheets:
      `app.slint:7-11` (the three structs) and `:33` (`edited`) and `:72-101`
      (the guarded container, PHASE-02 EX-2/EX-3/EX-4); `tree.rs:274-310`
      (`within_option`, `field_described`, `fields_in_option`, EX-6) and
      `:98-106` (`element_described`'s type filter, EX-7); `draft.rs` entire
      (PHASE-03 EX-1); `view_model.rs:47-115` (`FieldBlock`,
      `PresentationField`, `PresentationOption.blocks`, `Undrawn::FieldForm`
      and `GroupHint`, EX-3); `diagnostics.rs:191-210` (the two lines, EX-5).
- [x] T-2 — EX-1: `Prepared.draft`, defaulted in `receive`, which stays its only
      constructor. `absorb`'s three `Shift` arms took **no change at all**, as
      the criterion predicted. `Prepared` is built by literal nowhere else
      (grep).
- [x] T-3 — EX-8 + EX-9 + VT-6: `Refused::UnknownField`, its line verbatim from
      `design.md` §5.2, and the fourth case in `reception.rs`. The exhaustive
      match with no `_` made the line a compile error rather than an omission,
      which is the design's reason for `Refused` living in `diagnostics.rs`.
      **EX-9's stated warrant does not hold — F-1** — and the change is right
      anyway.
- [x] T-15 — **a fifth case: `Refused::Ingress`.** Beyond EX-9's letter and
      inside its intent; referred up and approved. The case's *name* claims
      every variant and had claimed it falsely since before this slice opened.
      Now mechanically true and cheap to re-check: `Refused` declares **five**
      variants and the case constructs **five**. Controlled — C-11.
- [x] T-4 — EX-2 + VT-1: `Controller::edit`. **The refusals turned out to be
      structurally forced, not chosen — D-1.** Five cases in VT-1, three
      refusal kinds, and the assertion that nothing was recorded by any of
      them.
- [x] T-5 — EX-3 + VT-2 + VT-3: `answer()` walks the answered option's blocks.
      `submitted`'s `cfg_attr(not(test), expect(dead_code))` removed in the same
      edit; the gate would have failed on `unfulfilled_lint_expectations`
      otherwise, exactly as its own reason said it would.
- [x] T-6 — EX-4 + VT-4: `Command::Edit` in `wire.rs` (`Eq` survived the
      derive), and `dispatch`'s arm — `.err().map(Err)`, which is `None` on
      success and `Some(Err(..))` on refusal in one expression. VT-4 drives it
      through the real channel and the production `serve`.
- [x] T-7 — EX-5: `install.rs`'s seventh installation, `editing`, and the
      module doc's "six" with it.
- [x] T-8 — EX-6 + VT-7 + VT-5: `option_rows` builds real blocks through
      `field_block`. `FieldBlock` names two types in that file and both keep
      their names (D-2). `present` stays total — no conditional write was
      added; T-13 records the read.
- [x] T-9 — PHASE-02's F-4, repaired: `accessible_enabled_of` gains
      `match_inherits("Button")`, the same filter EX-7 gave `element_described`
      and for the same reason. Its doc now says why. **`field_described` and
      `within_option` were lifted to `harness.rs` rather than copied into
      `wiring.rs` — D-3, an undeclared surface, referred up.**
- [x] T-10 — negative controls. **All six cases arrived green**, so each was
      controlled rather than believed. Ten controls, recorded below; every one
      reddened the cases it should and left the rest green, and all ten were
      reverted from a backup rather than by `git`.
- [x] T-11 — refactor. `selected` and `drawn_fields` factored out of `answer`
      and `edit` so the two refusals and the R-58 walk each have **one**
      statement; `model()` named in `glass.rs` so the two nested `ModelRc`
      constructions do not repeat; `edit`'s key cloned into named bindings
      rather than a positional tuple.
- [x] T-12 — the prose sweep. Four live homes of claims this phase falsifies,
      found by reading the files being changed. Listed under **The sweep**
      below.
- [x] T-13 — EX-7, S-6, S-9, VA-1, VA-2, and `design.md` §8 — **discharged by
      reading**, each with what was read. Under **Read, not run** below.
- [x] T-14 — EX-10: **`just check` exit 0. 531 tests across 21 binaries**, from
      525. Clippy `-D warnings` clean, `cargo fmt --all --check` clean, the
      example typecheck clean. Six new cases; the `reception.rs` fourth case
      extends an existing test and adds no count.

**The sweep** — homes of a claim this phase falsifies. Each found by reading a
file being changed, and none of them by grepping for the repaired wording.

1. `install.rs:14` — "six installations", now seven. The criterion named this
   one.
2. `controller.rs:110-115` — `Controller`'s doc enumerated the complete
   retained state and did not include the draft, which now sits inside
   `Prepared`. **No criterion names it**, and the compiler does not reach a
   doc-comment.
3. `controller.rs:615` — `dispatch`'s doc: *"The two diagnostics commands are
   done here and now and produce no exchange; the other two…"*. `Command` now
   has five variants and three of them produce no exchange. A count and a
   partition, both stale from one added variant.
4. `controller.rs:798` — the refusal site's comment scoped `refusal_re_arms`
   being `false` by construction to `Command::Choose`; it is equally true of
   `Command::Edit`, and leaving it unsaid would make the next reader re-derive
   it.
5. `tree.rs:95` — `element_described`'s doc said "(`field_described` below)".
   It is no longer below; it is `harness::field_described`.
6. `harness.rs:1-12` — the module doc named a **closed list** of case files
   (`wiring.rs`/`table.rs`/`scheduling.rs`) that the file's own rule does not
   imply; `tree.rs` became a caller. Rewritten to state the rule rather than
   an enumeration of who currently satisfies it — PHASE-01's F-5 and PHASE-02's
   T-9 the third and fourth times.
7. `wiring.rs:1-12` — the module doc enumerates which validation items the file
   claims; `mod editing` was not in it.

**The controls** — every case arrived green, so which kind of claim each makes
was asked first (PHASE-03's D-3). Three of the six make a claim the default
supplies in part, and those parts are named as unreachable rather than
controlled.

| # | the mutation | red | green |
|---|---|---|---|
| C-1 | `selected` drops the view-token check | VT-1 | the other five |
| C-2 | `edit` refuses `UnknownOption` where it should refuse `UnknownField` | VT-1, VT-4 | four |
| C-3 | `answer` submits `BTreeMap::new()` — the body before EX-3 | VT-2, VT-3 | four |
| C-4 | `Draft::state_of` keys by field alone, dropping the option half | VT-3, VT-5 | four |
| C-5 | the row model ignores the draft and always draws unticked | VT-5, VT-4 | four |
| C-6 | the form is read through a window at its own preferred size | VT-5 | five |
| C-7 | the blocks reach the row model in reverse declared order | VT-7 | five |
| C-8 | an untitled block is drawn under an invented heading | VT-7 | five |
| C-9 | an edit is treated as an exchange — `dispatch` returns `Some(Ok(..))` | VT-4 | five |
| C-10 | a refused edit is dropped rather than reported | VT-4 | five |
| C-11 | the ingress line drops its wire token, keeping the prose | VT-6's fifth case | the rest |

- **C-6 is a control on the *fixture*, not on the code**, and it is the one
  worth reading twice. With the declared viewport removed, VT-5 fails
  `no control described "read" under "morning"` — the whole form is clipped out
  of the `Flickable`'s rect. PHASE-02's **F-6 is unchanged by this phase and
  remains open**: the test can now see the form; the product still cannot show
  it at its own preferred size.
- **The rejected reading could not be implemented, and that is the finding.**
  The sharpest control for D6 would be `answer()` walking the draft instead of
  the blocks (PHASE-03's T-13). It **is not constructible**: `Draft`'s tuple
  field is private and its only two methods are `state_of` and `record`, so
  there is no iterator to walk and adding one is S-1. The absence that makes
  the control impossible is the same absence that makes D6 impossible, which is
  what "a property of the type rather than a convention" means when it is true.
- Where a claim **is** the default, no control reaches it: VT-5's three
  `Checked(false)` assertions and VT-1's "nothing was recorded" are the
  as-drawn value, which a `state_of` stubbed to a constant also supplies. The
  assertions that carry those cases are the ones asserting `true` — C-4 and C-5
  reach them, and the `false` assertions are load-bearing only in C-4, where a
  field-only key turns `evening/read` into a `true` the draft never held there.

**Read, not run** — the four criteria with no instrument in the gate, plus the
two STOP conditions in the same position. `cargo test -p goad-semantics` and the
other ADR-001 instruments stop short of stratum 3, so a green gate is evidence
for none of these.

- **EX-7 / S-6 / I-4 — the draft does not enter `Presentation`.** Read
  `view_model.rs` whole for `Draft`, `draft`, `checked`, `&mut self` and any
  `pub` mutable member: **two hits, both doc-comments** (`:72`, `:217`), naming
  `draft.rs` as the place the person's state lives instead. No type there holds
  a `Draft`, no `PresentationField` gained a `checked`, and `present()` still
  produces a value nothing mutates. `Presentation` is `view_model.rs` and
  `Draft` is `draft.rs`; the dependency runs one way, and `glass.rs` is where
  the two meet — by **lookup**, in `field_block`, and the row model is the only
  thing that carries both.
- **S-9 / I-5 — `Glass::present` stays total.** Read `present` line by line:
  every `set_*` is unconditional. Four branches, and **not one of them skips a
  write** — `set_mode`'s ternary, the `match frame.shown` that supplies empty
  values when nothing is shown, `notice`'s ternary, and the `Surface` match
  choosing `show()` or `hide()`. `self.options.set_vec(options)` replaces the
  whole vector, and `option_rows` rebuilds every `FieldBlock` and every
  `FieldRow` from the presentation and the draft on each call. `grep -rn
  'set_row_data\|row_changed' crates/goad/src` returns **nothing**: the reset
  path §5.4 rests on is the only one, so A-2 is not weakened.
- **VA-1 — AC-6.** `git diff` over the four files VA-1 names: `table.rs`,
  `scheduling.rs` and `ingress.rs` have **empty diffs**. `wiring.rs` has
  **two removed lines in the whole file** — one module-doc line, extended, and
  the `use crate::harness` list, widened. No assertion, no expected value and no
  fixture value was removed or changed; `accessible_enabled_of`'s type filter is
  an addition inside a **helper**, which S-2 names as neither. `Prepared`
  gaining a defaulted field is invisible to every existing case, which is what
  the three empty diffs say. Outside VA-1's list but inside AC-6's scope per
  `design.md` §9: `tree.rs`'s removed lines are the two helper bodies and one
  doc line, and `reception.rs` has **no removed lines at all** — EX-9 is a pure
  extension.
- **VA-2 — `answer()` against D6.** The walk is `drawn_fields(matched)`, which
  is `option.blocks.iter().flat_map(|block| block.fields.iter())` — the
  presentation's blocks for the matched option, and nothing else. `answer`
  names the draft exactly once, as `prepared.draft.state_of(&matched.id,
  &field.id)`, keyed by two ids taken from that walk. **`Draft` still exposes no
  enumeration**: `Draft(Vec<…>)`'s field is private, its `impl` block has
  `state_of` and `record` and nothing else, and there is no `IntoIterator`, no
  `Deref` and no keys accessor. Every mention of a draft outside `draft.rs` is
  one of five things — a field declaration, a `Default::default()`, one
  `state_of` for the screen, one `state_of` for the wire, one `record`.
- **`design.md` §8, re-read at the end of the phase** and not only at the
  start, which is PHASE-03's D-5. R-1 and R-2 are discharged and this phase
  turns both into live pins rather than paper ones — VT-7 compiles against the
  array-typed member and VT-5 reads a control the model reset re-established.
  R-3: no `Edited` variant was added. **R-4 is the one to check hardest** and is
  the same read as EX-7 above: no `checked` on `PresentationField`, no mutable
  member anywhere in `view_model.rs`. R-6: nothing written this phase is named
  for grouping — `field_block`, `drawn_fields` and `selected` name layout, a
  walk and a selector resolution. R-7 and R-8 are untouched.

**Decisions taken during execution**

- **D-1 — `edit`'s three refusals are structurally forced, not chosen.** The
  criterion reads as three checks an implementer must remember to write. It is
  not: `Draft` is keyed by `OptionId` and `FieldId`, and `OptionId::new` /
  `FieldId::new` are `pub(super)` in `goad-semantics`, so the renderer can only
  **clone** an id off the retained presentation and cannot mint one. An `edit`
  that recorded without finding the option and the declared field would have
  nothing to key by. The lookup that produces the key **is** the lookup that
  produces the refusal, and the two cannot come apart. That is worth writing
  down because it is also why VT-1 arrived green: the naive implementation is
  the correct one, and the controls (C-1, C-2) had to be built by deleting a
  check rather than by declining to write it.
- **D-2 — `selected` and `drawn_fields` are the refactor, and `drawn_fields` is
  load-bearing beyond DRY.** `answer` and `edit` made the same two refusals in
  the same order and walked the same fields. Factoring the walk into one named
  function means the walk R-58 rests on has **one** statement that VA-2 reads,
  rather than two that could drift; and it makes "a field that can be edited is
  exactly a field that will be submitted" true by construction rather than by
  two implementations agreeing. `edit` calls `selected` through a reborrow of
  `&mut Prepared`, and the ids are cloned before the write, which is what ends
  the presentation's borrow.
- **D-3 — `field_described` and `within_option` were lifted from `tree.rs` to
  `harness.rs` rather than copied into `wiring.rs`. Referred up; an undeclared
  surface.** VT-5 reads a control off a shown window by the option-scoped
  query, so `wiring.rs` needs the helper `tree.rs` owns. `harness.rs`'s own rule
  is "what two or more case files in this target need", and `tree.rs` and
  `wiring.rs` are now two. The alternative is a verbatim second copy of a
  fourteen-line query, which is the duplication CLAUDE.md forbids outright.
  **The cost is that `harness.rs` and `tree.rs` are not PHASE-04 surfaces.**
  S-2 expressly allows a change to a test helper's *implementation*, and
  `tree.rs`'s diff is two function bodies moving and one doc line; no assertion,
  no expected value and no fixture value changed. Recorded here and put to the
  orchestrator rather than taken silently.
- **D-4 — the row model's `checked` is read out with an irrefutable `let`.**
  `let Edited::Checked(checked) = draft.state_of(..);` compiles today because
  `Edited` has one variant, and becomes a **compile error** the moment a second
  lands. That is the behaviour wanted: what a checkbox row should show for a
  non-boolean value is a decision, and it belongs at the site that would have to
  make it rather than behind a `_ =>` arm that quietly picks one. Same shape as
  `draft.rs`'s own `let View::Choice(choice) = &view;`.
- **D-5 — EX-9 implemented as stated, its warrant recorded as false rather than
  reconciled, and the gap then closed by decision.** See F-1. The fourth case
  went in on the criterion's letter; the fifth — `Refused::Ingress`, which is
  what makes the test's name true — was **not** taken unilaterally, because it
  is beyond EX-9, and was put to the orchestrator instead. **Approved, and
  taken** (T-15). The distinction worth recording is that the fifth case is
  beyond the criterion's *letter* and inside its *intent*: EX-9 wants a test
  whose name is its claim to be telling the truth, and four of five does not do
  that. Implement the letter, report the discrepancy, do not quietly
  reconcile — and let the decision to go further be someone else's.

**Findings**

- **F-1 — EX-9's stated warrant is false: the test's name did not make the
  claim the criterion says it keeps.** EX-9 extends
  `every_refused_variant_renders_one_line_with_the_failure_prefix` "with a
  fourth variant, which keeps the claim its name makes rather than altering
  one". The count is right — three cases, now four — but `Refused` had **four**
  variants before this phase and the test covered three. `Refused::Ingress` has
  never been in it, and `grep -rn 'Refused::Ingress'` finds exactly two lines,
  both in `src/`: its rendering is asserted nowhere. So the test now covers four
  of five, and the criterion cites as its warrant a claim that was already
  untrue when it was written.
  **The criterion is still right, for a reason it does not give.** EX-9 says
  the extension *keeps* the claim the name makes. It does not: the name made a
  **false** claim that needed fixing, not a true one that needed keeping. What
  makes the criterion right is its second reason — `Diagnostics::refused` is
  exhaustive with no `_`, so a variant without a line is a compile error, and
  the case is what pins the wording a person reads. A criterion can be right
  and its stated warrant wrong at once, which is PHASE-02's F-3 a second time
  and the same handling: implement the letter, report the discrepancy, do not
  reconcile it in silence.
  **Closed by decision** (D-5, T-15): the `Ingress` case went in, and the name
  is now true — five variants declared, five constructed, a check an auditor can
  run in one line.
- **F-2 — `design.md` §9's closing rule and its own AC-8 row disagree, and
  PHASE-04 is where they meet.** §9 closes: *"Every field test either reads the
  wire or asserts something about the screen"*, and warns that a test reading
  the draft through `Controller` is a proxy because *"it would pass with
  `answer()` walking the draft's keys instead of the declared fields, which is
  D6"*. VT-2 and VT-3 assert on `answer()`'s returned `UserResponse` — neither
  the wire nor the screen. But §9's **AC-8 row** names `answer()` as R-58's
  vehicle in as many words, and `plan.md`'s Coverage assigns AC-8 to
  PHASE-04/VT-3 while assigning AC-1 wholly to PHASE-05/VT-1, saying
  explicitly that neither substitutes for the other.
  **The warning does not in fact reach these two cases**, and the reason is
  worth keeping: a walk over the draft's keys cannot produce a key for a field
  nobody touched, so VT-2's and VT-3's assertion that `stretched` is present and
  `false` is one D6 fails rather than one it also satisfies. That is PHASE-03's
  D-3 run in the other direction — asking which half of the claim the defect
  would still satisfy. The tension is between two sentences of the design, not
  between the design and the code, so it is **audit's** to settle: either §9's
  disjunction gains R-58's exception, or AC-8's row moves. VT-3's own doc-comment
  now says which vehicle it is, so a reader does not take it for AC-1's.
- **F-3 — `plan.md`'s PHASE-04 Surfaces line omits `crates/goad/src/draft.rs`,
  which EX-3 sends the phase to edit.** `submitted`'s
  `cfg_attr(not(test), expect(dead_code, …))` must come off the moment
  `controller.rs` calls it, and the gate fails on
  `unfulfilled_lint_expectations` if it does not — which the attribute's own
  reason predicted, and which is the one thing in this phase that self-reported.
  A plan-internal gap, no surface breach, and nothing for the audit's path diff
  to chase. **Third occurrence in this slice** — PHASE-01's F-1 was the same
  shape over `ui/app.slint`, and the class is now worth stating: a Surfaces line
  is an enumeration, and every enumeration in this slice has been a floor.
- **F-4 — `canon-delta.md` carries a claim about the code that this phase makes
  false, and it is draft canon, so it is not a phase's to edit.** Under *The
  gap*: *"`crates/goad/src/controller.rs:216` sends an empty map and there was
  no value to type."* It no longer sends an empty map, and `:216` no longer
  holds that line. The sentence is framed as the state of affairs the slice
  opened on — *"While no renderer drew a field, the gap cost nothing"* — so it
  may be read as history rather than as a claim about today, which is the same
  disposition PHASE-03's F-1 reached for `roadmap.md` and `slice-007.md`.
  Whether it wants repairing at promotion is audit's call. Recorded so it is a
  decision rather than an oversight.
- **F-5 — PHASE-02's F-6 is untouched and was re-measured in passing.** C-6
  removed VT-5's declared viewport and the case failed
  `no control described "read" under "morning"` — the form is clipped out of the
  `Flickable`'s rect at the window's preferred size, exactly as F-6 records. The
  test sees the form because the fixture declares 600×600, not because anything
  about the product changed. **AC-7 and AC-10 still observe it, at PHASE-06/VH-1
  and VH-2.**

### PHASE-05 — the form on the wire

**Objective:** what the person ticked is shown to leave the host as JSON, under
the option they pressed, surviving a present that changed nothing
(`plan.md:715-716`).

**Criterion ids, re-derived from `plan.md`** — T-0 below. `plan.md:713-806`,
PHASE-05 entire:

| kind | ids | count |
|---|---|---|
| entry | EN-1 | 1 |
| exit | EX-1 … EX-5 | 5 |
| verification (test) | VT-1 … VT-4 | 4 |
| verification (agent) | VA-1 | 1 |

**Pass 1** — every `^- <ID> —` bullet in the phase, in order: `EN-1`, `EX-1 …
EX-5`, `VT-1 … VT-4`, `VA-1`. Contiguous from 1 in each kind, no gap and no
duplicate. **Pass 2** — the same bullets bucketed by the `**Entry**` /
`**Exit**` / `**Verification**` heading each falls under: entry 1, exit 5,
verification 5 (4 VT + 1 VA). The two passes agree.

**Cross-references.** Outward: PHASE-05's body cites **no** `PHASE-0N/<id>` at
all — it is the last phase before the human one and depends on 04 by sequence,
not by citation. Inward, Coverage cites `PHASE-05/VT-1` (AC-1),
`PHASE-05/VT-2` (AC-4), `PHASE-05/VT-3` (AC-5), `PHASE-05/VT-4` (AC-3) and
`05/EX-5` (AC-9); all five exist.

**The line cites the phase makes, checked rather than trusted** —
`plan.md:36-45`'s rule, and the third time this slice a criterion's warrant has
not matched the code:

| cite | claim | holds? |
|---|---|---|
| `scheduling.rs:101-109` | `logging_scripted` lives there | yes, exactly |
| `scheduling.rs:127-142` | `absorbed_line`'s doc, and it states the ordering backwards | yes — the doc is `:127-142` and `:130-131` says *before it reads the request*, which is false of the script |
| `logs-the-request-then-answers.sh:19/:26/:32` | `request="$(cat)"`, the append, the answer | yes, all three |
| `harness.rs:1-8` | the module doc **enumerates its consumers** | **no — F-1** |

**Reading list**

- `docs/slices/007/plan.md:713-806` — PHASE-05 whole, including *Notes for the
  implementer*; `:102-142` — S-1..S-9; `:36-45` — the re-derivation rule;
  `:89-91` — why 05 is after 04; `:156-171` — Coverage.
- `docs/slices/007/design.md:1064-1107` (§9 — what each tier proves, the row
  for each of AC-1/AC-3/AC-4/AC-5, and the closing proxy rule),
  `:760-901` (§5.4 — the lifecycle VT-3 turns on: the rebuild that makes A-2
  hold, and `Retained` leaving the draft alone), `:1047-1063` (§8, re-read at
  the end of the phase as well as the start — PHASE-03's D-5).
- `docs/slices/007/canon-delta.md` — R-57 and R-58; VT-4 is R-58's
  "silent about an undrawn field" clause read off the wire.
- `docs/slices/007/slice-007.md:147-172` — AC-1, AC-3, AC-4, AC-5.
- The code: `tests/renderer/scheduling.rs:101-109` (`logging_scripted` —
  **read there; EX-2 moved it to `harness.rs:172-180`, so the cite is where it
  was**), `:111-125` (`request_kind`, the precedent EX-3 names), `:127-142`
  (`absorbed_line`); `tests/renderer/harness.rs` entire, module doc first;
  `tests/renderer/wiring.rs:1125-1560` (`mod editing` — the only module whose
  fixtures carry fields, `with_room_for_the_form` at `:1225-1236`,
  `checked_in_row_model` at `:1550-1560`), `:62-84` (`accessible_enabled_of`);
  `tests/renderer/tree.rs:94-107` (`element_described` — **read there; this
  phase's D-3 moved it to `harness.rs:101-109`, so the cite is where it was and
  not where it is**), `:330-400` (driving a checkbox with
  `invoke_accessible_default_action`, and the model-reset pin);
  `tests/backends/logs-the-request-then-answers.sh` (repo root, not under
  `crates/`); `tests/support/scripting.rs` (`logging_backend`, `invocations`);
  `tests/renderer/main.rs` (the roll-call); `src/install.rs` (the callback
  table — seven installations), `src/main.rs:85-90` (how production assembles
  `Wire` + `install` around a capacity-**1** channel), `src/controller.rs:701-880`
  (`serve`: present at the top of the iteration, `Edit` returning `None` from
  `dispatch`), `src/glass.rs:127-215` (`option_rows`, `field_block`, and
  `next_check` written from the frame every present).

**Assumptions**

- **A-a — a Slint callback installed by `install` fires under
  `init_no_event_loop`.** `tree.rs` drives `on_edited`/`on_chosen` set by the
  test itself; nothing in this target has yet driven the **production**
  callback table. `invoke_accessible_default_action` is synchronous, so the
  callback runs on the test's own thread inside the `LocalSet`. Cheap to
  falsify; falsified first.
- **A-b — a capacity-1 channel is enough if every action waits for an
  observable before the next.** Production is capacity 1 (`main.rs:85`). A
  click whose `try_send` finds it full is **dropped**, silently as far as the
  assertion is concerned — so every tick is followed by a wait on the draft's
  own projection, and a dropped one fails as a timeout rather than as a wrong
  value.
- **A-c — the first scheduled firing cannot reach these cases.** `driving::host`
  seeds `DEFAULT_POLL` at 30 minutes and the initial timer arm is
  `MINIMUM_SPACING` (3 s) from process start; the first exchange absorbs
  immediately and resets the deadline from its own `next_check`. Only a case
  that took longer than 3 s to reach its first absorb would see an extra
  invocation.

**STOP conditions** — `plan.md:102-142`, S-1..S-9 entire. The live ones:

- **S-7 — this phase's own.** A field test that neither reads an invocation log
  nor asserts something about the screen. It is not a style rule: a case that
  reads the draft back through `Controller` passes with `answer()` walking the
  draft's keys, which is D6.
- **S-1 — any wish to change a `src/` file.** The mechanism is finished; this
  phase is the proof it works from outside. No `src/` path is a declared
  surface.
- **S-2 — any changed assertion or fixture in an existing case file.** EX-2's
  lift is a *move* of a helper and an import; `scheduling.rs`'s assertions do
  not change.

**Tasks**

- [x] T-0 — re-derive the criterion ids from `plan.md`, and check every line
      cite the phase makes. Tables above. The two passes agree; all five
      inward cross-references resolve; three of the four line cites hold and
      **one does not — F-1**.
- [x] T-1 — EN-1. **`just check` exit 0 on the unmodified tree at `69c617b`,
      531 tests across 21 binaries**, clippy `-D warnings` clean, `cargo fmt
      --all --check` clean, `deno check` clean. PHASE-04's exit criteria
      spot-checked in the **code** rather than read off its sheet:
      `reception.rs` (`Prepared.draft`, EX-1); `controller.rs:257-276`
      (`edit`, EX-2), `:213-239` (`answer` walking `drawn_fields`, EX-3),
      `:343-345` (`drawn_fields` itself); `wire.rs:24-45` (`Command::Edit`,
      EX-4); `install.rs:19-72` (seven installations, EX-5); `glass.rs:127-215`
      (`option_rows`/`field_block`, EX-6); `diagnostics.rs` (`UnknownField`,
      EX-8); `harness.rs` (the two lifted queries, PHASE-04 D-3).
- [x] T-2 — EX-2: `logging_scripted` lifted from `scheduling.rs:101-109` to
      `harness.rs`, body unchanged, callers following the import. `PathBuf`
      became unused in `scheduling.rs` and its import narrowed to `Path` —
      the only other line that moved there. **No assertion in `scheduling.rs`
      changed** (S-2): its diff is the removed function, two `use` lines and
      nothing else. **EX-2's module-doc clause was not carried out, and F-1
      says why.** The doc gained one clause instead: its enumeration of *what
      kinds of thing* live in the file had gone incomplete, which is the same
      class one level down.
- [x] T-3 — EX-1 + EX-3 + VT-1. `fields.rs` drives the production `serve`
      against a real child process, with `install`'s own callback table on the
      window: **every** command these cases put on the channel is fired by
      activating a real element, the tray's `check_now` included, so no case
      builds a `Command`. EX-3's reader is `logged` + `submitted_values` +
      `answered_option`, in `fields.rs`, beside `scheduling.rs`'s
      `request_kind` precedent. VT-1 reads the screen immediately before the
      press and compares the wire **to it**, which is AC-1's own wording.
- [x] T-4 — VT-2. **Two ticks, not one — D-2.** The extra one is what makes
      VA-1's named injection reach the case at all.
- [x] T-5 — VT-3, the conjunction, with the `Retained` fold **waited for**
      through the next-check line the production glass wrote. Both directions
      of "neither half substitutes" are measured, not argued: C-2 and C-3.
- [x] T-6 — VT-4. Reads the log **and** the window's own `diagnostic-lines`;
      both halves independently reach C-5.
- [x] T-7 — VA-1. **Eight injections and three half-case variants of them**,
      eleven runs in all, each against the whole 182-case target, read, and
      reverted from a backup rather than by `git`. Table below — eleven rows,
      counted. **Every case arrived green**, so each was controlled rather than
      believed. The tree was diffed against the backup after the last revert
      and `src/` and `ui/` were byte-identical.
- [x] T-8 — refactor. `tick!` and `screen_of` named, so a tick is one
      statement of *click plus wait* rather than two literals repeated six
      times, and the screen is read in one line at each of its two sites.
      Then **`element_described` lifted from `tree.rs` to `harness.rs`** —
      referred up, approved, D-3 — so the Button-filtered description query has
      one statement rather than the three a local copy would have made.
      `fields.rs`'s `option_control` is one line over it. C-1 and C-4 re-run
      after each step, unchanged; **C-7 re-measured against the lifted helper**,
      because its doc's claim now covers `tree.rs`'s cases as well as this
      file's — still all 182 green.
- [x] T-11 — **the record against the diff**, run after the phase closed. Every
      path in the commit matched against this sheet: six of seven claimed, and
      the two that are not are `notes.md` and `phase-05-brief.md`, which are
      this sheet and its input. Every `path:line` the sheet cites resolved
      against the tree: `wiring.rs:1125` still `mod editing`; **two dead, both
      moved by this phase's own lifts** — F-8. Then the second pass, which no
      command reaches: every **decision** re-read against what the diff did.
      D-3 is the one that moved during the phase — it was written to record the
      lift being *declined* and rewritten when the lift was approved and taken;
      it now reads with the code (`notes.md:1698`, `harness.rs:101`,
      `tree.rs:25`). That pass is in F-8 as the half pass 1 cannot see.
- [x] T-9 — EX-4 and `design.md` §8, re-read over the finished file. Under
      **Read, not run** below. The same pass swept `fields.rs`'s own
      doc-comments for slice-local finding and control ids, per
      `docs/memory/cite-requirements-not-finding-ids.md`: **the file now cites
      none**, and each rationale that had been a pointer is written out as the
      claim itself — the clipping measurement, the option-half injection, the
      three control outcomes VT-3's and VT-4's halves rest on. It also caught a
      mis-citation of my own: *"exactly one control per option"* is
      `tree.rs`'s `heading_body_and_one_control_per_option_render_in_order`,
      not PHASE-02/VT-2, and the two tests are now named rather than numbered.
      **F-6.**
- [x] T-10 — EX-5: **`just check` exit 0. 535 tests across 21 binaries**,
      from 531. Four new cases; clippy `-D warnings` clean, `cargo fmt --all
      --check` clean, the example typecheck clean.

**The controls** — every case arrived green. Each injection was applied to a
pristine tree, run against the **whole** renderer target, read, and reverted.

| # | the mutation | where | red | green |
|---|---|---|---|---|
| C-1 | `Draft::state_of` keyed by field alone, dropping the option half | `draft.rs` | **VT-2**, and `wiring::editing`'s two | VT-1, VT-3, VT-4 |
| C-1b | C-1, with VT-2's two screen assertions removed | + `fields.rs` | VT-2, at the **wire** assertion | — |
| C-2 | `answer()` sends an empty map — the body before PHASE-04's EX-3 | `controller.rs` | all four, each at its own wire assertion | 176 |
| C-3 | the control stops reading the row model's `checked` (`checked: false`) | `ui/app.slint` | VT-1, VT-2, VT-3 at their **screen** assertions; `tree.rs`'s A-2 pin | VT-4 |
| C-3b | C-3, with VT-3's screen assertion removed | + `fields.rs` | **nothing — VT-3 passes** | — |
| C-4 | a `Shift::Retained` fold drops the draft | `controller.rs` | **VT-3 alone, of 182** | 181 |
| C-5 | a `text` field is drawn rather than reported undrawn | `view_model.rs` | VT-4, and nine others | VT-1, VT-2, VT-3 |
| C-5b | C-5, with VT-4's diagnostic-surface assertion removed | + `fields.rs` | VT-4, at the **wire** assertion (`noted` on the wire) | — |
| C-6 | the declared viewport removed — **a control on the FIXTURE** | `fields.rs` | all four, `no control described "read" under "morning"` | 178 |
| C-7 | the Button type filter dropped from the description query | `fields.rs`, then re-run on `harness.rs` after the lift | **nothing — all 182 green, both times** | all |
| C-8 | the `respond` names the last option rather than the one pressed | `controller.rs` | **VT-2 alone, of 182** | 181 |

- **C-1 is the injection VA-1 names, and it behaved exactly as VA-1 requires:
  VT-2 red, VT-1 green.** The brief's warning — *if both go red, your VT-1 is
  also resting on the scoping* — did not fire. VT-3 and VT-4 stayed green too,
  which is the same evidence for them.
- **VT-3's conjunction is measured in both directions, and neither half
  substitutes.** Under C-2 the screen assertion **passed** and the wire one
  failed. Under C-3, with the screen assertion removed, the case is **green
  while every box on the screen is wrong**. That is the disjunction EX-4
  states, shown to be the wrong rule for this one case rather than argued to
  be.
- **C-4 and C-8 each redden exactly one case out of 182.** VT-3 is the only
  thing in this workspace holding *a `Retained` fold leaves the draft alone*
  — which is also `design.md` §8/R-8's stated bound, which until now had no
  test — and VT-2's `answered_option` is the only thing holding *the request
  names the option that was pressed*: `wiring::editing` asserts key sets and
  never `UserResponse.option`.
- **C-6 is a control on the *fixture*, not on the code** — PHASE-04's C-6 a
  second time, and the same failure verbatim. The cases can see the form
  because `with_room_for_the_form` declares 600×600. **PHASE-02's F-6 is
  unchanged by this phase and remains open**: the product still clips its own
  second option at its preferred size, and PHASE-06/VH-1 and VH-2 observe it.
- **C-7 stayed green, and that is F-3.** The Button type filter is right and
  is pinned by nothing; no case is written to pin it, because a case that
  cannot be made to fail pins nothing. Re-measured after the lift, so the
  claim in `harness::element_described`'s doc covers both its callers.

**Read, not run** — the criteria with no instrument in the gate.

- **EX-4 / S-7 — every case reads the log or asserts the screen, and VT-3 does
  both.** Walked case by case over the finished file. VT-1: `submitted_values`
  **and** `screen_of`. VT-2: `submitted_values` + `answered_option` **and**
  two `checked_on_screen` reads. VT-3: both, both load-bearing, both measured.
  VT-4: `submitted_values` + `answered_option` **and** the window's own
  `diagnostic-lines` and `is_visible()`. **No case reads the draft through
  `Controller`**: `grep -n Controller crates/goad/tests/renderer/fields.rs`
  returns five lines — three prose, one `use`, and `Controller::new()` being
  handed to `serve`. There is no `controller.` call anywhere in the file, and
  no case holds a `Controller` to ask.
- **`design.md` §8, re-read at the end of the phase** (PHASE-03's D-5). R-1 and
  R-2 are discharged and this phase exercises both through a real present.
  **R-2's "signal it is happening" is AC-5**, which until now was a screen
  assertion in `wiring.rs`; VT-3 is the regression pin the row describes,
  driven through the production loop, and C-4 shows it is the only one.
  R-3: no `Edited` variant, and `fields.rs` never names `Edited` — it cannot,
  because nothing here builds a command. R-4: **no `src/` file was touched at
  all**, so the draft cannot have crept into `Presentation` this phase.
  **R-5 is the risk this phase exists to retire**, and its stated signal — *a
  field test that never reads an invocation log* — is absent by construction.
  R-6: nothing named this phase is a grouping concept. R-7: the cases pass
  under `material`, reading controls off a shown window. **R-8 gained a test
  it did not have**: its mitigation rests on *`view: null` folds to
  `Shift::Retained` and leaves the draft alone*, and that sentence is now
  VT-3's premise and C-4's subject.
- **`docs/memory/cite-requirements-not-finding-ids.md`, applied to the file
  this phase wrote.** `fields.rs` cites no `F-N` and no `D-N`; where a
  rationale is a measurement rather than a requirement, the measurement is
  written out (the 65px preferred height and what it clips; the option-half
  injection and which case it reddens; the three control outcomes VT-3's and
  VT-4's halves rest on). Criterion ids — `EX-1`, `VT-3`, `VA-1` — are kept:
  they are what every case file in this target uses to say what a case is, and
  they are phase-qualified when they point outward. The rule's *state*
  elsewhere is **F-6**.
- **The vocabulary scan does not reach this file at all.** `Scan.excluded_dirs`
  is `["tests", "target"]` and skips any directory of that name
  (`goad-boundary/src/scan.rs:128-132`), so `crates/goad/tests/` is never
  walked — not its comments, not its string literals, not its test names.
  PHASE-03's F-3 is about `src/`. A case file's fixtures are held by **review
  alone**; these use `morning`, `evening`, `stretched`, `read`, `tidied` and
  `noted`, none of them on the list and none of them a host concept.

**Decisions taken during execution**

- **D-1 — no helper in this file may be `async`, and that shaped the file.**
  `future_not_send` is `deny` workspace-wide (`Cargo.toml:201`) and reaches an
  `-> impl Future` return exactly as it reaches an `async fn` — measured, both
  shapes tried. Every future here holds a `PromptWindow`, which is `!Send` by
  construction. A `#[tokio::test]` body is not async after expansion, so the
  lint does not reach a case; it reaches any helper that wraps one. So the
  loop-driving and the tick are **macros** (`driving!`, `tick!`), expanding
  into each case, and the setup they take apart is a plain `struct Rig` built
  by a plain `fn rigged`. `table.rs:190`'s `observed!` is the precedent for a
  case-file macro in this target. **The alternative was the workspace's first
  `#[expect(clippy::future_not_send)]`**, which is a concession, and a
  concession is not a phase's to take on its own (`plan.md` §Execute).
  It is also the better shape for EX-1: the `serve` call is textually in each
  case rather than behind a helper.
- **D-2 — VT-2 ticks two boxes where the criterion ticks one, because one
  cannot fail.** VT-2 as written ticks the answered option's own box; under
  VA-1's named injection — the draft key's `option` half ignored — that case
  stays **green**, because one option's answer cannot tell the two readings
  apart. What tells them apart is ticking the shared id under the option that
  is **not** answered. Both of VT-2's own clauses are still there and the
  extra tick is what makes VA-1 reachable, so this is an extension inside the
  criterion's intent rather than a departure from it — PHASE-04's T-15 shape.
  Recorded rather than reconciled quietly.
- **D-3 — `element_described` lifted from `tree.rs` to `harness.rs`. Referred
  up, approved, and an undeclared surface.** `fields.rs` must press an option's
  own control, which needs a Button-filtered description query; that query
  already existed twice — `tree.rs`'s `element_described` and `wiring.rs`'s
  `accessible_enabled_of`, the same query with a different terminal, each with
  its **own** doc-comment explaining the same filter in different words. A
  third statement was the thing to avoid. This is PHASE-04's D-3 exactly:
  same target, same file, same rule (`harness.rs`'s own *two or more*), and
  `field_described`/`within_option` are already there from that lift.
  **`tree.rs`'s diff is one function body and one `use` line** — no assertion,
  no expected value, no fixture value, so not S-2, which expressly allows a
  change to a helper. `fields.rs` keeps a one-line `option_control` over it,
  which names the activation and holds the panic message; it restates no query.
  The lifted doc dropped one citation as it moved — `D10`, a slice-local
  decision id — per `docs/memory/cite-requirements-not-finding-ids.md`.
  **`slice-007.md` §Scope names `crates/goad/tests/renderer/` as a directory,
  so the audit's path diff sees no breach; the gap is in `plan.md`'s per-phase
  Surfaces line, which is the orchestrator's, not this phase's, to amend.**
  `wiring.rs::accessible_enabled_of` was deliberately **not** folded in —
  **F-3.**

**Findings**

- **F-1 — EX-2's warrant is stale: `harness.rs`'s module doc no longer
  enumerates its consumers, and the better mechanism is already there.** EX-2
  says the doc *"enumerates its consumers — `wiring.rs`/`table.rs`/`scheduling.rs`
  — and `fields.rs` is a fourth the list does not admit; it moves with the
  lift."* PHASE-04's own sweep (its T-12, item 6) had already replaced that
  closed list with the rule it was an instance of: *"The rule is `two or more`,
  and the set that satisfies it is not fixed … which is why nothing here names
  a closed list of case files"* (`harness.rs:11-13`). So the criterion's
  **intent** — that the doc not go stale when a fourth consumer arrives — is
  met, by a mechanism that cannot go stale at all, and the clause of EX-2 that
  sends the phase to edit the doc has nothing to do. Re-introducing an
  enumeration to satisfy the criterion's letter would undo PHASE-04's repair.
  **What did move** is one clause of the same doc's *other* enumeration — the
  list of what kinds of thing live in the file, which mentioned neither a
  logging backend nor an unscoped control query. `main.rs`'s roll-call moved
  too (EX-1's Surfaces line names it), and its two *caller* lists were replaced
  by the rule rather than extended, for the reason this finding is about.

  **The class, with all three instances, and it is not a defect count.** Three
  criteria in this slice have stated a warrant the code does not bear out:

  | | the criterion | what it claimed | what was true |
  |---|---|---|---|
  | PHASE-02 F-3 | EX-7 | a type filter keeps two named cases green | neither case is kept green by it — EX-4's own guard means those options have no container, and even with one the walk reaches the control first. The filter still belongs; the case written to pin it was vacuous and was deleted |
  | PHASE-04 F-1 | EX-9 | extending a test *keeps* the claim its name makes | the name had made a **false** claim since before the slice opened: `Refused::Ingress` was in no case and asserted nowhere. The criterion was right for a reason it did not give |
  | PHASE-05 F-1 | EX-2 | `harness.rs`'s doc enumerates its consumers, so the lift must move the list | PHASE-04 had already replaced the list with the rule. The criterion's intent was met by a better mechanism than the one it names |

  **Every one of the three was found by reading the thing the criterion cites,
  and in all three the criterion was still right.** That is the finding: a
  criterion that states its own reasoning is *checkable*, and checking it has
  paid three times out of three — two vacuous tests avoided, one false test
  name repaired, one doc repair not undone. A plan that said only *"lift the
  helper"* would have produced none of these and none of the repairs, and the
  phase would have complied with each criterion's letter without anyone
  noticing. **The audit should hear this as an argument for the plan's style,
  not as three errors in it.** What it costs is one task per phase — re-derive
  the ids, then read what each warrant cites — and `plan.md:36-45` already asks
  for the first half. The half worth adding is the second.
  **What did move** is one clause of the same doc's *other* enumeration — the
  list of what kinds of thing live in the file, which did not mention a logging
  backend. `main.rs`'s roll-call moved too (EX-1's Surfaces line names it), and
  its two *caller* lists were replaced by the rule rather than extended, for
  the reason F-1 is about.
- **F-2 — a case's fixture name is a shared path with no instrument, and two
  existing cases already collide.** `scripting::marker` turns a case name into
  `goad-invocations-<name>-<pid>` in the temp directory and **clears the file**
  on the way out; every case in the `renderer` target shares one pid and cargo
  runs them concurrently. `scheduling.rs:415` and `wiring.rs:1570` both call
  `scripted("vt8", …)`, so they share one invocation log — and
  `scheduling.rs`'s asserts `invocations(&log) == 1` while `wiring.rs`'s
  spawns a backend that appends to it. **Found by walking into it**: this
  phase's cases were first named `vt1`..`vt4`, `vt2`/`vt3`/`vt4` were already
  `scheduling.rs`'s, and a case failed intermittently — once in six runs —
  before the names were prefixed. Nothing in the gate or in any review checks
  a name for collision; the only defence is the prefixing convention
  `wiring.rs`, `table.rs` and `ingress.rs` already follow and `scheduling.rs`
  does not. **Not this phase's to fix** — `scheduling.rs` and `wiring.rs` are
  not PHASE-05 surfaces and a rename is a fixture change (S-2). For the audit,
  and a candidate for `docs/memory/`: *a test fixture keyed by a name is a
  namespace, and a namespace with no instrument collides.*
- **F-3 — `wiring.rs`'s `accessible_enabled_of` is now one line of duplication,
  and here is the line.** After D-3's lift it reduces, with **no behaviour
  change**, to:

  ```rust
  fn accessible_enabled_of(window: &PromptWindow, description: &str) -> Option<bool> {
    element_described(window, description).and_then(|element| element.accessible_enabled())
  }
  ```

  It is the same query — `match_inherits("Button")`, the same owned-description
  predicate, the same `find_first` — differing only in the terminal. **The
  duplication is doubled, because it is documented twice:** `wiring.rs:65-73`
  and `harness::element_described` each explain the same filter, for the same
  reason, in different words, so the collapse removes two statements of one
  rule rather than one. **Not taken here, deliberately.** `wiring.rs` is not a
  PHASE-05 surface, no criterion is behind the change, and two phases have
  already touched that file as an undeclared surface — a third with no
  criterion is how a scope holds in letter and dissolves in practice. Audit's,
  and cheap: one line, one import, two doc-comments deleted.

  **The filter itself is held by nothing, and that is measured.** Deleting
  `match_inherits("Button")` from the lifted helper leaves **all 182 cases
  green** — re-measured after the lift, so the claim covers `tree.rs` and
  `fields.rs` both. The markup happens to declare an option's control before
  its field container, which is the declaration order the helper's own doc says
  nothing pins. The filter belongs; what would be false is any suggestion that
  a test holds it. **No case was written to pin it** — PHASE-04's EX-7/F-3
  deleted exactly such a case as vacuous — and the doc now says so with the
  measurement, which is the only instrument this rule has.
- **F-4 — `design.md` §9/AC-1 and §8/R-5 both cite `scheduling.rs:95-125` for
  `logging_scripted`, which this phase moved to `harness.rs`.** Two line cites
  in a document that is a record of intent at a point in time and is not
  retro-fitted (`docs/AGENTS.md`). Recorded so the audit's reading of §8 and §9
  is not slowed by a helper that is not where the design says. Same disposition
  as PHASE-03's F-1: audit's call, not a phase's.
- **F-6 — `docs/memory/cite-requirements-not-finding-ids.md` says *"do not
  extend the practice"*, and the practice has been extended in every slice
  since, this one included.** The memory (settled at slice 001's audit, in
  force from 002) forbids a comment in `src/`, `tests/` or `examples/` citing a
  slice-local review-finding or design-decision id, and tolerates slice 001's
  existing ones by explicit user decision. `grep` finds bare `F-N` citations in
  `src/diagnostics.rs`, `src/controller.rs`, `src/reception.rs`, `src/wire.rs`,
  `tests/renderer/{ingress,startup,tree,wiring}.rs` — and two of them are
  **PHASE-04's**, at `wiring.rs:1224` and `:1229`, both citing PHASE-02's F-6.
  So the rule is either dead or unenforced; nothing in the gate reaches it and
  no reviewer has raised it in four phases. **PHASE-05's own file cites none**
  — every rationale is written out as the claim, which is what the memory's
  last bullet asks for — but that is one file against a codebase, and a rule
  one file follows is not a rule. Audit's: either the memory is amended to
  match what the code does (phase-qualified ids, which do disambiguate), or a
  sweep is scoped as its own slice. Not a phase's to decide, and too large to
  take in passing.
- **F-8 — a phase's own sheet is a home of claims that phase falsifies, and
  nothing sweeps it.** PHASE-04's T-12 established the discipline of sweeping
  for *homes of a claim this phase falsifies* and found seven, all in code,
  each by reading the file being changed. **This sheet was not one of the files
  read.** Its reading list cited `logging_scripted` at `scheduling.rs:101-109`
  and `element_described` at `tree.rs:94-107` — and this phase's EX-2 and D-3
  moved both, so by the time the phase closed, `scheduling.rs:101` was inside
  `request_kind` and `tree.rs:94` was a test function. The two dead cites were
  the two helpers the phase exists to have moved.
  **Found by a check worth keeping:** after the phase closed, every path in the
  commit was matched against the sheet (six of seven claimed; `notes.md` and
  the brief are the sheet itself and its input, so their absence is correct),
  and every `path:line` cite the sheet makes was resolved against the tree.
  That is what turned up the two. Both are now marked *where it was, not where
  it is*, which is the honest form for a reading list — it records what was
  read at phase-plan time — rather than a silent rewrite to today's lines.
  **The class:** VA-1 measures an injection against 182 cases; nothing measures
  the record against the diff. A phase's sweep should include the phase's own
  sheet — and it takes **two** passes, because they catch different things and
  only one of them is mechanical:

  1. **Resolve every `path:line` the sheet cites**, and match every path in the
     diff against the sheet. Mechanical, two commands. Catches this finding.
  2. **Re-read every *decision* the sheet records against what the diff
     actually did.** Not mechanical, and nothing can make it so: a decision
     recorded in prose names no path and no line, so pass 1 walks straight past
     it. A sheet can be internally consistent, cite nothing stale, and still
     record a choice the author changed their mind about an hour later — the
     sheet is written while the decision is being taken, and the decision can
     move afterwards.

  **Both passes belong before the status flips to `done`**, which is the
  operative instruction. Every defect this phase found in its own record was
  found *after* close — cheap to find, and free to have prevented. Strongest
  candidate this phase produced for `docs/memory/`.
- **F-7 — the viewport sizer has now been written three times by three agents,
  and so has the sentence that keeps it honest.** `wiring.rs`'s
  `with_room_for_the_form` (600×600), `mod busy`'s `with_room_for_every_control`
  (400×400) and `fields.rs`'s own each declare a window size so the element
  query can reach the whole form, and each says — independently, in its own
  words — that it **states what the test can see and is not a claim about what
  the window should be.** That sentence is the only thing standing between the
  fixture and a green test quietly closing the product question, which is why
  each agent reached for it unprompted. Three statements of one rule is the
  signal it wants one home. **Not consolidated here**, and the reason is the
  opposite of F-3's: the three differ in their *values* and in *why* — two
  sizes, two measurements, three sets of cases — so a single helper needs a
  parameter and a doc that covers all three, which is a design decision rather
  than a collapse. Audit's, and it should be taken together with the product
  question the sentence protects.
- **F-5 — `plan.md`'s PHASE-05 Surfaces line is complete, and it is the first
  in this slice that is.** PHASE-01's omitted `ui/app.slint`, PHASE-04's
  omitted `draft.rs`; this one names `fields.rs`, `main.rs`, `harness.rs` and
  `scheduling.rs`, and those are exactly the four paths touched. Recorded as
  the negative case for PHASE-04's F-3, so the class is *enumerations go stale*
  rather than *enumerations are always wrong*.

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-09-15 · PHASE-05, done · base commit `69c617b`

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
- The option-scoped query — `within_option`, `field_described` (EX-6) — and
  `fields_in_option`; `element_described` gains a type filter (EX-7). The first
  two landed in `tree.rs` and moved to `harness.rs` in PHASE-04 (its D-3).
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
- `Prepared.draft`, defaulted in `receive`; `absorb`'s three `Shift` arms
  unchanged, as EX-1 predicted (PHASE-04 EX-1).
- `Controller::edit`, `&mut self`, with three refusals that are **structurally
  forced** by ids the renderer can clone and cannot mint (D-1); `answer` still
  `&self` and its signature unchanged. `selected` and `drawn_fields` are the
  one statement each of the shared refusals and of the R-58 walk (EX-2, EX-3,
  D-2).
- `Command::Edit` in `wire.rs` (`Eq` survived), `dispatch`'s arm as
  `.err().map(Err)`, `install.rs`'s **seventh** installation `editing`, and
  `Refused::UnknownField` with the line `design.md` §5.2 states verbatim, which
  `Diagnostics::refused`'s exhaustive match made a compile error to omit
  (EX-4, EX-5, EX-8). `reception.rs`'s
  `every_refused_variant_renders_one_line_with_the_failure_prefix` now covers
  **five of five** — the fourth case EX-9 asked for, and a fifth that makes the
  name true (D-5).
- `glass.rs`'s `option_rows` builds real blocks through `field_block`; `checked`
  is a **lookup** on every present and is never stored in the row model as
  truth. `FieldBlock` names two types in that file, both keeping their names
  (EX-6).
- `field_described` / `within_option` lifted to `harness.rs` (D-3);
  `accessible_enabled_of` gains the type filter PHASE-02's F-4 predicted it
  would need. `wiring.rs` gains `mod editing` — six cases, ten negative
  controls. Gate at **531 tests, from 525**.
- `tests/renderer/fields.rs` — the one new file, and the **only module in this
  target that reads what left the host**. Four cases, all four driving the
  production `serve` against a real child process with `install`'s own callback
  table on the window: every command they put on the channel is fired by
  activating a real element, the tray's `check_now` included, so no case builds
  a `Command`. `logging_scripted` lifted to `harness.rs` (EX-2); `logged` /
  `submitted_values` / `answered_option` stay in `fields.rs` (EX-3);
  `element_described` lifted from `tree.rs` to `harness.rs` (D-3, referred up
  and approved), so the Button-filtered description query has one statement.
  Eleven injections in VA-1. Gate at **535 tests, from 531**.

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
- **An absence that makes a defect impossible also makes the control for it
  impossible, and that is the evidence rather than a gap in it.** The sharpest
  control for D6 would be `answer()` walking the draft instead of the blocks.
  It cannot be written: `Draft`'s field is private and its two methods are
  `state_of` and `record`, so there is no iterator, and adding one is S-1. When
  a design says a rule is "a property of the type rather than a convention",
  *the rejected reading failing to compile* is what that claim looks like when
  it is true. **It is stronger than any test, and the reason is worth saying
  outright: a test can be deleted by someone who does not know why it exists,
  and a type that cannot express the wrong thing cannot be.** A green test says
  the defect is absent today; an absent iterator says the defect has nowhere to
  be written. PHASE-04 T-10.
- **A control on the fixture is not a control on the code, and conflating the
  two is how a product defect gets closed by a green test.** PHASE-04's C-6
  removed VT-5's declared viewport and the whole form vanished from the query —
  which says the fixture is load-bearing, and says **nothing whatever** about
  the draft reaching the screen. Both facts are true at once: the test can see
  the form because it declares 600×600, and the product still clips one at its
  own preferred size. Keeping them apart is the entire reason F-6 exists as a
  separate finding rather than as a line in a fixture's doc-comment — the
  moment they merge, "the tests are green" starts meaning "the window is fine".
  Run the fixture control **as well as** the code control, and write down which
  one each was. PHASE-04 C-6, and PHASE-02's F-6 unmoved by it.
- **Ask whether the defect would still satisfy the *other half* of the
  assertion.** `design.md` §9 warns that a test reading the draft through
  `Controller` is a proxy because D6 would pass it. It does not pass VT-2 or
  VT-3, and the reason is one clause: a walk over the draft's keys cannot
  produce a key for a field **nobody touched**. The edited field's value is the
  half D6 also supplies; the untouched field's presence is the half it cannot.
  PHASE-03's D-3 asked which kind of claim a test makes; this is the same
  question asked clause by clause rather than test by test. PHASE-04 F-2.
- **A criterion's warrant can cite a claim that was already false when the
  criterion was written**, not merely one the phase falsifies. EX-9 rests on a
  test's name claiming every `Refused` variant, and says the extension *keeps*
  that claim; the name had been untrue since before the slice opened, because
  `Refused::Ingress` was never in the case and is asserted nowhere else either.
  Checking a warrant means **reading the thing it cites**, not confirming the
  count it asserts — the count was right, three to four, and the sentence built
  on it was not. PHASE-04 F-1, and PHASE-02's F-3 a second time.
- **A test whose name is its claim is trusted instead of counted, which is what
  makes a false one worse than a false comment.** Nobody enumerates five
  variants against five assertions when the name says "every". The repair is to
  make the name true *and* leave the check one line long — `Refused` declares
  five, the case constructs five — so the next reader can verify in a second
  rather than trust again. PHASE-04 T-15.
- **The refusals a criterion lists may be forced rather than chosen, and that
  changes what a test is worth.** `edit`'s three refusals cannot be omitted:
  the renderer can only clone an id off the retained presentation, so the
  lookup that builds the draft key **is** the lookup that refuses. The naive
  implementation is the correct one, which is why VT-1 arrived green and why
  its controls had to delete a check rather than decline to write one.
  PHASE-04 D-1.
- **A phase's own sheet is a home of claims that phase falsifies, and nothing
  sweeps it.** The discipline of hunting *homes of a claim this phase
  falsifies* has been run for two phases over the **code** being changed, and
  has found eleven. It has never been run over the sheet doing the recording.
  PHASE-05's reading list cited two helpers at the lines they occupied when it
  was written; the phase then moved both, and both cites landed on unrelated
  code by the time it closed.
  **The check has two passes and only the first is mechanical.** One: match
  every path in the diff against the sheet, and resolve every `path:line` the
  sheet cites — two commands. Two: re-read every **decision** the sheet records
  against what the diff actually did, which no command reaches, because a
  decision recorded in prose names no path and no line. The second is the one
  worth writing down precisely because it cannot be automated: a sheet is
  written while a decision is being taken, and the decision can move
  afterwards, leaving a record that is internally consistent and asserts the
  opposite of the diff. **Both passes go before the status flips to `done`.**
  A reading list is a record of what was read, so the repair for a dead cite is
  *where it was, not where it is* rather than a rewrite — but a dead pointer
  nobody marks is a dead pointer. PHASE-05 F-8.
- **A criterion that states its own reasoning is checkable, and checking it has
  paid three times out of three.** PHASE-02's EX-7 justified a type filter by
  two cases it does not keep green; PHASE-04's EX-9 said an extension *kept* a
  test name's claim when the name's claim had been false since before the slice
  opened; PHASE-05's EX-2 pointed at an enumeration PHASE-04 had already
  replaced with a rule. **In all three the criterion was still right**, and in
  all three the check was the same move: read the thing the warrant cites
  rather than confirm the count it asserts. The yield was two vacuous tests not
  written, one false test name repaired, and one doc repair not undone. A
  vaguer plan — *"lift the helper"* — would have produced none of them, and
  each phase would have complied with the letter while nobody noticed. So this
  is an argument for the plan's style rather than three defects in it, and the
  cost is one task per phase: `plan.md:36-45` already asks each phase to
  re-derive its ids; the half worth adding is *then read what each warrant
  cites*.
- **A test fixture keyed by a name is a namespace, and a namespace with no
  instrument collides.** `scripting::marker` turns a case name into
  `goad-invocations-<name>-<pid>` and **clears the file**; every case in a test
  target shares one pid and cargo runs them concurrently. Two cases given the
  same name share one log and truncate each other, and the symptom is an
  intermittent failure in whichever one asserts on it — once in six runs, here.
  `scheduling.rs:415` and `wiring.rs:1570` have both been `scripted("vt8", …)`
  since before this slice. Nothing in the gate, and no reviewer reading one
  file, can see it. PHASE-05 F-2.
- **A green case is not the same as a pinned mechanism, and a filter is the
  usual place the two come apart.** `option_control`'s `match_inherits("Button")`
  can be deleted with all 182 cases still green: the markup happens to declare
  an option's control before its field container, which is the declaration
  order the helper's own doc says nothing pins. Three files now carry this
  filter and three docs justify it; nothing would notice all three losing it.
  The repair is **not** a case written to pin it — PHASE-04's EX-7/F-3 deleted
  exactly such a case as vacuous — it is saying so in the doc, with the
  measurement. Two of the three are now one after PHASE-05's lift; the third,
  `wiring.rs::accessible_enabled_of`, collapses to a single line over it.
  PHASE-05 F-3.
- **A workspace lint can shape a test file's structure, and the shape it forces
  can be the better one.** `future_not_send` is `deny` workspace-wide and
  reaches an `-> impl Future` return exactly as it reaches an `async fn`, so
  **no async helper in this target may hold a component handle**. A
  `#[tokio::test]` body is not async after expansion, so the lint reaches
  helpers and never cases. The answer is macros rather than async fns —
  `driving!`, `tick!` — plus a plain struct for the setup; and it is also the
  better answer for *"the production `serve`, no second loop"*, because the
  `serve` call ends up textually inside each case. The alternative was the
  workspace's first `#[expect(clippy::future_not_send)]`, which is a
  concession and not a phase's to take. PHASE-05 D-1.
- **A criterion can name an injection its own case cannot be reddened by.**
  VA-1 requires that ignoring the draft key's option half turns VT-2 red; VT-2
  as written ticks and answers the *same* option, and under that injection it
  stays **green**, because one option's answer cannot tell the two readings
  apart. The fix is one more tick — the shared id under the option that is
  **not** answered. Reading a criterion's verification clause against its own
  test-shaping clause, before writing either, is what catches this. PHASE-05
  D-2.
- **The proxy rule can be checked, not just obeyed, and it costs two runs per
  case.** Assert the screen before the wire in the case body, then run the
  control that breaks each: the assertion that *fires* names which half caught
  it, and the assertion that *passed* is the half that would have been green
  alone. VT-3's conjunction is measured in both directions this way — green
  with the screen wrong, green with the wire wrong — rather than argued from
  the design. A disjunction anywhere else in the file is then a claim about
  that case, not a habit. PHASE-05 T-5.
- **The vocabulary scan does not reach `tests/` at all** — `Scan.excluded_dirs`
  is `["tests", "target"]` and skips any directory of that name
  (`goad-boundary/src/scan.rs:128-132`). PHASE-03's F-3, that the scan cuts
  comments but not string literals, is about `src/`; a case file's fixture ids,
  test names and assertion messages are held by **review alone**. Both halves
  matter and they are different claims: in `src/` the scan is sharper than a
  reader expects, and in `tests/` it is absent.
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
- **PHASE-02's F-4 — closed.** `wiring.rs`'s `accessible_enabled_of` has the
  type filter, and its doc says why. Listed here so an auditor reading the
  PHASE-02 findings does not go looking for it.
- **F-6 is unchanged and was re-measured.** PHASE-04's C-6 removed its own
  case's declared viewport and the form vanished from the query
  (`no control described "read" under "morning"`). The tests see a form because
  two fixtures declare a viewport; the product still clips one at its preferred
  size. **PHASE-06/VH-1 and VH-2.**
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
- **`Draft`'s three absences held through PHASE-05 and are still load-bearing.**
  No `BTreeMap`, no enumeration, no `PartialEq`. PHASE-05 wanted none of them
  either, and could not have used one: it never holds a `Controller`, and every
  value it asserts came off the wire or off the window. **PHASE-06 inherits the
  same three.**
- **PHASE-04 F-1 — closed, not open.** EX-9's warrant was false and the
  criterion right anyway; the `Refused::Ingress` case went in by decision and
  the test's name is now true. Listed here only so an auditor reading the
  findings does not go looking for it.
- **PHASE-04 F-2 — `design.md` §9's closing disjunction and its own AC-8 row
  disagree**, and PHASE-04's VT-2 / VT-3 sit exactly where they meet. Audit's:
  either §9's rule gains R-58's exception, or AC-8's row moves. No code is
  wrong either way.
- **PHASE-04 F-3 — a Surfaces line is an enumeration, and every enumeration in
  this slice has been a floor.** PHASE-04's omits `draft.rs`, which EX-3 sends
  the phase to edit; PHASE-01's omitted `ui/app.slint`. Third occurrence.
  Candidate for `docs/memory/` as a class, not as two instances.
- **PHASE-04 F-4 — `canon-delta.md`'s *The gap* says `controller.rs:216` "sends
  an empty map".** It no longer does, and `:216` no longer holds that line. Read
  as history it stands; read as a claim about today it does not. Audit's, at
  promotion — a phase does not edit draft canon.
- **PHASE-04 D-3 — `harness.rs` and `tree.rs` were touched and are not PHASE-04
  surfaces.** The two option-scoped query helpers moved there rather than being
  copied into `wiring.rs`. Referred up during execution; for the audit's path
  diff, so an undeclared path is a decision on the record rather than a lead.
- **F-6 was re-measured a third time and is unchanged.** PHASE-05's C-6 removed
  `fields.rs`'s declared viewport and all four cases failed
  `no control described "read" under "morning"` — the same message PHASE-04's
  C-6 produced. Three fixtures in two files now declare a viewport; the product
  still clips its own second option at its preferred size. **PHASE-06/VH-1 and
  VH-2**, and `design.md` §8/R-7's real observation.
- **PHASE-05 F-2 — two existing cases share one invocation log.**
  `scheduling.rs:415` and `wiring.rs:1570` are both `scripted("vt8", …)`, in
  one binary, so one pid and one temp path; `marker` clears it on handout and
  `scheduling.rs`'s case asserts `invocations(&log) == 1`. Pre-existing, not a
  PHASE-05 surface, and a fixture change (S-2). Audit's: rename one, and decide
  whether the prefixing convention wants an instrument. Candidate for
  `docs/memory/`.
- **PHASE-05 F-3 — one line of the Button-filter duplication is left, and the
  line is written down.** `element_described` is lifted into `harness.rs` and
  `tree.rs` and `fields.rs` both use it (D-3, approved). What remains is
  `wiring.rs:65-84`'s `accessible_enabled_of`, which collapses with no
  behaviour change to
  `element_described(window, description).and_then(|e| e.accessible_enabled())`
  — removing **two** statements of the rule, because both functions carry their
  own doc explaining the same filter. Not taken: `wiring.rs` is not a PHASE-05
  surface and no criterion is behind it. Audit's, and cheap.
  **The filter is held by nothing**, re-measured after the lift: deleting it
  leaves all 182 cases green, in both callers.
- **PHASE-05 F-7 — the viewport sizer is written three times by three agents,
  and so is the sentence that keeps it honest.** Consolidating them needs a
  parameter and a doc covering two sizes and three sets of cases, so it is a
  design decision rather than F-3's collapse. Audit's, together with the
  product question that sentence protects.
- **PHASE-05 F-4 — `design.md` §9/AC-1 and §8/R-5 both cite
  `scheduling.rs:95-125` for `logging_scripted`**, which EX-2 moved to
  `harness.rs`. Two line cites in a record of intent, not claims about
  behaviour. Same disposition as PHASE-03's F-1: audit's call.
- **PHASE-05 F-5 — the first complete Surfaces line in this slice.**
  PHASE-05's names `fields.rs`, `main.rs`, `harness.rs` and `scheduling.rs`,
  and those are exactly the four paths touched. Listed as the negative case for
  PHASE-04's F-3, so the class the audit holds is *enumerations go stale*
  rather than *enumerations are always wrong*.
- **PHASE-05 D-1 — no `src/` file was touched.** The gate's 535 tests include
  four new cases and no production change; the diff is one new test file and
  three test files. For the audit's path diff.
- **`design.md` §8/R-8 gained the test its mitigation rests on.** The row's
  bound is *`view: null` folds to `Shift::Retained` and leaves the draft
  alone*; VT-3 is now that sentence's pin, and C-4 shows it is the only case in
  the workspace holding it. The **risk** is unchanged and still accepted by
  D13 — what changed is that the mitigation's premise is now checked.

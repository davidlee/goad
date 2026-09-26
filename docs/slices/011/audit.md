# Audit & reconciliation — Slice 011

Written after the last phase is done. Two jobs in one document:

1. **Audit** — does the work match its design, its acceptance criteria, and
   canon? Every gap dispositioned, none left implicit.
2. **Reconcile** — make the record true again. The code is what shipped; the
   specs must say so, or the code must change.

## Brief

**Subject:** `d2617c1..fdc2229` on `main` — `d2617c1` is the accepted plan,
`fdc2229` the hand-over to audit (PHASE-03 done, VH-1 run by the user). The code
under audit is PHASE-01 (`94251b6`, `485980b`), PHASE-02 (`ccc9d61`, `f564c29`,
`0533411`) and PHASE-03 (`64f75d4`). Written before reading the code or the diff.

**Question:** the slice is finished when all of these hold.

1. **The gate is green on the tree as committed** — `just check` run here,
   not taken from `notes.md`.
2. **Every AC in `slice-011.md` is met by something in the tree, named by
   symbol**, or reported as pending with who owes it: AC-5 (canon promotion)
   and AC-7 (FU-2 strike) are audit/close work; AC-6 is the user's run.
3. **Every EX/VT/VA/VH in `plan.md` is discharged against the tree** — each
   phase sheet's claim checked by reading the named symbol or re-running the
   command, and marked *re-verified* or *taken from `notes.md`*. PHASE-03 is
   audited against its criteria as the user narrowed them (`plan-log.md`).
4. **The mutation evidence is real** — at least two of the design §9 controls
   the orchestrator did not re-run (it re-ran M0 and M8b) are re-run here,
   each confirmed to compile and to go red on the case that claims it.
5. **Nothing was touched that no phase declared** — the diff's paths against
   each phase's Surfaces; every undeclared path named.
6. **The invariants hold at the new code.**
   - CLAUDE.md's five: no domain vocabulary in anything new; wire parsing
     untouched (nothing crosses the backend boundary); no protocol capability
     narrowed; a backend failure still never stops the host or leaves it
     unable to invoke the backend again — here, a flood of refusals must not
     starve a command, a firing or the ingress-stopped report; and
     `goad-semantics` untouched (ADR-001 — no timer or clock enters stratum 1).
   - The design's own: **I-1** consecutive refusal-arm firings at least `I`
     apart; **I-2** a refusal decided while idle reaches the surface within `I`
     unless the loop ends or it is overwritten; **I-3** only the
     `Fired::Ingested`/`None` path and the new arm change behaviour; **I-4**
     `floor_until`, `event_floor_until` and `sleep` keep their one write site
     each, and the new arm writes none of them (SPEC-002/R-4, R-12, ADR-004).
   - **R-15's amended guarantee** (`canon-delta.md`) — attacked at its
     exceptions: that every `continue`/`break` in the new `'idle` region
     targets the label it must (design R2), that the leading edge is not a
     debounce and not moved by a person's own present (D5), that nothing
     other than `ingest`'s `None` is coalesced (D6), and that a command during
     a coalesced interval presents at once with the stale fold.
7. **Tests assert the property, not a proxy** — the timed cases read the
   window, not the retained model (D12, M0), and their margins are stated
   against the direction load moves them.
8. **The record can be made true** — every divergence between code and
   `design.md`/canon classified; `canon-delta.md`'s changes drafted as
   Reconciliation rows for the user's endorsement. Canon is not edited in this
   session.

## Evidence

<!-- What was run and what it said. Not a claim of correctness — the basis for
     one. -->

Provenance is marked on every line: **re-verified** means run or read by the
auditor on `73cce4e` (the Brief commit; code identical to `fdc2229`);
**from notes** means taken from `notes.md` without re-running.

### Tests / checks

- **`just check`** — re-verified: exits 0. Build, both test tiers (renderer
  target 228 passed, the rest green), the example typecheck, `cargo clippy
  --workspace --all-targets -- -D warnings`, `cargo fmt --all --check`.
- **Scoped ingress run** — re-verified: `cargo test -p goad --test renderer
  --no-fail-fast -- ingress::` green (15 passed) before and after the
  mutations below.
- **Mutations re-run by the auditor** (none of them re-run by the
  orchestrator, which re-ran M0 and M8b). Each applied to `controller.rs`
  from a scratchpad copy, compiled, run with the scoped command, restored
  from the copy; `md5sum` matched the original and `git diff --stat` was
  empty after each.

  | id | edit | result |
  |---|---|---|
  | M7 | `Fired::Command(command) => { let Some(resolved) = dispatch(..) else { continue 'idle; }; Some(resolved) }` | compiled; **only** T4 red, on its own assertion: "the command presents at once and carries B: 1.012077393s after sent" (notes: 1.014 s). |
  | R2 | the outer ingress-stopped fold's `continue 'serving;` → `continue 'idle;` | compiled; **only** VT-7 red, on assertion 3: "the fold reaches the window at once, not after a scheduled firing: 3.000925261s after spawn" (notes: 3.0019 s). |
  | AP-1 (auditor's probe, not in the plan) | `let mut surface_stale = false;` hoisted from before `'idle` to before `'serving` — D7's rejected shape without its clear, so the arm keeps firing once per interval after the first refusal | compiled; T3 red on R2's bound ("755.166866ms after sent"). D7 is held by a test after all, through T3, not only by review. |

- **Case-name resolution** — re-verified: every `ingress::` name cited by
  `canon-delta.md` Changes 3–5 and every case name in `design.md` §9 appears
  in `cargo test -p goad --test renderer -- --list`, except the *from* text
  the delta replaces (expected). The count-rule citation
  `ingress::the_reason_token_set_is_closed_at_eight` resolves in
  `crates/goad-shell/tests/integration/ingress.rs`.
- **AC-4 diff** — re-verified independently of notes: every `#[tokio::test]`
  body in `ingress.rs` at `d2617c1` compared with HEAD by name. Added: T2, T4,
  and T1/T3 under their new names. Removed: the two old names. Changed: VT-7
  (`a_dead_accept_task_…`) and PHASE-05/VT-6
  (`after_a_flood_of_malformed_envelopes_the_host_still_evaluates`) — the
  latter is the `flat_out` call respelled and its reply loop reading
  `record.reply`, with the same assertion; sanctioned by PHASE-01/EX-4
  ("keep their assertions, however their call is spelled"). No other case
  changed.
- **I-4 write sites** — re-verified by grep and reading `serve`:
  `floor_until` is assigned only in the `&mut sleep` arm; `event_floor_until`
  only in `ingest`; `next_refusal_present` is reset only in its own arm;
  `surface_stale = true` only in the `Fired::Ingested`/`None` branch.
- **ADR-001** — re-verified: the slice's diff touches no path under
  `crates/goad-semantics`; the only code paths are `crates/goad/src/controller.rs`
  and `crates/goad/tests/renderer/ingress.rs`. The domain-vocabulary scan is in
  the green gate (PHASE-02/VA-2 from notes).

### Acceptance criteria

| AC | state | discharged by | provenance |
|---|---|---|---|
| AC-1 | met | `research.md` Thread 3; `design-log.md` OQ-1 | from the design record |
| AC-2 | met | T2 `a_flood_of_refusals_updates_the_window_once_per_interval_with_the_latest` (a): M1 reds it at 56352 presents against a ceiling of 4; M9 at 600 ms reds it 10/10 at rest. Leading edge: T3 `a_too_soon_refusal_decided_while_idle_reaches_the_window_at_once` R1, redded by M5. | T2/T3 green re-verified; M1, M5, M9 from notes |
| AC-3 | met, with a reach note | T2(c), T3, T4 read `RecordingGlass`'s log, which reads the window's `diagnostic_lines` and `mode` after `SlintGlass::present` returns; M0 reds all three (from notes; re-run by the orchestrator). **Reach:** the log reads the window's *properties*, not rendered pixels. T2 runs in prompt mode, where the diagnostics pane is not displayed; T3's R2 and T4 assert `WindowMode::Diagnostic`. That is D12's stated choice, and AC-6 is the pixel-level witness. | green re-verified; M0 from notes |
| AC-4 | met | The AC-4 diff above; `scheduling.rs`, `wiring.rs`, `event_loop*` untouched and green. T4 (a command presents at once) held by M7; VT-7's added assertion held by R2 — **both re-run here**. An accepted arrival, a scheduled firing and a refusal during an exchange: unchanged paths (I-3, PHASE-02/VA-1 by reading, re-read here). | re-verified |
| AC-5 | **pending** — owed at this audit | Canon-delta Changes 1–5 are drafted as Reconciliation rows below; promotion awaits the user's endorsement. | — |
| AC-6 | met | The user's run, PHASE-03/VH-1 record in `notes.md`: run on 2026-09-26 against `64f75d4` (the code is identical at HEAD — later commits touch `notes.md` and `audit.md` only), following the seven hand-over steps, step 6 loosened to "the last key printed, or a few above it". The user's words: *"I ran that, it was all exactly as expected."* — one statement covering typing held, the pane opening promptly, pane and tooltip updating about once a second with rising `flood-<n>` keys, and the pane settling after Ctrl-C. | from notes (the user's own report) |
| AC-7 | **pending** — close | `docs/follow-ups.md` FU-2 strike with the restated *Dead when*. | — |

### Verification criteria (`plan.md`)

PHASE-03 is audited against its criteria **as the user narrowed them**
(`plan-log.md`, *load testing de-emphasised*; the PHASE-03 sheet's narrowing
table): EN-2, EX-4 and VA-2 dropped; EX-2 and EX-3 at rest only.

| criterion | state | evidence | provenance |
|---|---|---|---|
| 01/EN-1 | met | plan accepted at `d2617c1` (`plan-log.md`) | record |
| 01/EX-1 | met | `just check` green now | re-verified |
| 01/EX-2 | met | `RecordingGlass` delegates first, then reads `get_diagnostic_lines`/`get_mode`, stamps `Instant::now()`; holds `window.clone_strong()`; `Rc<RefCell<Vec<Presented>>>`; `CountingGlass` absent from the tree | re-verified by reading |
| 01/EX-3, VA-1 | met | M0 reds T3 and VT-7 (and, as an observation, `ingress_stopping_during_an_exchange_…`) | from notes; orchestrator re-ran |
| 01/EX-4 | met | `Timed { sent, reply, replied }`, `sent` before `connect`; one writer primitive `write_one`; `flat_out` takes `impl Fn(usize) -> String` | re-verified by reading |
| 01/EX-5 | met | AC-4 diff above | re-verified |
| 01/EX-6 | met, doc stale | the mirror exists; its doc still narrates PHASE-01 ("today's loop still presents every refusal at once") — finding A-2 | re-verified |
| 01/VT-1 | met | T3 green, asserts `at − sent ≤ I/2` for R1 and R2, R2 in `Diagnostic` | re-verified |
| 01/VT-2 | met | T1 keeps assertions 1 and 2; assertion 3, its wait and the counting glass gone; doc makes no per-refusal claim | re-verified (diff) |
| 01/VT-3 | met | VT-7 assertion 2 counts the log; assertion 3 `lag < MINIMUM_SPACING / 2` from `spawned` | re-verified; R2 re-run |
| 02/EN-1 | met | PHASE-01 done at `485980b` | record |
| 02/EX-1 | met | gate green | re-verified |
| 02/EX-2 | met | `serve` matches `design.md` §5.2: private constant with its doc; `next_refusal_present = Box::pin(sleep_until(started))`; `surface_stale` declared immediately before `'idle`; arm after `sleep`, directly above `ingress.arrival()`, `if surface_stale`, builds no `Fired`, `continue 'serving`; every `continue`/`break` inside `'idle` labelled | re-verified by reading |
| 02/EX-3 | met | I-4 above | re-verified |
| 02/EX-4 | met, one comment wrong | the four listed comments are rewritten and true, but the new comment on `surface_stale` misstates why it is never cleared — finding A-1 | re-verified by reading |
| 02/EX-5 | met | every M-row compiled and redded its named case; M8 on R1 not R2 (decided, `design-log.md`), M8b added and redding R2; M9 5/5 at rest | from notes; M7 and R2 re-run here |
| 02/EX-6 | met | AC-4 diff above | re-verified |
| 02/VT-1 | met | T2 green; red on today's loop on (a) | green re-verified; red from notes |
| 02/VT-2 | met | T4 green; red on today's loop on the precondition; M7 its evidence | green and M7 re-verified |
| 02/VT-3 | met | T1, T3, VT-7 green on the new loop | re-verified |
| 02/VA-1 | met | D4, D8, D6, I-3, I-4, R3 each confirmed by symbol in notes; D4, D6, D8, I-3 re-read here against `serve` and agree | re-verified by reading (R3 from notes) |
| 02/VA-2 | met | `site` appears in `controller.rs`'s new text only in comments; scan in the green gate | gate re-verified; read from notes |
| 03/EN-1 | met | PHASE-02 done at `0533411` | record |
| 03/EN-2 | dropped by the user | no load generated | narrowing |
| 03/EX-1 | met | gate green | re-verified |
| 03/EX-2 | met at rest | ten runs of the whole renderer target; every quantity's worst value and unused margin tabulated; tightest (T4 precondition, `I/4`) worst 0.86 ms | from notes |
| 03/EX-3 | met at rest | M9 10/10 red, "6 presents against a ceiling of 5", per-firing lag 0.1–1.9 ms | from notes |
| 03/EX-4 | dropped by the user | — | narrowing |
| 03/EX-5 | met | case names resolve | re-verified |
| 03/EX-6 | met | `flood.py` standard library only (`json`, `socket`, `sys`, `threading`, `time`); header says what it is for and that the gate does not run it; smoke-checked against a stand-in listener | imports and header re-verified; smoke from notes; the real host's replies seen at VH-1 |
| 03/EX-7 | met | §Open carries FU-3's citation, the memory re-check, the M8 drift | re-verified |
| 03/VA-1 | met at rest | as EX-2 | from notes |
| 03/VA-2 | dropped by the user | — | narrowing |
| 03/VH-1 | met | AC-6 above | from notes |

**Not reached by any phase, held by review:** the yield per arrival (R3),
D4's ordering (moving the arm below `ingress.arrival()` would red nothing
today), D8 (no present on the `Ending` arms), and the clock-overflow fallback
(`unwrap_or(now)`), which no test can drive.

### Surface delta

`git diff --stat d2617c1 fdc2229`, by commit, against each phase's declared
Surfaces.

| commit | paths | declared by | verdict |
|---|---|---|---|
| `94251b6`, `485980b` (PHASE-01) | `ingress.rs`, `notes.md` | PHASE-01 | declared |
| `ccc9d61`, `f564c29`, `0533411` (PHASE-02) | `controller.rs`, `ingress.rs`, `notes.md`, `design-log.md` | PHASE-02; `design-log.md` widened mid-phase by the orchestrator for the M8/M8b entry, recorded in the sheet | declared |
| `64f75d4`, `fdc2229` (PHASE-03) | `flood.py` (new), `notes.md` | PHASE-03 | declared |
| `9d3ce6f`, `9e9c943`, `079dd95`, `ebbbd0d` | `slice-011.md` (stage line), `plan-log.md`, `notes.md` | orchestrator bookkeeping between phases | expected |
| **`45e2ba4`** | `docs/follow-ups.md`, `docs/roadmap.md` | **no phase** | **undeclared, but not slice work**: "roadmap: the backend author's kit moves up to 012" renumbers FU-7, FU-35, FU-36's scheduled slices. It lands inside the range because it was committed after the plan. It touches neither FU-2 nor FU-3. Not a finding. |

**Declared but untouched:** `crates/goad/src/glass.rs` (mutation only —
correctly absent from the diff); `design.md` §9 and `canon-delta.md` (edit
only on a case rename — none happened, EX-5). Both as intended.

### Audit findings

Code or comment defects for the orchestrator, not fixed here. None changes
behaviour; all are in comments. The code review (`review-code.md`) may raise
the same; dedupe there.

- **A-1 — the `surface_stale` comment's reason is false.** In `serve`, the
  comment above `let mut surface_stale = false;` says "the only thing that ends
  the wait with the surface still stale is the arm below". A command, a
  scheduled firing, an accepted arrival and the ingress-stopped fold all leave
  `'idle` with `surface_stale` still `true`. The true reason is `design.md`
  §5.3's: every exit from `'idle` either presents (at the top or at the engage
  present) or ends the loop. *Code wrong (comment).*
- **A-2 — phase-time narrative left in `ingress.rs` doc comments.** The
  `REFUSAL_PRESENT_INTERVAL` mirror ("mirrored ahead of PHASE-02 … while
  today's loop still presents every refusal at once … the throttle this
  constant will then gate"); T3's doc ("Green on today's loop … nothing
  throttles a refusal's presentation yet … once the coalescing loop exists");
  `flat_out`'s ("PHASE-02's coalescing loop needs … this phase's own two").
  Each is false of the tree that shipped. T3's doc also names M5, M6 and M8 as
  its controls but not M8b, which is the one that holds R2's bound (D5).
  *Code wrong (comments).*
- **A-3 — bare citations resolve to slice 004's documents.** By this file's
  and `controller.rs`'s convention, a bare `design.md`/`plan.md`/`plan-log.md`
  is slice 004's (`plan.md` PHASE-01 notes). The new T2 and T4 docs cite
  "`plan-log.md` PL-6" — slice 004's PL-6 is *the startup decision is a named
  function*, not the pin; also bare "`design.md` §9 T2", "`plan.md` VT-1". T1's
  doc cites "slice 011 `design.md` VT-2", which is a `plan.md` id. In
  `controller.rs`, the new comments cite bare `design.md` §5.2, §5.3, D4, D6,
  D7, I-3 beside older bare citations of slice 004's §5.2/§5.4 — only
  `REFUSAL_PRESENT_INTERVAL`'s doc says `docs/slices/011/`. *Code wrong
  (comments); fix the class across both files.*


## Code review

Findings live in `review-code.md`, copied from
`docs/templates/review-ledger.md` — same ledger, same severity and disposition
vocabulary, subject `implementation`. Do not restate findings here.

- **Ledger:** [`review-code.md`](review-code.md) — written by a separate
  reviewer in parallel with this audit.
- **State:** open · outstanding blockers: pending (the ledger is not yet
  committed).

## Verdict

<!-- The slice's closure story, written once, here. Draws on the ledger's
     synthesis and on the evidence above; restates neither. Does this slice do
     what it set out to do, and what is being accepted knowingly? -->

## Reconciliation

<!-- Making the record true. One row per document that must change, and the
     change itself. Amending canon requires explicit user endorsement — ask
     before writing, not after. -->

| document | change | reason | done |
|----------|--------|--------|------|
| `specs/NNN-…md §4` | | code diverged at `path:line`; code is right | [ ] |
| `draft-spec.md` → `specs/NNN-slug.md` | promote | drafted during this slice | [ ] |

**Design drift not reconciled:** <where the implementation departs from
`design.md` and the design was left as-is, with the reason. The design is a
record of intent at a point in time; it is not retro-fitted to the code
without saying so.>

## Closure

- [ ] All findings dispositioned; no blockers outstanding
- [ ] All acceptance criteria met, or explicitly waived by the user
- [ ] Each verification criterion in `plan.md` walked against the code, or the gap measured and carried
- [ ] Tests and checks green
- [ ] Specs / policy / ADRs reconciled, with user endorsement where amended
- [ ] `draft-spec.md` / `canon-delta.md` promoted, or abandoned with the reason written down
- [ ] `notes.md` §Open swept against `slice-nnn.md` §Follow-ups; every entry dispositioned
- [ ] `slice-nnn.md` Summary and Follow-ups written
- [ ] `notes.md` Harvest current; durable facts lifted to `docs/memory/`
- [ ] `slice-nnn.md` stage set to `done`

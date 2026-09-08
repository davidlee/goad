# Plan log — Slice 004

Append-only working record for the plan stage. Survives compaction and
interruption; `plan.md` itself stays clean. Never rewrite an entry — supersede
it with a later one.

Decisions here are taken under the **standing autonomy grant** (`design-log.md`,
2026-09-08 *Autonomy grant for 004*: **decide everything except canon**), and
are recorded as a user's would be: Asked / Decided / Why / Rejected /
Consequence. **Plan acceptance is reserved to the user** and is not one of
these. Findings from an adversarial review of the plan would live in
`review-plan.md`, not here. Design-shaped decisions taken while planning are
cross-posted to `design-log.md`.

## Decisions

### 2026-09-08 — PL-1: seven phases, split 01 / 02 / 03 / 04 / 05 / 06 / 07

- **Asked:** how many phases, and where the seams fall. The slice has one
  unmeasured assumption, two stratum 2 modules, one stratum 3 mechanism, roughly
  thirty tests across three tiers, a startup change, an environment change and a
  human run.
- **Decided:** seven. 01 the A-1 probe alone; 02 the config key and the
  envelope; 03 the listener against a fake judge; 04 `serve`'s arms, the second
  anchor and the well-behaved renderer cases; 05 AC-6's three anchor cases, both
  sides of R-15 and the malformed flood; 06 startup, the environment and the
  human run; 07 the sweep and the gate.
- **Why:** the seams fall where the **fixtures** change, not where the criteria
  do. PHASE-03's cases all need a socket and a fake judge and no `serve`;
  PHASE-04's all need a scripted backend on the happy path; PHASE-05's each need
  a backend whose `next_check` sequence is part of the argument, or an exchange
  deliberately held open. Slice 003's PHASE-02 — one `select!` arm, a 22-site
  migration and six timed `serve` tests — is the calibration for one session,
  and no phase here exceeds it.
- **Rejected:** *(a)* folding the probe into PHASE-02 — its stated failure mode
  is *the design was wrong*, and a phase that may end that way must not carry
  production code that would be thrown away with it. *(b)* One stratum 3 phase
  covering PHASE-04 and PHASE-05 — that is a mechanism plus a 23-site migration
  plus eleven timed tests, well past the calibration. *(c)* Splitting PHASE-04's
  mechanism from its own tests — it lands a `select!` body nothing drives, which
  is the failure `docs/memory/expect-dead-code-ahead-of-caller-needs-cfg-attr.md`
  records and which slice 003's PL-1 rejected for the same reason.
- **Consequence:** 01 → 02 → 03 → 04 → 05 is a dependency chain; 06 needs 02, 03
  and 04; 07 needs everything. No two phases run in parallel.

### 2026-09-08 — PL-2: the listener's cases are a module of the existing `integration` target

- **Asked:** whether the stratum 2 ingress cases get a new `[[test]]` target or a
  module in `crates/goad-shell/tests/integration/`.
- **Decided:** a module — `crates/goad-shell/tests/integration/ingress.rs`,
  reached by one `mod ingress;` line in `integration/main.rs`. Its socket
  helpers and its fake judge stay in that file.
- **Why:** `design.md` §9 names that path in terms. A new target would need a
  manifest entry (`autotests = false`), and it would then have to satisfy
  `dead_code` over whatever `#[path]` helpers it included — the cost slice 003's
  FD-3 spent a whole decision on. `integration/harness.rs`'s own rule is *two or
  more consumers in this target live here, one consumer stays where it is*, and
  the socket helpers have one consumer.
- **Rejected:** a new `[[test]]` target; putting the helpers in `harness.rs`
  speculatively.

### 2026-09-08 — PL-3: temp sockets from `std::env::temp_dir()`, not the `tempfile` crate

- **Asked:** `design.md` §9 says the listener is tested "over a real socket in a
  tempdir". There is no `tempfile` dependency in the workspace, and
  `crates/goad-shell/Cargo.toml` has no `[dev-dependencies]` table at all.
- **Decided:** each case builds its own directory under `std::env::temp_dir()`
  from the case name and `std::process::id()`, creates it, binds inside it, and
  removes it. No new dependency.
- **Why:** adding `tempfile` means adding a name to the manifest allowlist's
  `STRATUM_2` (`crates/goad-boundary/tests/checks/allowlist.rs:19-27`), which is
  **one of the four ADR-001 instruments** POL-001 §Verification enumerates.
  Widening an instrument for a test convenience is the wrong trade, and the
  workspace already has the precedent twice:
  `crates/goad-shell/src/config.rs:226` and
  `tests/support/scripting.rs::marker`. Also
  `docs/memory/cargo-test-cwd-is-package-root-not-workspace-root.md` — every
  socket path has to be absolute anyway.
- **Rejected:** `tempfile`; a socket under the checkout (parallel cases would
  collide and `git status` would carry it).
- **Consequence:** recorded in `plan.md` as FD-2, so a phase agent reading §9
  alone does not reach for the crate.

### 2026-09-08 — PL-4: `net` **and** `sync` at `goad-shell`'s own manifest entry

- **Asked:** `design.md` §10 says "`tokio` gains `net`". The stratum 2 interface
  is built on `mpsc` and `oneshot`, and `goad-shell` has neither `net` nor
  `sync` today — it takes the workspace base set
  (`["process","time","rt","io-util","macros"]`) unmodified.
- **Decided:** `crates/goad-shell/Cargo.toml` becomes
  `tokio = { workspace = true, features = ["net", "sync"] }`. The workspace base
  set is not widened.
- **Why:** it mirrors what `crates/goad/Cargo.toml` already does with
  `rt-multi-thread` and `sync` — features are added at the member that needs
  them, so the base set stays the smallest thing every member wants. The
  POL-001 argument §10 makes for `net` holds verbatim for `sync`: tokio is not
  in `goad-semantics`'s graph (`jiff`, `serde`, `serde_json` only), so neither
  feature can unify into stratum 1, and the manifest allowlist is names-only so
  neither is visible to it.
- **Rejected:** widening `[workspace.dependencies]`'s base set — it would hand
  both features to `crates/goad` and to any future member for no reason.
- **Consequence:** recorded in `plan.md` as FD-1. §10's row is right about the
  conclusion and short by one feature name; squaring the document with what
  shipped is the audit's reconciliation, not a phase's.

### 2026-09-08 — PL-5: `ingress/` carries the arithmetic deny

- **Asked:** whether `crates/goad-shell/src/ingress/`'s two files carry a
  module-level `#![deny(clippy::arithmetic_side_effects)]`.
- **Decided:** yes, both.
- **Why:** D53's rule as amended is that the lint follows the **data**, not the
  directory. This module computes over byte counts and durations an untrusted
  writer paces — `ENVELOPE_LIMIT`'s accounting and `retry_after_ms`'s rounding —
  which is exactly the case `SPEC-001/R-46` names. `error.rs` and `config.rs`
  carry no such deny for the opposite reason, stated in their own doc comments.
- **Consequence:** `retry_after_ms`'s rounding cannot use `+` or `/`. `plan.md`
  PHASE-03's notes give the shape that is legal:
  `checked_add(Duration::from_nanos(999_999))` then `as_millis()`, narrowed with
  `u64::try_from` — `as_conversions` and the four `cast_*` lints are all `deny`,
  so there is no cast available either.

### 2026-09-08 — PL-6: the startup decision is a named function, so AC-7 is a test

- **Asked:** `design.md` §9's AC-7 row asks for *"with no key configured, no file
  is created at any path"*, and names no instrument. As the design sketches it,
  the decision is a four-line `match` inside `main::start`, which no test can
  reach.
- **Decided:** `crates/goad/src/startup.rs` carries
  `pub fn listener(configured: Option<&IngressConfig>) -> Result<Ingress,
  StartupError>`, and `main::start` calls it.
- **Why:** it stays inside the surface `design.md` §5.1's part table gives
  `startup`/`main` — *"binding before the loop starts, and the exit code when it
  cannot"* — and it is the same shape `startup.rs` already houses and tests in
  `arguments`: a pure-over-its-input startup decision, with the table of its
  behaviour written as a test rather than as a claim. Without it AC-7's second
  half is discharged by review of a `match` nobody can call.
- **Rejected:** *(a)* leaving the `match` inline and discharging AC-7's second
  half by review — a criterion the slice card writes as a behaviour deserves an
  instrument. *(b)* Putting the decision in stratum 2 as
  `ingress::from_config(Option<&IngressConfig>)` — it adds a public function the
  design does not name, in the crate whose module the design does describe item
  by item.
- **Consequence:** recorded in `plan.md` as FD-5, and as PHASE-06/EX-2 and
  VT-1/VT-2.

### 2026-09-08 — PL-7: the renderer tier's ingress cases get their own module

- **Asked:** whether the stratum 3 ingress cases join `renderer/scheduling.rs`
  or get `renderer/ingress.rs`.
- **Decided:** their own module, `crates/goad/tests/renderer/ingress.rs`.
- **Why:** the same call slice 003 made as its own PL-2, for the same reason —
  `wiring.rs` is 1232 lines and `scheduling.rs` is the timer's. A third subject
  in either is where a case gets read as belonging to the wrong mechanism.

### 2026-09-08 — PL-8: the counting glass lands where it is used, and moves if it is used twice

- **Asked:** where AC-5's presentation counter lives. It is a `Glass` decorator
  over `SlintGlass`, and `crates/goad/src/glass.rs:62` is the workspace's only
  `impl Glass` today.
- **Decided:** it lands in `renderer/ingress.rs` in PHASE-04. If PHASE-05 needs
  it as well, PHASE-05 **moves** it to `renderer/harness.rs`, unchanged in body
  and signature, with visibility widened — the one permitted difference, exactly
  as slice 003's PHASE-02/EX-11 handled the same move.
- **Why:** `harness.rs`'s module doc states the rule — two or more consumers in
  this target live there, one consumer stays where it is. Putting it there
  speculatively breaks the rule in the direction that is harder to notice.

### 2026-09-08 — PL-9: the one-liner is documentation, not a recipe and not a binary

- **Asked:** where the documented one-liner AC-13 turns on lives — a `just emit`
  recipe, a script under `examples/`, or a comment.
- **Decided:** in `examples/demo.toml`'s header comment block, in both the
  `socat` and the `nc` forms, beside the `[ingress]` key it exercises.
- **Why:** `slice-004.md` §Non-goals says a shell one-liner **is** this slice's
  client and that `goad emit` is slice 005's, which fires ADR-002's T2. A `just`
  recipe is not a binary, but it is a second place the contract is stated and a
  thing a reader would then treat as the interface. The two forms are both
  written down because the newline-or-EOF framing (D-8) exists for exactly them,
  and A-2 names them as its two witnesses.
- **Rejected:** a `just emit` recipe; a script under `examples/`; documenting
  only the `socat` form, which would leave half of D-8 unwitnessed.

### 2026-09-08 — PL-10: round 1 of the plan review dispositioned; the listener is two phases

- **Asked:** how to dispose of `review-plan.md`'s round 1 — fourteen findings,
  F-1..F-14 — and, inside F-12, whether PHASE-03 is one session's work.
- **Decided:** all fourteen `fix-now`, under the standing autonomy grant. Every
  citation was verified against the tree before acting on it; none was withdrawn.
  The substantive changes:
  - **F-12 — PHASE-03 is split.** Eight phases now, run **01, 02, 03, 08, 04,
    05, 06, 07**. The seam is the reply: PHASE-03 owns everything the filesystem
    can get wrong plus the accepted path (eight cases); **PHASE-08** owns
    everything the writer can get wrong — the two read budgets and their
    enforcement, the rest of `Refusal`'s payloads, `retry_after_ms`'s rounding,
    the closed reason set (five). Criterion ids are preserved across the move, so
    PHASE-08's sequence is non-monotonic by design.
  - **F-1 — the bind can now see the config.** PHASE-06/EX-3 moves
    `Host::new` below `runtime.enter()`, one line inside `main.rs`, rather than
    adding an accessor to `Host` or `Clone` to `Config`.
  - **F-5 — `examples/shell/backend.sh` is modified after all.** It answers an
    ingested evaluation with a view naming the event's `source` and `kind`, so
    that VH-1 observes a *different* prompt rather than a byte-identical redraw.
    PHASE-06/EX-8 said the opposite and said it was the point; it was the defect.
  - **F-4 — no umask anywhere.** The host sets the socket's mode itself with
    `set_permissions` after `bind`, which is what `SPEC-003/R-2` asks for.
  - **F-3 — AC-9's exit code is held by review, not by a test** (PHASE-06/VA-3),
    and says so in the phase and in the Coverage table.
  - **F-7 — the closed-channel path gains a test**, PHASE-04/VT-7, whose load
    -bearing assertion is that the presentation count does not advance.
  - **F-6, F-10 — two class rules and one initial value**: PHASE-04/EX-11 and
    PHASE-05/EX-5 require every timed case to pin the `next_check` of every
    exchange it lets complete; `event_floor_until` is initialised already
    elapsed.
  - **F-2, F-8, F-9, F-11, F-13, F-14** — the test module's real home, two
    surface bounds that understated their edits, a margin rule that fired by
    construction, two `deny` lints on the listener's core path, and a citation
    sweep that fixed seven `path:line` errors across the plan.
- **Why:** the reviewer's evidence held in every case. F-12 in particular: slice
  003's calibration was argued on **re-reading cost**, and fourteen bespoke
  socket cases plus a from-scratch five-type async module is not the same work as
  eight cases sharing a harness plus a uniform 22-site edit. The user's standing
  rule is that a session wraps at 200-250k tokens and that more smaller phases
  beat fewer heroic ones.
- **Rejected:** re-arguing PHASE-03's size on criterion count (that was the
  original defect); splitting PHASE-03 by production-code-then-tests, which
  breaks red/green; a binary-running harness for AC-9's exit code, which
  `renderer/startup.rs:9-11` forbids in that file's own words; and a
  test-only constructor on `Ingress` for F-7, which is PHASE-03's surface and a
  design question about what `Ingress` exposes.
- **Consequence:** `plan.md` rewritten in the places named above;
  `review-plan.md` carries a disposition and a response per finding, with
  `Outcome` left for the reviewer. Two things are **reported to the user rather
  than repaired**: AC-9 needs a reading recorded in `slice-004.md` beside AC-1,
  AC-3, AC-6 and AC-7, and `design.md` §5.3's state table does not give
  `event_floor_until` an initial value where the row beside it argues one.

### 2026-09-08 — PL-11: round 2 of the plan review dispositioned; the seam moves to the refusal vocabulary

- **Asked:** how to dispose of `review-plan.md`'s round 2 — nine findings,
  F-15..F-23 — and, inside F-18, where the seam between the two listener phases
  actually belongs.
- **Decided:** all nine `fix-now`, under the standing autonomy grant. Every
  citation was verified against the tree and the line before it was acted on;
  none failed, and nothing was withdrawn.
- **The class this round:** bookkeeping that did not keep up with a structural
  change. Round 1 split a phase, moved criteria and replaced a mechanism; round
  2 found the maps, the counts, the cross-references and the design's own text
  still describing the arrangement before. F-18 is another structural change, so
  its bookkeeping — id lists, both Coverage tables, the *Size* arithmetic, the
  §Sequencing rationale — was written in the same pass rather than left to a
  third round.
- **The substantive changes:**
  - **F-18 — the seam is the refusal vocabulary, not the reply.** The two read
    budgets, their enforcement, their two cases (VT-13, VT-14), the margin check
    (VA-2) and its STOP (S-3) all move **back to PHASE-03**, so the listener's
    read is written **once** and is bounded from the moment it exists. The
    alternative had PHASE-03 write the read unbounded and PHASE-08 rewrite it
    bounded — the cost this plan refuses when it declines to split `serve` — and
    left `SPEC-003/R-7` unheld for a whole phase, which `draft-spec.md` §6.4
    names as the defect SPEC-001/R-43 records on the other socket. PHASE-03/EX-10
    grows from three `Refusal` variants to five accordingly; PHASE-08 keeps the
    writer-facing vocabulary — the remaining payloads, `reserved_source`, the
    rounding, and the set asserted as closed at eight — and adds no bound.
    **PHASE-03 is now eleven cases against PHASE-08's three**, which is stated
    rather than absorbed: measured in re-reading, the unit slice 003's
    calibration was argued in, it is about nine distinct conditions against
    eight, because VT-13 and VT-14 are two further arms of the read VT-5 already
    exercises three arms of. PHASE-03 is the largest phase in the slice.
  - **F-15 — three divergences fixed at their source, and an instrument added.**
    `design.md` §5.4's startup order, §9's AC-9 and AC-10 rows and
    `draft-spec.md` §7's R-2 and R-4 rows are **amended**, because the design was
    wrong about what is possible rather than the plan wrong about the design.
    See `design-log.md` for that decision. New **PHASE-07/EX-7** requires a
    `## Design drift` section in `notes.md` listing every departure for the
    auditor, seeded with F-10's `event_floor_until` gap and these three
    amendments; **PHASE-07/S-4** stops on a departure no criterion authorised.
  - **F-16, F-17, F-19..F-23** — VT-10's non-vacuity threshold recomputed
    (`0o177`, not `0o077`); PHASE-04/VT-7 added to EX-11's member list with its
    two windows separated and its live read route named; R-13's Coverage row
    carried across the split; two wrong citations fixed plus two more found by
    re-sweeping for the same two shapes; the *Size* counts; the id bookkeeping
    stated under **both** phases where each phase's reader meets it; and two
    cross-references that pointed at text saying the opposite.
- **Why the seam moved rather than being annotated:** the plan's own argument
  against splitting `serve` is that editing the same lines twice is a cost, and
  the same argument applies to the listener's read. Choosing the framing spelling
  and its two bounds is one decision — `read_until` admits neither a cap nor a
  deadline on its own — so the phase that makes it must be the phase that lives
  with it.
- **Rejected:** leaving the budgets in PHASE-08 and recording R-7's gap as
  tolerated intermediate state (it is a rewrite of production code, not an
  ordering detail); re-balancing the split by moving cases back to PHASE-08 to
  make the halves even (the seam is at the subject, and evenness bought by
  splitting one read is what F-18 objects to); annotating the three design
  divergences as drift rather than amending (they are not drift — no code has
  been written, and the design is wrong about what this workspace can do);
  renumbering criterion ids to close the gaps the split left (the file's header
  comment says edits append).
- **Consequence:** `plan.md` rewritten in the places named above — §Overview,
  §Sequencing, *Size*, the decisions list (PL-10 superseded in part), both
  Coverage tables, PHASE-03, PHASE-08, PHASE-04, PHASE-06 and PHASE-07.
  `review-plan.md` carries a disposition and a response per finding, with
  `Outcome` left for the reviewer. `design.md` and `draft-spec.md` were edited
  **only** for F-15's three divergences.

### 2026-09-08 — PL-12: F-24, raised late in round 2 — the anchor's boundary, stated and made testable

- **Asked:** how to dispose of `review-plan.md` F-24, raised after the round-2
  repairs began. Two parts: a consequence sentence that overshoots in both
  `design.md` §5.3 and `plan.md` PHASE-04/EX-6, and an unspecified `<` versus
  `<=` at step 3.
- **Decided:** `fix-now`, both parts. Every citation verified first —
  `draft-spec.md:106` and `:236`, `design.md:576`, and
  `renderer/scheduling.rs:168` (the renderer tier seeds `Stimulus::Requested`
  explicitly, so no criterion breaks today).
  - **The consequence is narrowed.** *"The first envelope after startup is
    accepted"* is not the anchor's to promise: the anchor is step 3, and an
    envelope arriving during the startup exchange is refused at step 2,
    `engaged`. Both documents now say **the startup evaluation never makes an
    envelope `too_soon`**. `design.md`'s half is a decision and is in
    `design-log.md`.
  - **Step 3 refuses on `<`, and `SPEC-003/R-14` forces it.** Rounding
    `retry_after_ms` **up** puts a writer that waits exactly that long at
    `now >= event_floor_until`; under `<=` that writer is refused and R-14's own
    sentence is false of the host. PHASE-04/EX-6 states it as forced rather than
    chosen, and records the design gap the way F-10's repair did.
  - **The comparison becomes a named private free function, and PHASE-04/VT-8
    is the case** — three assertions at the floor, one nanosecond either side, in
    `controller.rs`'s existing `#[cfg(test)] mod tests`. **No socket-level case
    can discriminate**: rounding up plus a real `sleep`'s overshoot puts an
    end-to-end waiter strictly past the floor, where `<` and `<=` agree. The
    precedent is `controller.rs:524-530`'s own stated reason for that module.
- **Also folded into F-22's remedy:** `VT-7` names **three** unrelated criteria
  — PHASE-02's, PHASE-04's and PHASE-08's — and this repair makes `VT-8` a
  three-way clash as well. All six are named in the file's header comment, in
  both *Ids across the split* paragraphs, and at PHASE-04/VT-7. Ids are
  phase-local by the header comment's own rule and are not renumbered; `VT-1`
  already names five tests, so the clash is the rule working, not a defect.
- **Rejected:** holding the comparison by review, the way AC-9's exit code is
  held (it is one line of production logic and a unit case reaches it exactly —
  review is for what no instrument can reach); an end-to-end boundary case
  (cannot discriminate, and would read as evidence while proving nothing);
  extending PHASE-08/VT-9 (it has no `serve` and no anchor in the picture).
- **Consequence:** `plan.md` header comment, Coverage R-12 and R-14 rows,
  PHASE-03 and PHASE-08 id paragraphs, PHASE-04/EX-5, EX-6, EX-11, VT-7, VT-8
  and VA-2, and PHASE-07/EX-7's seeded list. `design.md` §5.3, one sentence.

### 2026-09-08 — PL-13: round 3 of the plan review dispositioned — a wrong drift seed struck, four bookkeeping inaccuracies corrected

- **Asked:** how to dispose of `review-plan.md` F-25 and F-26.
- **Decided:** `fix-now`, both. F-25 — PHASE-07/EX-7's seeded drift list named
  two entries and neither is drift: `design.md:369-377` already states
  `event_floor_until`'s initial value and its P-3 forcing argument, and
  `draft-spec.md:106` (R-14) already forces the step-3 `<` comparison, so a
  design that does not restate it is conforming, not departing. Both struck
  from the seed, leaving the three-amendments entry as the sole seed, and the
  list's intro reworded so it reads as a decision already recorded rather than
  an open gap. F-26 — four inaccuracies verified and corrected: the header
  comment's "`VT-1` is five different tests" (it is seven — every phase but
  PHASE-08 has one); PHASE-08's *Ids across the split* claiming `VA-4` "could
  not take" `VA-3`'s number when the paragraph's own rule says ids are
  phase-local (rewritten as a naming choice, not a necessity); PHASE-04's
  *Verification* preamble and EX-11 both describing all cases as driving
  `serve` in `renderer/ingress.rs`, false of VT-8, which is a unit case in
  `controller.rs` (both corrected to carve VT-8 out, EX-11 given the same
  reason as the preamble); and FD-4's `canonical.rs:490-497` row marked `✓`
  when the struct actually closes at `:496` (`:497` is blank) — corrected there
  and in PHASE-02's implementer note, matching PHASE-06/EX-8's already-correct
  `:490-496`.
- **Noted, not fixed (out of scope for F-25/F-26):** PHASE-04/EX-6's *Design
  gap reported, not filled* paragraph still says "PHASE-07/EX-7 lists it for
  the auditor," which is no longer true once the entry is struck — reported to
  the lead rather than edited, since it sits outside both findings' locations.
- **Consequence:** `plan.md` header comment; FD-4's `canonical.rs` row;
  PHASE-02's implementer note; PHASE-08's *Ids across the split*; PHASE-04's
  *Verification* preamble, EX-11; PHASE-07/EX-7's seeded list and its *Notes
  for the implementer* cross-reference. `review-plan.md` F-25 and F-26
  dispositioned `fix-now`.

### 2026-09-08 — the plan is accepted, and AC-9 takes a reading

- **Asked:** acceptance of the seven-phase plan after a four-round review
  resolved 27 findings, and how AC-9's exit-code clause should be held now that
  no instrument can hold it.
- **Recommended:** accept, and record the reading — the alternatives were a
  test harness that links the binary, which is an unscoped surface and breaks a
  rule `tests/renderer/startup.rs` states about itself, or narrowing AC-9,
  which is a scope change.
- **Decided:** both. Accepted, and the reading is recorded.
- **Consequence:** the plan stage closes and execution opens at PHASE-01, the
  A-1 probe. `slice-004.md` §Readings carries AC-9 beside AC-1, AC-3, AC-6 and
  AC-7; the clause is discharged by the argument at `main.rs:21-29`, and the
  rest of AC-9 by tests. `draft-spec.md` §7's R-4 row already says so.

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

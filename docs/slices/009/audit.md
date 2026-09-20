# Audit & reconciliation — Slice 009

Written after the last phase is done. Two jobs in one document:

1. **Audit** — does the work match its design, its acceptance criteria, and
   canon? Every gap dispositioned, none left implicit.
2. **Reconcile** — make the record true again. The code is what shipped; the
   specs must say so, or the code must change.

## Brief

**Subject:** `a698217..HEAD` on `main` — 107 commits, 35 files and +7535/-595
under `crates/`, 19 under `docs/`. `a698217` is the pre-slice spike.

**Question.** For this slice to be finished, three things must hold that a
green gate does not establish: every drawn kind submits what `R-57` types and
nothing else reaches the wire; the mechanism that made a present harmless is
actually harmless to *a person*, not only to an element counter; and the
record — canon included — is true about a host that now draws all five kinds.
This audit checks those three, and the lines of attack below are chosen
because each is a place where the gate is structurally incapable of
disagreeing with them.

Written before looking. One inherited result is assumed rather than
rediscovered: **AC-5 is unmet** (VH-1, `notes.md` PHASE-09 sheet). This audit
does not re-establish it. It asks how wide the class is.

### Lines of attack

1. **The `busy` class, beyond the slider.** VH-1 landed one instance: a
   `TouchArea` disabled mid-press loses its grab. `busy` reaches seven
   `enabled: !root.busy` sites. A disabled `ComboBox` popup, a disabled picker
   mid-chain, a `LineEdit` disabled mid-keystroke — each is the same defect
   with a different cost, and only one of them has been priced. Fix the class,
   not the instance means knowing the class first. Whether any of them reaches
   **AC-4** (a dropped character) rather than only AC-5 decides the severity.

2. **What each AC's named case would survive.** The project's recorded failure
   mode (`docs/memory/a-green-test-can-assert-a-proxy.md`, and slice 004's four
   green tests) is a case that asserts a proxy. VH-1 found one — `fields.rs:558`
   drives `set-value` for a drag. The audit asks the same question of every
   AC-bearing case, and in particular of AC-5's own instrument: `reasserts` and
   `inits` cannot observe a pointer grab, so the criterion and the thing
   measuring it were never the same claim.

3. **A backend input that panics the host.** `Alternatives::first` carries an
   `expect` and a `# Panics` section, admitted by a lint exception, resting on
   an invariant normalization is supposed to hold. *A backend failure never
   takes the host down* is one of the five. Either normalization refuses a
   `choice` with no alternatives before that call is reachable, or the
   exception is a crash on input. This is checked against the normalizer, not
   against the doc comment.

4. **Narrowing the wire to fit the renderer.** The invariant the project exists
   to hold, applied in its inverse direction: this slice deletes the renderer
   subset, so the question is no longer what is undrawn but what is *quietly
   refused*. `slider_bounds` answering `None`, `Finite` refusing a non-finite,
   `f64`→`f32` at the markup boundary (§8 **R7**, two found in one round — a
   class, not two accidents), an `f32` index, a `max`-only range. Every legal
   `R-16`/`R-17`/`R-18` field must still draw and still answer.

5. **Unbounded host state.** `pending.rs` is a map keyed by (option, field)
   whose entries leave on enqueue, with a timer that re-arms while it is
   non-empty. A backend presenting fast, or a full channel, are both host-side
   growth conditions the design prices in prose. Also: whether the timer stops.

6. **Undeclared paths.** The strongest lead per `AGENTS.md`. Diff the paths
   actually touched against each phase's declared Surfaces, both directions.
   `examples/shell/backend.sh` is already known to be stale and outside every
   phase's Surfaces (`notes.md`); the question is what else is.

7. **Whether CD-1 and CD-2 are true of the tree.** CD-1 states five per-kind
   untouched values and one consequence; CD-2 claims three Verification rows
   went false, one of them *permanently* reducing what the suite can assert.
   Both are checked against the code and the suite independently of PHASE-09's
   EX-9, which is the same agent marking its own work.

### Invariants held to

The five in `CLAUDE.md`, in full — the domain-vocabulary boundary, permissive
wire against canonical internals, no renderer-driven narrowing, no backend
input that downs the host or leaves a refusal unattributed, and one-way strata.

**SPEC-001** R-16, R-17, R-18, R-35, R-52, R-53, R-55, R-57, R-58 — the set
`slice-009.md` declares binding, each walked rather than cited.
**POL-001** — and specifically its §Verification boundaries: the gate is four
ADR-001 instruments plus the vocabulary scan plus one residue nothing enforces
(**R8**, the `jiff` feature). The residue is held by this audit's reading,
because nothing else holds it.

### Out of scope, and why

The three upstream `std-widgets` defects (VH-1 lead 4). Not this project's, and
recorded as such. They are not re-litigated here; whether the project should
own its own pickers is a slice, not a finding.

## Evidence

### Tests and checks

`just check` — **exit 0**, run at `b1f9de4`. **592** tests across 25 targets,
matching the total PHASE-09 recorded. The gate is build, both test tiers, the
`deno check` of `examples/typescript/backend.ts`, `cargo clippy --workspace
--all-targets -D warnings`, and `cargo fmt --all --check`.

What that does **not** establish is stated once, here, rather than implied by
the number. `POL-001` §Verification names the boundaries: the gate is four
ADR-001 instruments plus the domain-vocabulary scan plus **one residue nothing
enforces** — a feature switched on in a dependency stratum 1 shares. This slice
takes that residue (`jiff`'s `tz-system` and `tzdb-zoneinfo`), and
`design.md` §10 carries the argument `POL-001` requires in place of a check.
Read and accepted: the argument is complete, it measures rather than predicts
what the feature does and does not gate, and it correctly narrows rather than
retires `clock.rs`'s workaround. **PHASE-04/VA-1 discharged.**

### Surface delta

Every path touched under `crates/` is inside some phase's declared Surfaces,
with two exceptions, both trivial and neither a lead:

- `crates/goad/tests/renderer/main.rs` — undeclared, +3/-1, module declarations
  for nothing but the files that *were* declared.
- `Cargo.lock` — undeclared, and a consequence of the `Cargo.toml` lines that
  were.

`crates/goad-semantics/src/protocol/canonical.rs` is **stratum 1 and was
touched**, which reads as a breach until the plan is consulted: PHASE-09's
Surfaces carry it explicitly, *"`Alternatives::first` alone — added 2026-09-19
with explicit user endorsement"*. The path is declared and the change is
sound. What is stale is `slice-009.md` §Governing canon, which still reads
*"all of this slice is stratum 3; nothing reaches `goad-semantics`"* — see
Reconciliation.

Declared and untouched: `crates/goad/tests/renderer/table.rs` (PHASE-01) and
`tests/support/` (PHASE-01, conditional on a helper being extracted — it was
not). Neither is dropped work.

`spike-fields/` is deleted, which D-19 required at design close.

**The undeclared path that matters is one nobody touched.**
`examples/shell/backend.sh` is outside every phase's Surfaces and is now false
about the running product in three places: its header says the form *"carries a
field of a kind this renderer does not draw, on purpose"* (`:15`); its `note`
field is annotated *"it will not appear in the window"* (`:101`) and *"its id
will be absent from the `values` recorded above"* (`:102`). All three were true
when written and none is now. The last is not stale prose — it describes
runtime behaviour a person running `just demo` will watch the host contradict.
The gate cannot see it: the example typecheck covers
`examples/typescript/backend.ts` and nothing covers this file.

### Stratum purity

`cargo test -p goad-semantics` builds and passes as part of the gate.
`Alternatives::first` is pure — it names no `src/shell/`, reads no clock,
filesystem or subprocess, and adds no dependency. ADR-001 holds.

### Acceptance criteria

<!-- filled from the human run, the wire log, and review-code.md's suite
     dimension -->

### Verification criteria

<!-- VT / VA / VH per phase -->

## Code review

Findings live in `review-code.md`, copied from
`docs/templates/review-ledger.md` — same ledger, same severity and disposition
vocabulary, subject `implementation`. Do not restate findings here.

- **Ledger:** `review-code.md`
- **State:** open | resolved · outstanding blockers: none | <ids>

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
- [ ] Tests and checks green
- [ ] Specs / policy / ADRs reconciled, with user endorsement where amended
- [ ] `draft-spec.md` / `canon-delta.md` promoted, or abandoned with the reason written down
- [ ] `slice-nnn.md` Summary and Follow-ups written
- [ ] `notes.md` Harvest current; durable facts lifted to `docs/memory/`
- [ ] `slice-nnn.md` stage set to `done`

# Plan log — Slice 001

Append-only working record for the plan stage. Survives compaction and
interruption; `plan.md` itself stays clean. Never rewrite an entry — supersede
it with a later one.

Decisions here are the **user's**. Findings from an adversarial review of the
plan live in `review-plan.md`, not here; a finding that prompts a decision
produces one of each, cited to one another.

## Decisions

### 2026-08-26 — Which TOML parser, and how is it gated?

- **Asked:** `design.md` §5.2 specifies a TOML configuration file and a parsed
  `Config`, and §5.1's manifest lists serde, serde_json, jiff and tokio. No TOML
  parser is named anywhere in the design, the draft spec or the design log.
  Adding one is a dependency addition — which `docs/AGENTS.md` §Execute says to
  stop and consult on, and which ADR-002's Verification section requires be
  checked against the triggers and recorded in the slice's design.
- **Recommended:** `toml`, **optional, inside the `shell` feature**, exactly as
  tokio is. `Config` lives in `shell/config.rs`, so the parser is a stratum 2
  dependency; a non-optional one would put a dependency stratum 1 does not need
  back into stratum 1's graph and undo half of D49.
- **Decided:** `toml`, optional, inside the `shell` feature.
- **Consequence:** ADR-002's T1 does not fire, for D49's reason and by D49's
  mechanism — nothing stratum 1 must not need is required in order to build
  stratum 1. PHASE-01 writes it into the manifest; PHASE-07 is the phase that
  uses it. `design.md` §5.1's manifest and §3's trigger analysis are both a line
  short and should gain one during audit's reconciliation, since the design is a
  record of intent at a point in time and this decision post-dates it.

### 2026-08-26 — Does the integration tier need a tokio dev-dependency, and does that weaken the build gate?

- **Asked:** raised by the planner as a suspected defect of the same class as
  F-51. The integration tier must drive an async API from a `#[test]`, which
  appeared to require tokio under `[dev-dependencies]` — and Cargo forbids
  optional dev-dependencies, so that entry would be unconditional. Measured in a
  scratch crate reproducing §5.1's manifest: with such an entry present,
  `cargo test --no-default-features` still skips the `integration` target and
  still refuses to compile a `semantics/` file naming tokio, but it *does*
  compile tokio, and plain `cargo tree --no-default-features` lists it under a
  `[dev-dependencies]` heading. On that reading three statements in
  `design.md` §5.1, §9 and `canon-delta.md` CD-1 were false.
- **User's question:** how does that interact with the requirement to test the
  application logic without async?
- **Answered by measurement, and the finding was withdrawn.** The premise was
  wrong. A test target has the package's **regular** dependencies in scope,
  optional ones included, whenever the feature enabling them is on — so no
  dev-dependency is needed at all. Verified on a manifest with no
  `[dev-dependencies]` section: `#[tokio::test]` compiles and runs in
  `tests/integration/` in the default column, while in the
  `--no-default-features` column `tests/protocol/` naming `tokio` fails with
  `error[E0433]: cannot find module or crate 'tokio'`. `cargo tree
  --no-default-features` is clean with no `-e` filter.
- **Decided:** no `[dev-dependencies]` section. No correction is owed to
  `design.md` or to CD-1 — all three claims are true as written.
- **Consequence:** the answer to the user's question is stronger than the design
  claimed for it. Stratum 1 cannot be tested *with* an async runtime even by
  accident, because the test target cannot name one; that is Cargo's resolution
  rather than review or a grep, so AC-15's boundary test does not need extending
  to cover `tests/protocol/`. PHASE-01/EX-3 asserts it by breaking it and
  reverting, and PHASE-01's implementer notes forbid adding a
  `[dev-dependencies]` section later, since an unconditional tokio entry there
  is precisely what would put a runtime back in reach of the stratum 1 tier.

### 2026-08-26 — Adversarial review of the plan?

- **Asked:** whether `plan.md` goes to a fresh adversarial reviewer before
  acceptance, as the design did over five rounds.
- **Decided:** yes — one round, fresh reviewer with no thread history.
- **Consequence:** ledger at `review-plan.md`, copied from
  `docs/templates/review-ledger.md`, subject `plan`. Rounds 4 and 5 of the design
  review both used a fresh reviewer and both reached past what the accumulating
  thread of rounds 1-3 managed; round 5's thread
  (`01a03af5-bc55-79a1-a216-ff9c7e7ee4e1`) is spent and is not reused.

### 2026-08-27 — Closing the plan review: four rounds, closed on a clean one

- **Asked:** whether round 4 closes `review-plan.md`, or whether the residual
  rate justifies a fifth.
- **Decided:** closed. Round 4 raised **no findings**, confirmed F-12, F-13 and
  F-14 individually with cited evidence, found the tests carve-out's *reasoning*
  sound on its first attack, and judged `plan.md` **executable as it stands**.
- **Why not a fifth round.** The defect rate per repair fell 4/6 → 2/5 → 0/3 and
  the round that produced the zero was the first run against text the author had
  already restatement-swept. A fifth round would be testing the sweep, not the
  plan, and the review's stated close condition — no blocker, nothing a repair
  cannot close — was met. The one review decision that cost this slice most was
  the *design* review's opposite: closing with sixteen repairs unverified.
- **Consequence:** `review-plan.md` `State: closed`, Synthesis written. Two risks
  are recorded there and carried into `notes.md`'s handover rather than resolved:
  round 4 verified PHASE-06/VT-6's mechanism on the documents and did **not**
  re-run the tokio metrics probe, and priority 3's sites — the author's five
  self-sweep repairs, PHASE-08's split comment, PHASE-06's cancellation note —
  got no per-site verdict, only the round's overall silence.
- **Also recorded, not as a finding:** the round 4 packet itself carried two
  stale restatements, found at dispatch — its reading list still named rounds 1
  and 2 and pointed the reviewer at F-7…F-11, and its finding-format example was
  headed `F-7` against an instruction to number from F-15. Author-found,
  author-repaired before dispatch. The document written to hunt the restatement
  defect had it. That is the strongest argument in this slice for making the
  sweep mechanical rather than intentional.
- **Next:** `plan.md` to the user for acceptance. No code before it.

### 2026-08-29 — Plan accepted

- **Asked:** acceptance of `plan.md`, per `docs/AGENTS.md` §Plan and `CLAUDE.md`'s
  "no code without an accepted plan".
- **Decided:** accepted. The user's instruction to write PHASE-01's phase sheet
  is the acceptance; there was no separate ceremony and none is owed.
- **Consequence:** PHASE-01/EN-1 is discharged. The phase sheet is in `notes.md`
  under `## Phase sheets`; execution may begin. Sheets stay one at a time,
  immediately before each phase — `docs/AGENTS.md` §Phase plan, and a sheet
  written three phases early is fiction.
- **Raised while expanding the phase, and owed back to the plan:** two of
  PHASE-01's *Notes for the implementer* describe a tree that has since changed.
  The plan says `.gitignore` does not exist — it has since commit `4fc8637` —
  and says not to duplicate the user's global `*.local.*` ignore, which that
  commit already did, narrower, as `*.local.md`. `docs/AGENTS.md` §Phase plan
  forbids repairing this in the sheet, so it is recorded and left to the user.
  Neither is a criterion and neither blocks PHASE-01.

### 2026-08-29 — Package metadata and licence

- **Asked:** whether to carry `[package]` metadata now, and under what licence.
- **Decided:** yes, now. **MIT**, repository `https://github.com/davidlee/goad`,
  fields lifted from `~/dev/doctrine`'s manifest. `LICENSE` written at the
  repository root, © 2026 David Lee. `publish = false` stays and is not
  provisional.
- **Consequence:** `Cargo.toml` `[package]` gains `license`, `repository`,
  `readme`, `keywords` and `categories`. `LICENSE` is a new root file, outside
  PHASE-01's declared surfaces — a user-directed scaffolding change of the same
  kind as `Cargo.toml`, `clippy.toml` and `justfile` before it, not phase work.
- **The reason it was raised was wrong, and the correction matters more than the
  change.** PHASE-01's sheet flagged `clippy::cargo_common_metadata` as an
  anticipated gate failure. It cannot fire: **`publish = false` silences it
  outright.** Measured in a scratch crate carrying `cargo = "deny"` — clean with
  `publish = false`, five errors without it (`license`, `repository`, `readme`,
  `keywords`, `categories`). The metadata is therefore carried for correct
  attribution, not to pass the gate, and the STOP condition is withdrawn.
- **A positive control was run rather than assumed**, per the review's own
  lesson that a mechanism needs one: the same probe with goad's new `[package]`
  block and `publish = false` **removed** also passes clean. So the metadata is
  complete on the lint's terms, not merely hidden behind `publish = false`.

### 2026-08-29 — The two stale `.gitignore` notes in PHASE-01

- **Asked:** disposition of two stale *Notes for the implementer* surfaced while
  expanding PHASE-01 — `plan.md` claimed `.gitignore` did not exist (false since
  `4fc8637`) and told the implementer not to duplicate the user's global
  `*.local.*` ignore, which that commit had already done, narrower, as
  `*.local.md`.
- **Decided:** widen the repository `.gitignore` to `*.local.*`, and rewrite the
  plan's note to describe the tree as it is.
- **Why widen rather than defer to the global file.** A rule that lives only in
  one machine's global config is not a property of the repository. Under
  `*.local.md` alone, `transport-probe.local.rs` and
  `transport-probe-Cargo.local.toml` were ignored by `~/.gitignore_global` and by
  nothing in the repo, so a clean clone elsewhere would track them — and AC-1 is
  a claim about a clean clone.
- **Consequence:** `.gitignore` now carries `*.local.*` with the reason written
  at the site. `git ls-files` was checked first: no tracked file matched, so
  nothing was silently un-tracked. `git check-ignore -v` now resolves all three
  probe and packet files to `.gitignore:4` rather than to the global file.
  `plan.md:282` rewritten; PHASE-01's sheet records the closure.

### 2026-08-29 — Two gaps in PHASE-02, surfaced by expanding its phase sheet

- **Asked:** disposition of two things `plan.md`'s PHASE-02 entry did not settle.
  (1) Its declared surfaces name no test file and no line for
  `src/semantics/mod.rs`, which the phase must edit to declare
  `semantics::protocol` at all — it is the only phase in the plan that writes
  tests and names none. (2) The design gives the scalar newtypes no constructors,
  while PHASE-07 mints `view_id` in `shell/state.rs` (D13) and its surfaces
  exclude `canonical.rs`, so whatever it needs must ship in PHASE-02.
- **Decided:** (1) surfaces gain `src/semantics/mod.rs`, and the phase's tests are
  **colocated `#[cfg(test)]` modules**; `tests/protocol/` stays PHASE-03's.
  (2) **`ViewId` and `Timestamp` get public constructors; `OptionId`, `FieldId`
  and `AlternativeId` do not.**
- **Why the test home is forced rather than chosen.** `tests/protocol/` is an
  external crate. Under D30, `Opt`, `Field` and `Alternative` have `pub(super)`
  fields and no public constructor, so an external test cannot build the
  `Vec<Opt>` that `Options::new` must reject or the two same-id `Field`s VT-3
  must accept. The only way to write VT-1 and VT-3 there is to widen the
  canonical types, which is R10 — the exact risk the same phase's VA-2 exists to
  detect. VT-2 could have gone either way and is colocated with them: one home
  per phase, and no early touch on the target PHASE-03 owns.
- **Why the constructor split falls on host-authored versus backend-authored.**
  A `ViewId` is minted by the host and a `Timestamp` is a clock read; neither
  asserts that a backend said anything, and stratum 2 must be able to build both.
  An `OptionId`, `FieldId` or `AlternativeId` is an address a backend chose, so a
  public constructor would let a caller mint an id no backend ever sent — the
  same hole D30 closes one level up on the canonical types. A caller answering a
  view clones the id out of the view through its accessor, which is what the
  design's `Clone` derive is already for.
- **Consequence:** `plan.md:311` rewritten — surfaces amended and the colocation
  rule stated with its reason; EX-1 amended to name which scalars carry a public
  constructor and why. PHASE-02's sheet in `notes.md` records both as closed.

### 2026-08-30 — `Fields` may be empty; EX-3 was over-general

- **Asked:** PHASE-02's EX-3 required the checked constructors to reject "empty
  `Options`/`Alternatives`/`Fields`". Raised while writing VT-1's rejection
  cases, before any constructor existed — the empty-`Fields` case had no error
  variant to assert.
- **Decided:** `Fields::new` checks id-uniqueness only and permits zero
  elements. `Options::new` and `Alternatives::new` keep both checks. No
  `EmptyFields` variant is invented.
- **Why.** Four sources say an option may carry no fields: R-15
  (`draft-spec.md:106`) states it normatively; R-15's own verification row
  (`:364`) asks for "an option with and without fields"; `brief.md:131` and
  `:567` say it twice; and the spec's example response (`:232`) contains
  `{ "id": "yes", "label": "Now" }` with no `fields` key. The negative evidence
  agrees — §5.5's edge table has rows for `options: []`, duplicate option ids,
  duplicate field ids and empty alternatives, but **no empty-fields row**, and
  the landed taxonomy has `EmptyOptions` and `EmptyAlternatives` and no
  `EmptyFields`. `Opt.fields` is a `Fields` rather than an `Option<Fields>`, so
  an option with no fields must be a `Fields` holding none. Implementing EX-3
  literally would have made the spec's own example unnormalizable at PHASE-04.
- **Where the error came from, which is the part worth keeping.**
  `design.md:704`'s comment over the three newtypes reads "all three for the
  same reason: >= 1 element, and ids unique within the collection." The F-52
  paragraph immediately beneath it argues only about *duplicates* — two fields
  sharing an id have one response key between them — and never argues
  non-emptiness for fields. The blanket comment over-generalised a rule that
  holds for two of the three, and EX-3 restated the blanket comment rather than
  the argument. **That is this slice's recurring defect for the third time**: a
  rule applied at the named site and restated too broadly where it is repeated.
- **Consequence:** `plan.md`'s EX-3 rewritten with the rule and its evidence.
  PHASE-02's sheet records it as the third thing the plan did not settle, and
  task 2's case list drops the empty-`Fields` case. `design.md:704`'s comment is
  **left as written** and goes to audit reconciliation, on the same footing as
  the `toml` line: the design is a record of intent at a point in time and this
  finding post-dates it.
- **It improves task 9 rather than complicating it.** The refactor was framed as
  "three copies of one rule collapse into one helper". The rule is now genuinely
  two: uniqueness over all three, non-emptiness over two. A helper that takes
  both checks as one lump would have been the same over-generalisation in code.

### 2026-09-02 — A phase that adds a module owns its parent's `mod` line; three Surfaces amended

Raised while expanding PHASE-03's phase sheet. `src/semantics/schedule.rs`
cannot exist without `pub mod schedule;` in `src/semantics/mod.rs`, and
PHASE-03's Surfaces did not name that file.

**Decision:** amend the Surfaces of **PHASE-03, PHASE-04 and PHASE-07** in one
edit rather than one phase at a time — `src/semantics/mod.rs`,
`src/semantics/protocol/mod.rs` and `src/shell/mod.rs` respectively. Chosen from
three options; the alternatives were amending PHASE-03 alone and re-raising
later, or dropping the rule that a parent `mod` line must be declared at all.

- **Why fix the class.** This is the **second** instance of the same omission —
  PHASE-02's Surfaces were amended for exactly this on 2026-08-29 — and three
  more were latent. The slice's recurring defect is a rule applied at the named
  site and not at the sites that restate it; the same shape appears here in the
  plan's own structure, so closing it at the class is the consistent move.
- **Why not drop the rule.** Undeclared paths are the audit's strongest lead
  (`AGENTS.md`, *Audit & reconcile*). A convention that a `mod` line is
  implicitly in scope makes every phase's diff carry one path the Surfaces list
  does not, which is exactly the signal audit reads. The cost of declaring it is
  one clause.
- **Scope, corrected while applying it.** The question named PHASE-03, 04 and 05;
  the actual remaining instances are **03, 04 and 07**. PHASE-05 already declares
  both `src/shell/mod.rs` and `src/shell/backend/mod.rs`, and PHASE-07 — which
  adds `config`, `state` and `host` to `src/shell/` — did not. PHASE-01 and
  PHASE-05 were the only two phases that had written it down.
- **One thing found by the same edit.** `src/semantics/protocol/mod.rs`'s doc
  comment says "`wire` and `normalize` arrive in PHASE-03 and PHASE-04". Both
  arrive in PHASE-04; PHASE-03 writes `semantics/schedule.rs` and nothing under
  `protocol/`. PHASE-04's amended Surfaces line names the file and the comment,
  so the phase that makes the sentence true is the one that can fix it.


### 2026-09-02 — PHASE-04's Surfaces gain the test-target Rust; VA-2's path corrected

Both raised while expanding PHASE-04's phase sheet, and both amend `plan.md`
rather than being absorbed into the sheet.

**Decision 1: add `tests/protocol/runner.rs` and `tests/protocol/main.rs` to
PHASE-04's Surfaces.** The entry named `tests/protocol/fixtures/**` and no Rust
under `tests/`, but a fixture file asserts nothing on its own — the protocol
corpus needs a checker reading its own `expect` tags, a `Corpus` const and a
`#[test]`. Chosen from three options; the alternatives were naming `runner.rs`
alone, and leaving the plan unchanged to be raised mid-execution.

- **Why both files.** Naming `runner.rs` alone commits PHASE-04 to extending
  that file, growing it from two halves to four. *The fixture format* in
  `notes.md` deliberately left the split to "PHASE-04's call, taken with its own
  surfaces in hand", and a new file under `tests/protocol/` needs `main.rs` to
  declare it. Naming both is what puts the call in hand without taking it.
- **Why not defer.** A STOP mid-execution is the cost, and the phase sheet
  exists to pay it in advance. PHASE-03's expansion found four gaps, closed all
  four before starting, and was the first phase to need nothing mid-flight.
- **Same class as the parent-`mod` omission** closed earlier the same day: a
  phase's Surfaces listing the artefact it adds and not the declaration that
  reaches it. Third instance of that shape in this plan.

**Decision 2: VA-2's path corrected to `src/semantics/protocol/normalize.rs`.**
It read `src/semantics/normalize.rs`, which is not a file — `normalize` is under
`protocol/`, as EX-1 and the Surfaces line in the same phase entry both say. No
change of intent: the criterion still asks for both break-and-revert forms in
host code. Recorded rather than silently fixed because it is plan text.

**One thing found by the same expansion, needing no decision.** VT-2's `NaN`
fixture cannot be written in the inherited fixture format. Measured rather than
reasoned: `serde_json` refuses `NaN` with *expected value* and `1e400` with
*number out of range*, and does so when the **envelope** is parsed, so such a
file lands as `Fault::Malformed` and never asserts its protocol claim. The same
measurement confirms F-36 and D39 hold exactly as written — neither literal
reaches bounds validation, so `NotFinite` stays unreachable from the wire. This
is implementer latitude and is settled in the phase sheet: a second `Corpus`
whose `input` is raw document text, over the shared half unchanged.


### 2026-09-02 — PHASE-04's Surfaces gain `canonical.rs`, scoped to four lint attributes

Raised during PHASE-04's execution, from a measurement rather than a reading.

**Decision: PHASE-04 removes the four `#[cfg_attr(not(test), expect(dead_code,
…))]` attributes in `src/semantics/protocol/canonical.rs`, and the file joins
the phase's Surfaces with that scope written into the line.**

PHASE-02 landed those attributes on `OptionId::new`, `AlternativeId::new`,
`FieldId::new` and `Hints::new` — the four `pub(super)` constructors whose only
caller is normalization. Each reason text says, in as many words, that the
attribute comes off once PHASE-04 calls the constructor, because
`unfulfilled_lint_expectations` fails the gate until it does.

Measured, not predicted: a single `OptionId::new` call from a stub
`normalize.rs` gives `error: this lint expectation is unfulfilled`, and the lib
does not compile. The tuple fields are private to `canonical`, so `new` is the
only construction path from a sibling module — there is no way to write
`normalize.rs` that avoids this.

- **Why it is not R10.** R10 is a constructor added to `canonical.rs`, or a
  field widened to `pub`. This adds nothing and widens nothing; it deletes four
  attributes that PHASE-02 wrote as temporary and said so. The phase sheet's
  STOP condition — "wanting a constructor or a wider field on `canonical.rs`" —
  stands exactly as written.
- **Why the plan is amended rather than the sheet alone.** Same class as the
  parent-`mod` omission and the test-target Rust: a phase's Surfaces naming what
  it adds and not the declaration that reaches it. Fourth instance in this plan,
  and the second where the previous phase left the obligation in a comment that
  the Surfaces line did not carry.
- **Alternative rejected:** removing them without amending `plan.md`, on the
  grounds that PHASE-02 wrote the obligation into the code. It would leave the
  audit's surface diff showing an undeclared path, which `docs/AGENTS.md` calls
  the strongest lead.


### 2026-09-02 — PHASE-05's three expansion gaps, all closed the same day

Raised by PHASE-05's expansion, before any of stratum 2 was written. All three
are plan text; none was repaired in the sheet.

**1. Decision: VT-5's source-text checks land in a new
`tests/protocol/transport_shape.rs`, and that file plus `tests/protocol/main.rs`
join PHASE-05's Surfaces. PHASE-06 gains the same file, since VT-6 re-asserts
one of the checks against the finished module.**

VT-5 asks for three greps over `process.rs`'s source text "in the same tier and
with the same found-no-files guard as PHASE-01's boundary checks". That tier is
`tests/protocol/`, and neither the file that would hold them nor the `main.rs`
that declares it was named. **Fifth instance** of a phase's Surfaces naming what
it adds and not the file that reaches it, and the second where the omission
spans two phases at once.

- **Why a new file rather than `boundary.rs`.** `Scan` is a forbidden-token walk
  over a *directory*: `{ root, forbidden }`, matched case-insensitively per
  line. Only one of VT-5's three checks is that shape. The first permits the
  token `spawn` but constrains its occurrence *shape* to `Command::spawn`; the
  third is *region*-scoped — no `?` between the spawn and the cleanup budget.
  Neither is expressible as configuration, and `boundary.rs`'s own doc comment
  instructs the next phase to extend the configuration and not the walk.
- **Alternative rejected:** generalising `Scan` to carry a per-line predicate
  and a region state machine. It reworks PHASE-01's surface so one type can
  serve two unrelated questions, which trades a small duplication for a large
  coupling. The guard is what is worth sharing, and it is an idea rather than
  code: here its form is "the file was found and read".
- **Fixed as a class, not an instance:** both phases amended in one edit, as the
  parent-`mod` omission was applied to three phases at once on 2026-09-02.

**2. Decision: PHASE-05 defines no `Config`. The transport holds
`command: Vec<String>` and `timeout: Duration` directly, and PHASE-07
constructs one from a loaded `Config`.**

PHASE-05's implementer notes told the harness to build "a `Config` pointing at
one", but `src/shell/config.rs` is PHASE-07's surface and PHASE-07/EX-1 owns the
type whole — brief §5's three values, the three load-time rejections, and the
jiff duration grammar. Nothing in PHASE-05's Surfaces can define it.

- §5.4 needs no `Config` anyway. `Backend::exchange` takes only `&mut self` and
  the request, so the timeout is already the transport's state; `config.timeout`
  in the sketch is where the value *came from*, not where it lives.
- **Alternative rejected:** adding `config.rs` to PHASE-05 with `BackendConfig`
  only. It splits one type across two phases and hands PHASE-07 a
  half-built surface to finish, for no gain — the harness constructs a transport
  either way.
- This is the one of the three that is closest to a reading rather than a
  defect. It was raised because it is plan text and because it settles the
  constructor's signature.

**3. Decision: `BackendError::Protocol` is not PHASE-05's. VT-3 drops the
`Protocol(Json)` clause; PHASE-07 gains EX-8 and VT-6, and R-38's framing rule
goes with them.**

EX-1 fixes `Exchange.result` as `Result<Vec<u8>, BackendError>`. The transport
returns bytes and parses nothing, so it has no way to raise a `Json` error, and
VT-3 required a case for one. The design is specific where it matters: invalid
UTF-8 "becomes a `Protocol(Json)` error **via `from_slice`**" (`design.md:1052`),
and `from_slice` runs in `Host`.

- **R-38 travels with it.** Exactly one JSON document, trailing content an
  error — `from_slice` refuses trailing content by itself, so the requirement is
  discharged by the same call. PHASE-04's sheet passed R-38 to PHASE-05 on the
  grounds that framing is the transport's; that is right about the *tier* and
  wrong about the *phase*.
- **The backend script stays in PHASE-05.** EX-4's claim about a zero exit with
  unparseable stdout is that the **stderr** survives, which is a transport fact.
  The test asserts `result` is `Ok(bytes)`, that those bytes do not parse, and
  that the stderr arrived. Only the variant claim moved.
- **It gets cheaper by moving.** PHASE-07 asserts it against the fake `Backend`
  it already builds for VT-1 — handing `Host` a byte string needs no spawn,
  where PHASE-05 would have paid one per case.
- **Alternative rejected:** having the transport parse purely to validate
  framing and still return raw bytes. Two parses of every response, and two
  places that can raise the same error.

### 2026-09-03 — PHASE-05's fourth gap, raised at the end of execution

Found by diffing the paths PHASE-05 touched against its declared Surfaces, which
`docs/AGENTS.md` calls the strongest lead an audit has. The phase was otherwise
green; the sheet's STOP list names "a fourth gap of the same kind" explicitly, so
it was raised rather than repaired in the sheet.

**Decision: PHASE-05's Surfaces name `tests/integration/**` rather than
`tests/integration/main.rs` and `tests/integration/harness.rs`.**

The phase's cases live in `tests/integration/transport.rs`, which nothing
declared. Same class as the five before it — a phase's Surfaces naming what it
adds and not the file that carries it — except that here the undeclared file is
the phase's own deliverable rather than the `mod` line that reaches it.

- **The glob is not a widening, it is the form the plan already uses.**
  PHASE-06's Surfaces say `tests/integration/**` and always did. Two phases
  writing into one directory under two different conventions is what let this
  through, and after the amendment there is one convention.
- **Fixed as a class, not an instance.** Naming `transport.rs` alone was the
  cheaper edit and was rejected: the next integration file — PHASE-08 adds
  one — walks into the same wall.
- **Alternative rejected:** moving the cases into `tests/integration/main.rs`,
  which is already a Surface. It needs no plan amendment and costs the
  convention the protocol tier established, where a target root declares its
  modules and asserts nothing. `tests/protocol/main.rs` is four `mod` lines and
  a doc comment for that reason.
- The scripts under `tests/backends/` were already declared as `*.sh` and are
  untouched by this.

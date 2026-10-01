# Notes — Slice 012

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 | done | 2026-10-01 |
| PHASE-02 | pending | 2026-10-01 |
| PHASE-03 | pending | 2026-10-01 |
| PHASE-04 | pending | 2026-10-01 |
| PHASE-05 | pending | 2026-10-01 |
| PHASE-06 | pending | 2026-10-01 |
| PHASE-07 | pending | 2026-10-01 |
| PHASE-08 | pending | 2026-10-01 |
| PHASE-09 | pending | 2026-10-01 |
| PHASE-10 | pending | 2026-10-01 |
| PHASE-11 | pending | 2026-10-01 |
| PHASE-12 | pending | 2026-10-01 |

## Phase sheets

<!-- One block per phase, written at phase-plan time, immediately before
     execution. Disposable detail — it exists to get one agent through one
     phase. -->

### PHASE-01 — refusals name a requirement and a side

**Written by a phase-sheet agent, not the executor**, at 7b549d4 (*012: plan
accepted*). This sheet is the plan's second reading. Where it restates a plan
criterion it quotes it. It repairs nothing: what reads as wrong in the plan is
under **Findings** as a PLAN QUESTION, and the tasks it blocks are marked `[!]`.

**Objective** (quoted, `plan.md` PHASE-01): *every refusal kind the host can
report answers, by a total match, the requirement and the side SPEC-001/R-59
(`canon-delta.md` SPEC-001 Change 1) assigns it, each prints itself, and the
fixture corpus witnesses the protocol and schedule answers.*

**Entry — EN-1** (quoted): *`design.md` and `canon-delta.md` approved
(`design-log.md` 2026-09-30, *the design as a whole, after review*); `just
check` green at HEAD.*
- **Discharged 2026-10-01 at 7b549d4.** `just check` exited 0: build, both
  test tiers (644 passed, 0 failed, summed over every `test result` line),
  `deno check`, `cargo clippy --workspace --all-targets -- -D warnings` with no
  warning, `cargo fmt --all --check`. The approval half is the design-log
  entry the criterion names.

**Surfaces — a closed list, copied from `plan.md`. Anything else is a STOP.**
- `crates/goad-semantics/src/error.rs`
- `crates/goad-semantics/tests/protocol/normalize.rs`
- `crates/goad-semantics/tests/protocol/runner.rs` — *"only if the discard
  witness needs the schedule corpus's envelope"*. It does: see *Assumptions,
  verified now*, A-V3.
- `tests/fixtures/protocol-text/R-17-a-nan-literal-for-a-bound.json`,
  `tests/fixtures/protocol-text/R-17-an-infinite-literal-for-a-bound.json`,
  `tests/fixtures/protocol/R-52-a-choice-field-with-no-alternatives.json` —
  *"their `requirement` arrays only"*.
- `crates/goad-shell/src/error.rs`
- `docs/slices/012/canon-delta.md` — *"test names only"*.
- `docs/slices/012/notes.md` — this sheet, §Status, §Harvest (bookkeeping, by
  `docs/AGENTS.md` §Execute).

**Reading list** (by symbol; `grep -n` then `sed -n`, not whole files)
- `docs/slices/012/plan.md` — §Overview's first paragraph (`just check` is
  every phase's last exit); *Test names are commitments*; *Mutation evidence*
  under §Sequencing & rationale; §PHASE-01 whole.
- `docs/slices/012/design.md` §5.2.3 whole: the two type declarations,
  *Meaning of the id*, **the table**, the rationale bullets, *The witness*,
  *Its reach*. §5.5 A-1 (`normalize_alternative` is the only raiser of
  `InapplicableKey { key: "fields" }`). §9 *Stratum 1*, *Stratum 2*, and the
  first bullet of *Mutation checks*.
- `docs/slices/012/canon-delta.md` SPEC-001 Change 1 (R-59 as it will be
  stated) and Change 2 (R-59's §7 row, which names the test cases).
- `crates/goad-semantics/src/error.rs`: `ProtocolError`, `BoundsError`,
  `ScheduleError` (the taxonomies this phase adds methods to); `SpanFault`
  (EX-3: gets neither method); `impl From<serde_json::Error> for
  ProtocolError` (why `Json` cannot tell R-17 from R-38). In `mod tests`:
  `must_name` and its siblings `bounds_must_name`, `schedule_must_name` (the
  exhaustive-match pattern VT-1's tables sit beside); the builders
  `every_protocol_error`, `every_bounds_error`, `every_schedule_error` (the
  plan: *"extend it, do not write a second builder"*); `assert_names`.
- `crates/goad-shell/src/error.rs`: `BackendError`, `CleanupFailure`,
  `StateError` (methods added); `ConfigError` (EX-3: neither). The file has no
  `mod tests` today; VT-2 creates it.
- `crates/goad-shell/src/ingress/envelope.rs`: `EnvelopeFault` (EX-3:
  neither; read only, not a surface).
- `crates/goad-semantics/tests/protocol/runner.rs`: `Envelope` (where a
  fixture's `requirement` array is typed: `Vec<String>` of `"R-N"` strings,
  `deny_unknown_fields`, and `read_envelope` refuses an empty array);
  `Fixture`, `Corpus`, `Corpus::run` (its vacuity guard), `fixture_paths`,
  `read_envelope`, `outcome_tag`, `assert_corpus`; `schedule_error_name`;
  `check_schedule`; `SCHEDULE` (private `const`).
- `crates/goad-semantics/tests/protocol/normalize.rs`: `render_error`,
  `render_discarded`, `compare`, `check_protocol` (input is a JSON value,
  serialized with `serde_json::to_vec`), `check_protocol_text` (input is a
  JSON string of document text), `PROTOCOL`, `PROTOCOL_TEXT`, `fixtures_of`,
  `tags_named_by_fixtures`,
  `every_reachable_error_in_the_taxonomy_is_named_by_a_fixture` (VT-3's
  witnesses sit beside it), `a_schedule_failure_is_named_by_a_fixture_as_a_discard`
  (the existing walk over `Discarded` fixtures), and this file's own
  `every_protocol_error` builder (see Findings).
- `crates/goad-semantics/src/protocol/normalize.rs`: `read_response` (the
  path both witnesses must run), `Normalized`, `Discarded::Schedule { raw,
  reason }`, `inapplicable` and its `"fields"` call in
  `normalize_alternative`.
- `crates/goad-semantics/src/schedule.rs`: `parse` (what `check_schedule`
  runs; the discard witness runs the same).
- Prior art: `docs/memory/` — *negative-control-must-compile*,
  *tests-asserting-proxies*, *mutation-check-the-coverage-claim*.

**How a fixture carries its `requirement`.** Every fixture in
`tests/fixtures/{protocol,protocol-text,schedule}/` is one JSON object with
`requirement` (array of `"R-N"` strings), `description`, `now` (RFC 3339),
`input`, `expect`. `expect` is a single-key object: `{"error": …}` or
`{"accepted": {"canonical": …, "discarded": [ … ]}}` in the protocol corpora;
`{"error": "<ScheduleError variant>"}` or `{"instant": …}` in the schedule
corpus. The existing corpus tests hold that each `expect` is what the code
produces, so the produced error of a fixture can be read off its `expect`
(A-V5 below relies on this).

**Assumptions — verified now** (at 7b549d4, by reading; nothing was run but
`just check`)
- **A-V1 — every §5.2.3 row maps to a real variant, and every variant has a
  row.** Read `ProtocolError`, `BoundsError`, `ScheduleError` in
  `goad_semantics::error` and `BackendError`, `CleanupFailure`, `StateError`
  in `goad_shell::error` against the table, variant by variant: each table
  variant exists with that name, and no variant of those enums lacks a row.
  `InapplicableKey`'s `key` is `&'static str`, so the split on `"fields"` is a
  string compare in the arm. `ExitStatus { code: Option<i32> }` is one variant
  (one row), whatever `code` holds.
- **A-V2 — A-1 holds today.** `grep -rn 'inapplicable(' crates/goad-semantics/src`:
  the only call passing `"fields"` is in `normalize_alternative`; the others
  pass `"min"`, `"max"`, `"options"`. `MissingField`'s only raise site is
  the missing `view` in `normalize_response` (`grep -rn MissingField`), as the
  table's rationale states.
- **A-V3 — the corpora reachable from `normalize.rs`.** `PROTOCOL` and
  `PROTOCOL_TEXT` are `const`s in `normalize.rs`, and `fixtures_of(&Corpus)`
  enumerates either as raw `serde_json::Value`s, so `requirement`, `now` and
  `input` are readable without `runner.rs`' `Envelope`. **The schedule corpus
  is not reachable**: `SCHEDULE` is a private `const` in `runner.rs`, and
  `check_schedule` hides its outcome. So the discard witness needs `runner.rs`,
  and the plan's conditional surface is engaged. The narrowest edits are
  `SCHEDULE` to `pub(crate)` (then `fixtures_of(&SCHEDULE)`), or
  `fixture_paths`/`read_envelope` to `pub(crate)` (then the typed `Envelope`,
  which also parses `now` once). Which is the executor's local decision; a
  second `Corpus` naming the schedule directory in `normalize.rs` is a
  parallel copy and is not one of the options.
- **A-V4 — `fixtures_of` drops the path.** It returns `Vec<Value>`. VA-4
  requires the failure to *name* `protocol/R-25-next-check-of-the-wrong-type`
  and `schedule/R-25-not-a-string`, so the witnesses need each fixture's path:
  `fixtures_of` returns path and value, and its existing callers
  (`tags_named_by_fixtures`, `a_schedule_failure_is_named_by_a_fixture_as_a_discard`)
  ignore the path. Both are in `normalize.rs`.
- **A-V5 — the predicted red set is exactly EX-4's fixtures.** Each error
  fixture's `expect` tag (held equal to the produced error by
  `every_protocol_fixture_states_what_a_wire_document_means`,
  `what_a_json_value_cannot_carry_is_refused_from_the_document_text` and
  `every_scheduling_fixture_states_what_the_protocol_does`, green at EN-1),
  mapped through §5.2.3, against its own list. Read with `jq` over `protocol/`,
  `protocol-text/` and `schedule/`. Only the rows that differ from "id in list" are noted here; every
  other error fixture's id is in its list.

  | corpus / fixture | produced | §5.2.3 id | list today | witness |
  |---|---|---|---|---|
  | `protocol-text/R-17-a-nan-literal-for-a-bound` | `Json` | R-44 | [R-17] | **red** |
  | `protocol-text/R-17-an-infinite-literal-for-a-bound` | `Json` | R-44 | [R-17] | **red** |
  | `protocol/R-52-a-choice-field-with-no-alternatives` | `EmptyAlternatives` | R-16 | [R-52, R-53] | **red** |

  The rest, for the record: every `Shape` fixture lists R-44; each
  `UnsupportedPrimitive` fixture lists R-12; `EmptyOptions` [R-13];
  `DuplicateOptionId` [R-14, R-52]; `Bounds(Inverted)` [R-17]; `NestedHints`
  [R-18, R-47]; `UnsupportedProtocolVersion` [R-3]; `MissingField` [R-10];
  `InapplicableKey` with `min`/`options` [R-50] and with `fields` [R-53];
  `DuplicateAlternativeId` [R-52, R-53]; `DuplicateFieldId` [R-52];
  `DuplicateKey` [R-44] and [R-52, R-44]. **Discard side, green from the first
  run:** the only `Discarded` fixture, `protocol/R-25-next-check-of-the-wrong-type`
  (`NotAString`, R-25, list [R-25, R-51]); `protocol-text/` has no accepted
  fixture. Schedule errors: `TimeOfDay` fixtures [R-21, R-25], `MissingOffset`
  [R-22, R-25], `CalendarUnit` [R-23, R-25], `Unparseable` and `OutOfRange`
  [R-25], `NotAString` [R-21, R-25] — each contains its id.
- **A-V6 — VA-2 and VA-4 are reachable mutations.** `NestedHints` → R-3:
  the only `NestedHints` fixture, `protocol/R-18-a-nested-hints-object`, lists
  [R-18, R-47], so R-3 is outside it. `NotAString` → R-3: neither
  `protocol/R-25-next-check-of-the-wrong-type` [R-25, R-51] nor
  `schedule/R-25-not-a-string` [R-21, R-25] lists R-3.
- **A-V7 — `Requirement`'s printed form matches the fixtures'.** Every
  `requirement` entry is spelled `R-N` (no zero padding, no `SPEC-001/`
  prefix), which is EX-1's display. Comparing the produced id's `Display` to the
  list's strings is therefore exact.
- **A-V8 — the existing builder has one `InapplicableKey`, with `key: "min"`.**
  VT-1's *"exhaustive table of §5.2.3's rows"* has two `InapplicableKey` rows,
  so `every_protocol_error` gains an `InapplicableKey { key: "fields", .. }`
  instance. Its doc (*"One instance per `ProtocolError` variant"*) then needs
  rewording to "per row". The other tests over it
  (`every_protocol_error_display_names_what_it_carries`,
  `the_taxonomy_implements_error_and_wrapping_variants_expose_their_source`)
  hold for any extra instance.
- **A-V9 — stratum 2 has no builder.** `goad_shell::error` has no `mod tests`;
  VT-2's tables need one instance per row of `BackendError` (with a
  `Protocol(p)` delegating row), `CleanupFailure`, `StateError`.
  `std::io::Error` and `ViewId` are constructible in a test (`ViewId` from
  `goad_semantics::protocol::canonical`).
- **A-V10 — lint posture.** `clippy::all` and `unwrap_used`/`expect_used`/
  `panic`/`indexing_slicing` are `deny` workspace-wide, relaxed in tests by
  `clippy.toml`'s `allow-*-in-tests`. Nothing denies a `_` arm, so *"no `_`
  arm"* is held by review (VA-1), not by a lint.

**Assumptions — first tested by this phase**
- **A-T1 — the refusal witness reds on exactly A-V5's set**, no more and no
  fewer, once the methods it calls exist (EX-5). A different set is a STOP.
- **A-T2 — the discard witness reports every failure, across both halves,
  before it fails.** VA-4 requires one failure to name a fixture from each
  half; a witness that panics on the first, or that runs the halves as two
  `assert`s, cannot. The same for the refusal witness and EX-5's set.
- **A-T3 — each witness's per-corpus non-vacuity guard** (VT-3: *"it asserts
  it read at least one fixture from each corpus it covers, counted per corpus,
  and for the discard witness the `Discarded` fixtures and the schedule error
  fixtures separately"*) fails when its directory is empty or renamed. The
  plan asks for no mutation of it; one is cheap (point a root at a missing
  directory), and is offered in the mutation table as optional.
- **A-T4 — `ProtocolError::Schedule` delegation never fires in the refusal
  witness.** `Schedule` is never an `Err` (`normalize.rs`' block comment above
  `every_protocol_error`), so the witness reaches `ScheduleError::requirement()`
  only through `Discarded::Schedule { reason }` and `schedule::parse`.

**STOP conditions** (consult the user; do not improvise)
- From `plan.md` PHASE-01 Notes (quoted): *"STOP if any other fixture's
  produced error falls outside its own list: that is a §5.2.3 row the design
  got wrong, not a list to edit."* Concretely: the EX-5 red set differs from
  A-V5's.
- From the same Notes and `design.md` §5.2.3 *Its reach*: *"the corrections
  EX-4 names are the only lists edited to agree with the code"*. Editing any
  other fixture, or any field of these but `requirement`, is a STOP.
- `design.md` §5.5 A-1: a second raiser of `InapplicableKey { key: "fields" }`
  appears. The witness would fail; the row is the design's.
- Any variant found with no §5.2.3 row, or a row with no variant (A-V1 says
  none today; the tree can move).
- A §5.2.3 row that cannot be written as a total match with no `_` arm.
- A file outside **Surfaces**, including any reporter, `goad`'s renderer or
  the diagnostics surface (`notes.md` §Open: *"`goad`'s own reporters do not
  carry R-59's side and requirement"* is a candidate follow-up, not this
  phase).
- Adding `requirement()` or `fault()` to `ConfigError`, `EnvelopeFault` or
  `SpanFault` (EX-3; R-59 puts them outside its scope).
- A dependency addition (e.g. a variant-enumerating derive: `notes.md` §Open,
  declined by the user).
- ~~**PLAN QUESTION 1** (Findings) unanswered when execution starts: how
  `goad-shell` constructs a `Requirement`.~~ Resolved: see Findings.
- ~~**PLAN QUESTION 2** (Findings) unanswered: the Notes order cannot produce
  EX-5's red.~~ Resolved: see Findings.

**Tasks** — in `plan.md` PHASE-01 Notes order (`plan-log.md` 2026-10-01,
*PHASE-01's red-first order*), quoted: *"1. `Requirement` with its constants
(EX-1), its `Display`, and stratum 1's `requirement()` on `ProtocolError`,
`BoundsError` and `ScheduleError`, red by VT-1's tables first — their
requirement column; `fault()` does not exist yet. 2. The witnesses (VT-3), red
on exactly EX-4's fixtures (EX-5). 3. The list corrections (EX-4); the refusal
witness goes green. 4. `fault()`, stratum 2's `requirement()` and `fault()`
(VT-2), and `AtFault`'s `Display`; VT-1's tables gain their side column."*

- [x] Set PHASE-01 to `in progress` in §Status.
- [x] Print `git log -1 --oneline`; it must be 7b549d4 or a descendant whose
      only changes since are this sheet and the documents its two PLAN
      QUESTIONs' resolutions amended (`design.md`, `plan.md`, the two logs).
      *7bd5cf4 012: Requirement constants; R-56 from stratum 1; PHASE-01
      order* — descendants since 7b549d4 touch only `notes.md`, `design.md`,
      `plan.md`, `design-log.md`, `plan-log.md`.
- **1. `Requirement`, its constants and stratum 1's `requirement()`, red by
  VT-1**
  - [x] EX-1 (quoted, its `Requirement` half): *"`Requirement`'s field is
        private, it has no constructor, and it has exactly the associated
        constants §5.2.3 lists: `R3`, `R10`, `R12`, `R13`, `R14`, `R16`,
        `R17`, `R18`, `R21`, `R22`, `R23`, `R25`, `R32`, `R40`, `R41`, `R43`,
        `R44`, `R45`, `R48`, `R50`, `R52`, `R53` — one per id the table
        answers — and `R56`, for the checker's claim"*. Its `Display` prints
        `R-N` (A-V7).
  - [x] VT-1, requirement column: extend `every_protocol_error` (A-V8);
        write `every_protocol_error_names_a_requirement_and_a_side` and the
        bounds and schedule siblings beside `must_name` — each an exhaustive
        `match` whose expected id is copied from §5.2.3's table, not from the
        code (*tests-asserting-proxies*), and, quoted, *"spelled as the table
        spells it (`"R-44"`) and compared with `Requirement`'s `Display`, so a
        constant whose value disagrees with its name reds it"*. See them red.
  - [x] Stratum 1, in `goad_semantics::error`: `requirement()` on
        `ProtocolError`, `BoundsError`, `ScheduleError` (EX-2), each a total
        `match` with no `_` arm naming a `Requirement` constant,
        `InapplicableKey` split on `key`, `Bounds` and `Schedule` delegating.
        VT-1's requirement column green.
        *Done.* Red: the three tables compiled against `todo!()` bodies and
        failed by panic (a compiling red, *negative-control-must-compile*);
        green with the arms. The expected ids live in `protocol_row`,
        `bounds_row`, `schedule_row` in `error.rs`' `mod tests`, each an
        exhaustive match returning the table's spelling.
- **2. Witnesses, red (EX-5)**
  - [x] `normalize.rs`: give `fixtures_of` the fixture's path (A-V4); reach
        the schedule corpus through `runner.rs` by the narrowest edit (A-V3).
        Extract the per-corpus input route (`to_vec` for `PROTOCOL`, `as_str`
        for `PROTOCOL_TEXT`) so the checkers and the witness share it rather
        than repeating it.
  - [x] Write `every_refusal_fixture_names_a_requirement_in_its_own_list` over
        `PROTOCOL` and `PROTOCOL_TEXT`: run `read_response` on each fixture's
        input at its `now`; for each `Err`, its `requirement()`'s display is in
        the fixture's `requirement`; collect every failure with the fixture
        path, the produced variant and id, and the list; fail once naming all.
        Non-vacuity per corpus (VT-3, A-T3).
  - [x] Write `every_discard_fixture_names_a_requirement_in_its_own_list` over
        `PROTOCOL`'s `Discarded` items and the schedule corpus's error
        fixtures (`schedule::parse`), the same way; non-vacuity counted for the
        `Discarded` fixtures and the schedule error fixtures separately.
  - [x] Run with `--no-fail-fast`. **EX-5**: record the refusal witness's
        failure output below (fixture names, verbatim) and check it is A-V5's
        set exactly (A-T1). The discard witness is green (Notes: *"green from
        its first run; VA-4 is what shows it can fail"*).

        **EX-5 — recorded at the first run, before any list was edited.**
        `cargo test -p goad-semantics --test protocol --no-fail-fast`: 6
        passed, 1 failed. The failure, verbatim (paths absolute in the
        output; the crate-root prefix
        `/home/david/dev/goad/crates/goad-semantics/../../` trimmed here):

        ```
        tests/fixtures/protocol/R-52-a-choice-field-with-no-alternatives.json: {"EmptyAlternatives":{"at":"view.options[0].fields[0].options"}} names R-16, outside ["R-52", "R-53"]
        tests/fixtures/protocol-text/R-17-a-nan-literal-for-a-bound.json: {"Json":null} names R-44, outside ["R-17"]
        tests/fixtures/protocol-text/R-17-an-infinite-literal-for-a-bound.json: {"Json":null} names R-44, outside ["R-17"]
        ```

        Exactly A-V5's set, no more and no fewer: A-T1 holds, no STOP.
        `every_discard_fixture_names_a_requirement_in_its_own_list` green
        from its first run. A-T2 holds by construction: both witnesses push
        every failure, the non-vacuity ones included, into one list and
        assert once.
- **3. List corrections, green**
  - [x] EX-4: `R-17-a-nan-literal-for-a-bound` and
        `R-17-an-infinite-literal-for-a-bound` → `["R-17", "R-44"]`;
        `R-52-a-choice-field-with-no-alternatives` → `["R-52", "R-53",
        "R-16"]`. `requirement` arrays only. Refusal witness green.
  - [x] `git diff --stat -- tests/fixtures` shows exactly those files (EX-4:
        *"no other fixture list is edited"*).
        *Done.* Three files, one line each, the `requirement` line only;
        the refusal witness green.
- **4. `fault()`, stratum 2, `AtFault`'s `Display`**
  - [x] EX-1, its `AtFault` half: `AtFault { Backend, Host, Configuration,
        Environment }`, whose `Display` is *"a total match printing `backend`,
        `host`, `configuration`, `environment`"*. VT-1:
        `every_side_displays_as_the_word_a_report_prints` — *"a table over
        each `AtFault` variant beside an exhaustive match"* — red, then green.
  - [x] VT-1's tables gain their side column, copied from §5.2.3; then
        `fault()` on `ProtocolError`, `BoundsError`, `ScheduleError` (EX-2),
        total, no `_` arm, `Bounds` and `Schedule` delegating.
        *Done.* `every_side_displays_as_the_word_a_report_prints` red against
        a `Display` writing `""` (`left: "" right: "backend"`), then green.
        The three tables, given their side column (each `assert_eq!`s
        `(id display, side)` with the row), red against `todo!()` bodies,
        then green.
  - [x] VT-2: new `mod tests` in `goad_shell::error` with a builder per enum
        (A-V9) and `every_backend_error_names_a_requirement_and_a_side`, and
        the cleanup and state siblings, each from §5.2.3, *"expected ids
        spelled as VT-1's"*. See them red.
  - [x] Stratum 2, in `goad_shell::error`: `requirement()`/`fault()` on
        `BackendError` (`Protocol` delegating), `CleanupFailure`,
        `StateError`, each arm naming a `Requirement` constant. VT-2 green.
        *Done.* VT-2's three tables red against `todo!()` bodies ("not yet
        implemented", all three), then green. `every_backend_error` carries
        `ExitStatus` with and without a code, and `Protocol` wrapping two
        errors with different answers (`MissingField`, `NestedHints`), so the
        delegating row is seen to follow the inner error rather than a fixed
        answer.
  - [x] EX-3: `grep -n 'fn requirement\|fn fault'` over
        `crates/goad-semantics/src/error.rs`, `crates/goad-shell/src/error.rs`,
        `crates/goad-shell/src/ingress/envelope.rs`; no hit in an `impl` of
        `ConfigError`, `EnvelopeFault` or `SpanFault`. Record the hits.
        *Done.* Hits: `impl ProtocolError`, `impl BoundsError`,
        `impl ScheduleError` in stratum 1, and `impl BackendError`,
        `impl CleanupFailure`, `impl StateError` in stratum 2 — a
        `requirement` and a `fault` in each. The one other hit is
        `envelope.rs`' `fn fault(bytes: &str) -> EnvelopeFault`, a free
        helper inside its `mod tests`, not a method. None on `ConfigError`,
        `EnvelopeFault` or `SpanFault`.
- **Refactor**
  - [x] Read the diff for duplication between the witnesses and the existing
        corpus tests (the input route, `now` parsing, the walk). Docs on the
        new types cite §5.2.3 and R-59 by name, never by line.
        *Done.* The input route is one function per corpus (`wire_value`,
        `document_text`, typed `Route`), shared by `check_protocol`,
        `check_protocol_text` and the refusal witness. The walk is the one
        `fixtures_of`. The remaining duplication is `now_of`, which repeats
        `Corpus::run_case`'s `now` parse in one expression: see Decisions.
        Both strata's tables first went through an `assert_row` helper;
        clippy's `needless_pass_by_value` objected in stratum 2, and a
        direct `assert_eq!` of `(id display, side)` against the row was
        less code in both, so the helper went. `BackendError::requirement`'s
        doc named only `Io` for R-45; it now names `PipeMissing` too.
- **Verification**
  - [x] VA-1 (quoted): *"the §5.2.3 table and the code agree row for row, read
        side by side and recorded in the phase sheet; this is the review the
        witness's stated reach leaves (F-23). The same read holds
        `Requirement`'s constants to §5.2.3's list: none missing, none surplus
        — nothing else holds a surplus constant (§5.2.3)."* Record one line
        per row under Decisions or a VA-1 block: variant, table id/side, arm
        id/side; then the constants against §5.2.3's list. Confirm no `_` arm
        in any `requirement()`, `fault()` or `AtFault` `Display` match by
        reading each (A-V10). *Done:* see the VA-1 block below.
  - [x] VA-2: mutation table row 1.
  - [x] VA-3 (quoted): *"`canon-delta.md` SPEC-001 Change 2's test names
        resolve to the shipped cases."* `grep -n` each name Change 2 cites
        (`every_protocol_error_names_a_requirement_and_a_side`, its bounds and
        schedule siblings, `every_backend_error_names_a_requirement_and_a_side`
        and its cleanup and state siblings, both witnesses) in the file Change 2
        places it. A name or file that differs is updated in `canon-delta.md` in
        the same commit and noted here (*Test names are commitments*).
        *Done, no edit.* Change 2 cites
        `crates/goad-semantics/src/error.rs::tests::every_protocol_error_names_a_requirement_and_a_side`,
        `crates/goad-shell/src/error.rs::tests::every_backend_error_names_a_requirement_and_a_side`,
        `normalize.rs::every_refusal_fixture_names_a_requirement_in_its_own_list`
        and `::every_discard_fixture_names_a_requirement_in_its_own_list`,
        and the siblings by description only. `grep -c "fn <name>"` finds each
        once in the file Change 2 places it; the siblings shipped as
        `every_bounds_error_…`, `every_schedule_error_…` (stratum 1) and
        `every_cleanup_failure_…`, `every_state_error_…` (stratum 2), with
        the suffix `_names_a_requirement_and_a_side`. Nothing in
        `canon-delta.md` changed.
  - [x] VA-4: mutation table row 2.
  - [x] `just check` exits 0 on the final commit. Record it.
        *Exit 0* (see *Exit — the gate* below for the commit): build, both
        test tiers (659 passed, 0 failed, summed over every `test result`
        line: EN-1's 644, plus the nine new cases in `--workspace` and the
        six stratum-1 ones again under `-p goad-semantics`), `deno check`,
        clippy with no warning, `cargo fmt --all --check`.
  - [x] §Status: PHASE-01 `done`, with the date.
  - [x] Harvest updated in place (*Fresh as of*, Produced, Learned, Open).

**Mutation evidence** (`plan.md` *Mutation evidence*: copy the file to the
scratchpad and back, never `git checkout`/`git stash`; `--no-fail-fast`;
`git status` clean after each restore; a mutation that does not compile is not
evidence). "Cases it must red" names the plan's case first; the VT table that
also asserts the arm is expected to red with it and is listed second.

| edit | cases it must red | compiled? | redded |
|---|---|---|---|
| VA-2: `ProtocolError::requirement()`'s `NestedHints` arm R-18 → R-3 | `every_refusal_fixture_names_a_requirement_in_its_own_list`, naming `protocol/R-18-a-nested-hints-object`; also `every_protocol_error_names_a_requirement_and_a_side`. The discard witness stays green. | yes | `cargo test -p goad-semantics --no-fail-fast`: exactly `every_protocol_error_names_a_requirement_and_a_side` (lib, 33/1) and `every_refusal_fixture_names_a_requirement_in_its_own_list` (protocol, 6/1), whose one line is `…/tests/fixtures/protocol/R-18-a-nested-hints-object.json: {"NestedHints":{"at":"view.options[0].fields[0]"}} names R-3, outside ["R-18", "R-47"]`. Discard witness green. Restored by copy; `git diff` on the file empty. |
| VA-4: `ScheduleError::requirement()`'s `NotAString` arm R-25 → R-3 | `every_discard_fixture_names_a_requirement_in_its_own_list`, its one failure naming both `protocol/R-25-next-check-of-the-wrong-type` and `schedule/R-25-not-a-string`; also the schedule sibling of `every_protocol_error_names_a_requirement_and_a_side`. The refusal witness stays green (A-T4). | yes — `NotAString` split out of the R-25 or-pattern into its own `=> Requirement::R3` arm | exactly `every_schedule_error_names_a_requirement_and_a_side` (lib, 33/1) and `every_discard_fixture_names_a_requirement_in_its_own_list` (protocol, 6/1), one failure of two lines: `…/protocol/R-25-next-check-of-the-wrong-type.json: NotAString names R-3, outside ["R-25", "R-51"]` and `…/schedule/R-25-not-a-string.json: NotAString names R-3, outside ["R-21", "R-25"]`. Refusal witness green (A-T4 holds), and so is `every_protocol_error_names_a_requirement_and_a_side`, whose `Schedule` instance is `Unparseable`. Restored; diff empty. |
| optional (A-T3): a witness's corpus root pointed at a missing directory | that witness, by its non-vacuity guard | yes — `SCHEDULE.root` pointed at `../../tests/fixtures`, which exists and holds no `.json` file. A *missing* directory makes `fixtures_of` panic on the read before any guard runs, so it would not test the guard. | `every_discard_fixture_names_a_requirement_in_its_own_list` with `../../tests/fixtures: no schedule error fixture read — renamed, emptied, or misspelled`, and `every_scheduling_fixture_states_what_the_protocol_does` by `Corpus::run`'s guard. Restored; diff empty. |

**Decisions taken during execution**
<!-- Small and local: how, within what the design already settled. A choice that
     changes the design is not one of these — stop, consult the user, and record
     it in `design-log.md`. -->

- **The schedule corpus is reached by `SCHEDULE` → `pub(crate)`** (A-V3's
  first option), then `fixtures_of(&SCHEDULE)`. That is one visibility change
  in `runner.rs`, with a doc line saying why. Exposing `read_envelope` and
  `fixture_paths` would have opened two functions and the `Fault` type.
- **`fixtures_of` returns `(PathBuf, Value)` and sorts by path**, so a
  witness's failure lists fixtures in the same order every run. Its two older
  callers ignore the path.
- **The input route is a `Route = fn(&Value) -> Result<Vec<u8>, String>`**,
  one per protocol corpus (`wire_value`, `document_text`). The checkers keep
  their `fn(&Fixture)` shape, which `Corpus` needs.
- **`now_of` parses `now` a second time**, outside the runner. Sharing it
  with `Corpus::run_case` would mean exposing a parse helper from `runner.rs`
  for one expression. The witnesses read fixtures the corpus tests have
  already held well-formed, so `now_of` `expect`s instead of reporting a
  `Malformed` fault.
- **The refusal witness counts refusals, not fixtures**, for its per-corpus
  guard, and the discard witness counts discards and schedule errors. A
  corpus of fixtures that are all accepted would still be a vacuous witness.
- **A row's side is compared by value (`AtFault`), its id by `Display`.**
  `AtFault`'s printed word is held once, by
  `every_side_displays_as_the_word_a_report_prints`.
- **Compiling reds used `todo!()` bodies** for `requirement()` and `fault()`,
  and `write!(f, "")` for `AtFault`'s `Display`; `todo!()` there tripped
  `unused_variables` on `f` and would not compile.
- **Commit 7451420 is red by design.** It records EX-5 with the witness
  failing, and a4087f5 makes it green. No history was rewritten; the next
  phase starts from a green HEAD.

**Findings**
<!-- Things noticed in passing that are not this phase's job: a defect
     elsewhere, drift from the design, a surprise. Defects in this phase's own
     work get fixed, not recorded. These feed the audit; the ones that outlive
     the slice become Follow-ups. -->

- **PLAN QUESTION 1 — how does stratum 2 build a `Requirement`?**
  **Resolved:** associated constants in stratum 1, `Requirement::R44` and so
  on; the field stays private and no crate mints an id; the checker's R-56
  claim is `Requirement::R56` (`design-log.md` 2026-10-01, *`Requirement` is
  built from named constants*; `design.md` §5.2.3; `plan.md` PHASE-01/EX-1).
  The question as raised: `design.md`
  §5.2.3 declares `pub struct Requirement(u16)` with a private field, and
  states no constructor. `goad_shell::error`'s arms (`Spawn` → R-44, `Timeout`
  → R-41, …) are in another crate and cannot write `Requirement(44)`. Neither
  the design, the plan nor either review ledger settles it (`grep -n
  'Requirement('` over the slice's documents finds only the declaration).
  Candidates: a `pub const fn` constructor; named constants in
  `goad_semantics::error`; a public field. Each is new public API that
  `goad-check` will also see, and I-1 (*"`goad-check` contains no requirement
  id … except the R-56 probe's"*) bears on which. Not repaired here.
- **PLAN QUESTION 2 — the Notes order cannot produce EX-5's red.**
  **Resolved:** the order proposed below was taken, and `plan.md` PHASE-01's
  Notes now give it (`plan-log.md` 2026-10-01, *PHASE-01's red-first
  order*); the Tasks follow it. The question as raised: The Notes
  say *"write the witness tests, see the refusal witness fail on the fixtures
  EX-4 names (EX-5), correct the lists, then the methods"*, and the brief for
  this sheet orders Display last. But the witness calls `requirement()` on the
  produced error and compares its printed form with the fixture's strings, so
  it does not compile until `Requirement`, its `Display`, and `requirement()`
  on `ProtocolError`, `BoundsError` and `ScheduleError` exist — and a compile
  failure is not a red (*negative-control-must-compile*). A stub cannot stand
  in: `todo!()` or a constant reds nearly every fixture, not EX-5's set. EX-5's
  red is on exactly A-V5's fixtures only when the stratum-1 arms already answer
  §5.2.3, which is what `design.md` §5.2.3 assumes (*"Today the witness fails on
  … That failure is the red step, and the list corrections are the green
  one"*). An executable order would be: `Requirement` with its `Display`, and
  stratum-1 `requirement()` driven red by VT-1's tables → witnesses red on
  A-V5's set (EX-5) → list corrections → `fault()`, stratum 2, `AtFault`'s
  `Display`. That is a reordering of the plan, so it is the user's to endorse,
  not this sheet's to make.
- **A second `every_protocol_error` builder exists**, in `normalize.rs` (an
  integration target, which cannot reach `error.rs`' `#[cfg(test)]` one). The
  plan's *"do not write a second builder"* is about `error.rs`; this one
  predates the slice and serves `every_reachable_error_in_the_taxonomy_is_named_by_a_fixture`.
  Not this phase's to merge; noted for audit. It was not extended with an
  `InapplicableKey { key: "fields" }` instance: that test checks tags, and
  one `InapplicableKey` covers the tag.
- **VT-2's `Protocol` row asserts delegation, not an id.** Stratum 2 cannot
  see stratum 1's private tables, so `backend_row`'s `Protocol(inner)` arm
  expects `(inner.requirement(), inner.fault())`. Two `Protocol` instances
  with different answers keep a hard-coded arm from passing. That the
  inner answers are right is VT-1's job.
- **`every_protocol_error_names_a_requirement_and_a_side` does not see a
  `NotAString` flip.** Its one `Schedule` instance is `Unparseable`, so
  VA-4's mutation reds the schedule sibling and the discard witness, not the
  protocol table. This is as the mutation table predicted, and it is
  sufficient: `Schedule` delegates, and `schedule_row` is exhaustive.

**VA-1 — §5.2.3's table against the code, row by row** (read side by side at
the final commit; `requirement()` / `fault()` arm as written)

| taxonomy | variant | table id / side | arm id / side |
|---|---|---|---|
| `ProtocolError` | `Json` | R-44 / backend | `R44` / `Backend` |
| | `Shape` | R-44 / backend | `R44` / `Backend` |
| | `DuplicateKey` | R-44 / backend | `R44` / `Backend` |
| | `NestedHints` | R-18 / backend | `R18` / `Backend` |
| | `UnsupportedProtocolVersion` | R-3 / backend | `R3` / `Backend` |
| | `UnsupportedPrimitive` | R-12 / backend | `R12` / `Backend` |
| | `InapplicableKey`, `key == "fields"` | R-53 / backend | `R53` / `Backend` |
| | `InapplicableKey`, otherwise | R-50 / backend | `R50` / `Backend` |
| | `MissingField` | R-10 / backend | `R10` / `Backend` |
| | `EmptyOptions` | R-13 / backend | `R13` / `Backend` |
| | `DuplicateOptionId` | R-14 / backend | `R14` / `Backend` |
| | `DuplicateFieldId` | R-52 / backend | `R52` / `Backend` |
| | `DuplicateAlternativeId` | R-52 / backend | `R52` / `Backend` |
| | `EmptyAlternatives` | R-16 / backend | `R16` / `Backend` |
| | `Bounds(b)` | `b.requirement()` / `b.fault()` | `inner.requirement()` / `inner.fault()` |
| | `Schedule(s)` | `s.requirement()` / `s.fault()` | `inner.requirement()` / `inner.fault()` |
| `BoundsError` | `NotFinite` | R-17 / backend | `R17` / `Backend` |
| | `Inverted` | R-17 / backend | `R17` / `Backend` |
| `ScheduleError` | `NotAString` | R-25 / backend | `R25` / `Backend` |
| | `MissingOffset` | R-22 / backend | `R22` / `Backend` |
| | `TimeOfDay` | R-21 / backend | `R21` / `Backend` |
| | `CalendarUnit` | R-23 / backend | `R23` / `Backend` |
| | `OutOfRange` | R-25 / backend | `R25` / `Backend` |
| | `Unparseable` | R-25 / backend | `R25` / `Backend` |
| `BackendError` | `Spawn` | R-44 / configuration | `R44` / `Configuration` |
| | `Timeout` | R-41 / backend | `R41` / `Backend` |
| | `ExitStatus` | R-40 / backend | `R40` / `Backend` |
| | `OutputTooLarge` | R-43 / backend | `R43` / `Backend` |
| | `PipeMissing` | R-45 / host | `R45` / `Host` |
| | `Io` | R-45 / environment | `R45` / `Environment` |
| | `Protocol(p)` | `p.requirement()` / `p.fault()` | `inner.requirement()` / `inner.fault()` |
| `CleanupFailure` | `TimedOut` | R-48 / backend | `R48` / `Backend` |
| | `Io` | R-48 / environment | `R48` / `Environment` |
| `StateError` | `NoOutstandingView` | R-32 / host | `R32` / `Host` |
| | `StaleViewId` | R-32 / host | `R32` / `Host` |

Every row agrees, and every variant of the six enums has a row (A-V1 holds at
the final commit). **Constants:** `impl Requirement` declares `R3`, `R10`,
`R12`, `R13`, `R14`, `R16`, `R17`, `R18`, `R21`, `R22`, `R23`, `R25`, `R32`,
`R40`, `R41`, `R43`, `R44`, `R45`, `R48`, `R50`, `R52`, `R53`, `R56`. That is
§5.2.3's list and `R56`: none missing, none surplus. Each id the table
answers is named by at least one arm above, and `R56` by none, as its doc
says. **No `_` arm:** read in each `requirement()` and `fault()` of the six
impls and in `AtFault`'s `Display`. `InapplicableKey` is one arm with an `if`
on `key`, so the match stays total over variants. The tests' row functions
(`protocol_row` and the rest) are exhaustive too. `protocol_row` splits
`InapplicableKey` with a `key: "fields"` pattern followed by the general
arm, which is not a `_` arm.

**Exit — the gate.** `just check` exited 0 at the tree committed as the
phase's final commit (see §Harvest *Fresh as of*).

### PHASE-02 — the host's kinds and R-57 values live in stratum 1

**Written by a phase-sheet agent, not the executor**, at 68f8ec4 (*012
PHASE-01: verification, sheet and harvest*). This sheet is the plan's second
reading. Where it restates a plan criterion it quotes it. It repairs nothing:
what reads as wrong in the plan is under **Findings** as a PLAN QUESTION, and
the tasks it blocks are marked `[!]`.

**Objective** (quoted, `plan.md` PHASE-02): *`Stimulus`, `Submitted`,
`Finite` and the as-drawn value per kind each have one encoding, in
`goad_semantics::protocol::canonical`, and `goad` names no kind string and
decides no submitted value's JSON type.*

**Entry**
- **EN-1** (quoted): *"PHASE-01 done (PHASE-01..PHASE-03 run in sequence,
  §Sequencing)."* **Discharged 2026-10-01 at 68f8ec4.** §Status has PHASE-01
  `done`. `just check` exited 0: build, both test tiers (659 passed, 0
  failed, summed over every `test result` line; PHASE-01's exit count),
  `deno check`, clippy with no warning, `cargo fmt --all --check`.
- **EN-2** (quoted): *"`design.md` §5.2.4 says how `view_model::as_drawn`
  delegates to `Submitted::as_drawn`: a `FieldKind` rebuilt from the
  `DrawnKind`, and a private `as_edited` in `view_model.rs` back."*
  **Discharged.** §5.2.4's `Submitted::as_drawn` bullet says it, citing
  `design-log.md` 2026-10-01 *plan review round 1: design-touching
  dispositions* (F-11), which supersedes G1's placement in `draft.rs`.

**Surfaces — a closed list, copied from `plan.md`. Anything else is a STOP.**
- `crates/goad-semantics/src/protocol/canonical.rs`
- `crates/goad/src/wire.rs`, `crates/goad/src/draft.rs`,
  `crates/goad/src/view_model.rs`
- `crates/goad/src/controller.rs`, `crates/goad/src/install.rs`,
  `crates/goad/src/main.rs`, `crates/goad/src/glass.rs` — *"imports and the
  delegation only"*. `glass.rs` needs no edit as far as this reading finds
  (A-V9).
- `crates/goad/tests/` — *"imports of `Stimulus`"*.
- `crates/goad/Cargo.toml` — *"its dependency comment only"*.
- `crates/goad-shell/src/ingress/envelope.rs` — *"`HOST_SOURCE`,
  `plan-log.md` PL-2"*.
- `docs/slices/012/canon-delta.md` — *"test names only"*.
- `docs/slices/012/notes.md` — this sheet, §Status, §Harvest (bookkeeping, by
  `docs/AGENTS.md` §Execute).

Not surfaces, and so a STOP if the work seems to need them:
`crates/goad-semantics/Cargo.toml` (no dependency or feature changes),
`crates/goad/src/lib.rs`, `crates/goad/tests/renderer/fields.rs` (EX-4: green
*unchanged*), any `crates/goad-boundary` file, and any canon document.

**Reading list** (by symbol; `grep -n` then `sed -n`, not whole files)
- `docs/slices/012/plan.md` — §Overview's first paragraph (`just check` is
  every phase's last exit); *Test names are commitments*; *Mutation evidence*
  and *Invariant reads* under §Sequencing & rationale; §PHASE-02 whole.
- `docs/slices/012/design.md` §5.2.4 whole; §5.5 I-2 (the checker reads
  values from `Submitted`, never maps a kind) and the edge *"A `number` field
  declaring `max: -10` and no `min` is answered with `0`"*; §9 *Stratum 1*
  `canonical.rs` bullet and *Stratum 3* whole.
- `docs/slices/012/design-log.md` 2026-10-01 *two gaps the plan draft found
  (G1, G2)* — G1 only; and *plan review round 1: design-touching
  dispositions* — F-11 only.
- `docs/slices/012/plan-log.md` 2026-10-01 *placements the plan draft put to
  the user*, PL-2 and PL-3; *plan review round 1: dispositions*, F-9 and F-10
  (the origins of EX-6's scope and EX-4's literal).
- `docs/slices/012/canon-delta.md` SPEC-001 Change 3 (R-56's row) and Change
  4 (R-57's row) — the test names VA-3 resolves.
- `docs/specs/001-host-backend-protocol.md` §7, rows R-56, R-57 — today's
  citations. They stay until audit promotes the delta.
- Prior art: `docs/memory/` — *negative-control-must-compile*,
  *tests-asserting-proxies*, *mutation-check-the-coverage-claim*. PHASE-01's
  §Harvest *Learned*: a `todo!()` body is a compiling red for a method.

*The moved and touched symbols, every definition and caller.* Found by
`grep -rn --include=*.rs -w '<symbol>' crates` for each type, and `grep -rnw
--include=*.rs '<fn>' crates` for each function, at 68f8ec4; each hit read.

- **`Stimulus`** — defined in `crates/goad/src/wire.rs`, with `Stimulus::kind`
  and `Stimulus::event` (whose doc cites `canonical.rs` by line, EX-5). Unit
  tests in `wire.rs`' `mod tests`: `a_scheduled_stimulus_names_itself_scheduled`,
  `a_scheduled_stimulus_s_event_carries_the_three_normative_fields`; that
  module's `use super::{…, Stimulus, …}` and its `instant` helper and
  `serde_json::Value` import serve only these two.
  Callers in `src`: `main.rs` (`use goad::wire::{…, Stimulus, …}`;
  `Stimulus::Startup` in `start`), `install.rs` (`use crate::wire::{…}`;
  `Stimulus::Requested` in `install`), `controller.rs` (`use crate::wire::{…,
  Stimulus}`; `Stimulus::Scheduled` in `serve`; doc comments on `dispatch`
  and in `serve`). `wire.rs` itself: `Command::Evaluate(Stimulus)`.
  Callers in `crates/goad/tests/`, each importing `goad::wire::{…, Stimulus,
  …}`: `event_loop_answer/answer.rs`, `event_loop_drain/drain.rs`,
  `event_loop_full/full.rs`, `event_loop_schedule/scheduling.rs`,
  `renderer/ingress.rs`, `renderer/scheduling.rs`, `renderer/wiring.rs` (its
  top-level import, and the nested test modules' own `use goad::wire::{…}`
  lines; one nested module takes it through `use super::{…}`, which follows
  the top-level import). No other crate names it.
- **`Submitted`** — no occurrence today. New.
- **`Finite`** — defined in `crates/goad/src/draft.rs` (`Finite::ZERO`,
  `Finite::new`, `Finite::get`, and the doc on why it has no `Eq`); its test
  `a_finite_refuses_every_number_json_cannot_carry` in `draft.rs`' `mod
  tests`. Callers: `view_model.rs` (`use crate::draft::{Edited, Finite,
  Reported}`; `drawn_number`, `adjusted`, `held_number`, `interpret`; the
  test module's import and its `finite` helper); `wire.rs` (the doc on
  `Command` only). `glass.rs` reads `Edited::Adjusted { number, .. }` and
  calls `number.get()` without naming the type.
- **`Edited`** — defined in `draft.rs`, mapped by `draft::submitted`, held by
  `Draft::state_of` / `Draft::record`. Callers: `view_model.rs` (`adjusted`,
  `held_number`, `as_drawn`, `untouched`, `interpret`, tests); `glass.rs`
  (`use crate::draft::Edited`; `overlaid`, `field_value`); `controller.rs`
  (via `as_drawn` and `submitted` in `answer`); `wire.rs` (docs only).
- **`DrawnKind`** — defined in `view_model.rs`, built only by `drawn_form`
  (whose `Choice` arm clones `alternatives.first().id()` into `first`, with a
  comment that EX-7 makes stale). Matched in `view_model.rs` by `as_drawn`
  (the only reader of `first`), `untouched`, `interpret` (`Choice {
  alternatives, .. }`), and the tests (`a_choice` builds `first` by
  `alternatives.as_slice()[0]`; `as_drawn_answers_every_kind` destructures
  it). Matched in `glass.rs` by `markup_kind`, `slider_bounds_of`,
  `alternatives_of` — each with `Choice { .. }` or `Choice { alternatives, ..
  }`, so none breaks when `first` goes. `lib.rs` names it in a comment only.
- **`view_model::as_drawn`** — callers: `controller.rs` `answer` (`state_of(…)
  .unwrap_or_else(|| as_drawn(&field.kind))`); `view_model::untouched`; tests
  `as_drawn_answers_every_kind` and
  `an_untouched_field_submits_what_canon_delta_cd_1_states`.
- **`adjusted`** (private, `view_model.rs`) — callers: `as_drawn`'s number arm,
  and `interpret`'s `Reported::AdjustedValue` arm. Its doc names both, and
  must stay true (EX-3).
- **`spelled`** (private, `view_model.rs`) — callers: `adjusted`; test
  `a_number_spells_short_and_re_parses_to_the_number_it_came_from` and its
  round-trip sibling.
- **`draft::submitted`** (`pub(crate)`) — callers: `controller.rs` `answer`;
  `draft.rs` tests `a_boolean_field_submits_a_json_boolean`,
  `each_kind_submits_the_json_type_r_57_names`,
  `a_picked_datetime_submits_the_offset_it_was_picked_in`; `view_model.rs`
  test `an_untouched_field_submits_what_canon_delta_cd_1_states`.
- **`drawn_number`** (private, `view_model.rs`, not named by the brief but
  part of the untouched policy) — callers: `as_drawn`'s number arm, and
  `interpret`'s `Reported::AdjustedText` fallback. See PLAN QUESTION 2.
- **`HOST_SOURCE`'s literals** — `grep -rn '"host"' crates/*/src`:
  production code: `Stimulus::event` (`wire.rs`); `envelope.rs`' `envelope`
  (`if source == "host"`); **`AtFault`'s `Display` in
  `goad_semantics::error`** (`Self::Host => "host"`, a side, not a source —
  PLAN QUESTION 1). Comments: `envelope.rs` (`EnvelopeFault::ReservedSource`'s
  doc, `envelope`'s doc), `goad-shell/src/ingress/mod.rs`, `goad-emit/src/args.rs`,
  `controller.rs` (`dispatch`'s doc). Tests: `canonical.rs`
  (`an_evaluate_serializes_to_the_spec_s_wire_form`,
  `every_request_kind_carries_the_version_and_a_discriminant`), `envelope.rs`
  (`a_reserved_source_is_refused_with_every_other_field_valid`), `wire.rs`
  (the moving Stimulus test), `goad_semantics::error`'s `mod tests` (the
  `AtFault` table).
- **Kind literals** — `grep -rn '"startup"\|"requested"\|"scheduled"'
  crates/*/src`: in `crates/goad/src`, only `wire.rs` (`Stimulus`'s doc,
  `Stimulus::kind`, the two moving tests). Elsewhere, `canonical.rs`' request
  serialization tests only.
- `crates/goad-semantics/src/protocol/canonical.rs`: `Event` (where
  `Stimulus` lands beside); `FieldKind`, `NumberRange` (`min`, `max`),
  `Alternatives::first` and its doc and `#[expect]` reason (both cite
  `Alternatives::new` by line — see Findings), `AlternativeId::new`
  (`pub(super)`: minted only inside `protocol`), `Timestamp`; the
  module-level `#![deny(clippy::arithmetic_side_effects)]`; the test
  module's helpers `alternative`, `instant`, `json`.
- `crates/goad-boundary/tests/checks/purity.rs`:
  `the_real_stratum_1_source_names_none_of_the_nine` (VA-2's reach);
  `structure.rs`: `wire_rs_names_tokio_spawn_only_after_its_cfg_test_line`
  and `the_subject_directories_are_found_and_are_not_empty` (A-V8).

**Assumptions — verified now** (at 68f8ec4, by reading and grep; nothing was
run but `just check`)
- **A-V1 — no move makes a stratum-1 type name stratum 2 or 3 (ADR-001).**
  `Stimulus` names `Event`, `Timestamp`, `serde_json::Value`. `Submitted`
  names `bool`, `String`, `Finite`, `AlternativeId`, `Timestamp`,
  `jiff::tz::Offset`. `Finite` names `f64`. `Submitted::as_drawn` reads
  `FieldKind`, `NumberRange`, `Alternatives::first`. `HOST_SOURCE` is a
  `&str`. Every one is stratum 1 or a stratum-1 dependency. The arrows that
  remain point down: `goad` (3) names `Stimulus`, `Submitted`, `Finite`;
  `envelope.rs` (2) names `HOST_SOURCE` and already imports
  `goad_semantics::protocol::canonical::{Event, Timestamp}`. The one upward
  reference is prose: see Findings, *`Finite`'s doc names stratum 3*.
- **A-V2 — no feature is needed (VA-1, POL-001's residue).** Stratum 1 takes
  `jiff` with `default-features = false` (workspace `Cargo.toml`). Read in the
  locked `jiff` 0.2.35 source: `Timestamp::display_with_offset`,
  `tz::Offset`, `Offset::constant`, `Offset::UTC`,
  `Timestamp::UNIX_EPOCH` carry no `cfg` gate. `serde_json::Value::from(f64)`
  is unconditional. `canonical.rs` already formats a `Timestamp` through
  `Display` (`impl Serialize for Timestamp`).
- **A-V3 — `goad-boundary`'s instruments reach the new code.** The purity
  scan's `Scan` in `the_real_stratum_1_source_names_none_of_the_nine` walks
  `crates/goad-semantics/src` recursively, excluding only directories named
  `tests` and `target`; `canonical.rs` is under it and already scanned. The
  vocabulary scan walks every workspace member. The manifest allowlist's
  `STRATUM_1` is `jiff`, `serde`, `serde_json`, and no manifest changes. The
  crate edge is Cargo resolution. `cargo test -p goad-semantics` is in the
  gate. Landing in `canonical.rs` (EX-1, EX-2 name it) creates no file, so
  VA-2's planted-breach arm does not fire.
- **A-V4 — `FieldKind` can be rebuilt in `goad`.** It is a `pub enum` with no
  `#[non_exhaustive]`, so `FieldKind::Number(range)` and `FieldKind::Choice {
  alternatives }` are constructible outside the crate. `NumberRange` is
  `Copy`; `Alternatives` is `Clone`.
- **A-V5 — `goad` cannot mint an `AlternativeId`.** `AlternativeId::new` is
  `pub(super)`. So EX-4's literal expectation (`"first"`) must be compared
  through `AlternativeId::as_str` (or `submitted` to JSON), not by
  constructing `Edited::Chosen(AlternativeId)`. VT-3 and VT-4 in `goad` take
  their ids off a normalized view, as `draft.rs`' `an_alternative_id` and
  `view_model.rs`' `a_choice` already do. Inside `canonical.rs`' `mod tests`
  the moved value tests can mint one (`alternative`).
- **A-V6 — `draft.rs` imports nothing from `view_model` today** (its `use`
  lines: `goad_semantics::protocol::canonical`, `jiff::tz::Offset`). EX-3
  holds it.
- **A-V7 — EX-1's literal rule is satisfiable.** Every `"startup"`,
  `"requested"`, `"scheduled"` in `crates/goad/src` is in `wire.rs`, in what
  moves (Kind literals above).
- **A-V8 — `goad-boundary`'s structure checks survive the move.**
  `wire_rs_names_tokio_spawn_only_after_its_cfg_test_line` needs `wire.rs` to
  name `tokio::spawn` after its `#[cfg(test)]`: it is in
  `stopped_does_not_resolve_until_stop_is_called`, which stays.
  `the_subject_directories_are_found_and_are_not_empty`'s floor for
  `crates/goad/src` is 1000 production lines against roughly 1900 measured;
  the move takes out well under a hundred.
- **A-V9 — `glass.rs` needs no edit.** It names `Edited` and `DrawnKind` but
  not `Stimulus`, `Finite` or `first`; its `DrawnKind::Choice` patterns use
  `..`.
- **A-V10 — `fields.rs` holds the canon-cited R-57/R-58 cases**
  (`every_untouched_kind_leaves_the_host_with_the_json_type_r57_names`,
  `every_operated_kind_leaves_the_host_with_the_json_type_r57_names`,
  `an_untouched_datetime_reads_not_set_on_screen_and_submits_the_epoch`,
  `choosing_an_alternative_submits_its_id_where_the_field_id_is_the_options_own`,
  `an_unbounded_number_submits_what_was_typed_and_invents_no_range`,
  `a_field_id_shared_by_two_options_is_two_keys_and_only_the_answered_ones_are_sent`),
  each found once by `grep -c "fn <name>"`. It imports no `Stimulus`, so
  EX-4's *unchanged* means no diff to the file at all.
- **A-V11 — `goad` still needs `serde_json` after the move** (EX-5 rewrites
  the comment, it does not drop the dependency): `draft::submitted` returns
  `serde_json::Value`; `view_model.rs`' `Run::of` matches
  `serde_json::Value::String` on a hint; the tests build documents with
  `serde_json::json!`.

**Assumptions — first tested by this phase**
- **A-T1 — the moved tests compile unchanged in `canonical.rs`.** Lints are
  workspace-wide, so the same `clippy.toml` test allowances apply; the
  module's own `arithmetic_side_effects` deny is the one difference.
- **A-T2 — every new case can be made red by a compiling stub** (`todo!()` in
  `Submitted::to_json`, `Submitted::as_drawn`, `Edited::submitted`,
  `as_edited`), per PHASE-01's *Learned*. The moved cases have no red: a move
  of a passing case is green on arrival, and the move is the refactor of
  existing behaviour, not new behaviour. The tasks record each moved case as
  *moved, green*.
- **A-T3 — the renderer tier sees no change** (EX-4): the delegated
  `view_model::as_drawn` returns, for every `DrawnKind`, the `Edited` it
  returns today, including `Adjusted`'s text spelled by `spelled`.
- **A-T4 — `Stimulus` needs no `Serialize`/`Eq` beyond its derives today**
  (`Debug, Clone, Copy, PartialEq, Eq`), and `Submitted` needs `Debug,
  Clone, PartialEq` (no `Eq`: it carries a `Finite`).

**STOP conditions** (consult the user; do not improvise)
- From `plan.md` PHASE-02 Notes (quoted): *"`clippy::pub_use` is denied:
  `goad` imports from `goad_semantics`, no re-export."* A `pub use` anywhere
  to keep an old path alive is a STOP.
- (quoted) *"`DrawnKind` is deliberately not `FieldKind` (its own doc, D10).
  Do not make it one."* Replacing `DrawnKind` with `FieldKind`, or carrying a
  `FieldKind` inside it, is a STOP.
- (quoted) *"`DrawnKind::Choice.first` goes in the refactor step, after the
  delegation is green (PL-3)."* Removing it before is an ordering breach, not
  a STOP; record it.
- From `design.md` §5.2.4: `as_edited` is private to `view_model.rs` and
  *"not a crate-wide `From` impl"*. A `From<Submitted> for Edited`, or the
  conversion anywhere but beside `adjusted`, is a STOP.
- `draft.rs` gaining any `use crate::view_model` (EX-3).
- Any dependency or feature change, in any manifest (VA-1; POL-001's
  residue). A clippy or compile error that seems to need one is a STOP.
- A new file under `crates/goad-semantics/src`. EX-1 and EX-2 name
  `canonical.rs`; VA-2's reach proof then applies, but the placement itself
  departs from the plan.
- `crates/goad/tests/renderer/fields.rs` needing any edit, or any
  renderer-tier case going red (EX-4). That is a behaviour change.
- A test name differing from `design.md` §9 or `canon-delta.md`: update
  `canon-delta.md` in the same commit and say so here (*Test names are
  commitments*). Not a STOP, but never silent.
- A file outside **Surfaces**.
- [!] **PLAN QUESTION 1** unanswered when EX-6's grep is recorded.
- [!] **PLAN QUESTION 2** unanswered when the delegation task starts.
- [!] **PLAN QUESTION 3** unanswered when the value tests move.

**Tasks** — the plan gives no red-first order for this phase beyond PL-3's
placement of EX-7. The order below makes every new case red by a compiling
stub before its body (A-T2), and puts each move before the code that depends
on it.

- [ ] Set PHASE-02 to `in progress` in §Status.
- [ ] Print `git log -1 --oneline`; it must be 68f8ec4 or a descendant whose
      only changes since are this sheet and whatever the PLAN QUESTIONs'
      resolutions amended.
- **1. `Stimulus` and `HOST_SOURCE` (EX-1, EX-5, EX-6, VT-1)**
  - [ ] Move `Stimulus`, `Stimulus::kind`, `Stimulus::event` into
        `canonical.rs` beside `Event` (in the *Outbound: requests* section,
        after `Alternatives`, so no line citation above it moves — Findings).
        EX-1 (quoted): *"`kind` and `event` unchanged"*. `Stimulus::event`'s
        doc cites `Event` by symbol, not `canonical.rs:490-497` (EX-5).
  - [ ] VT-1: move `a_scheduled_stimulus_names_itself_scheduled` and
        `a_scheduled_stimulus_s_event_carries_the_three_normative_fields`
        *"verbatim"* into `canonical.rs`' `mod tests` (its `instant` helper
        has the same shape). Drop `wire.rs`' now-unused test imports. Moved,
        green (A-T2).
  - [ ] Every caller imports `goad_semantics::protocol::canonical::Stimulus`:
        `main.rs`, `install.rs`, `controller.rs`, and the test files
        listed in the reading list. `wire.rs` drops `Event`, `Timestamp` and
        `serde_json::Value` from its production imports if nothing else uses
        them.
  - [ ] EX-6: `pub const HOST_SOURCE: &str` beside `Stimulus`;
        `Stimulus::event` writes `HOST_SOURCE.to_owned()`; `envelope.rs`'
        `envelope` compares `source == HOST_SOURCE`. Tests keep `"host"`
        (quoted: *"they witness the wire spelling, and are not a second
        encoding of it"*).
  - [ ] EX-1's grep: `grep -rn '"startup"\|"requested"\|"scheduled"'
        crates/goad/src` — no hit. Record it.
  - [!] EX-6's grep (quoted command): `grep -rn '"host"' crates/*/src`, each
        hit read and recorded as a comment or a test. **It cannot come out
        as worded** — PLAN QUESTION 1. Record every hit regardless.
- **2. `Finite` and `Submitted::to_json` (EX-2, VT-2's value half)**
  - [ ] Move `Finite` into `canonical.rs` *"unchanged, together with its doc
        on why it has no `Eq`"* (`design.md` §5.2.4; see Findings on that
        doc). `goad`'s users import it from `goad_semantics`.
  - [ ] Declare `Submitted` — quoted: *"`Boolean(bool)`, `Text(String)`,
        `Number(Finite)`, `Choice(AlternativeId)`, and `DateTime { instant:
        Timestamp, offset: Offset }`"* — and `Submitted::to_json(&self) ->
        serde_json::Value` with a `todo!()` body.
  - [!] Move `draft.rs`' value tests into `canonical.rs` against
        `Submitted`, and write `every_submitted_kind_writes_the_json_type_r57_names`
        — which of the moved cases it is, if any, is PLAN QUESTION 3.
        `a_finite_refuses_every_number_json_cannot_carry` moves with
        `Finite` and is green on arrival. See the rest red against the stub.
  - [ ] `Submitted::to_json`'s body is today's `draft::submitted` body over
        `Submitted` (`design.md` §5.2.4), its comments moved with their
        arms. Green.
- **3. The projection (EX-2, VT-3)**
  - [ ] `Edited::submitted(&self) -> Submitted` with a `todo!()` body; write
        `the_projection_to_submitted_is_the_identity_on_each_kind` in
        `draft.rs` — each `Edited` variant against the `Submitted` it
        projects to, `Adjusted`'s text dropped (ids off `an_alternative_id`,
        A-V5). Red.
  - [ ] Implement it, deciding no JSON type. `draft::submitted` becomes
        (quoted) *"`edited.submitted().to_json()`"*. Green; renderer tier
        green.
- **4. `Submitted::as_drawn` (EX-3's first half, VT-2's as-drawn half)**
  - [ ] `Submitted::as_drawn(&FieldKind) -> Submitted` with a `todo!()`
        body; write `an_as_drawn_choice_submits_the_first_alternative` and a
        sibling per kind (quoted: *"including the `number` min-or-zero and
        `datetime` epoch cases"*; the `max: -10`, no-`min` case is §5.5's
        edge and belongs here). Each expectation is a literal from the
        fixture or §5.2.4's list, never the expression the code computes
        (*tests-asserting-proxies*). Red.
  - [ ] Implement: *"`false`, `""`, the minimum or `0`, the first
        alternative, and the epoch at `+00:00`"*, the choice arm over
        `Alternatives::first` (F-25). Green.
- **5. The delegation (EX-3, VT-4)**
  - [ ] Private `as_edited(Submitted) -> Edited` beside `adjusted`, with a
        `todo!()` body; write
        `as_edited_projects_back_to_the_submitted_it_was_given_on_each_kind`
        in `view_model.rs`' tests. Red.
  - [ ] Implement, spelling a number *"through `adjusted`"*. Green.
  - [ ] `view_model::as_drawn` keeps its signature and delegates: rebuild the
        `FieldKind` from the `DrawnKind`, call `Submitted::as_drawn`, convert
        through `as_edited`. `as_drawn_answers_every_kind`,
        `an_untouched_field_submits_what_canon_delta_cd_1_states` and the
        renderer tier green unchanged.
  - [!] `interpret`'s untouched-number fallback, and with it
        `drawn_number` — PLAN QUESTION 2.
- **Refactor**
  - [ ] EX-7 (PL-3): `DrawnKind::Choice` loses `first` and *"its stale doc
        goes with it"*. `drawn_form`'s `Choice` arm and its comment (which
        argues for cloning `first`) follow; so does any doc that still says
        the kind carries the first id (`grep -n 'first' view_model.rs`).
  - [ ] EX-4 (quoted): *"the helper `a_choice` drops `first`, and
        `as_drawn_answers_every_kind`'s choice expectation becomes the
        fixture's literal alternative id, `"first"` — not
        `alternatives.first()`"*. Compared through `as_str` (A-V5). No other
        `as_drawn` or `untouched` test changes.
  - [ ] Docs made true by the move, by symbol: `draft.rs`' module doc
        (*"`submitted` is the single application of `SPEC-001/R-57`"*),
        `Edited`'s doc (*"the only thing `submitted` maps"*),
        `draft::submitted`'s doc (the site that breaks on a new protocol kind
        is now `Submitted::as_drawn`'s match too, per Change 4); `adjusted`'s
        doc names its callers as they now are. `wire.rs`' `Command` doc
        still reads true.
  - [ ] EX-5: `crates/goad/Cargo.toml`'s comment no longer names
        `Stimulus::event` among `goad`'s reasons for `serde_json` (A-V11 has
        the reasons that remain).
  - [ ] Read the diff for a second statement of any value the phase moved:
        a kind string, a JSON type choice, an untouched value, `"host"`.
- **Verification**
  - [ ] EX-3's structural half: `grep -n 'use crate::view_model'
        crates/goad/src/draft.rs` — no hit; `untouched` still calls
        `as_drawn` for every kind but `datetime`.
  - [ ] EX-4: `git diff --stat 68f8ec4 -- crates/goad/tests/renderer/fields.rs`
        empty; renderer tier green.
  - [ ] VA-1 (quoted): *"`cargo test -p goad-semantics` (the gate's stratum-1
        command) builds the moved code with stratum 1's own features; no
        feature was added to a dependency shared with stratum 1 (POL-001's
        residue)."* Record the command's result and `git diff 68f8ec4 --
        '*Cargo.toml'` (EX-5's comment only).
  - [ ] VA-2 (quoted): *"If they land in `canonical.rs`, record that no new
        file was created."* `git diff --stat --diff-filter=A 68f8ec4 --
        crates/goad-semantics/src` empty. If not empty, mutation table row 1.
  - [ ] VA-3 (quoted): *"`canon-delta.md` SPEC-001 Changes 3–4's test names
        resolve."* `grep -c "fn <name>"` in the file each change places it:
        `canonical.rs` — `a_scheduled_stimulus_names_itself_scheduled`,
        `a_scheduled_stimulus_s_event_carries_the_three_normative_fields`,
        `every_submitted_kind_writes_the_json_type_r57_names`; `draft.rs` —
        `the_projection_to_submitted_is_the_identity_on_each_kind`;
        `crates/goad/tests/renderer/fields.rs` — Change 4's cases (A-V10).
        Change 3's `goad-check` names are PHASE-04's and PHASE-12's. A name
        that differs is updated in `canon-delta.md` in the same commit.
  - [ ] `design.md` §9's PHASE-02 names, by the same grep:
        `an_as_drawn_choice_submits_the_first_alternative`,
        `as_edited_projects_back_to_the_submitted_it_was_given_on_each_kind`;
        record the siblings' shipped names.
  - [ ] Optional mutation rows (below); none is a plan criterion.
  - [ ] `just check` exits 0 on the final commit. Record it.
  - [ ] §Status: PHASE-02 `done`, with the date.
  - [ ] Harvest updated in place (*Fresh as of*, Produced, Learned, Open).

**Mutation evidence** (`plan.md` *Mutation evidence*: copy the file to the
scratchpad and back, never `git checkout`/`git stash`; `--no-fail-fast`;
`git status` clean after each restore; a mutation that does not compile is not
evidence). The plan's VA items name one planted breach, and only
conditionally; the rows marked optional are offered because no VA item
otherwise shows a new case can fail.

| edit | cases it must red | compiled? | redded |
|---|---|---|---|
| VA-2, **only if a new file was created under `crates/goad-semantics/src`**: a `std::fs` call planted in that file, in code that compiles (e.g. `let _ = std::fs::metadata(".");` in a function body) | `the_real_stratum_1_source_names_none_of_the_nine`. Compilable as worded: `std::fs` resolves in stratum 1, which is why the scan exists. If no file was created, this row reads *not applicable*, with VA-2's record. | | |
| optional: `Submitted::to_json`'s `Number` arm writes `Value::String(number.get().to_string())` | `every_submitted_kind_writes_the_json_type_r57_names`; through `draft::submitted`, `an_untouched_field_submits_what_canon_delta_cd_1_states` and the renderer tier's `every_untouched_kind_leaves_the_host_with_the_json_type_r57_names` | | |
| optional: `Submitted::as_drawn`'s `Choice` arm takes the last alternative (`alternatives.as_slice().last()`, falling back to `first()` to stay total) | `an_as_drawn_choice_submits_the_first_alternative`; `as_drawn_answers_every_kind` after EX-4 — the case EX-4's literal exists to make fail; `an_untouched_field_submits_what_canon_delta_cd_1_states` | | |
| optional, expected **not** to red: `as_edited`'s number arm builds `Edited::Adjusted { text: number.get().to_string(), number }`, bypassing `adjusted` | none expected: VT-4's round trip discards the text, and every spelled number in the tests is under 24 characters, where `spelled` and `to_string` agree. Recording it shows EX-3's *"through `adjusted`"* is held by review | | |

**Decisions taken during execution**
<!-- Small and local: how, within what the design already settled. A choice that
     changes the design is not one of these — stop, consult the user, and record
     it in `design-log.md`. -->

**Findings**
<!-- Things noticed in passing that are not this phase's job: a defect
     elsewhere, drift from the design, a surprise. Defects in this phase's own
     work get fixed, not recorded. These feed the audit; the ones that outlive
     the slice become Follow-ups. -->

- **PLAN QUESTION 1 — EX-6's `"host"` rule cannot be met as worded.** EX-6
  says *"Outside comments and `#[cfg(test)]` modules, no `"host"` string
  literal remains in `crates/*/src` … each hit read and recorded as a comment
  or a test."* Production hits remain by construction, and none is a
  comment or a test:
  - `HOST_SOURCE`'s own definition, `pub const HOST_SOURCE: &str = "host";`,
    which EX-6 itself creates.
  - `AtFault`'s `Display` in `goad_semantics::error` (`Self::Host =>
    "host"`), which PHASE-01/EX-1 requires. It is the side word a report
    prints, not the reserved source, and routing it through `HOST_SOURCE`
    would couple two meanings that happen to share a spelling.
  Review F-9 scoped the rule before PHASE-01 added `AtFault`'s `Display`,
  and did not consider the constant's own line. Options: (a) EX-6 records
  each hit as a comment, a test, *`HOST_SOURCE`'s definition*, or *`AtFault`'s
  side word*, and nothing else; (b) (a) without the `AtFault` exception, by
  changing `AtFault`'s printed form — a PHASE-01 and §5.2.3 change, not
  recommended. **Recommendation: (a).** PHASE-12's I-1 grep is over
  `crates/goad-check/src` only and is unaffected.
- **PLAN QUESTION 2 — `interpret` restates the untouched number.**
  EX-3 says *"`Submitted::as_drawn` is the one statement of the untouched-value
  policy"*. `drawn_number` (min-or-zero, with the doc that states the
  `max: -10` consequence) has two callers: `as_drawn`'s number arm, which the
  delegation replaces, and `interpret`'s `Reported::AdjustedText` fallback for
  an untouched field (*"the number it was drawn showing"*, held by
  `an_untouched_numeric_field_falls_back_to_what_it_was_drawn_showing`). If
  `drawn_number` stays for `interpret`, the policy is stated twice; neither
  the design nor the plan names it. Options:
  (a) `interpret` derives the fallback from `as_drawn(kind)` through
  `held_number`, with a `Finite::ZERO` default for the `None` the types cannot
  rule out but that cannot occur; `drawn_number` is deleted. No new API; the
  dead default follows `drawn_number`'s own precedent (*"a total expression is
  cheaper than an argument"*), and no behaviour reads it.
  (b) Stratum 1 exposes the number half as one public function (e.g. on
  `NumberRange` or `Finite`) that `Submitted::as_drawn`'s number arm and
  `interpret` both call. Total, no dead default, but new stratum-1 API that
  §5.2.4 does not list — a design change.
  (c) Keep `drawn_number` for `interpret` and narrow EX-3 to *what is
  submitted*. Two statements of one rule.
  **Recommendation: (a)** — no design change, one statement, and
  `an_untouched_numeric_field_falls_back_to_what_it_was_drawn_showing`
  holds it unchanged.
- **PLAN QUESTION 3 — is `every_submitted_kind_writes_the_json_type_r57_names`
  a new case or the moved `each_kind_submits_the_json_type_r_57_names`?**
  VT-2 names both *"`every_submitted_kind_writes_the_json_type_r57_names`"*
  and *"`draft.rs`' value tests moved … and the rest of that group"*. The rest
  of that group is `each_kind_submits_the_json_type_r_57_names`, which
  asserts the JSON type of every kind but `boolean`; `canon-delta.md` Change 4 cites only
  the new name. Moving both would assert the same types twice. Options:
  (a) the moved case is renamed `every_submitted_kind_writes_the_json_type_r57_names`
  and gains the `boolean` clause so *every* is true;
  `a_boolean_field_submits_a_json_boolean` moves verbatim as the
  both-directions case SPEC-001's row has cited; (b) both, as worded.
  **Recommendation: (a)** — the name canon will cite covers every variant,
  and nothing is asserted twice. A test-name decision, so the user's.
- **`Finite`'s doc names stratum 3.** It argues the private field from
  *"`Controller::edit` is public"*, and the missing `Eq` from the derives on
  `Edited` and `wire.rs`' `Command`. §5.2.4 says it moves *"unchanged,
  together with its doc"*. In `canonical.rs` that is a stratum-1 doc citing
  stratum-3 symbols. No instrument reads it (the crate edge is Cargo
  resolution), and the argument stays true. The executor may keep it verbatim
  as designed; noted for audit, with the alternative of restating the reasons
  as *both writers* (the renderer and the checker).
- **Line citations in `canonical.rs` already break the symbol rule.**
  `Alternatives::first`'s doc and its `#[expect]` reason cite
  `Alternatives::new` as *":361"*, *":362-364"* and *"twenty lines above"*;
  `Stimulus::event`'s doc cites *"`canonical.rs:490-497`"*, already wrong
  (`Event` has moved since). EX-5 covers the second. Placing the new items
  after `Alternatives` keeps the first set from rotting further; converting
  them to symbol citations is in a surface file and is the refactor step's
  call.
- **`canon-delta.md` Change 3 does not mention `HOST_SOURCE`.** Its emission
  sentence becomes *"`Stimulus::event` the one place it is built"*, still
  true; the reserved spelling then has a name the row could cite. Test names
  only are this phase's to edit, so noted for audit.

**Exit — the gate.** *(Executor: record `just check` on the phase's final
commit.)*

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-10-01 · PHASE-01 done · the commit after 0f16450 (*012 PHASE-01: verification, sheet and harvest*)

### Produced
<!-- What now exists: modules, contracts, docs. -->

- `goad_semantics::error::{Requirement, AtFault}`. `Requirement` is built
  from associated constants only (§5.2.3's list and `R56`) and displays
  `R-N`. `AtFault` displays as the word a report prints.
- `requirement()` / `fault()` on `ProtocolError`, `BoundsError`,
  `ScheduleError` (stratum 1) and `BackendError`, `CleanupFailure`,
  `StateError` (stratum 2). Each is a total match answering §5.2.3.
- Tables `every_*_names_a_requirement_and_a_side` (six), and
  `every_side_displays_as_the_word_a_report_prints`.
- Corpus witnesses in `normalize.rs`:
  `every_refusal_fixture_names_a_requirement_in_its_own_list` and
  `every_discard_fixture_names_a_requirement_in_its_own_list`. `runner.rs`'
  `SCHEDULE` is now `pub(crate)`.
- EX-4's three corrected `requirement` lists.

### Learned
<!-- Durable facts a future agent would otherwise rediscover. Candidates for
     `docs/memory/`. -->

- **A missing corpus directory does not reach a witness's vacuity guard.**
  `fixtures_of` panics on the read first. A guard's mutation must point at
  a directory that exists and is empty of fixtures (PHASE-01 mutation table,
  optional row).
- **A `todo!()` stub is a compiling red for a method, not for a `Display`
  impl.** The unused `Formatter` fails `-D unused`; `write!(f, "")` works.

### Open
<!-- Still unresolved at this point. Candidates for follow-ups. -->

- **Two `every_protocol_error` builders** (`error.rs` `mod tests`,
  `normalize.rs`). They predate the slice and cannot share across an
  integration target. Audit's call (PHASE-01 Findings).

- **Brief §21 AC-14 has no home.** The interstitial-journal scenario (brief §18)
  is to be replaced by a better one the user has in mind; neither 012 nor a
  slice of its own discharges the journal as written (`design-log.md`,
  2026-09-26, OQ-1). The roadmap's coverage row 14 still says *"012, or its own
  slice"*. Candidate follow-up at close.

- **`claude plugin eval` and `claude plugin details` exist** (Claude Code
  2.1.280). `eval` runs eval cases against a plugin with a no-plugin baseline
  arm, and `details` reports a plugin's projected token cost. Both bear on
  AC-1/AC-8's walk method, which was decided before they were known
  (`research.md` R-d). For design: adopt, or say why not.
- **SPEC-004 OQ-2** — `goad-emit`'s statuses are owned but not governed. The
  checker entering §4 makes that asymmetry sharper (`research.md`
  *Cross-thread* 4).
- **`claude plugin eval` as a regression harness for kit edits.** Not adopted
  for AC-1 (`design-log.md`, 2026-09-27, OQ-7). Candidate follow-up at close.
- **`goad`'s own reporters do not carry R-59's side and requirement.** R-59 was
  narrowed to what a refusal names; the diagnostics surface (SPEC-003/R-15)
  and `goad`'s stderr are not bound to show both (`design-log.md`, 2026-09-30).
  Rendering work once R-59 lands. Candidate follow-up at close.
- **`Shape` refusals cite R-44 only.** Naming the requirement the backend
  misread needs normalization to report a path for serde failures
  (`design-log.md`, 2026-09-26, OQ-8). Candidate follow-up at close.
- ~~**R-16 has no non-empty clause for a `choice` field's `options`.**~~
  **Closed in this slice** (`design-log.md` 2026-09-30, *design review round
  2: dispositions*; review F-38): `canon-delta.md` SPEC-001 Change 6 adds "at
  least one" to R-16, and `EmptyAlternatives` cites R-16.
- **The jail library fixes the home per profile.** A home-name parameter
  upstream in `davidlee/nix-config` is cleaner than a local bind (design.md
  §6 OQ-7). The walk no longer needs either: it runs in an oubliette capsule
  (2026-09-30). Candidate follow-up only if a jail is used again.
- **An untouched `number` field can submit a value outside its own range.**
  `view_model::as_drawn` answers min-or-zero, so `max: -10` with no `min`
  submits `0`. Existing host behaviour; the checker mirrors it (design.md §5.5
  edges). Candidate follow-up against the renderer.
- **`goad-check --now`.** Not built: a time-gated backend is checked at an
  hour it speaks, and the report says when no view was returned
  (`design-log.md` 2026-09-30, U3; review F-7). Build it only if walk friction
  names it; it strains SPEC-001/R-7's "current instant" and reaches only
  backends that read `now`. Candidate follow-up.
- **A variant-enumerating derive for the reference coverage test.** The
  coverage test's instances are hand-kept, so a new variant compiles once its
  arm exists, with or without an instance: compile gate plus review, as
  SPEC-003 §7's R-14 row states for the same pattern (design.md §5.2.6;
  review F-22). A derive such as `strum`'s would make it an assertion; the
  user declined it for now (`design-log.md` 2026-09-30, *R-59 reframed;
  PipeMissing; F-22; round 2's unbriefed repairs*). Candidate follow-up, with
  SPEC-003's R-14 case as a second user.
- **FU-7's citation extends to `goad-check`.** Nothing bills a stratum-3
  manifest, so `goad-check` linking no renderer is held by its manifest
  comment and review (design.md §5.5 I-6; review F-26). Extend FU-7 at close.
- **FU-5's citation extends to whatever `goad-check`'s tests copy.** The
  fence scanner is not a copy: it is shared from `tests/support/`, included by
  `goad-check`'s `kit` target and `goad-shell`'s `integration` target, and it
  replaces `round_trip.rs`' `fenced_block` (design.md §5.2.6; `design-log.md`
  2026-10-01, *plan review round 1: design-touching dispositions*).
  `goad-check`'s binary tier includes the existing `tests/support/` files
  where it uses every symbol in them; each helper it copies instead — the
  binary-tier helpers FU-5 names (the spawn, `code_of`, `stderr_of`,
  `stdout_of`) or a helper promising a unique temp path — is named here by
  symbol when it ships, with the file it could not include (plan.md
  PHASE-04/VA-7, PHASE-12/VA-7). Extend FU-5 at close with those names, or
  record that none was copied.

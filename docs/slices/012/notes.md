# Notes — Slice 012

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 | done | 2026-10-01 |
| PHASE-02 | done | 2026-10-01 |
| PHASE-03 | done | 2026-10-01 |
| PHASE-04 | done | 2026-10-01 |
| PHASE-05 | pending | 2026-10-01 |
| PHASE-06 | pending | 2026-10-01 |
| PHASE-07 | pending | 2026-10-01 |
| PHASE-08 | pending | 2026-10-01 |
| PHASE-09 | pending | 2026-10-01 |
| PHASE-10 | pending | 2026-10-01 |
| PHASE-11 | pending | 2026-10-01 |
| PHASE-12 | done | 2026-10-01 |

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
- ~~**PLAN QUESTION 1** unanswered when EX-6's grep is recorded.~~
  Resolved: `plan-log.md` 2026-10-01, *PHASE-02 sheet questions*, 1.
- ~~**PLAN QUESTION 2** unanswered when the delegation task starts.~~
  Resolved: `design-log.md` 2026-10-01, *the drawn number has one home:
  `NumberRange::drawn`*.
- ~~**PLAN QUESTION 3** unanswered when the value tests move.~~
  Resolved: `plan-log.md` 2026-10-01, *PHASE-02 sheet questions*, 3.

**Tasks** — the plan gives no red-first order for this phase beyond PL-3's
placement of EX-7. The order below makes every new case red by a compiling
stub before its body (A-T2), and puts each move before the code that depends
on it.

- [x] Set PHASE-02 to `in progress` in §Status.
- [x] Print `git log -1 --oneline`; it must be 68f8ec4 or a descendant whose
      only changes since are this sheet and whatever the PLAN QUESTIONs'
      resolutions amended.
- **1. `Stimulus` and `HOST_SOURCE` (EX-1, EX-5, EX-6, VT-1)**
  - [x] Move `Stimulus`, `Stimulus::kind`, `Stimulus::event` into
        `canonical.rs` beside `Event` (in the *Outbound: requests* section,
        after `Alternatives`, so no line citation above it moves — Findings).
        EX-1 (quoted): *"`kind` and `event` unchanged"*. `Stimulus::event`'s
        doc cites `Event` by symbol, not `canonical.rs:490-497` (EX-5).
  - [x] VT-1: move `a_scheduled_stimulus_names_itself_scheduled` and
        `a_scheduled_stimulus_s_event_carries_the_three_normative_fields`
        *"verbatim"* into `canonical.rs`' `mod tests` (its `instant` helper
        has the same shape). Drop `wire.rs`' now-unused test imports. Moved,
        green (A-T2).
  - [x] Every caller imports `goad_semantics::protocol::canonical::Stimulus`:
        `main.rs`, `install.rs`, `controller.rs`, and the test files
        listed in the reading list. `wire.rs` drops `Event`, `Timestamp` and
        `serde_json::Value` from its production imports if nothing else uses
        them.
  - [x] EX-6: `pub const HOST_SOURCE: &str` beside `Stimulus`;
        `Stimulus::event` writes `HOST_SOURCE.to_owned()`; `envelope.rs`'
        `envelope` compares `source == HOST_SOURCE`. Tests keep `"host"`
        (quoted: *"they witness the wire spelling, and are not a second
        encoding of it"*).
  - [x] EX-1's grep: `grep -rn '"startup"\|"requested"\|"scheduled"'
        crates/goad/src` — no hit. Record it.
  - [x] EX-6's grep (quoted, as amended by PLAN QUESTION 1's resolution):
        *"`grep -rn '"host"' crates/*/src`, each hit read and recorded as a
        comment, a test, `HOST_SOURCE`'s definition or `AtFault::Host`'s
        word, and nothing else"*. Record every hit with its class.
        `AtFault`'s `Display` keeps its literal; it is not routed through
        `HOST_SOURCE`.
  - *Done.* `git log -1` printed `0ef7fff` at start. VT-1's two cases moved
    verbatim, green on arrival (`cargo test -p goad-semantics stimulus`: 2
    passed). `wire.rs` lost `Event`, `Timestamp`, `serde_json::Value` from
    production imports and `Timestamp`, `Value`, `instant` from its tests.
    `canonical.rs`' test module gained `use serde_json::Value` for the moved
    case. Callers import `goad_semantics::protocol::canonical::Stimulus`;
    where a file already imported from `canonical`, the name joined that
    line (`controller.rs`, `answer.rs`, `drain.rs`, `full.rs`, `scheduling.rs`
    under `renderer/`, and `wiring.rs`' nested module holding
    `FieldId, UserResponse`). `wiring.rs`' nested `use super::{…, Stimulus,
    …}` follows the top-level import unchanged.
  - **EX-1 record:** `grep -rn '"startup"\|"requested"\|"scheduled"'
    crates/goad/src` — no hit.
  - **EX-6 record:** `grep -rn '"host"' crates/*/src`, each hit read:
    - `canonical.rs` `HOST_SOURCE` — **`HOST_SOURCE`'s definition**.
    - `canonical.rs` `an_evaluate_serializes_to_the_spec_s_wire_form` (two
      hits), `every_request_kind_carries_the_version_and_a_discriminant`,
      `a_scheduled_stimulus_s_event_carries_the_three_normative_fields` —
      **tests** (inside `mod tests`).
    - `goad_semantics::error` `AtFault`'s `Display` (`Self::Host => "host"`)
      — **`AtFault::Host`'s word**.
    - `goad_semantics::error` `every_side_displays_as_the_word_a_report_prints`
      (`AtFault::Host => "host"`) — **test**.
    - `envelope.rs` `EnvelopeFault::ReservedSource`'s doc, `envelope`'s doc,
      the `VT-9` section comment in `mod tests` — **comments**;
      `a_reserved_source_is_refused_with_every_other_field_valid` — **test**.
    - `goad-shell/src/ingress/mod.rs` (a doc on the refusal cases),
      `goad-emit/src/args.rs` (a doc on value meaning), `controller.rs`
      `dispatch`'s doc — **comments**.
    Nothing else. `dispatch`'s doc says `Stimulus::event` *"hard-codes
    `source: "host"`"*; still true of the value, so left (Findings).
- **2. `Finite` and `Submitted::to_json` (EX-2, VT-2's value half)**
  - [x] Move `Finite` into `canonical.rs` *"unchanged, together with its doc
        on why it has no `Eq`"* (`design.md` §5.2.4; see Findings on that
        doc). `goad`'s users import it from `goad_semantics`.
  - [x] Declare `Submitted` — quoted: *"`Boolean(bool)`, `Text(String)`,
        `Number(Finite)`, `Choice(AlternativeId)`, and `DateTime { instant:
        Timestamp, offset: Offset }`"* — and `Submitted::to_json(&self) ->
        serde_json::Value` with a `todo!()` body.
  - [x] Move `draft.rs`' value tests into `canonical.rs` against
        `Submitted` (VT-2, as amended by PLAN QUESTION 3's resolution,
        quoted): *"`each_kind_submits_the_json_type_r_57_names` moved as
        `every_submitted_kind_writes_the_json_type_r57_names`, gaining the
        `boolean` clause so it covers every `Submitted` variant — one case,
        not a second asserting the same types"*.
        `a_boolean_field_submits_a_json_boolean` and
        `a_picked_datetime_submits_the_offset_it_was_picked_in` move as
        named. `a_finite_refuses_every_number_json_cannot_carry` moves with
        `Finite` and is green on arrival. See the rest red against the stub.
  - [x] `Submitted::to_json`'s body is today's `draft::submitted` body over
        `Submitted` (`design.md` §5.2.4), its comments moved with their
        arms. Green.
- **3. The projection (EX-2, VT-3)**
  - [x] `Edited::submitted(&self) -> Submitted` with a `todo!()` body; write
        `the_projection_to_submitted_is_the_identity_on_each_kind` in
        `draft.rs` — each `Edited` variant against the `Submitted` it
        projects to, `Adjusted`'s text dropped (ids off `an_alternative_id`,
        A-V5). Red.
  - [x] Implement it, deciding no JSON type. `draft::submitted` becomes
        (quoted) *"`edited.submitted().to_json()`"*. Green; renderer tier
        green.
- **4. `Submitted::as_drawn` (EX-3's first half, VT-2's as-drawn half)**
  - [x] `NumberRange::drawn(&self) -> Finite` with a `todo!()` body; write
        `an_untouched_number_is_drawn_at_its_minimum_or_zero` in
        `canonical.rs` (VT-2, quoted: *"a declared minimum, no bounds, and
        `max: -10` with no `min`"*), each expectation a literal. Red.
        Implement it as `drawn_number`'s body, and move `drawn_number`'s doc
        onto it (the `max: -10` consequence, the CD-1 pointer; `design.md`
        §5.2.4). Green.
  - [x] `Submitted::as_drawn(&FieldKind) -> Submitted` with a `todo!()`
        body; write `an_as_drawn_choice_submits_the_first_alternative` and a
        sibling per kind (quoted, VT-2 as amended: *"including the `number`
        case (the minimum, or zero, through `NumberRange::drawn`) and the
        `datetime` epoch case"*; the `max: -10`, no-`min` case is §5.5's
        edge and belongs here). Each expectation is a literal from the
        fixture or §5.2.4's list, never the expression the code computes
        (*tests-asserting-proxies*). Red.
  - [x] Implement: *"`false`, `""`, the minimum or `0`, the first
        alternative, and the epoch at `+00:00`"*, the number arm calling
        `NumberRange::drawn`, the choice arm over `Alternatives::first`
        (F-25). Green.
  - *Done.* `an_untouched_number_is_drawn_at_its_minimum_or_zero` red against
    `NumberRange::drawn`'s stub (1 of 20 failed), green with
    `drawn_number`'s body over `self.min`. Its doc moved, CD-1 now *slice
    007's*. `Submitted::as_drawn`'s cases red against its stub (the stub's
    parameter spelled `_kind`, since an unused one fails `-D unused`):
    `an_as_drawn_choice_submits_the_first_alternative`,
    `an_as_drawn_boolean_submits_false`,
    `an_as_drawn_text_submits_the_empty_string`,
    `an_as_drawn_number_submits_its_minimum_or_zero` (the minimum, no bounds,
    and `max: -10` with no `min`),
    `an_as_drawn_datetime_submits_the_epoch_at_utc` — 5 of 25 failed; green
    with the body. Every expectation a literal: the choice's is
    `AlternativeId::new("first")`, minted in-module; the epoch's is
    `instant("1970-01-01T00:00:00Z")`. The datetime arm's comment moved from
    `view_model::as_drawn`.
- **5. The delegation (EX-3, VT-4)**
  - [x] Private `as_edited(Submitted) -> Edited` beside `adjusted`, with a
        `todo!()` body; write
        `as_edited_projects_back_to_the_submitted_it_was_given_on_each_kind`
        in `view_model.rs`' tests. Red.
  - [x] Implement, spelling a number *"through `adjusted`"*. Green.
  - [x] `view_model::as_drawn` keeps its signature and delegates: rebuild the
        `FieldKind` from the `DrawnKind`, call `Submitted::as_drawn`, convert
        through `as_edited`. `as_drawn_answers_every_kind`,
        `an_untouched_field_submits_what_canon_delta_cd_1_states` and the
        renderer tier green unchanged.
  - [x] `interpret`'s untouched-number fallback calls `NumberRange::drawn`,
        and `drawn_number` is deleted (EX-3 as amended by PLAN QUESTION 2's
        resolution, quoted: *"`NumberRange::drawn` is the one statement of
        the drawn number — its minimum, or zero — called by
        `Submitted::as_drawn`'s number arm and by `view_model::interpret`'s
        untouched fallback; `view_model::drawn_number` is gone"*).
        `an_untouched_numeric_field_falls_back_to_what_it_was_drawn_showing`
        green unchanged.
  - [x] EX-3's greps (quoted): *"`grep -rn 'drawn_number' crates` finds
        nothing; `grep -rn 'min()' crates/*/src`, each hit read and
        recorded, finds no second minimum-or-zero rule"*. Record each hit.
  - *Done.* `as_edited_projects_back_to_the_submitted_it_was_given_on_each_kind`
    red against `as_edited`'s stub (`not yet implemented`; 1 of 13 in `-p
    goad --lib view_model`), green with the body; its `Number` arm calls
    `adjusted`. `view_model::as_drawn` rebuilds the `FieldKind` and delegates;
    `interpret` calls `range.drawn()`; `drawn_number` deleted, with
    `view_model.rs`' now-unused `Timestamp` and `jiff::tz::Offset` imports.
    `cargo test -p goad --no-fail-fast` green throughout, with
    `as_drawn_answers_every_kind`,
    `an_untouched_field_submits_what_canon_delta_cd_1_states`,
    `an_untouched_numeric_field_falls_back_to_what_it_was_drawn_showing` and
    the renderer tier unchanged.
  - **EX-3 record.** `grep -rn 'drawn_number' crates` first found two hits
    this reading did not foresee: the new sibling
    `an_as_drawn_number_submits_its_minimum_or_zero` (*as_drawn_number* is a
    substring), renamed `an_as_drawn_range_submits_its_minimum_or_zero`; and
    a comment in `glass.rs`' `field_value` (*"the same trade
    `view_model::drawn_number` takes"*), re-pointed at `NumberRange::drawn`
    (Decisions). Then no hit. `grep -rn 'min()' crates/*/src`: `canonical.rs`
    `a_range_with_one_bound_or_none_is_accepted` — a test;
    `view_model::slider_bounds` (`exact_f32(range.min()?)?`) — reads the
    minimum and defaults nothing. `NumberRange::drawn` reads the field
    `self.min`, inside `NumberRange`'s own `impl`. No second
    minimum-or-zero rule.
- **Refactor**
  - [x] EX-7 (PL-3): `DrawnKind::Choice` loses `first` and *"its stale doc
        goes with it"*. `drawn_form`'s `Choice` arm and its comment (which
        argues for cloning `first`) follow; so does any doc that still says
        the kind carries the first id (`grep -n 'first' view_model.rs`).
  - [x] EX-4 (quoted): *"the helper `a_choice` drops `first`, and
        `as_drawn_answers_every_kind`'s choice expectation becomes the
        fixture's literal alternative id, `"first"` — not
        `alternatives.first()`"*. Compared through `as_str` (A-V5). No other
        `as_drawn` or `untouched` test changes.
  - [x] Docs made true by the move, by symbol: `draft.rs`' module doc
        (*"`submitted` is the single application of `SPEC-001/R-57`"*),
        `Edited`'s doc (*"the only thing `submitted` maps"*),
        `draft::submitted`'s doc (the site that breaks on a new protocol kind
        is now `Submitted::as_drawn`'s match too, per Change 4); `adjusted`'s
        doc names its callers as they now are; `view_model::as_drawn`'s doc
        no longer says `interpret` applies it for the fallback number
        (`interpret` calls `NumberRange::drawn`); `NumberRange::drawn`'s
        moved doc says *slice 007's* CD-1, not bare `canon-delta.md` CD-1.
        `wire.rs`' `Command` doc still reads true.
  - [x] EX-5: `crates/goad/Cargo.toml`'s comment no longer names
        `Stimulus::event` among `goad`'s reasons for `serde_json` (A-V11 has
        the reasons that remain).
  - [x] Read the diff for a second statement of any value the phase moved:
        a kind string, a JSON type choice, an untouched value, `"host"`.
  - *Done* (040233a). EX-7: `DrawnKind::Choice` is `{ alternatives }`; its
    doc is one line; `drawn_form`'s `Choice` arm and the comment arguing for
    `first` are gone. `drawn_form`'s doc paragraph on PHASE-05 said
    `DrawnKind::Choice` *"carries"* the first id — put in the past tense, the
    history it is. `view_model.rs`' own `Choice { alternatives, .. }`
    patterns lost the `..`; `glass.rs`' keep theirs (they compile, and
    `glass.rs` is imports-only). EX-4: `a_choice` drops `first`;
    `as_drawn_answers_every_kind`'s choice clause now reads the `Edited::Chosen`
    out and compares `as_str()` with `"first"`. No other `as_drawn` or
    `untouched` test changed. Docs: `draft.rs`' module doc, `Edited`'s doc
    (the match is `Edited::submitted`'s), `draft::submitted`'s doc (two sites
    break on a new protocol kind: `Submitted::as_drawn` and `drawn_form`),
    `adjusted`'s callers (`as_edited`, for `as_drawn`, and the slider),
    `view_model::as_drawn` (*two* call sites; it delegates), `interpret`'s
    doc (applies `NumberRange::drawn`). `Alternatives::first`'s doc and
    `#[expect]` reason now cite `Alternatives::new` by symbol, without line
    numbers or *"twenty lines above"* (the Findings item; a surface file).
    EX-5: `Cargo.toml`'s comment names `draft::submitted`'s return type,
    `view_model.rs`' reading of a `group` hint, and `driving.rs`.
  - **Second-statement read.** `grep -rn
    'UNIX_EPOCH\|Value::Bool\|Value::from(\|display_with_offset\|unwrap_or(0.0)\|Checked(false)\|Typed(String::new())'
    crates/*/src`, non-comment hits read: the R-57 and untouched values occur
    in `canonical.rs` (`NumberRange::drawn`, `Submitted::as_drawn`,
    `Submitted::to_json`) and in tests only. Others, not this phase's values:
    `instant.rs`' clock-failure fallback to the epoch; `glass.rs`' on-screen
    `display_with_offset` of a picked datetime (presentation); ingress and
    `error.rs` reads of JSON. Kind strings and `"host"`: EX-1, EX-6 records.
- **Verification**
  - [x] EX-3's structural half: `grep -n 'use crate::view_model'
        crates/goad/src/draft.rs` — no hit; `untouched` still calls
        `as_drawn` for every kind but `datetime`.
  - [x] EX-4: `git diff --stat 68f8ec4 -- crates/goad/tests/renderer/fields.rs`
        empty; renderer tier green.
  - [x] VA-1 (quoted): *"`cargo test -p goad-semantics` (the gate's stratum-1
        command) builds the moved code with stratum 1's own features; no
        feature was added to a dependency shared with stratum 1 (POL-001's
        residue)."* Record the command's result and `git diff 68f8ec4 --
        '*Cargo.toml'` (EX-5's comment only).
  - [x] VA-2 (quoted): *"If they land in `canonical.rs`, record that no new
        file was created."* `git diff --stat --diff-filter=A 68f8ec4 --
        crates/goad-semantics/src` empty. If not empty, mutation table row 1.
  - [x] VA-3 (quoted): *"`canon-delta.md` SPEC-001 Changes 3–4's test names
        resolve."* `grep -c "fn <name>"` in the file each change places it:
        `canonical.rs` — `a_scheduled_stimulus_names_itself_scheduled`,
        `a_scheduled_stimulus_s_event_carries_the_three_normative_fields`,
        `every_submitted_kind_writes_the_json_type_r57_names`; `draft.rs` —
        `the_projection_to_submitted_is_the_identity_on_each_kind`;
        `crates/goad/tests/renderer/fields.rs` — Change 4's cases (A-V10).
        Change 3's `goad-check` names are PHASE-04's and PHASE-12's. A name
        that differs is updated in `canon-delta.md` in the same commit.
  - [x] `design.md` §9's PHASE-02 names, by the same grep:
        `an_as_drawn_choice_submits_the_first_alternative`,
        `as_edited_projects_back_to_the_submitted_it_was_given_on_each_kind`;
        record the siblings' shipped names.
  - [x] Optional mutation rows (below); none is a plan criterion.
  - [x] `just check` exits 0 on the final commit. Record it.
  - [x] §Status: PHASE-02 `done`, with the date.
  - [x] Harvest updated in place (*Fresh as of*, Produced, Learned, Open).
  - **Records.** EX-3: `grep -n 'use crate::view_model'
    crates/goad/src/draft.rs` — no hit; `untouched` unchanged, calling
    `as_drawn` for every kind but `datetime`. EX-4: `git diff --stat 68f8ec4
    -- crates/goad/tests/renderer/fields.rs` — empty; renderer tier green.
    VA-1: `cargo test -p goad-semantics` green inside `just check` (lib 46
    passed, `protocol` tier 7); `git diff 68f8ec4 -- '*Cargo.toml'` is EX-5's
    comment in `crates/goad/Cargo.toml` and nothing else. VA-2: `git diff
    --stat --diff-filter=A 68f8ec4 -- crates/goad-semantics/src` — empty; no
    file created, mutation row 1 not applicable. VA-3: `grep -c "fn <name>()"`
    is 1 for each: in `canonical.rs`
    `a_scheduled_stimulus_names_itself_scheduled`,
    `a_scheduled_stimulus_s_event_carries_the_three_normative_fields`,
    `every_submitted_kind_writes_the_json_type_r57_names`; in `draft.rs`
    `the_projection_to_submitted_is_the_identity_on_each_kind`; in
    `fields.rs` each A-V10 case. `canon-delta.md` needed no change. §9:
    `an_as_drawn_choice_submits_the_first_alternative` (`canonical.rs`) and
    `as_edited_projects_back_to_the_submitted_it_was_given_on_each_kind`
    (`view_model.rs`) — 1 each; siblings as shipped under Decisions.

**Mutation evidence** (`plan.md` *Mutation evidence*: copy the file to the
scratchpad and back, never `git checkout`/`git stash`; `--no-fail-fast`;
`git status` clean after each restore; a mutation that does not compile is not
evidence). The plan's VA items name one planted breach, and only
conditionally; the rows marked optional are offered because no VA item
otherwise shows a new case can fail.

Each row: `cargo test --workspace --no-fail-fast` on the mutated tree (exit
101, no compile error), file restored by copying the scratchpad backup back,
`git status --short` empty after each restore. The tree mutated was 040233a.

| edit | cases it must red | compiled? | redded |
|---|---|---|---|
| VA-2, **only if a new file was created under `crates/goad-semantics/src`**: a `std::fs` call planted in that file, in code that compiles (e.g. `let _ = std::fs::metadata(".");` in a function body) | `the_real_stratum_1_source_names_none_of_the_nine`. Compilable as worded: `std::fs` resolves in stratum 1, which is why the scan exists. If no file was created, this row reads *not applicable*, with VA-2's record. | not applicable | not applicable — no file created (VA-2) |
| optional: `Submitted::to_json`'s `Number` arm writes `Value::String(number.get().to_string())` | `every_submitted_kind_writes_the_json_type_r57_names`; through `draft::submitted`, `an_untouched_field_submits_what_canon_delta_cd_1_states` and the renderer tier's `every_untouched_kind_leaves_the_host_with_the_json_type_r57_names` | yes | all expected, and more: `every_submitted_kind_writes_the_json_type_r57_names`, `an_untouched_field_submits_what_canon_delta_cd_1_states`, `fields::every_untouched_kind_leaves_the_host_with_the_json_type_r57_names`, `fields::every_operated_kind_leaves_the_host_with_the_json_type_r57_names`, `fields::an_unbounded_number_submits_what_was_typed_and_invents_no_range`, `fields::a_number_draws_a_slider_where_one_can_be_operated_and_a_text_field_otherwise`, `fields::a_number_too_long_to_write_out_is_drawn_in_scientific_notation`, `fields::a_numeric_text_the_parse_refuses_is_recorded_and_leaves_the_number_alone`, `numeric_guard::a_present_inside_the_window_does_not_write_a_zero_back_over_a_cleared_field`, `wiring::editing::an_answer_carries_a_value_for_every_drawn_field_of_the_option_it_names` |
| optional: `Submitted::as_drawn`'s `Choice` arm takes the last alternative (`alternatives.as_slice().last()`, falling back to `first()` to stay total) | `an_as_drawn_choice_submits_the_first_alternative`; `as_drawn_answers_every_kind` after EX-4 — the case EX-4's literal exists to make fail; `an_untouched_field_submits_what_canon_delta_cd_1_states` | yes (`alternatives.as_slice().last().unwrap_or_else(\|\| alternatives.first())`) | all expected — `an_as_drawn_choice_submits_the_first_alternative`, `as_drawn_answers_every_kind`, `an_untouched_field_submits_what_canon_delta_cd_1_states` — and `fields::every_untouched_kind_leaves_the_host_with_the_json_type_r57_names`, `fields::every_operated_kind_leaves_the_host_with_the_json_type_r57_names`, `fields::a_choice_field_draws_a_combo_box_over_the_alternatives_labels`, `fields::choosing_an_alternative_submits_its_id_where_the_field_id_is_the_options_own`, `reassert::a_second_present_corrects_nothing_and_a_widget_the_host_never_heard_from_is_corrected` |
| `NumberRange::drawn` returns `Finite::ZERO`, ignoring the minimum | `an_untouched_number_is_drawn_at_its_minimum_or_zero`; through `interpret`, `an_untouched_numeric_field_falls_back_to_what_it_was_drawn_showing` | yes | all expected — `an_untouched_number_is_drawn_at_its_minimum_or_zero`, `an_untouched_numeric_field_falls_back_to_what_it_was_drawn_showing` — and `an_as_drawn_range_submits_its_minimum_or_zero`, `as_drawn_answers_every_kind`, `an_untouched_field_submits_what_canon_delta_cd_1_states`, `fields::a_number_draws_a_slider_where_one_can_be_operated_and_a_text_field_otherwise`, `fields::a_number_too_long_to_write_out_is_drawn_in_scientific_notation`, `fields::a_numeric_text_the_parse_refuses_is_recorded_and_leaves_the_number_alone` |
| optional, expected **not** to red: `as_edited`'s number arm builds `Edited::Adjusted { text: number.get().to_string(), number }`, bypassing `adjusted` | none expected: VT-4's round trip discards the text, and every spelled number in the tests is under 24 characters, where `spelled` and `to_string` agree. Recording it shows EX-3's *"through `adjusted`"* is held by review | yes | **one, against expectation:** `fields::a_number_too_long_to_write_out_is_drawn_in_scientific_notation` — the renderer tier draws an untouched number whose `Display` exceeds 24 characters, so `spelled` and `to_string` disagree there. EX-3's *"through `adjusted`"* is held by that case, not by review alone (Findings) |

**Decisions taken during execution**
<!-- Small and local: how, within what the design already settled. A choice that
     changes the design is not one of these — stop, consult the user, and record
     it in `design-log.md`. -->

- **`glass.rs`' one comment edit.** `glass.rs` is a surface for *"imports and
  the delegation only"*, and A-V9 found it needed no edit. EX-3's
  `drawn_number` grep found a comment in `field_value` naming the deleted
  function. EX-3 requires no hit, and a comment naming a deleted symbol is
  false, so it now names `NumberRange::drawn` — the same trade, at its new
  home. One comment word; no code. Taken as within the surface, since the
  file is listed and the edit is what an exit criterion requires; flagged for
  the orchestrator in case it reads as a STOP.
  **Accepted by the user:** `plan-log.md` 2026-10-01, *PHASE-02's `glass.rs`
  comment; PHASE-03 sheet questions; the push before a lock bump*.
- **Sibling names** for `an_as_drawn_choice_submits_the_first_alternative`:
  `an_as_drawn_boolean_submits_false`,
  `an_as_drawn_text_submits_the_empty_string`,
  `an_as_drawn_range_submits_its_minimum_or_zero` (*range*, not *number*,
  so EX-3's `drawn_number` grep stays empty),
  `an_as_drawn_datetime_submits_the_epoch_at_utc`.
- **`Edited::submitted` is `pub`**, like `Edited` itself; **`HOST_SOURCE`'s
  doc cites `SPEC-001/R-56`.**

**Findings**
<!-- Things noticed in passing that are not this phase's job: a defect
     elsewhere, drift from the design, a surprise. Defects in this phase's own
     work get fixed, not recorded. These feed the audit; the ones that outlive
     the slice become Follow-ups. -->

- **PLAN QUESTION 1 — EX-6's `"host"` rule cannot be met as worded.**
  **Resolved** as recommended, (a): `plan-log.md` 2026-10-01, *PHASE-02
  sheet questions*, 1; `plan.md` EX-6 amended. As raised: EX-6
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
  **Resolved** as (b), not the recommended (a): `NumberRange::drawn`
  (`design-log.md` 2026-10-01, *the drawn number has one home:
  `NumberRange::drawn`*); `design.md` §5.2.4 and `plan.md` EX-3, VT-2
  amended. As raised: EX-3 says *"`Submitted::as_drawn` is the one
  statement of the untouched-value policy"*. `drawn_number` (min-or-zero, with the doc that states the
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
  **Resolved** as recommended, (a): `plan-log.md` 2026-10-01, *PHASE-02
  sheet questions*, 3; `plan.md` VT-2 amended. As raised: VT-2 names both
  *"`every_submitted_kind_writes_the_json_type_r57_names`"* and *"`draft.rs`' value tests moved … and the rest of that group"*. The rest
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

- **Mutation row 5 redded a case.** The sheet expected none; the renderer
  tier's `a_number_too_long_to_write_out_is_drawn_in_scientific_notation`
  went red, because an untouched number past 24 characters is spelled
  differently by `to_string`. So `as_edited`'s *"through `adjusted`"* is held
  by a test for long numbers, and by review for the rest.
- **`dispatch`'s doc in `controller.rs`** says `Stimulus::event`
  *"hard-codes `source: "host"`"*. Still true of the value, now spelled
  through `HOST_SOURCE`; left, since `controller.rs` is imports-only. Audit
  may re-point it.
- **`envelope.rs`' docs** on `EnvelopeFault::ReservedSource` and `envelope`
  still spell `source == "host"`; true, and they witness the wire spelling.
  Left.
- **`glass.rs`' `DrawnKind::Choice { alternatives, .. }` / `{ .. }`
  patterns** keep a `..` that now binds nothing. Compiles and lints clean;
  `glass.rs` is imports-only. A tidy for whoever next edits it.
- **`Finite`'s doc** moved verbatim, as designed, and still says *"`Edited`
  and … `wire.rs`'s `Command` above it"* — stratum-3 names in a stratum-1
  doc, the item above. Unchanged here.

**Exit — the gate.** `just check` exited 0 on the phase's final commit:
build, both test tiers (679 passed, 0 failed, summed over every `test
result` line), `deno check`, clippy with no warning, `cargo fmt --all
--check`.

### PHASE-03 — the ground the checker stands on

**Written by a phase-sheet agent, not the executor**, at 1e86bde (*012
PHASE-02: verification, mutation evidence and harvest*). This sheet is the
plan's second reading. Where it restates a plan criterion it quotes it. It
repairs nothing: what reads as wrong in the plan is under **Findings** as a
PLAN QUESTION, and the tasks it blocks are marked `[!]`.

**Objective** (quoted, `plan.md` PHASE-03): *the exercisers are renamed and
no longer read as the file to copy, `goad-emit` exits 2 when its answer
cannot be written, and `config::Command::from_argv` is public.*

The objective names three of the phase's four parts. The fourth,
`version_line` in one home (EX-8, `plan-log.md` PL-7), is as binding.

**Entry**
- **EN-1** (quoted): *"PHASE-02 done (PHASE-01..PHASE-03 run in sequence,
  §Sequencing)."* **Discharged 2026-10-01 at 1e86bde.** §Status has PHASE-02
  `done`. `just check` exited 0: build, both test tiers (679 passed, 0
  failed, summed over every `test result` line; PHASE-02's exit count),
  `deno check`, clippy with no warning, `cargo fmt --all --check`. `just -n
  check` printed POL-001 §Compliance's block verbatim (the VA-1 baseline).

**Surfaces — a closed list, copied from `plan.md`. Anything else is a STOP.**
- `examples/` → `exercisers/` (`git mv`): `shell/backend.sh`,
  `typescript/{backend.ts,README.md}`, `demo.toml`. The moved files'
  contents are surfaces: EX-1 rewrites each header, and EX-2 needs their
  own `examples/` mentions gone.
- Every site in `design.md` §5.2.7's rename table except POL-001 (audit) and
  README's kit line (PHASE-05): `exercisers/demo.toml`,
  `exercisers/typescript/README.md`, `justfile` (`typecheck`, `demo`),
  `crates/goad-shell/tests/integration/{harness.rs, round_trip.rs}`,
  `README.md` (the `just demo` paragraph), `.gitignore` and `flake.nix`
  (comments), `docs/roadmap.md`, and the `docs/memory/` files the table names
  (`a-backend-exchange-has-no-useful-duration`, `deno-run-does-not-typecheck`,
  `path-flake-ref-breaks-on-demo-socket`, `cite-requirements-not-finding-ids`).
- `crates/goad-emit/src/main.rs`, `crates/goad-emit/tests/binary/exchange.rs`.
- `crates/goad-shell/src/config.rs`: *"`from_argv`'s and `unsigned`'s
  visibility, and the `Command` doc, PL-1"*.
- `version_line`'s homes: *"`crates/goad-shell/src/version.rs`, new, and
  `crates/goad-shell/src/lib.rs`, its `mod` line only;
  `crates/goad/src/diagnostics.rs`, `crates/goad-emit/src/render.rs` and
  their callers; PL-7, its placement amended by"* `plan-log.md` 2026-10-01,
  *PHASE-02's `glass.rs` comment; PHASE-03 sheet questions; the push before
  a lock bump*. `crates/goad-shell/src/report.rs` is no longer a surface: it
  was one for this alone, and EX-8 leaves it unchanged. The callers are
  `goad`'s `diagnostics::print_version` (in `diagnostics.rs`) and
  `goad-emit`'s `main`. `goad`'s `main.rs` names
  `version_line` in a comment only; it calls `print_version`, which stays.
- *"`crates/goad/src/startup.rs` and `crates/goad/tests/binary/main.rs`,
  doc-only, for `version_line`'s old home"* (PLAN QUESTION 1, resolved).
- `docs/slices/012/canon-delta.md` — *"test names only"*.
- `docs/slices/012/notes.md` — this sheet, §Status, §Harvest (bookkeeping,
  `docs/AGENTS.md` §Execute).

Not surfaces, and so a STOP if the work seems to need them: any canon
document, POL-001 included; `docs/brief.md`; closed slices' docs; any
`Cargo.toml`; `crates/goad-shell/src/lib.rs` beyond its `mod` line;
`crates/goad-shell/src/report.rs`; `tests/support/`;
`crates/goad-emit/tests/binary/main.rs`; `crates/goad/tests/binary/process.rs`;
and any edit to `crates/goad/src/startup.rs` or
`crates/goad/tests/binary/main.rs` that is not to a doc.

**Reading list** (by symbol; `grep -n` then `sed -n`, not whole files)
- `docs/slices/012/plan.md` — §Overview's first paragraph (`just check` is
  every phase's last exit); *Owed to audit and close* (POL-001 Change 1 ends
  EX-3's departure); *Test names are commitments*; *Mutation evidence* and
  *Invariant reads* under §Sequencing & rationale; §PHASE-03 whole. I-1 and
  I-2 read `crates/goad-check`, which does not exist yet: neither applies here.
- `docs/slices/012/design.md` §5.2.1 (the *Argv form* bullet, its
  *Opportunity* sub-bullet, and the `--timeout` sub-bullet); §5.2.5's last
  paragraph, *`goad-emit`'s unwritten answer*; §5.2.7 whole; §9 *`goad-emit`*
  and *Shared test support*.
- `docs/slices/012/design-log.md` 2026-10-01 *two gaps the plan draft found
  (G1, G2)* — G2 only.
- `docs/slices/012/plan-log.md` 2026-10-01 *placements the plan draft put to
  the user*, PL-1, PL-4 and PL-7; *plan review round 1: dispositions*, F-17
  (supersedes PL-4's rationale: the recipe departs from POL-001 §Statement
  mid-slice because canon is not edited mid-slice); *PHASE-02's `glass.rs`
  comment; PHASE-03 sheet questions; the push before a lock bump* (PLAN
  QUESTIONs 1 and 2 resolved; PL-7's placement moved to
  `goad_shell::version`).
- `docs/slices/012/canon-delta.md` SPEC-004 Change 3 (R-8..R-10, R-14),
  Change 5 (the R-8..R-10 and R-14 rows, which name
  `exchange.rs::an_answer_that_cannot_be_written_exits_2` and the existing
  `help_…` and `version_…` cases); POL-001 Change 1 (the line the recipe
  will carry, less the kit path).
- `docs/policy/001-the-phase-gate.md` §Statement and §Compliance.
- Prior art: `crates/goad/tests/binary/exit_codes.rs`
  `an_answer_that_cannot_be_written_exits_2` (VT-1's model) and
  `crates/goad/tests/binary/process.rs` `goad_with_stdout_full` (the
  `/dev/full` spawn); `goad`'s `run` (`main.rs`), whose `Launch::Help` and
  `Launch::Version` arms map a write failure to
  `StartupError::AnswerUnwritten`; `goad_shell::report::{line_to,
  try_line_to}` and their docs, and `report.rs`' module doc (*"Not a
  formatter"*), which EX-8 keeps true; `goad-shell`'s `lib.rs` module list,
  where `pub mod version;` joins. `docs/memory/` — *negative-control-must-compile*,
  *tests-asserting-proxies*, *mutation-check-the-coverage-claim*,
  *gui-launch-needs-a-pipe*, *hand-over-the-steps-not-the-pointer*. §Harvest
  *Learned*: a `todo!()` stub is a compiling red for a method, with ignored
  parameters spelled `_name`; run an exit grep before naming new tests.

*The rename's referencing sites.* Found at 1e86bde by `git grep -n
'examples/' -- ':!docs/slices' ':!docs/brief.md'` (EX-2's own command), then
`git grep -n 'examples/' -- docs/slices/012` and `git grep -n 'examples' --
':!docs/slices' ':!docs/brief.md'` for the bare word. Each hit read.

| site (by symbol or passage) | in Surfaces? |
|---|---|
| `examples/demo.toml`: `[backend] command` | yes (the moved file) |
| `examples/shell/backend.sh`: the header (*"the config names `["bash", "examples/shell/backend.sh"]`"*; *"Unlike `examples/typescript/backend.ts`"*) and the comment above the `GOAD_DEMO_DELAY` sleep (*"`examples/demo.toml` sets that bound"*) | yes (the moved file) |
| `examples/typescript/README.md`: the fenced `toml` block's `command` | yes |
| `examples/typescript/backend.ts`: the header's *"`deno check examples/typescript/backend.ts`"* | yes (the moved file) |
| `justfile`: `typecheck`'s command; `demo`'s argument | yes |
| `crates/goad-shell/tests/integration/harness.rs`: `example`'s doc and its `CARGO_MANIFEST_DIR` join | yes |
| `crates/goad-shell/tests/integration/round_trip.rs`: `the_readme_s_own_config_loads_and_runs_the_example`'s `include_str!`; `shell_example`'s doc and its `CARGO_MANIFEST_DIR` join | yes |
| `README.md`: the *Try it* paragraph under `just demo` | yes |
| `.gitignore`: the comment above `/goad-demo.sock` | yes |
| `flake.nix`: the `goadShot` usage comment | yes |
| `docs/roadmap.md`: the *"`just demo` starts it against …"* sentence | yes |
| `docs/memory/a-backend-exchange-has-no-useful-duration.md`: *The fact* table | yes |
| `docs/memory/deno-run-does-not-typecheck.md`: *Why it matters here* | yes |
| `docs/memory/path-flake-ref-breaks-on-demo-socket.md`: *The fact* and *Why* | yes |
| `docs/memory/cite-requirements-not-finding-ids.md`: *The rule* (`examples/` as a directory) | yes |
| `docs/policy/001-the-phase-gate.md` §Compliance's `deno check` line | **no** — audit's (POL-001 Change 1); EX-2's one expected hit |
| `docs/brief.md` | excluded by EX-2's pathspec; unchanged (§5.2.7) |
| `docs/slices/012/` — `slice-012.md`, `design.md`, `design-log.md`, `plan.md`, `canon-delta.md`, `research.md`, `review-design.md`, `review-plan.md`, `notes.md` (`git grep -l`) | excluded by EX-2's pathspec; records or working authority, not edited for the rename |
| bare word, no path: `round_trip.rs`' test `the_shell_examples_branch_is_the_hosts_to_decide_and_not_a_watchers`; `docs/roadmap.md` *"the examples (moved, not copied)"*; doc prose in `normalize.rs`, `wire.rs`, a SPEC-001 sentence, a fixture's description, one memory file | not rename sites: none names the directory |

No other tracked file names the directory: no `include_str!` or `env!` path
outside the two integration files, nothing under `nix/`, no deno
configuration.

*`version_line`'s sites.* `grep -rn 'version_line\|GOAD_REVISION\|CARGO_PKG_VERSION'
crates --include=*.rs`, each hit read:
- Definitions: `goad::diagnostics::version_line` (`pub`, `#[must_use]`) and
  `goad-emit`'s `render::version_line` (`pub(crate)`, `#[must_use]`). Same
  body: `revision.filter(non-empty)`, then `"{version} ({revision})"` or the
  bare version, the version from `env!("CARGO_PKG_VERSION")`.
- Unit tests, the same three names in each file's `mod tests`:
  `a_stamped_build_names_its_revision_beside_the_version`,
  `an_unstamped_build_says_only_the_version`,
  `a_build_stamped_with_an_empty_revision_is_an_unstamped_build`.
- Callers: `diagnostics::print_version` (passes `revision` through, writes
  with `try_line_to`); `goad-emit`'s `main`, `Invocation::Version` arm
  (`option_env!("GOAD_REVISION")`, written with `to_stdout`).
- Prose naming it: `diagnostics.rs`' module doc (*"006/PHASE-03 adds
  `version_line` and `print_version`"*); `goad`'s `run` (`main.rs`) comment
  in `Launch::Version`; `goad-emit` `main`'s comment in `Invocation::Version`;
  `render::version_line`'s doc (*"the two are separate because stratum 3 has
  two binaries and no shared crate"* — the claim the lift ends); **in
  Surfaces, doc-only, since PLAN QUESTION 1's resolution:** `goad`'s
  `startup.rs` `arguments` doc table (an intra-doc link to
  `crate::diagnostics::version_line`) and `crates/goad/tests/binary/main.rs`'
  module doc (*"`diagnostics.rs`'s own unit case is the real assertion"*).
- Binary cases (VT-4): `goad`'s
  `version::version_prints_the_package_version_on_stdout_and_exits_0`
  (`crates/goad/tests/binary/version.rs`) and `goad-emit`'s
  `exchange::version_prints_the_package_version_on_stdout_and_exits_0`. Each
  compares stdout with the test crate's own `env!("CARGO_PKG_VERSION")`.
- No canon document cites any of the above (`grep` over `docs/specs`,
  `docs/policy`, `docs/adr`, `docs/memory`, `docs/follow-ups.md`).

*`config.rs`.* `Command` (its doc, `Command::new`, private `from_argv`);
`Config::parse` (the only caller of `from_argv` and `unsigned`); `signed`
(the grammar and positivity check) and `unsigned` (`signed`, then the
conversion to `std::time::Duration`); `mod tests`
`a_zero_timeout_is_rejected_because_it_fails_every_exchange`,
`a_negative_duration_is_rejected_as_non_positive`,
`an_empty_command_is_rejected_because_there_is_nothing_to_spawn`.
`goad_shell::error::ConfigError::{Duration, NonPositive}`.

*`goad-emit`.* `main` (its `Invocation::Help` and `Invocation::Version` arms;
`to_stdout`, `to_stderr`); `render::USAGE`; `render`'s `*_line` functions and
the `goad-emit: ` prefix they begin with; `exchange.rs`' module doc (*"the
nine cases"*) and its helpers `emit`, `code_of`, `stderr_of`, `stdout_of`.

**Assumptions — verified now** (at 1e86bde, by reading, grep, and the runs
named)
- **A-V1 — `env!("CARGO_PKG_VERSION")` expands in the crate that compiles
  it** (PL-7). A `version_line` in `goad-shell` reading it would print
  `goad-shell`'s version. Every member takes `version.workspace = true`, so
  today that is `0.1.0` for all of them, the same as both binaries'. EX-8's
  parameter is therefore correct and **no test can see it missing**
  (Findings; mutation row 6).
- **A-V2 — `GOAD_REVISION` is read only at the binaries' call sites**, by
  `option_env!` at compile time: `goad`'s `run`, `Launch::Version` arm, and
  `goad-emit`'s `main`, `Invocation::Version` arm. `flake.nix` stamps it on
  each binary's crane derivation (`GOAD_REVISION = revision`, `revision =
  self.shortRev or self.dirtyShortRev or ""`). Nothing in the gate sets it,
  so every test sees `None`. *Set-but-empty is unset* is decided inside
  `version_line` (the `filter`), and its test moves with it. The lift
  leaves both reads where they are and passes them in, as today.
- **A-V3 — `goad-emit` ignores a failed answer today.** Both arms write
  through `to_stdout`, which is `report::line_to` (best effort), and return
  `ExitCode::SUCCESS`. Measured on the gate's debug build:
  `goad-emit --help > /dev/full` and `goad-emit --version > /dev/full` each
  exit 0 with empty stderr. `goad --version > /dev/full` exits 2 with `goad:
  the answer could not be written to standard output: No space left on
  device (os error 28)`. So VT-1 is red against today's `main` on its status
  assertion, in code that compiles.
- **A-V4 — `config::unsigned` and `from_argv` can be made public as
  `positive_duration` and `from_argv` without breaking a caller.** `grep -rnw
  'from_argv\|unsigned\|positive_duration' crates --include=*.rs`: only
  `config.rs`, where `Config::parse` calls both. No test calls either
  directly; no other crate names them; no canon cites them. `config` is a
  `pub mod` of `goad-shell`'s `lib.rs`, so `unreachable_pub` does not fire.
  `unsigned` takes `key: &'static str`, which the literal `"--timeout"`
  satisfies. Lint consequence: `clippy::pedantic` is denied workspace-wide,
  so a `pub fn` returning `Result` needs a `# Errors` section
  (`missing_errors_doc`); `from_argv` returns `Option` and needs none;
  `must_use_candidate` is allowed.
- **A-V5 — `Command::new` and the public fields stay open** (PL-1): built
  directly by tests in `goad` (`renderer/table.rs`, `event_loop/closing.rs`),
  `goad-shell` (`integration/transport.rs`, `harness.rs`, `round_trip.rs`,
  `failure_matrix.rs`) and by `from_argv`.
- **A-V6 — the rename's path semantics.**
  - Cargo: the root manifest is virtual (no `[package]`), so a root
    `examples/` is no package's auto-discovered examples directory. No target
    changes.
  - `flake.nix`: `src` keeps `craneLib.filterCargoSources` plus `.slint` and
    `assets/`. `examples/demo.toml` passes that filter as a `.toml`, but no
    derivation reads it and every derivation has `doCheck = false`. Only the
    `goadShot` comment names the path.
  - `justfile`: `typecheck` and `demo` (Reading list table).
  - deno: no `deno.json` or lock file is tracked (`git ls-files`).
    `backend.ts` imports nothing, so `deno check` depends only on its own
    path.
  - `goad-shell` integration: `include_str!("../../../../examples/…")` is
    relative to `round_trip.rs` and resolved at compile time, so a stale path
    is a compile error. `harness::example` and `round_trip::shell_example`
    join `CARGO_MANIFEST_DIR` with `../../examples/…` at run time, so a stale
    path is a failed spawn. The README's fenced `command` is relative to the
    workspace root and rebased by `rooted_at_the_workspace`.
  - `.gitignore`: a comment only. Its patterns are `/goad-demo.sock` and
    `/goad-demo.sock.lock`, from `demo.toml`'s `[ingress] path`, which does
    not change.
  - `goad-boundary`: every scan walks `workspace.members` (`members::members`)
    or a named crate path. `examples/` is in neither, so no instrument's reach
    changes.
- **A-V7 — VT-1 has a model and a helper to copy.** `exit_codes.rs`'
  `an_answer_that_cannot_be_written_exits_2` loops over `["--help",
  "--version"]` with `goad_with_stdout_full`, asserting status 2 and a stderr
  prefix. `goad-emit`'s binary tier has no `/dev/full` spawn. Its helpers
  live in `exchange.rs`, and `goad`'s `process.rs` cannot be included from
  another crate's target without a `tests/support/` move, which is not a
  surface. So VT-1 copies the helper into `exchange.rs` (FU-5's class,
  Findings).
- **A-V8 — the README's "ten-line shell backend" is a count, and false**:
  `backend.sh` is about two hundred lines. `backend.ts`' header says *"in about
  eighty lines"* (it is about two hundred too). VA-2 covers the first; EX-1's
  header rewrite drops the second.

**Assumptions — first tested by this phase**
- **A-T1 — `goad-emit` on `/dev/full` fails through `try_line_to`** as `goad`
  does: the same function, over `std::io::stdout().lock()`.
- **A-T2 — the lifted `version_line`'s cases go red against a `todo!()`
  body** with its parameters spelled `_version`, `_revision` (§Harvest
  *Learned*).
- **A-T3 — VT-2's cases going green on the new paths proves they read them.**
  After `git mv`, nothing exists at `examples/`, so a green case cannot have
  read the old path. VT-2 has no red of its own: the rename changes no
  behaviour, and between the move and the path edit `round_trip.rs` does not
  compile (`include_str!`). A compile failure is not a red.
- **A-T4 — VT-3 has no red against the unmutated tree.** `positive_duration`
  is `unsigned` renamed. Its refusals exist today, so the new case is green
  on arrival. Mutation rows 3 and 4 are its evidence that it can fail.

**STOP conditions** (consult the user; do not improvise)
- From `plan.md` PHASE-03 Notes (quoted): *"`docs/brief.md` and closed
  slices' docs are not edited (§5.2.7)."* Any edit to either is a STOP.
- (quoted) *"The TypeScript exerciser stays; `harness.rs` and `round_trip.rs`
  drive it."* Retiring or deleting it is a STOP.
- Any canon edit, POL-001's block included. EX-3's departure is the plan's
  answer to the gate breaking (F-17).
- The kit path in the `justfile` (PHASE-06's), or README's kit line
  (PHASE-05's).
- `version_line` reading the build environment itself (`env!`,
  `option_env!`) inside `goad-shell`, or a second definition left behind,
  including a forwarding wrapper (EX-8: *"defined once"*).
- Closing `Command::new` or `Command`'s public fields (PL-1 leaves them open).
- Any change to what `positive_duration` accepts or refuses. It is `unsigned`
  renamed and made public.
- Any dependency or feature change, in any manifest. `goad-emit` already
  depends on `goad-shell`.
- Any existing `goad-emit`, `goad` or `goad-shell` case going red other than
  through the moved `version_line` tests' own red step. In particular `help_…`
  and `version_…` stay green unchanged in both binaries (VT-4).
- A file outside **Surfaces**, including the two PLAN QUESTION 1 names while
  it is open.
- A test name differing from `design.md` §9 or `canon-delta.md` SPEC-004
  Change 5. Update `canon-delta.md` in the same commit and say so here (VT-1;
  *Test names are commitments*). Not a STOP, but never silent.
- `git stash`, `git checkout`, `git reset`, or any history rewrite.

**Tasks** — the plan gives no red-first order. In the order below, each new
case goes red against code that compiles before its body lands. VT-2 and VT-3
have no honest red (A-T3, A-T4). The rename lands in one commit with its
paths, so `just check` is never red at a commit.

- [x] Set PHASE-03 to `in progress` in §Status.
- [x] Print `git log -1 --oneline`. It must be this sheet's commit or a
      descendant whose only changes since are the PLAN QUESTIONs'
      resolutions. *251f525 012: PHASE-03 amended — goad_shell::version; copy
      claims; push before lock bump.*
- **1. `config.rs` (EX-5, EX-6, EX-7, VT-3)**
  - [x] Rename `unsigned` to `positive_duration` and make it `pub`, with
        `Config::parse` calling it. Its doc names its two callers, the
        configuration's `backend.timeout` and `goad-check`'s `--timeout`
        (EX-6, quoted: *"its doc naming `goad-check`'s `--timeout` as its
        second caller (`design-log.md` 2026-10-01, G2)"*), and gains `#
        Errors` (A-V4). No behaviour change.
  - [x] VT-3 (quoted): *"`config.rs` unit tests: `positive_duration` refuses
        `0s` and `-1s` under the key it is given."* Use a key no
        configuration line uses (e.g. `"--timeout"`), and assert
        `ConfigError::NonPositive { key }` with that literal for each.
        Green on arrival (A-T4); record it, and record mutation rows 3 and 4
        as its red. *Shipped as
        `positive_duration_refuses_a_zero_and_a_negative_span_under_the_key_it_is_given`
        (no name is committed for it in `design.md` §9 or `canon-delta.md`).
        Green on arrival, as predicted; its red is mutation rows 3 and 4.*
  - [x] EX-5 (quoted): *"`config::Command::from_argv` is `pub`, with its doc
        naming the checker's argv form as its second caller."* Keep its
        *`None` for the empty vector and for an empty program* sentence.
  - [x] EX-7 (quoted): *"`config::Command`'s doc names the routes that hold
        the empty command out (`Config::parse`, `from_argv`) and no longer
        claims it is unrepresentable (PL-1)."* Say what still admits one
        (`Command::new`, the public fields), since §5.2.1 *Opportunity*
        names them.
  - [x] `cargo test -p goad-shell --lib config` and `cargo clippy -p
        goad-shell --all-targets -- -D warnings` green. *20 passed; clippy
        clean.*
- **2. `version_line` in one home (EX-8, VT-4)**
  - [x] EX-8 (quoted): *"`version_line(version, revision)` is defined once,
        in a new module `goad_shell::version`, taking the package version as
        a parameter … `goad_shell::report` is unchanged, its "not a
        formatter" module doc still true."* Create
        `crates/goad-shell/src/version.rs` with a `//!` module doc (what the
        module owns: the `--version` line's text, which every binary
        shares), and add `pub mod version;` to `lib.rs` in its alphabetical
        place. `pub`, not `pub(crate)`: a private module's `pub fn` warns
        under `unreachable_pub`, which the gate's `-D warnings` denies.
  - [x] In it, `pub fn version_line(version: &str, revision: Option<&str>) ->
        String` with a `todo!()` body (parameters `_version`, `_revision`),
        and the three cases moved from `diagnostics.rs` under their names,
        now passing the version as a literal argument. Red: 3 failed.
        Lints: it returns `String`, not `Result`, so `clippy::pedantic`'s
        `missing_errors_doc` asks for no `# Errors`; keep `#[must_use]`, as
        both copies carry it (`must_use_candidate` is allowed, so nothing
        else holds it); `module_name_repetitions` is allowed workspace-wide,
        so `version::version_line` passes; `clippy::todo` is denied, so the
        `todo!()` stub never reaches a commit. *Red seen: `cargo test -p
        goad-shell --lib version`, 0 passed, 3 failed, each panicking at the
        `todo!()` (A-T2 holds).*
  - [x] The body: today's, over `version` instead of
        `env!("CARGO_PKG_VERSION")`. Green. *3 passed.*
  - [x] Callers: `diagnostics::print_version` passes `goad`'s
        `env!("CARGO_PKG_VERSION")`, and `goad-emit`'s `main` passes its own.
        Delete `diagnostics::version_line` and `render::version_line` and
        both sets of their cases. EX-8 (quoted): *"`goad` and `goad-emit`
        call it, and their `--version` output is unchanged."*
  - [x] VT-4 (quoted): *"`goad`'s and `goad-emit`'s existing `--version`
        cases stay green across the lift."* `cargo test -p goad --test binary`
        and `cargo test -p goad-emit --no-fail-fast` green. *`goad` binary
        tier 8 passed, `version_prints_the_package_version_on_stdout_and_exits_0`
        among them; `goad-emit` unit 34 passed, binary 9 passed,
        `exchange::version_prints_the_package_version_on_stdout_and_exits_0`
        among them.*
  - [x] EX-8's parameter is invisible to every test (A-V1), so record a read:
        `grep -rn 'CARGO_PKG_VERSION\|GOAD_REVISION' crates/goad-shell/src`
        finds only prose, and each binary's call site passes its own
        `env!("CARGO_PKG_VERSION")`. Run mutation row 6. *Read: the grep's
        hits are all in `version.rs`, in `version_line`'s doc and its
        `mod tests` docs; none is code. `grep -rn 'version_line(' crates`
        outside `version.rs`: `goad`'s `diagnostics::print_version` passes
        `env!("CARGO_PKG_VERSION")` and its caller's revision; `goad-emit`'s
        `main`, `Invocation::Version` arm, passes `env!("CARGO_PKG_VERSION")`
        and `option_env!("GOAD_REVISION")`. Row 6 recorded below.*
  - [x] `report.rs` unchanged: `git diff --stat` over the phase shows no
        `crates/goad-shell/src/report.rs`, and its module doc (*"Not a
        formatter … This module owns only the last step"*) stays true.
        Record it. *`git diff --stat 251f525` names no `report.rs`. Its doc
        names `diagnostics` and `render` as composers, by example (*"—
        `crates/goad`'s `diagnostics` for the host, `render` for a
        command-line binary"*); the version line is now composed in
        `goad_shell::version`, which the sentence does not list but does not
        deny. Still true.*
  - [x] Docs made true, in Surfaces: the moved function's
        doc carries the reasoning both copies held (no placeholder, no
        prefix, *set-but-empty is unset*, the revision a parameter so the
        rule is a test) and says why the version is a parameter too;
        `diagnostics.rs`' module doc; the `Launch::Version` and
        `Invocation::Version` comments, which name `render::version_line` or
        `version_line` as where *set-but-empty* is decided.
  - [x] Surfaces (quoted): *"`crates/goad/src/startup.rs` and
        `crates/goad/tests/binary/main.rs`, doc-only, for `version_line`'s
        old home"*. `startup.rs`' `arguments` doc table: the `--version` row
        links `goad_shell::version::version_line`.
        `crates/goad/tests/binary/main.rs`' module doc names the unit case's
        new home, `goad_shell::version`. Then `grep -rn
        'diagnostics::version_line\|render::version_line' crates` is empty.
        *Empty. The `binary/main.rs` module doc names the type in two places
        (`version_line` rendering it; the stamped branch's real assertion),
        both now `goad_shell::version`. `cargo doc` over `goad`,
        `goad-shell` and `goad-emit` with `-D rustdoc::broken_intra_doc_links`
        reports no broken link (the three warnings it prints are pre-existing
        private-item links).*
- **3. `goad-emit`'s unwritten answer (EX-4, VT-1)**
  - [x] Run the exit grep first (§Harvest *Learned*): `grep -rn
        'an_answer_that_cannot_be_written_exits_2' crates` finds only `goad`'s
        `exit_codes.rs` case and the doc in
        `crates/goad/tests/renderer/startup.rs` that cites it. *As stated:
        `goad`'s `exit_codes.rs` definition and the `renderer/startup.rs`
        doc citing it; nothing else.*
  - [x] VT-1 in `exchange.rs` (quoted): *"`an_answer_that_cannot_be_written_exits_2`
        (`--help` and `--version`, stdout on `/dev/full`; status 2; the last
        stderr line begins `goad-emit: `), modelled on `goad`'s
        `exit_codes.rs` case of the same name; seen red before `main.rs`
        changes."* Add an `emit_with_stdout_full` helper beside `emit` (A-V7).
        Assert on the **last** line of stderr, as canon-delta's R-14 row
        says, not on `starts_with` over the whole stream. Red: status 0 on
        each question (A-V3). Record the failure message. *Red, compiling,
        before `main.rs` changed: `assertion left == right failed: --help:`,
        `left: 0`, `right: 2`, empty stderr; 9 passed, 1 failed. The loop
        stops at `--help`; mutation row 2 shows the `--version` iteration
        reds on its own.*
  - [x] The stderr line is composed in `render`, beside its siblings,
        beginning `goad-emit: ` and interpolating the `io::Error` as its
        siblings interpolate theirs (`format!`, `{fault}`). Spell it as `goad`'s
        `StartupError::AnswerUnwritten` reads (*"the answer could not be
        written to standard output: …"*). One rule for the edge (§5.2.5).
        *`render::answer_unwritten_line`. Observed: `goad-emit --help >
        /dev/full` and `--version > /dev/full` each print `goad-emit: the
        answer could not be written to standard output: No space left on
        device (os error 28)` and exit 2.*
  - [x] `main`'s `Invocation::Help` and `Invocation::Version` arms write
        through `report::try_line_to` on `std::io::stdout().lock()`. On `Err`,
        write the line through `to_stderr` and return `ExitCode::from(2)`.
        EX-4 (quoted): *"an unwritten answer exits 2 with a `goad-emit: …`
        line on stderr."* `to_stdout` then has no caller; delete it rather
        than leave `dead_code`. Green: VT-1, and every other `exchange.rs`
        case unchanged. *Both arms call one private `answer(line) ->
        ExitCode` (D-1); `to_stdout` deleted. `goad-emit` unit 34 passed,
        binary 10 passed.*
  - [x] Not a `StartupFault` variant: that type is *why the envelope never
        left*, and a question sends no envelope. A local decision; record it.
        *D-2, and in `answer`'s doc.*
  - [x] `exchange.rs`' module doc says *"the nine cases"*. Replace the count
        with what the file holds (*"the spawn helpers, the fake listener, and
        the cases"*). Name, never count.
  - [x] `main`'s doc (*"Three exit codes … 2 emit got no usable answer — a
        usage error, …"*) gains the unwritten answer among 2's causes.
- **4. The rename (EX-1, EX-2, EX-3, VT-2, VA-1, VA-2)** — one commit.
  - [x] `git mv examples exercisers`.
  - [x] Paths: `exercisers/demo.toml` (`command = ["bash",
        "exercisers/shell/backend.sh"]`); `exercisers/typescript/README.md`'s
        fenced `command` (`./exercisers/typescript/backend.ts`);
        `round_trip.rs`' `include_str!` and `shell_example`'s join;
        `harness::example`'s join; `justfile` `demo` (`run
        "exercisers/demo.toml"`); each moved file's own mentions (Reading
        list table). *Every site in the Reading list table, and nothing
        else.*
  - [x] EX-3 (quoted): *"the `justfile`'s `typecheck` is `deno check
        exercisers/typescript/backend.ts`: `canon-delta.md` POL-001 Change
        1's line less the kit path, which PHASE-06 adds. Its comment is true
        of this step: it names the one exerciser it typechecks and why (`deno
        run` does not), and says the recipe departs from POL-001's command
        block until audit promotes POL-001 Change 1."* The comment must not
        spell the kit example's path: it contains `examples/` and would hit
        EX-2's grep. The `justfile` header's *"Change the policy first, then
        mirror"* stays, and is the rule EX-3's comment says this step
        departs from. *The comment names the TypeScript exerciser, why
        (`deno run` does not typecheck), and the departure from POL-001
        §Compliance until slice 012's audit promotes POL-001 Change 1, citing
        `canon-delta.md`. It spells no kit path. The header's *"Change the
        policy first, then mirror"* is unchanged.*
  - [x] EX-1 (quoted): *"each header says it is a host exerciser and points
        at `kit/`; every present-tense claim in the exercisers and their
        tests that they are the thing to copy is rewritten — `backend.ts`'
        "Copy this file", `round_trip.rs`' "the file a person copies" and its
        `the_readme_s_own_config_loads_and_runs_the_example` doc,
        `backend.sh`'s header, and the TypeScript README's opening among them
        — and `git grep -n -i 'cop\(y\|ies\)' -- exercisers
        crates/goad-shell/tests/integration/round_trip.rs README.md` is
        recorded, each remaining hit classed."* `kit/` does not exist until
        PHASE-05; the pointer is to where it will be. The historical
        *"the defect propagated by copying"*
        (`the_shell_example_escapes_the_values_it_carries_into_a_view`'s doc)
        is true and stays, classed as history (PLAN QUESTION 2). *Headers:
        `demo.toml`, `backend.sh`, `backend.ts` and the TypeScript README
        each open by calling the file a host exerciser and point at `kit/`.
        Rewritten copy claims: `backend.ts`' *"Copy this file. It is meant to
        be edited"*; `backend.sh`'s *"what a backend author copies from here
        is the contract"* (now *"what this file exercises"*); `round_trip.rs`'
        *"the file a person copies"* (`shell_example`'s doc) and *"the one a
        reader copies"*
        (`the_readme_s_own_config_loads_and_runs_the_example`'s doc); the
        README's *"A minimal goad backend … Point a config at it"*. The grep,
        after:*
        ```
        crates/goad-shell/tests/integration/round_trip.rs:241:/// author what a backend looks like, so the defect propagated by copying
        ```
        *One hit, `the_shell_example_escapes_the_values_it_carries_into_a_view`'s
        doc: history, true, stays.*
  - [x] The remaining rename sites: `README.md`'s *Try it* paragraph,
        `.gitignore`'s comment, `flake.nix`'s `goadShot` comment,
        `docs/roadmap.md`'s sentence, and the four `docs/memory/` files.
        `deno-run-does-not-typecheck` says *"as its seventh command"*. The
        gate is six, and the sentence is edited anyway: name the command
        instead of counting it. `cite-requirements-not-finding-ids` names
        `examples/` as a directory of comments; it becomes `exercisers/`.
        *Done. `deno-run-does-not-typecheck` now names the `justfile`'s
        `typecheck` recipe instead of *"its seventh command"*, and calls the
        file the exerciser.*
  - [x] VA-2 (quoted): *"the README's counts touched here are replaced by
        names ("the ten-line shell backend")."* Also false (A-V8). Record the
        before and after. *Before: "`examples/demo.toml` and the ten-line
        shell backend in `examples/shell/backend.sh`". After:
        "`exercisers/demo.toml` and the shell backend in
        `exercisers/shell/backend.sh`". `backend.ts`' *"in about eighty
        lines"* went with its header (EX-1).*
  - [x] VT-2 (quoted): *"`round_trip.rs`'s
        `the_readme_s_own_config_loads_and_runs_the_example` and `harness.rs`'
        deno cases are green on the new paths."* The cases that read an
        exerciser path: `the_readme_s_own_config_loads_and_runs_the_example`,
        `the_deno_example_completes_a_round_trip` (through
        `harness::example`), and `round_trip.rs`' shell cases through
        `shell_example`:
        `the_shell_example_escapes_the_values_it_carries_into_a_view` and
        `the_shell_examples_branch_is_the_hosts_to_decide_and_not_a_watchers`.
        `cargo test -p goad-shell --test integration --no-fail-fast` green,
        and `test ! -e examples` (A-T3). Record both. *96 passed, 0 failed;
        the four named cases each `ok`. `test ! -e examples` true. `just
        typecheck` checks `exercisers/typescript/backend.ts` clean.*
  - [x] EX-2 (quoted): *"`git grep -n 'examples/' -- ':!docs/slices'
        ':!docs/brief.md'` finds only POL-001's command block, which audit
        amends."* Record the output.
        ```
        docs/policy/001-the-phase-gate.md:58:deno check examples/typescript/backend.ts
        ```
  - [x] VA-1 (quoted): *"`just -n check` prints POL-001 §Compliance's command
        block with its `deno check` line replaced by EX-3's, and no other
        difference; recorded."* Diff the two and record it. *`diff` of
        §Compliance's fenced block against `just -n check`'s output:*
        ```
        4c4
        < deno check examples/typescript/backend.ts
        ---
        > deno check exercisers/typescript/backend.ts
        ```
        *No other difference.*
- **Refactor**
  - [x] Read the diff for a second statement of anything moved: a
        `version_line` body, a `/dev/full` spawn beyond the one copy VT-1
        needs, an `examples/` path. *None: one `version_line` body
        (`goad_shell::version`); one `/dev/full` spawn in `goad-emit`'s tier
        (`exchange::emit_with_stdout_full`, the copy VT-1 needs); no
        `examples/` path outside POL-001 (EX-2).*
  - [x] `git grep -n -i 'examples' -- exercisers` and the touched docs: no
        sentence still reads the exercisers as examples to follow.
        `harness::example` and the `*_example*` test names may stay; renaming
        a canon-cited test (`the_deno_example_completes_a_round_trip`, SPEC-001
        R-53) is not this phase's. *Read over `exercisers`, the touched docs,
        `harness.rs`, `round_trip.rs` and the `justfile`. Prose that called
        an exerciser *"the example"* now says *"the exerciser"*
        (`round_trip.rs`' comments and assertion messages, `harness.rs`'
        `example` doc and `prompting_event` doc, the `justfile`'s `demo`
        comment). Kept: the `example` helper and `*_example*` test names;
        `demo.toml`'s *"a worked example of the wire"* (an illustration of
        SPEC-003, not a claim about the file); `docs/roadmap.md`'s uses,
        which name the kit's examples or a brief criterion.*
- **Verification**
  - [ ] Mutation rows (below).
  - [x] **Canon-delta test names:** `grep -c 'fn <name>()'
        crates/goad-emit/tests/binary/exchange.rs` is 1 for each name
        SPEC-004 Change 5's R-8..R-10 and R-14 rows place there:
        `an_answer_that_cannot_be_written_exits_2`,
        `help_prints_the_usage_block_on_stdout_and_exits_0`,
        `version_prints_the_package_version_on_stdout_and_exits_0`,
        `an_accepted_envelope_exits_0_and_says_nothing`,
        `a_refusal_exits_1_with_the_reason_token_on_stderr`,
        `a_too_soon_refusal_also_shows_retry_after_ms`,
        `a_usage_error_exits_2_before_anything_is_opened`,
        `a_path_with_nothing_listening_exits_2_and_names_the_path`,
        `a_reply_that_breaches_6_3_exits_2_rather_than_1`. `design.md` §9's
        PHASE-03 name is the first. A name that differs is updated in
        `canon-delta.md` in the same commit. *Each `grep -c` is 1. No
        `canon-delta.md` edit.*
  - [x] `just check` exits 0 on the final commit. Record it. *Exit 0 at
        1c74dec (the rename) and again on the phase's final commit, which
        changes only this file: build, both test tiers (678 passed, 0 failed,
        summed over every `test result` line), `deno check
        exercisers/typescript/backend.ts`, clippy with no warning, `cargo fmt
        --all --check`. 678 is PHASE-02's 679, less the six `version_line`
        cases deleted from `goad` and `goad-emit`, plus the three moved to
        `goad_shell::version`, VT-1 and VT-3.*
  - [x] VH-1 (quoted): *"a person runs `just demo` on the renamed exerciser
        and sees the window prompt, as before."* Hand the person the command
        block, not a pointer (`docs/memory/hand-over-the-steps-not-the-pointer.md`).
        Record what they saw. Do not launch it from the agent's shell with
        `&` (`docs/memory/gui-launch-needs-a-pipe.md`). *Met 2026-10-01 at
        26eac25: the user ran `just demo` and reported it good — the window
        showed the demo's prompt, as before.*
  - [x] §Status: PHASE-03 `done`, with the date.
  - [x] Harvest updated in place (*Fresh as of*, Produced, Learned, Open).
        §Open gains: *the `justfile`'s `typecheck` departs from POL-001
        §Compliance until audit promotes POL-001 Change 1* (PHASE-10/VA-4
        checks §Open carries every owed item); the `/dev/full` helper copied
        into `exchange.rs`, by symbol, for FU-5; and whatever Findings
        outlive the phase.

**Mutation evidence** (`plan.md` *Mutation evidence*: copy the file to the
scratchpad and back, never `git checkout` or `git stash`; `--no-fail-fast`;
`git status` clean after each restore; a mutation that does not compile is
not evidence). PHASE-03's VA items (VA-1, VA-2) are reads, not mutations, and
the plan names no mutation for this phase. Every row below is offered
because no VA item otherwise shows a new case can fail. Each row: `cargo test
--workspace --no-fail-fast` on the mutated tree, the file restored by copying
the scratchpad backup back, `git status --short` empty after.

| edit | cases it must red | compiled? | redded |
|---|---|---|---|
| 1. `goad-emit` `main`, `Invocation::Help` arm: `match try_line_to(std::io::stdout().lock(), render::USAGE) { Ok(()) \| Err(_) => ExitCode::SUCCESS }` | `exchange::an_answer_that_cannot_be_written_exits_2` (its `--help` iteration) | yes | `exchange::an_answer_that_cannot_be_written_exits_2` alone, *"--help: left: 0, right: 2"*; every other `test result` `0 failed`. Restored by copy. Run before the task's commit, so `git status` showed the task's own edits; the file matched its backup |
| 2. The same for the `Invocation::Version` arm | `exchange::an_answer_that_cannot_be_written_exits_2` (its `--version` iteration). With row 1, shows each question is held, not only the first | yes | `exchange::an_answer_that_cannot_be_written_exits_2` alone, *"--version: left: 0, right: 2"*: the `--help` iteration passed first. Restored; `git status --short` empty |
| 3. `config.rs` `signed`: drop `resolved.is_zero() \|\|` | VT-3's `0s` clause; `a_zero_timeout_is_rejected_because_it_fails_every_exchange`; `a_zero_default_poll_is_rejected_because_it_is_a_busy_loop` | yes | the three named, and no other; VT-3's message *"0s was not refused as non-positive under its key: Ok(0ns)"*. Restored; `git status --short` empty |
| 4. `positive_duration`'s parameter spelled `_key`, and `signed` called with `"backend.timeout"` | VT-3 only, on its key. The existing configuration cases pass `"backend.timeout"`, so they stay green | yes | VT-3 alone: *"0s was not refused as non-positive under its key: Err(NonPositive { key: "backend.timeout" })"*. Restored; `git status --short` empty |
| 5. `render`'s unwritten-answer line drops its `goad-emit: ` prefix | `exchange::an_answer_that_cannot_be_written_exits_2` | yes | that case alone, on its last-line assertion: *"--help: the answer could not be written to standard output: No space left on device (os error 28)"*. Restored; `git status --short` empty |
| 6. expected **not** to red: `goad_shell::version::version_line` ignores `_version` and formats `env!("CARGO_PKG_VERSION")` | none expected: `goad-shell`'s version equals both binaries' (A-V1). Recording it shows EX-8's parameter is held by review and task 2's grep, not by a test | yes | none, as predicted: every `test result` line `0 failed`. Restored by copy; `version.rs` matches its backup |

**Decisions taken during execution**
<!-- Small and local: how, within what the design already settled. A choice that
     changes the design is not one of these — stop, consult the user, and record
     it in `design-log.md`. -->

- **D-1 — one `answer` in `goad-emit`'s `main`.** Both question arms call a
  private `answer(line) -> ExitCode`: `try_line_to` on stdout, 0 on `Ok`, and
  on `Err` the `render::answer_unwritten_line` through `to_stderr` and 2. The
  sheet's two-arm wording would have spelled the `Err` branch twice. The
  mutation rows replace an arm's `answer(…)` call with the stated `match`,
  which compiles.
- **D-2 — not a `StartupFault` variant**, as the sheet says: that type is why
  the envelope never left, and a question sends no envelope. Said in
  `answer`'s doc.
- **D-3 — VT-3's name** is
  `positive_duration_refuses_a_zero_and_a_negative_span_under_the_key_it_is_given`.
  No document commits one.
- **D-4 — `version.rs`' doc cites `crates/goad`'s `build.rs`** for the
  *set-but-empty* rule, not a bare `build.rs`: `goad-shell` has none.

**Findings**
<!-- Things noticed in passing that are not this phase's job: a defect
     elsewhere, drift from the design, a surprise. Defects in this phase's own
     work get fixed, not recorded. These feed the audit; the ones that outlive
     the slice become Follow-ups. -->

- **PLAN QUESTION 1 — two docs outside Surfaces name `version_line`'s old
  home.** EX-8 deletes `diagnostics::version_line`. Two files name it and
  are not *"their callers"*:
  - `crates/goad/src/startup.rs`, `arguments`' doc table: the `--version`
    row links `` [`crate::diagnostics::version_line`] ``. That becomes a
    broken intra-doc link. The gate runs no `cargo doc`, so nothing reds.
  - `crates/goad/tests/binary/main.rs`' module doc: *"`diagnostics.rs`'s own
    unit case is the real assertion for the other branch"*. The case moves
    to `goad_shell::report`.
  Options: (a) add both to Surfaces as doc-only edits; (b) leave both stale
  and record them for audit; (c) keep a `diagnostics::version_line` that
  forwards to the lifted one. That is a second public name for one rule,
  contrary to EX-8's *"defined once"*. **Recommendation: (a).** Each is one
  line, and the stale form is a false statement in the tree from this commit
  on.
  **Resolved: (a)** — `plan-log.md` 2026-10-01, *PHASE-02's `glass.rs`
  comment; PHASE-03 sheet questions; the push before a lock bump*; `plan.md`
  PHASE-03's Surfaces amended.
- **PLAN QUESTION 2 — EX-1 names two copy claims; the exercisers carry
  more.** `git grep -n -i 'cop\(y\|ies\)' -- examples
  crates/goad-shell/tests/integration/round_trip.rs README.md` finds, beyond
  EX-1's two:
  - `round_trip.rs`, `the_readme_s_own_config_loads_and_runs_the_example`'s
    doc: *"The README's own config — the one a reader copies"*;
  - `backend.sh`'s header: *"what a backend author copies from here is the
    contract"*;
  - `round_trip.rs`, `the_shell_example_escapes_the_values_it_carries_into_a_view`'s
    doc: *"the defect propagated by copying"*, which is history and true.
  The TypeScript README also opens *"A minimal goad backend … Point a config
  at it"*, an invitation without the word. Every one is in a surface file.
  Options: (a) read EX-1 as its objective does, *"no longer read as the file
  to copy"*: rewrite every present-tense copy claim and the README's
  opening, leave the historical one, and record `git grep -n -i
  'cop\(y\|ies\)' -- exercisers
  crates/goad-shell/tests/integration/round_trip.rs README.md` with each
  remaining hit classed; (b) EX-1 literally, only the two it names.
  **Recommendation: (a).** AC-5's half here is that the exercisers stop
  presenting themselves as the thing to copy. (b) would leave the
  `backend.sh` header saying so.
  **Resolved: (a)** — `plan-log.md` 2026-10-01, *PHASE-02's `glass.rs`
  comment; PHASE-03 sheet questions; the push before a lock bump*; `plan.md`
  PHASE-03/EX-1 amended.
- **`report.rs` stops being "not a formatter".** Its module doc says what a
  line *says* belongs to whoever composed it, and the module owns only the
  last step. PL-7 places a composer there. EX-8 is executable as written,
  and the doc is in Surfaces, so task 2 rewrites it. A design note for audit:
  the module's single responsibility becomes *the edge every binary shares*
  rather than *the sink*. A separate `goad_shell::version` module would keep
  the old sentence true. That would be a new file and a `lib.rs` edit, so a
  plan change, and is not proposed here.
  **Resolved:** the orchestrator raised it; `goad_shell::version` adopted
  (`plan-log.md` 2026-10-01, *PHASE-02's `glass.rs` comment; PHASE-03 sheet
  questions; the push before a lock bump*); `plan.md` PHASE-03/EX-8 amended.
- **EX-8's parameter has no test.** Every member is `0.1.0` from the
  workspace, so `goad-shell`'s `env!("CARGO_PKG_VERSION")` equals each
  binary's, and VT-4 stays green if the lift reads its own. Held by review,
  task 2's grep and mutation row 6. A version split across members would
  make VT-4 a real witness. None is planned.
- **VT-1 copies a spawn helper across crates.** `goad`'s
  `process::goad_with_stdout_full` cannot be included by `goad-emit`'s
  target without moving it to `tests/support/`, which is not a surface. The
  copy is FU-5's class; §Open names it when it ships.
- **`goad-emit`'s binary-tier doc is already false.**
  `crates/goad-emit/tests/binary/main.rs` says *"Every case passes
  `--socket`"*. The `--help` and `--version` cases do not, and VT-1 adds
  another. Not a surface; for audit.
- **A count in canon's neighbourhood.** `deno-run-does-not-typecheck`'s
  *"seventh command"* predates the six-command gate. It is fixed here
  because the sentence is edited for the path (task 4).
- **`goad`'s `print_usage` doc begins mid-sentence** (*"stdout. The only
  caller is `--help`"*), as `print_version`'s did. Each lost its opening line
  at some edit before this slice. `print_version`'s is rewritten here, since
  its call changed; `print_usage`'s is left. For audit.
- **`report.rs`' composer list is by example.** Its module doc names
  `diagnostics` and `render` as where a line is composed; the version line
  is now composed in `goad_shell::version`. The sentence does not claim to be
  complete, so it is still true, and EX-8 keeps the file unchanged.
- **`goad-emit`'s binary-tier doc** (*"Every case passes `--socket`"*,
  above) now has one more counter-case, VT-1's.

### PHASE-04 — `goad-check`: the crate and its edges

**Written by a phase-sheet agent, not the executor**, at 99208da (*012
PHASE-03: VH-1 met — just demo shows the prompt after the rename*). This
sheet is the plan's second reading. Where it restates a plan criterion it
quotes it. It repairs nothing: what reads as wrong in the plan is under
**Findings** as a PLAN QUESTION, and the tasks it blocks are marked `[!]`.
Every PLAN QUESTION is now resolved (Findings), the amended criteria are
re-quoted below, and no task is blocked.

**Objective** (quoted, `plan.md` PHASE-04): *a headless `goad-check` binary
parses both command forms, loads its configuration and event files by the
host's rules, writes its report through the report writer, and exits 2 on
every failure as `canon-delta.md` SPEC-004 R-11..R-15 state, with every way
to status 2 in its binary tier. It makes no exchange yet, so it judges
nothing and delivers no verdict: until PHASE-12 a run ends with status 2.
The exchange, the verdict and statuses 0 and 1 are PHASE-12's.*

PHASE-04 has no EX-2. It was removed when the run split out
(`review-plan.md` F-13's repair: *"PHASE-04/EX-2 and VT-3 are removed, now
PHASE-12/EX-1 and VT-3"*). The VT-3 below is a later, different case (F-23);
Findings.

**Entry**
- **EN-1** (quoted): *"PHASE-01, PHASE-02 and PHASE-03 done."*
  **Discharged 2026-10-01 at 99208da.** §Status has all three `done`.
  `just check` exited 0: build, both test tiers (**678 passed, 0 failed**,
  summed over every `test result` line), `deno check
  exercisers/typescript/backend.ts`, clippy with no warning, `cargo fmt
  --all --check`. `just -n check` differs from POL-001 §Compliance's
  command block in one line only:
  ```
  4c4
  < deno check examples/typescript/backend.ts
  ---
  > deno check exercisers/typescript/backend.ts
  ```
  This is PHASE-03/EX-3's departure, which audit ends (POL-001 Change 1).
- **EN-2** (quoted): *"`design.md` §5.2.1 judges `--timeout` by
  `config::positive_duration` (`design-log.md` 2026-10-01, G2), public since
  PHASE-03/EX-6."* **Discharged at 99208da.**
  `goad_shell::config::positive_duration(key: &'static str, raw: &str) ->
  Result<std::time::Duration, ConfigError>` is `pub`. Its doc names
  `goad-check`'s `--timeout` as its second caller.

**Surfaces — a closed list, copied from `plan.md`. Anything else is a STOP.**
- (quoted) *"`crates/goad-check/` (new: `Cargo.toml`, `src/`,
  `tests/binary/` and its bash fixtures)"*. A committed config or event
  file that a binary case reads lives under `tests/binary/`, inside this
  surface.
- (quoted) *"the root `Cargo.toml`'s `members` (appended after
  `goad-emit`, before `goad-boundary`) and `Cargo.lock`"*. Only the
  `members` array. Its comment (*"The order is the strata, then the member
  that is not one"*) stays true with that placement.
- (quoted) *"`crates/goad-boundary/tests/checks/allowlist.rs` (its module
  doc's member list only)"*.
- (quoted) *"`canon-delta.md` (test paths only)"*.
- `docs/slices/012/notes.md`: this sheet, §Status, §Harvest (§Open's FU-5
  and FU-7 rows, for VA-5 and VA-7). This is bookkeeping (`docs/AGENTS.md`
  §Execute).
- (quoted) *"`tests/support/` is read and may be included, not edited."*

Not surfaces, and so a STOP if the work seems to need them: any other
`Cargo.toml` (the workspace's `[workspace.dependencies]` included); any
canon document; `crates/goad-shell/` (no new public item, no visibility
change); `crates/goad-emit/` (its helpers are copied, not moved);
`tests/support/`; `flake.nix` and the `justfile` (PHASE-05's, `plan-log.md`
PL-6); `README.md`; `design.md` and `plan.md`.

**Reading list** (by symbol; `grep -n` then `sed -n`, not whole files)
- `docs/slices/012/plan.md`: §Overview's first paragraph (`just check` is
  every phase's last exit); *Owed to audit and close*; *Test names are
  commitments*; §Sequencing & rationale, *Why the checker is two phases*,
  *Mutation evidence* and *Invariant reads*; §PHASE-04 whole; §PHASE-12's
  EX-6 and VT-2, which replace and extend what this phase ships.
- `docs/slices/012/design.md`: §5.2.1 whole; §5.2.2's sequence diagram and
  its two `Note` lines (where status 2 arises); §5.2.5 whole; §5.2.6's
  paragraph on `goad-check`'s binary tier and `tests/support/`; §5.3; §5.4
  (*the checker's phases*, *Concurrency*); §5.5 I-1, I-2, I-6 and *Edges*;
  §9 *`goad-check`*.
- `docs/slices/012/design-log.md` 2026-10-01: *two gaps the plan draft found
  (G1, G2)* (G2); *plan review round 1: design-touching dispositions*
  (F-13, F-18).
- `docs/slices/012/plan-log.md` 2026-10-01: *placements the plan draft put
  to the user* (PL-1, PL-6, PL-7); *plan review round 1: dispositions* (F-8,
  F-12); *plan review round 2: dispositions* (F-23); *PHASE-02's `glass.rs`
  comment; PHASE-03 sheet questions; the push before a lock bump*
  (`goad_shell::version`).
- `docs/slices/012/review-plan.md`: F-8 (I-1 reads non-comment code), F-13
  (the split), F-15 (VA-6), F-23 (the interim end).
- `docs/slices/012/canon-delta.md` SPEC-004 Change 3 (R-11..R-15), Change 4
  (`goad-check`'s table), and Change 5's R-14, R-11..R-13 and R-15 rows.
- Canon: `docs/specs/004-process-exit-status.md` §3 (P-B: the line on
  stderr), §5; `docs/specs/003-host-event-ingress.md` R-13 (the reserved
  source); `docs/policy/001-the-phase-gate.md` §Compliance and
  §Verification (the four ADR-001 instruments, the vocabulary check, the
  residue); `docs/adr/001-one-way-strata.md`;
  `docs/adr/003-the-host-splits-into-a-workspace-of-strata.md`.
- Prior art, `goad-emit` (the shape to mirror): `args.rs` (`Invocation`,
  `UsageError`, `parse`, its `mod tests` table, including
  `help_wins_over_version_in_either_order` and
  `a_help_or_version_token_in_value_position_is_a_value`); `render.rs`
  (`USAGE`, `usage_error_line`, `answer_unwritten_line`, the `goad-emit: `
  prefix, the module doc *"Every line the binary can write, as a `String`
  with no sink"*); `main.rs` (`main`, `answer`, `to_stderr`, the
  `Invocation::Version` arm calling `goad_shell::version::version_line`);
  `Cargo.toml` (its dependency comment, which EX-5 mirrors; `autotests =
  false`; `[[test]] name = "binary"`); `tests/binary/main.rs` (the
  `#[cfg(test)] mod …;` declaration and why) and `exchange.rs` (`emit`,
  `emit_with_stdout_full`, `code_of`, `stderr_of`, `stdout_of`,
  `an_answer_that_cannot_be_written_exits_2`).
- Prior art, `goad`: `main.rs` `start` (the config's two arms split where
  the path is in hand, `clock::wall_clock`, `ProcessBackend::new`, the
  runtime, `Host::new`); `tests/binary/exit_codes.rs`
  `an_answer_that_cannot_be_written_exits_2`.
- Stratum 2: `goad_shell::config::{Config, BackendConfig, ScheduleConfig,
  Command, default_path, positive_duration}`, `Config::load`,
  `Command::from_argv`; `goad_shell::error::ConfigError` (its `Display`);
  `goad_shell::ingress::envelope::{normalize, EnvelopeFault}` (its
  `Display`); `goad_shell::clock::{wall_clock, ClockError}`;
  `goad_shell::backend::process::ProcessBackend::new`;
  `goad_shell::host::Host::new`; `goad_shell::report::{try_line_to,
  line_to}`; `goad_shell::version::version_line`.
- `goad-boundary`: `tests/checks/vocabulary.rs`
  (`no_workspace_member_names_the_users_domain`, `domain_scan`, `DOMAIN`);
  `tests/checks/allowlist.rs`' module doc; `src/members.rs` `members`.
- Shared test support: `tests/support/driving.rs`, `scripting.rs`,
  `waiting.rs`, each read for its whole exported surface (VA-7).
- `docs/memory/`: `autotests-false-hides-an-undeclared-test-target`,
  `clippy-toml-test-exemptions-are-a-hidden-boundary`,
  `negative-control-must-compile`, `tests-asserting-proxies`,
  `cargo-test-cwd-is-package-root-not-workspace-root`,
  `shared-test-helper-lives-at-workspace-root-via-path`,
  `a-substring-scan-and-a-word-boundary-scan-are-different-instruments`,
  `exit-2-means-two-different-failures`,
  `nix-build-in-a-checkout-reads-the-git-tree`. The user's memory adds
  *a standing guard may not reach a new file* and *mutation-check the
  coverage claim*: both bind this phase's reach rows.
- §Harvest *Learned*: a `todo!()` stub is a compiling red, with ignored
  parameters spelled `_name`; run an exit grep before naming new tests.

**Assumptions — verified now** (at 99208da, by reading, grep, and the runs
named)
- **A-V1 — every stratum-2 symbol the plan names exists and is public.**
  Verified by symbol: `config::Command::from_argv(argv: Vec<String>) ->
  Option<Self>` is `pub`, and its doc names `goad-check`'s argv form;
  `config::positive_duration` (EN-2); `config::default_path(env: &dyn
  Fn(&str) -> Option<OsString>) -> Option<PathBuf>`, `None` when neither
  `XDG_CONFIG_HOME` nor `HOME` names a directory; `Config::load(&Path) ->
  Result<Config, ConfigError>`; `envelope::normalize(&[u8]) -> Result<Event,
  EnvelopeFault>`; `clock::wall_clock() -> Result<Timestamp, ClockError>`;
  `ProcessBackend::new(Command, Duration)`; `Host::new(Config, B,
  Timestamp)`; `report::try_line_to(impl Write, &str) -> io::Result<()>`;
  `version::version_line(&str, Option<&str>) -> String`.
- **A-V2 — the argv form can build a `Config` without a file.** `Config`,
  `BackendConfig` and `ScheduleConfig` have public fields only, so a struct
  literal works: `ingress: None` (the checker opens no socket),
  `default_poll` the fixed `30m` (§5.2.1). `default_poll` is a
  `jiff::SignedDuration`. Either `jiff` becomes a direct dependency
  (`tests/support/driving.rs`' `DEFAULT_POLL` spells
  `jiff::SignedDuration::from_mins(30)`), or the value comes from
  `goad_semantics::schedule::parse_span("30m")`, whose `Err` arm cannot
  arise and still has to be written. The executor chooses and records a
  decision.
- **A-V3 — the errors the checker names all carry a `Display`, and none
  spells a requirement id.** `ConfigError`'s `Read` arm reads
  *"configuration could not be read: {inner}"* and names no file, so the
  checker names the path, as `goad`'s `start` does. `EnvelopeFault`'s
  `ReservedSource` arm reads *"source \"host\" is reserved to evaluations the
  host originates"*: the `"host"` literal is in `goad-shell`, not in
  `goad-check`, so I-1 does not see it. `ClockError` has a `Display`. So
  printing through each `Display` keeps I-1 clean. A line written in
  `goad-check` that cites a SPEC-003 id, as `goad-emit`'s `render.rs` does
  (*"which SPEC-003/R-8 forbids"*), would be an I-1 hit. A dry run of I-1's
  command over `crates/goad-emit/src` finds exactly that line.
- **A-V4 — `goad-emit`'s shape, and where `goad-check` must differ.**
  `goad-emit` writes `ExitCode::from(2)` in `main`'s usage-error arm, in
  `exchange`, in `startup_failed` and in `answer`. EX-4
  wants **one** `ExitCode::from(2)` reading no cause, so `goad-check` must
  not copy that shape. `goad-emit` writes stderr through `to_stderr`
  (`report::line_to` on `std::io::stderr().lock()`). `print_stdout`,
  `print_stderr`, `dbg_macro` and `use_debug` are denied workspace-wide, so
  every line goes through `report`.
- **A-V5 — the workspace's tokio already has what a current-thread runtime
  needs.** `[workspace.dependencies]` `tokio` carries `rt`, so
  `Builder::new_current_thread().enable_all().build()` needs no feature
  added. `goad`'s `start` builds a **multi-thread** runtime (its manifest
  adds `rt-multi-thread`). EX-7 asks for current-thread (§5.4
  *Concurrency*), so "the way `start` builds them" binds the `Host` and
  the backend, not the runtime's flavour.
- **A-V6 — the residue.** Stratum 1's dependencies are `jiff`, `serde` and
  `serde_json` (`allowlist.rs` `STRATUM_1`). A feature `goad-check` adds to
  any of them unifies into stratum 1 under `--workspace`, and nothing in the
  gate rejects it (POL-001 §Verification, *the residue*). `tokio` is not
  stratum 1's dependency.
- **A-V7 — what each standing guard reaches, read from the instrument.**
  - *Crate edges* (Cargo resolution): stratum 1's direction rule. It does
    not apply to a stratum-3 crate. It **does** hold I-6 at the source
    level: with no `slint` entry in `goad-check`'s manifest, naming `slint`
    is `error[E0433]`. Only the manifest says so (FU-7).
  - *Manifest allowlist* (`allowlist.rs`): its subjects are named
    (`the_real_stratum_1_manifest_is_clean`,
    `the_real_stratum_2_manifest_is_clean`). It does **not** reach
    `goad-check`, by design: *"a stratum-3 manifest is billed by nothing
    here"* (its module doc). What holds I-6 instead is the manifest's
    comment and review (EX-5, VA-5).
  - *Stratum 1 purity scan*: reads stratum 1's sources only. Not
    applicable.
  - *`cargo test -p goad-semantics`*: rejects nothing. A feature on a
    shared dependency is the residue (A-V6).
  - *Vocabulary scan*: `no_workspace_member_names_the_users_domain` walks
    `members::members` over the root manifest, reading `.rs` and `.slint`
    under each member and **excluding `tests/` and `target/`**
    (`domain_scan`). So it reaches `crates/goad-check/src` once the member
    is listed, and never `tests/binary/`. String literals are read: the scan
    cuts comments, not strings
    (`a_string_hides_no_token_that_follows_it_on_the_same_line`).
    `no_member_manifest_names_the_users_domain_in_its_own_crate_name`
    reaches the new manifest's `[package].name` the same way. The words are
    `DOMAIN`'s, matched as whole words; `site`, `goal` and `compliance` are
    among them, so a report string such as *"call site"* is a breach.
  - *Clippy* (`--workspace --all-targets -- -D warnings`): reaches every
    declared target. `clippy.toml` exempts `unwrap_used`, `expect_used`,
    `panic` and `indexing_slicing` in test code. **Measured at 99208da**:
    `.unwrap()` planted in `goad-emit`'s `exchange::code_of` (a
    `#[cfg(test)]` module in the `binary` target): `cargo clippy -p
    goad-emit --all-targets -- -D warnings` exit **0**. A `let _planted:
    std::collections::HashMap<u8, u8> = std::collections::HashMap::new();`
    at the same site: exit **101**, *"use of a disallowed type
    `std::collections::HashMap`"*. Restored by copy; `git status --short`
    empty. PLAN QUESTION 2 (resolved: the `tests` half plants `HashMap`).
  - *fmt* (`cargo fmt --all --check`): reaches each target's module tree.
    A file under `tests/binary/` that no `mod` names is neither compiled
    nor formatted.
  - *`cargo test --workspace`*: with `autotests = false`, a `tests/binary/`
    directory without its `[[test]]` block is silently not built
    (`docs/memory/autotests-false-hides-an-undeclared-test-target.md`).
  - *`goad-boundary`'s `structure.rs`*: fixed subject directories
    (`crates/goad/src`, `crates/goad-shell/src`). Not applicable.
- **A-V8 — `tests/support/` at this phase.** `driving.rs` composes a `Host`
  and its `Outcome` readers; `scripting.rs` spawns scripted backends;
  `waiting.rs` polls. A PHASE-04 binary case spawns only `goad-check`, never
  a backend, so it uses no file's whole surface. The expected outcome of
  VA-7 is **none included**. The spawn and readers are copied from
  `goad-emit`'s `exchange.rs` (`emit`, `emit_with_stdout_full`, `code_of`,
  `stderr_of`, `stdout_of`), which another crate's target cannot include.
  Each copy is named in §Open's FU-5 row. This is a prediction; VA-7 is the
  read.
- **A-V9 — the binary never spawns the backend at PHASE-04.** `Config::load`
  does not check that the program exists, and `Host::new` spawns nothing.
  So VT-3's loadable configuration may name any command, and PHASE-04 needs
  no bash fixture. Paths a case reads are `CARGO_MANIFEST_DIR`-joined
  (`docs/memory/cargo-test-cwd-is-package-root-not-workspace-root.md`).
- **A-V10 — the instrument commands must run under the system `grep`.** In
  the agent's shell, `grep` is a function that runs `ugrep` with
  `--ignore-files`, recursing into a directory unasked. System `grep` given
  a directory without `-r` prints *"Is a directory"* and exits 2 (measured:
  `command grep -n 'ExitCode' crates/goad-emit/src`). Run every recorded
  instrument as `command grep …`, so the record is what a person's shell
  would print. PLAN QUESTION 3 (resolved: VA-6 now says so).
- **A-V11 — I-1's command, dry run.** `command grep -rnE
  'R-?[0-9]+|AtFault::|"(backend|host|configuration|environment)"'
  crates/goad-emit/src | command grep -vE '^[^:]+:[0-9]+:[[:space:]]*//'`
  runs and prints one hit (A-V3). So the command is sound. *"Outside
  comments"* is how VA-1 reads it: the second `grep` drops a line whose
  first non-blank characters are `//` (so `///` and `//!` too). Each
  surviving hit is read by eye and classed: code, or a trailing `//`
  comment on a code line, which is recorded as a comment. A `/* … */`
  comment is not dropped; there should be none. The command reads `src`
  only, so unit tests in `src` (VT-1's) are inside its reach.
- **A-V12 — EX-8's symbol.** `goad_shell::version::version_line` exists,
  and EX-8 names it (quoted below). `goad-emit`'s `main` `Invocation::Version`
  arm is the model: it passes its own `env!("CARGO_PKG_VERSION")` and
  `option_env!("GOAD_REVISION")`. Every member is `0.1.0` from the
  workspace, so no test can see the version argument (PHASE-03 A-V1).

**Assumptions — first tested by this phase**
- **A-T1 — the interim end compiles with no dead code.** The run builds a
  `Host`, a runtime and a list of normalized events, and uses none of them.
  Bindings spelled `_host`, `_runtime` and `_events` raise no lint, and
  `unused_crate_dependencies` is paused. Watch for a type or field that only
  PHASE-12 would read: `dead_code` is a gate error. Do not reach for
  `#[expect(dead_code)]` ahead of a caller
  (`docs/memory/expect-dead-code-ahead-of-caller-needs-cfg-attr.md`). Any
  such site is a decision, recorded.
- **A-T2 — one `ExitCode::from(2)` can serve every cause.** A single
  function such as `not_judged(line) -> ExitCode` writes the line through
  stderr and returns 2. Every path to 2 returns through it, the interim end
  and an unwritten answer included. VA-6 reads it.
- **A-T3 — the binary-tier cases can be seen red against code that
  compiles.** A `main` that parses and returns `ExitCode::SUCCESS` reds
  every status-2 case on status.
- **A-T4 — the new `[[test]]` target runs under `cargo test --workspace`.**
  Proven by its `Running tests/binary/main.rs (…goad_check…)` line and its
  case count, not by a green exit.

**STOP conditions** (consult the user; do not improvise)
- From `plan.md` PHASE-04 Notes (quoted): *"A new external dependency (a
  temp-dir crate, an argument parser) is a STOP."* A workspace dependency
  already in the lockfile is not new (EX-5). A **feature** added to a
  dependency shared with stratum 1 (`jiff`, `serde`, `serde_json`) is also a
  STOP: it is the residue, and the slice taking it argues it (A-V6).
- A file outside **Surfaces**, including any change to `goad-shell` (a new
  helper, a visibility change). The plan treats stratum 2 as finished
  ground.
- Any requirement-id spelling or side literal in `crates/goad-check/src`
  outside a comment (I-1, VA-1). In particular, no cited SPEC-003 or
  SPEC-004 id in a stderr line (A-V3).
- Any mapping from a field kind to a JSON type or value (I-2).
- A second `ExitCode::from(2)` site, or a 2 that reads its cause (EX-4,
  VA-6).
- An exchange, a verdict line, or a status 0 or 1 for a run. These are
  PHASE-12's (EX-3, EX-4).
- `goad-check` opening a socket or naming an `ingress` item other than
  `envelope::normalize` (§5.2.1; PHASE-12/EX-5 holds it later).
- A domain word in `crates/goad-check/src` (A-V7).
- A test name differing from `design.md` §9 or `plan.md` VT-2/VT-3. Update
  `canon-delta.md` in the same commit and say so here (*Test names are
  commitments*). Not a STOP, but never silent.
- `git stash`, `git checkout`, `git reset`, or any history rewrite.

**Tasks** — red first. Each new case goes red against code that compiles
before its body lands. Reach is proven as soon as the crate exists, not at
the end: a guard that is green over a crate it never read proves nothing.

- [x] Set PHASE-04 to `in progress` in §Status.
      *Done in f4e9ccd.*
- [x] Print `git log -1 --oneline`. It must be this sheet's commit, or a
      descendant whose only changes since are the PLAN QUESTIONs'
      resolutions.
      *`83cf674 012 PHASE-04: decisions applied to design, plan and sheet`:
      the resolutions' commit, a descendant of this sheet's.*
- [x] Exit grep first (§Harvest *Learned*): `command grep -rn
      'goad-check\|goad_check' crates --include=*.rs` and `command grep -rn
      'an_unreadable_config_exits_2_and_says_who_spoke\|a_reserved_source_event_file_exits_2\|an_empty_argv_is_a_usage_error\|a_report_that_cannot_be_written_exits_2\|a_run_with_no_exchange_exits_2_with_no_verdict'
      crates`. At 99208da the first finds only `goad-shell`'s `config.rs`
      docs (`Command::from_argv`, `positive_duration`, and a `mod tests`
      doc), and the second finds nothing. Record any difference. The
      `help_…` and `version_…` names are left out: they are `goad`'s and
      `goad-emit`'s too, by design.
      *As predicted: the first grep found only `goad-shell`'s `config.rs` docs
      (`Command::from_argv`, `positive_duration`, the `mod tests` doc), exit
      0; the second found nothing, exit 1.*
- **1. The crate exists, and every guard reaches it (EX-5, EX-6, VA-3)**
  - [x] `crates/goad-check/Cargo.toml`: workspace package keys,
        `autotests = false`, `[lints] workspace = true`, `[[test]] name =
        "binary"`, `path = "tests/binary/main.rs"`. `[dependencies]` holds
        only what this phase uses, each `{ workspace = true }` and with no
        added feature on `jiff`, `serde` or `serde_json` (A-V6). Expected:
        `goad-semantics`, `goad-shell`, `tokio`, and `jiff` only if A-V2's
        choice needs it. Keep every inline table on one line: crane parses
        every member's manifest with a TOML 1.0 parser (root `Cargo.toml`'s
        `tokio` comment), and the gate cannot see a break. EX-5 (quoted):
        *"`crates/goad-check/Cargo.toml` names only strata 1 and 2 and
        workspace dependencies already in the lockfile, with a comment
        arguing it links no renderer, as `goad-emit`'s does (I-6)."* The
        comment says why `tokio` is here when `goad-emit` has none: one
        current-thread runtime drives `Host`.
        *f4e9ccd. `[dependencies]`: `goad-semantics`, `goad-shell`, `jiff`,
        `tokio`, each `{ workspace = true }`, no feature added; one-line
        inline tables. `jiff` per Decision D-1.*
  - [x] Root `Cargo.toml` `members`: `"crates/goad-check"` after
        `"crates/goad-emit"`, before `"crates/goad-boundary"`. `Cargo.lock`
        gains the package entry and nothing else. Check with `git diff
        Cargo.lock`: no version or source line moves.
        *f4e9ccd. `git diff Cargo.lock`: one added `[[package]]` block,
        `goad-check` with its four dependencies; no version or source line
        moved.*
  - [x] A minimal `src/main.rs` that compiles clean: a `//!` doc and a
        `main` returning `ExitCode::SUCCESS`. A `tests/binary/main.rs` with
        the `#[cfg(test)] mod …;` declaration and its reason, as
        `goad-emit`'s has.
        *f4e9ccd; the module is `statuses.rs`.*
  - [x] EX-6 (quoted): *"`allowlist.rs`' module doc names `goad-check`
        among the stratum-3 members, by name and without a count."* The doc
        now says *"Three of the workspace's five members carry no allowlist
        here. `goad` and `goad-emit` are stratum 3"*. That count is false
        once `goad-check` lands. Rewrite the sentence to name the members
        that carry no allowlist, with no count. The later sentence about
        `crates/goad-emit`'s freedom from the renderer stays true and is
        left alone (*"member list only"*).
        *f4e9ccd. Now: "Only stratum 1 and stratum 2 carry an allowlist here.
        `goad`, `goad-emit` and `goad-check` are stratum 3, …". No count; the
        `crates/goad-emit` sentence untouched.*
  - [x] **Reach, before any behaviour:**
    - [x] `cargo test --workspace --no-fail-fast 2>&1 | command grep -n
          'Running.*goad_check\|Running tests/binary/main.rs'`: the target's
          `Running` line names `goad-check`'s binary (A-T4). Record it.
          *At f4e9ccd's tree: `Running unittests src/main.rs
          (target/debug/deps/goad_check-420588ca26d5e988)` then `Running
          tests/binary/main.rs (target/debug/deps/binary-c539832a98b86fdd)`,
          each `0 passed` (no case yet). A-T4 held.*
    - [x] VA-3, vocabulary: see the mutation table, row R-1.
          *Red; table.*
    - [x] VA-3, clippy `src`: row R-2. Clippy `tests`: row R-3.
          *Both red; table.*
    - [x] fmt, `src` and `tests/binary/`: row R-4 (offered by this sheet;
          not a plan criterion).
          *Red; table.*
- **2. The command line (EX-1, VT-1)** — `src/args.rs`, pure
  - [x] `Invocation` and `UsageError` types, and `parse(argv: impl
        Iterator<Item = OsString>) -> Result<Invocation, UsageError>` with a
        `todo!()` body (ignored parameter spelled `_argv`).
        *Also `Request` and `Source` (the two forms).*
  - [x] VT-1 (quoted): *"`args.rs` unit tests: the invocation table
        (config form, argv form, `--event` order, `--timeout` with
        `--config` refused, `--timeout` with neither `--config` nor `--`
        refused, `--config` with `--` refused, `--timeout 0s` and `-1s`
        refused, an empty argv and an empty program refused, help,
        version)."* One case per row, named by behaviour. Red: each panics
        at the `todo!()`. Record the count.
        *Red: 25 of 25 panicked at `parse`'s `todo!()`, compiling. Green at c9dd5b4. Names by behaviour; the config form:
        `no_arguments_is_the_config_form_at_the_default_path`,
        `config_names_the_file_to_load`; argv form:
        `the_command_after_the_separator_is_the_argv_form_with_a_five_second_timeout`,
        `timeout_sets_the_argv_forms_timeout`; order:
        `events_are_kept_in_the_order_given`; refusals:
        `a_zero_timeout_is_refused`, `a_negative_timeout_is_refused`,
        `an_empty_command_is_refused`, `an_empty_program_is_refused`; help and
        version: `help_is_help_wherever_it_appears_before_the_separator`,
        `version_is_version`.*
  - [x] Among those rows, the three refused combinations EX-1 names, each
        asserting a `UsageError`: `--timeout` with `--config`; `--timeout`
        with neither `--config` nor `--` (the default-path config form);
        `--config` with `--`. Red with the rest, at the `todo!()` (PLAN
        QUESTION 5, resolved).
        *`a_timeout_with_config_is_refused`,
        `a_timeout_with_neither_config_nor_a_command_is_refused` (both
        `UsageError::TimeoutWithoutCommand`, D-2),
        `config_with_a_command_is_refused` (`ConfigWithCommand`).*
  - [x] Rows the sheet adds, from `goad-emit`'s table and §5.2.1: help wins
        over version in either order; after `--` every token is the
        command's, `--help` included; an unknown flag; a flag with no value;
        a repeated `--config` or `--timeout`; `--event` repeatable. Also
        `--timeout` absent in the argv form gives the `5s` default.
        *`help_wins_over_version_in_either_order`,
        `every_token_after_the_separator_is_the_commands`,
        `an_unknown_flag_is_refused_naming_it`,
        `a_flag_with_no_value_is_refused`, `a_repeated_config_is_refused`,
        `a_repeated_timeout_is_refused`; repeatable `--event` is
        `events_are_kept_in_the_order_given`; the `5s` default is the
        argv-form row. Added beyond the sheet (D-5):
        `a_timeout_that_is_not_a_span_is_refused`,
        `a_help_token_in_value_position_is_a_value`,
        `a_bare_argument_before_the_separator_is_refused`,
        `an_empty_config_or_event_path_is_refused`,
        `a_command_token_that_is_not_utf8_is_refused`.*
  - [x] The body. EX-1 (quoted): *"the command line of `design.md` §5.2.1:
        config form (`--config`, else `config::default_path`), argv form
        after `--` through `Command::from_argv`, `--timeout` (default `5s`,
        accepted only in the argv form), repeatable `--event FILE` through
        `envelope::normalize`, `-h`, `--help`, `--version`. `--config` and
        `--` exclude each other. Each refused combination — `--timeout`
        with `--config`, `--timeout` with neither `--config` nor `--`,
        `--config` with `--` — is a usage error, status 2 (`design-log.md`
        2026-10-01, *`goad-check`'s flag exclusions, stated whole*).
        `args.rs` is pure and returns one `Invocation`."* `args.rs` does
        not read the environment. `default_path` is applied in `main`, where the
        environment is read (§5.2.1, last bullet). `args.rs` carries the
        event *paths*; `main` reads and normalizes them (EX-7).
        `--timeout`'s value goes through `config::positive_duration` with
        the key `"--timeout"`, and its `ConfigError` becomes a
        `UsageError`. Green.
        *c9dd5b4. `args.rs` reads no environment; `--timeout` goes through
        `config::positive_duration(TIMEOUT, …)`, its `ConfigError` carried as
        `UsageError::Timeout`; the command through `Command::from_argv`.*
- **3. The lines (EX-3)** — `src/render.rs`, pure
  - [x] EX-3 (quoted): *"the report writer: a `render`-style module owns
        every line's text, and every stdout line goes through
        `report::try_line_to`. The no-view line is written here. Until
        PHASE-12 a run makes no exchange, so it ends with no verdict: its
        report is the no-view line, a stderr line says the run is not yet
        implemented, and it exits 2. Nothing on `main` reports an acceptance
        it did not judge. PHASE-12/EX-6 replaces this end with the
        verdict."*
        *`render.rs` owns every line; `main`'s `unjudged_end` writes
        `render::NO_VIEW` through `try_line_to`, then
        `render::NOT_YET_IMPLEMENTED` through `not_judged`.*
  - [x] `render`: `USAGE`, the usage-error line, a config line naming the
        path (both `ConfigError` arms, as `start` splits them), an event-file
        line naming the path (read failure and `EnvelopeFault`), clock and
        runtime lines, the answer-unwritten line, the no-view line (§5.2.5:
        *"no exchange returned a view … respond was not exercised"*), and
        the not-yet-implemented line. Every stderr line begins `goad-check:
        `. Each one interpolates the fault's own `Display`. None spells a
        requirement id or a side word as a literal (A-V3, I-1). Unit cases
        over the lines, in the style of `goad-emit`'s `render` `mod tests`.
        Red against `todo!()` bodies first.
        *Red: 4 cases panicked at the stubs (the `const` cases passed, having
        no body to stub). Green at c9dd5b4. Lines: `USAGE`,
        `usage_error_line`, `startup_error_line` (the config arms split as
        `start` does, the event-file arms, the clock, the runtime, and
        `NoPath`), `answer_unwritten_line`, `report_unwritten_line` (D-3),
        `NO_VIEW`, `NOT_YET_IMPLEMENTED`.*
- **4. The binary tier, red (VT-2, VT-3)** — `tests/binary/`
  - [x] Read each `tests/support/` file's whole exported surface (VA-7) and
        record the include decision for each (A-V8).
        *Read every `pub(crate)` item of `driving.rs` (`CLEANUP_LIMIT`,
        `DEFAULT_POLL`, `config`, `host`, `host_from`, `quiet_event`, `event`,
        `instant`, `failure_or_nothing`, `choice`, `answer_first_option`,
        `presented`), `scripting.rs` (`backend`, `claim`, `marker`, `clear`,
        `logging_backend`, `invocations`, `scripted`) and `waiting.rs`
        (`LIVENESS_BOUND`, `within`). The tier spawns only `goad-check` and
        uses none: none included (A-V8 held).*
  - [x] Helpers: the spawn over `env!("CARGO_BIN_EXE_goad-check")`, the
        `/dev/full` spawn, `code_of`, `stderr_of`, `stdout_of`, and a
        last-stderr-line reader. Copies of `goad-emit`'s `exchange.rs`, each
        doc saying so.
        *`check`, `check_with_stdout_full`, `code_of`, `stderr_of`,
        `stdout_of`, each doc naming its `goad-emit` original; plus `fixture`,
        `assert_not_judged` (status 2, last stderr line prefixed) and
        `assert_no_report`, which are not copies.*
  - [x] VT-2 (quoted): *"binary tier, `tests/binary/`, the status-2 cases
        §9 names: `an_unreadable_config_exits_2_and_says_who_spoke`,
        `a_reserved_source_event_file_exits_2`,
        `an_empty_argv_is_a_usage_error`, and
        `a_report_that_cannot_be_written_exits_2` with its `--help` half
        (PHASE-12/VT-2 adds the run half). Each asserts status 2 and the
        `goad-check: ` prefix on the **last** stderr line. Each but
        `a_report_that_cannot_be_written_exits_2` also asserts stdout is
        empty: each fails before the first report line, at PHASE-12 too, and
        without it the interim end of EX-3 would pass it. And the two
        answered cases §9 names,
        `help_prints_the_usage_block_on_stdout_and_exits_0` and
        `version_prints_the_package_version_on_stdout_and_exits_0`, as
        `goad`'s and `goad-emit`'s (`plan-log.md` 2026-10-01, *PHASE-04 sheet
        questions*, Q1, Q4)."* Fixtures: a
        nonexistent `--config` path; a committed event file under
        `tests/binary/` with `"source": "host"`, run with `--event` (it must
        reach normalization: give it a loadable `--config` or an argv form,
        so no earlier step fails first); `goad-check --`.
        *`crates/goad-check/tests/binary/statuses.rs`. Fixtures: `absent.toml`
        (never created), `loadable.toml`, `reserved-source.json`, run with
        `--config loadable.toml --event reserved-source.json`.*
  - [x] Each VT-2 status-2 case except
        `a_report_that_cannot_be_written_exits_2` asserts **stdout is
        empty** (PLAN QUESTION 1, resolved). Without it, each one is green
        against the interim end (rows M-3, M-5).
        *`assert_no_report`; M-3 and M-5 red on it.*
  - [x] VT-3 (quoted): *"binary tier, the plan's own interim case:
        `a_run_with_no_exchange_exits_2_with_no_verdict` — a loadable
        configuration; status 2, stdout the no-view line and no verdict
        line, the last stderr line beginning `goad-check: `. PHASE-12/EX-6
        deletes it."* A committed config under `tests/binary/`, passed with
        `--config`. Assert stdout is **exactly** the no-view line: no
        verdict line, nothing else.
        *`assert_eq!(stdout, "no exchange returned a view, so respond was not
        exercised\n")`.*
  - [x] `help_prints_the_usage_block_on_stdout_and_exits_0` and
        `version_prints_the_package_version_on_stdout_and_exits_0`, modelled
        on `goad-emit`'s `exchange.rs` cases of those names: status 0, the
        answer on stdout, stderr empty (PLAN QUESTION 4, resolved).
  - [x] Red, against task 1's `main` (A-T3), compiling: each status-2 case
        fails on status; the `help_…` and `version_…` cases, which that
        `main` already exits 0 for, fail on stdout. `cargo test -p
        goad-check --test binary --no-fail-fast`. Record each failure
        message.
        *7 of 7 failed. Status cases, `assertion left == right failed, left:
        0, right: 2` at `assert_not_judged`:
        `an_unreadable_config_exits_2_and_says_who_spoke`,
        `a_reserved_source_event_file_exits_2`,
        `an_empty_argv_is_a_usage_error`,
        `a_report_that_cannot_be_written_exits_2`,
        `a_run_with_no_exchange_exits_2_with_no_verdict`. `help_…` failed on
        `stdout.starts_with("usage: goad-check")`; `version_…` on `left: "",
        right: "0.1.0"`. A-T3 held.*
- **5. `main` (EX-4, EX-7, EX-8)**
  - [x] EX-7 (quoted): *"the steps before the first exchange, each ending
        the run with no verdict, status 2, on failure: the configuration
        loaded, each `--event` file normalized in the order given, the clock
        read, a current-thread runtime and a `Host` built the way `goad`'s
        `main.rs` `start` builds them."* Read `start` first (Notes). In the
        config form: an explicit `--config`, else `default_path` over
        `std::env::var_os` (`std::env::var` is a `disallowed-method`). A
        `None` from it is status 2 with a line. In the argv form: A-V2's
        struct literal, with `ingress: None`. `ProcessBackend::new(command,
        timeout)`, then `Host::new(config, backend, now)` (A-V5: the runtime
        is current-thread).
        *1008cc9. `prepare`: `configuration` (`--config`, else
        `config::default_path` over `std::env::var_os`, `None` is
        `StartupFault::NoPath`; the argv form a struct literal with `ingress:
        None` and `DEFAULT_POLL`), `normalized` (each file read then
        `envelope::normalize`, in order), `clock::wall_clock`,
        `Builder::new_current_thread().enable_all().build()`,
        `ProcessBackend::new(command.clone(), timeout)`, `Host::new(config,
        backend, now)`. A-T1 held: `_events`, `_runtime`, `_host` raised no
        lint, and no type or field was built for PHASE-12 (D-4).*
  - [x] EX-4 (quoted): *"status 2 as §5.2.5: every cause of 2, the interim
        end of EX-3 included, reaches one `ExitCode::from(2)` that reads no
        cause; `main` returns an `ExitCode` built from literals, one per
        class it can reach; the last stderr line on 2 begins `goad-check: `.
        The verdict line and the cut to 0 or 1 are PHASE-12's (EX-6):
        written here, nothing would feed them, and the gate's lint refuses
        dead code."* The classes reachable here are 0 (a question answered)
        and 2.
        *`not_judged` is the one `ExitCode::from(2)`; `answer` holds the one
        `ExitCode::SUCCESS`. VA-6 traces every path.*
  - [x] EX-8 (quoted): *"`--version` prints
        `goad_shell::version::version_line` with this crate's package
        version and its compilation's `GOAD_REVISION`, as `goad-emit`'s does
        (PHASE-03/EX-8)."* Pass `env!("CARGO_PKG_VERSION")` and
        `option_env!("GOAD_REVISION")` at the call site (A-V12).
        *`Invocation::Version` arm:
        `answer(&version_line(env!("CARGO_PKG_VERSION"),
        option_env!("GOAD_REVISION")))`. M-13 shows the version case reads
        it.*
  - [x] `--help` and `--version` are written through `try_line_to` on
        `std::io::stdout().lock()`. On `Err`, the answer-unwritten line,
        and 2 through the one site.
        *`answer`. M-11 shows the `/dev/full` case reads it.*
  - [x] Green: VT-1, the render cases, VT-2, VT-3. `cargo test -p
        goad-check --no-fail-fast`, and `cargo clippy -p goad-check
        --all-targets -- -D warnings` clean.
        *1008cc9: 31 unit, 7 binary passed (30 unit after the refactor,
        e3b55ee); `cargo clippy --workspace --all-targets -- -D warnings`
        clean.*
- **6. Reads and records (VA-1, VA-2, VA-4..VA-7)**
  - [x] VA-1 (quoted): *"I-1 by the command under *Invariant reads*: no hit
        outside a comment at this phase, in any of the spellings that
        command matches; the R-56 claim and its `Requirement::R56` are
        PHASE-12's. Recorded."* Run `command grep -rnE
        'R-?[0-9]+|AtFault::|"(backend|host|configuration|environment)"'
        crates/goad-check/src | command grep -vE
        '^[^:]+:[0-9]+:[[:space:]]*//'` (A-V10, A-V11). Read each surviving
        line and class it. Expected: none. Record the output verbatim.
        *First run, at e3b55ee: one hit, `crates/goad-check/src/render.rs:201:
        br#"{"source":"host",…}"#`, a JSON fixture in `render`'s unit test. A
        source value, not a side, but a hit outside a comment, which the
        criterion forbids. Repaired at 35185f5 (D-7): the case names
        `EnvelopeFault::ReservedSource`. Rerun: the pipeline prints nothing,
        `PIPESTATUS` `0 1`. The first grep's one hit, all dropped as comments:
        `crates/goad-check/src/main.rs:176:/// A question's answer, on stdout:
        status 0 only if it arrived (SPEC-004/R-8's`.*
  - [x] VA-2 (quoted): *"I-2 by the command under *Invariant reads*; each
        match found is read and recorded."* `command grep -rn 'FieldKind'
        crates/goad-check/src crates/goad-check/tests`. Expected: none.
        Record it.
        *`command grep -rn 'FieldKind' crates/goad-check/src
        crates/goad-check/tests`: no output, exit 1. No match to read.*
  - [x] VA-6 (quoted): *"R-13 and R-15, structurally: `grep -rn 'ExitCode'
        crates/goad-check/src` shows the one `ExitCode::from(2)`, reached by
        every status-2 path without reading its cause, and one literal per
        class; read and recorded. Every instrument command this phase
        records is run as `command grep`, the system `grep`, so the record
        is what a person's shell prints (`plan-log.md` 2026-10-01, *PHASE-04
        sheet questions*, Q3)."* Run `command grep -rn 'ExitCode'
        crates/goad-check/src`, then trace every path to 2 by reading `main`.
        Name each cause and the call that carries it to the one site.
        *`command grep -rn 'ExitCode' crates/goad-check/src` prints the `use`,
        the four signatures (`main`, `unjudged_end`, `answer`, `not_judged`),
        `ExitCode::SUCCESS` in `answer` and `ExitCode::from(2)` in
        `not_judged`, one each. Every path to 2, read in `main`: a usage error
        (`main`'s `Err` arm, `not_judged(&render::usage_error_line(…))`);
        every `StartupFault` (`NoPath`, `ConfigUnreadable`,
        `ConfigUnparseable`, `EventUnreadable`, `EventRefused`, `Clock`,
        `Runtime`), returned by `prepare` and passed by `main` to
        `not_judged(&render::startup_error_line(…))`; an unwritten `--help` or
        `--version` (`answer`'s `Err` arm); an unwritten report line
        (`unjudged_end`'s `Err` arm); and the interim end (`unjudged_end`'s
        `Ok` arm). `not_judged` takes a `&str` and reads nothing else.*
  - [x] VA-4 (quoted): *"`canon-delta.md` SPEC-004 Change 5's R-11..R-13 row
        gives each `goad-check` case this phase ships its own shipped path
        (`crates/goad-check/tests/binary/<file>.rs::<name>`); PHASE-12's
        cases keep `…::`; the R-14 row is untouched (`plan-log.md`
        2026-10-01, *PHASE-04 sheet questions*, Q6)."* The cases that row
        names and this phase ships:
        `an_unreadable_config_exits_2_and_says_who_spoke`,
        `a_reserved_source_event_file_exits_2`,
        `a_report_that_cannot_be_written_exits_2` (PLAN QUESTION 6,
        resolved).
        *d8932c3: the row's
        `::an_unreadable_config_exits_2_and_says_who_spoke`,
        `::a_reserved_source_event_file_exits_2` and
        `::a_report_that_cannot_be_written_exits_2` each now read
        `crates/goad-check/tests/binary/statuses.rs::<name>`; PHASE-12's keep
        `::`; one line changed (`git diff --stat`: 1 insertion, 1 deletion);
        the R-14 row untouched.*
  - [x] VA-5 (quoted): *"`notes.md` §Open's FU-7 row names
        `crates/goad-check/Cargo.toml`'s comment as what holds I-6."* Edit
        §Open's *FU-7's citation extends to `goad-check`* bullet.
        *d8932c3: the FU-7 bullet quotes the manifest comment and cites row
        R-5.*
  - [x] VA-7 (quoted): *"shared helpers: the binary tier reads each
        `tests/support/` file's whole exported surface, and includes each
        file whose every symbol it uses (`design.md` §5.2.6). Each helper it
        copies instead is named by symbol in `notes.md` §Open's FU-5 row,
        with the file it could not include."* Edit §Open's *FU-5's citation
        extends to whatever `goad-check`'s tests copy* bullet, naming each
        copy and its source, `crates/goad-emit/tests/binary/exchange.rs`.
        *d8932c3: the FU-5 bullet names `check`, `check_with_stdout_full`,
        `code_of`, `stderr_of`, `stdout_of` and their source, and records no
        `tests/support/` include.*
- **Refactor**
  - [x] Read the diff for a second statement of anything: a line's text
        outside `render`, a path to 2 outside the one site, a rule
        `goad-shell` already states (the timeout rule, the empty command,
        the default path) restated here.
        *e3b55ee. Every line's text is in `render`; every 2 goes through
        `not_judged`; the timeout rule, the empty command and the default path
        are `goad-shell`'s, called. Two prose restatements remain, by the
        precedent of `goad-emit`'s `USAGE`: `USAGE` restates `args::parse`'s
        forms and the `5s` default (its doc names `args::parse` as the
        authority). Dropped: `render`'s
        `an_unreadable_event_file_does_not_read_as_a_configuration` (the
        distinctness check in
        `every_startup_fault_says_what_failed_and_names_its_file` already
        holds it), and `USAGE`'s half-true *Exit status* line, which would go
        stale at PHASE-12.*
  - [x] Every doc in the new crate cites by symbol and counts nothing
        (CLAUDE.md *Name, never count*).
        *d8932c3 removed two counts that can grow: `args.rs`' "three flags and
        a separator" and `main`'s "Two statuses are reachable". Remaining
        number words name closed sets (the two strata below, the two forms,
        the two sources `ConfigWithCommand` names) or a test's own fixture.*
- **Verification**
  - [x] Mutation and reach rows (below), each recorded.
        *R-1..R-5 and M-1..M-16 run; every row compiled (R-5 apart, where the
        compile is the red), every row redded, `git status --short` clean
        after each restore. Rows that redded more than predicted: Findings.*
  - [x] *Test names are commitments*: `command grep -c 'fn <name>()'` over
        `crates/goad-check/tests/binary` is 1 for each VT-2 and VT-3 name.
        Any difference is updated in `canon-delta.md` in the same commit.
        *`command grep -rc 'fn <name>()' crates/goad-check/tests/binary`: 1,
        in `statuses.rs`, for each of the four VT-2 status-2 names, both
        answered names, and VT-3's. No difference; `canon-delta.md` unchanged
        on names.*
  - [x] (Offered, not a criterion; Findings.) `git add` the new crate, then
        `nix build --no-link .#goad .#goad-emit` succeeds. crane parses
        every member's manifest, and the gate cannot see a manifest it
        refuses. Record it, or record that it was skipped.
        *Run at d8932c3 (the crate committed, so the git input sees it): `nix
        build --no-link .#goad .#goad-emit` exit 0. crane parsed the new
        manifest; it builds no `goad-check` output yet (PHASE-05).*
  - [x] `just check` exits 0 on the final commit. Record passed and failed,
        summed over every `test result` line. The count is 678 plus the
        cases this phase adds.
        *At d8932c3: exit 0. Build, both test tiers, `deno check
        exercisers/typescript/backend.ts`, clippy with no warning, `cargo fmt
        --all --check`. **715 passed, 0 failed**, summed over all 33 `test
        result` lines: 678 plus this phase's 37 cases (25 `args`, 5 `render`,
        7 binary-tier), each counted once because `goad-check` runs under
        `cargo test --workspace` only. `just -n check` differs from POL-001
        §Compliance's block in the `deno check` line only (`exercisers/` for
        `examples/`), PHASE-03/EX-3's departure. The commits after d8932c3
        touch `notes.md` only; rerun at c263acc: exit 0, 715 passed, 0
        failed.*
  - [x] §Status: PHASE-04 `done`, with the date.
        *2026-10-01.*
  - [x] Harvest updated in place (*Fresh as of*, Produced, Learned, Open).

**Exit criteria** (quoted in the tasks above; listed here to be ticked)
- [x] EX-1 *`args::parse`; VT-1; M-1, M-2, M-3, M-14, M-15.*
- [x] EX-3 *`render`; `unjudged_end`; VT-3; M-7, M-12.*
- [x] EX-4 *`not_judged`, the one `ExitCode::from(2)`; VA-6; M-6, M-16.*
- [x] EX-5 *`crates/goad-check/Cargo.toml`: strata 1 and 2, `jiff` and `tokio` from the workspace, no feature added; the comment argues I-6; R-5.*
- [x] EX-6 *f4e9ccd; no count.*
- [x] EX-7 *`prepare`; `a_reserved_source_event_file_exits_2`, `an_unreadable_config_exits_2_and_says_who_spoke`, VT-3; M-5. The clock and runtime faults are headless-unreachable, as canon-delta's R-11..R-13 row says.*
- [x] EX-8 *`Invocation::Version` arm; M-13.*
- [x] VT-1 *25 cases, red at `todo!()`, green at c9dd5b4.*
- [x] VT-2 *Six cases in `statuses.rs`, red 6 of 6 at the scaffold `main`, green at 1008cc9.*
- [x] VT-3 *`a_run_with_no_exchange_exits_2_with_no_verdict`; red, then green at 1008cc9.*
- [x] VA-1 *Clean at 35185f5, after one hit repaired (D-7).*
- [x] VA-2 *No match.*
- [x] VA-3 *R-1, R-2, R-3 red; R-4 and R-5 offered, red.*
- [x] VA-4 *d8932c3.*
- [x] VA-5 *d8932c3.*
- [x] VA-6 *Recorded and traced.*
- [x] VA-7 *None included; five copies named in §Open's FU-5 bullet.*
- [x] `just check` exits 0 on the final commit (§Overview) *715 passed, 0 failed.*
- VA-3 (quoted, as the reach rows discharge it): *"reach:
  `goad-boundary`'s `no_workspace_member_names_the_users_domain` reads
  `crates/goad-check/src` — a planted domain word in a string literal, in
  code that compiles, reds it; restored. Clippy reaches the crate's `src`
  and `tests` — a planted `.unwrap()` in `src`, and a planted
  `std::collections::HashMap` in `tests` (`clippy.toml`'s
  `allow-unwrap-in-tests` exempts an `.unwrap()` there; `disallowed_types`
  is never test-exempt), each reds `cargo clippy --workspace --all-targets
  -- -D warnings`; restored. Both recorded."*

**Mutation evidence** (`plan.md` *Mutation evidence*: copy the file to the
scratchpad and back, never `git checkout` or `git stash`; `--no-fail-fast`;
`git status --short` clean after each restore; a mutation that does not
compile is not evidence, except where the compiler is the instrument
(row R-5)). The edits name code that does not exist yet. Each row states
the change by the symbol it will touch, and the executor quotes the exact
edit in the *edit* column when it runs. Command for M-rows: `cargo test -p
goad-check --no-fail-fast`. Run the M-rows marked "expected **not** to red"
too: they are predictions (§Harvest *Learned*).

*Run at e3b55ee by one script (`mutate.py`, scratchpad): back up the file,
apply the edit, `cargo test -p goad-check --no-run` (every M-row compiled),
`cargo test -p goad-check --no-fail-fast`, collect the failing cases,
restore, `git status --short`. Every M-row exited 101.*

*Reach rows* (VA-3, and what the sheet offers beside it):

| row | file | edit | command | must red | compiled? | result |
|---|---|---|---|---|---|---|
| R-1 | `crates/goad-check/src/main.rs` (production code, not a test module) | `let _planted = "habit";` before `main`'s `ExitCode::SUCCESS` | `cargo test -p goad-boundary --test checks --no-fail-fast` | `vocabulary::no_workspace_member_names_the_users_domain`, naming the file | yes (`cargo build -p goad-check` ok) | **red as predicted**, exit 101: that case alone failed (45 passed, 1 failed), *"…/crates/goad-check/src/main.rs:9: forbidden token `habit`"*. Restored by copy; `git status --short` clean |
| R-2 | `crates/goad-check/src/main.rs`, production code | `let _planted = std::env::args_os().next().unwrap();` before `main`'s `ExitCode::SUCCESS` (run at the scaffold, f4e9ccd, before `main` held an `Option` of its own; the lint is per-site, so the site does not matter to reach) | `cargo clippy --workspace --all-targets -- -D warnings` | `clippy::unwrap_used` at that line | yes | **red as predicted**, exit 101: *"used `unwrap()` on an `Option` value --> crates/goad-check/src/main.rs:9:18"*, in both the `bin` and `bin … test` targets. Restored; clean |
| R-3 | `crates/goad-check/tests/binary/statuses.rs` | appended `fn _planted() { let _planted: std::collections::HashMap<u8, u8> = std::collections::HashMap::new(); }` (run at the scaffold; VA-3; an `.unwrap()` here is exempt, A-V7) | as R-2 | `clippy::disallowed_types` in the `binary` target | yes (`cargo test -p goad-check --no-run` ok) | **red as predicted**, exit 101: *"use of a disallowed type `std::collections::HashMap` --> crates/goad-check/tests/binary/statuses.rs:4:17"* (and `:4:53`). Restored; clean |
| R-4 | `src/main.rs` and `tests/binary/statuses.rs` | `main`'s `ExitCode::SUCCESS` re-indented to four spaces; `fn _planted() {\n    let _planted = 1;\n}` appended to `statuses.rs` | `cargo fmt --all --check` | a diff naming each file | yes | **red as predicted**, exit 1: *"Diff in …/crates/goad-check/src/main.rs:6"* and *"Diff in …/crates/goad-check/tests/binary/statuses.rs:1"*. Restored; clean |
| R-5 | `crates/goad-check/src/main.rs` | `use slint as _;` after `use std::process::ExitCode;` | `cargo build -p goad-check` | `error[E0432]`/`E0433`: the crate edge holds I-6 at the source. Here the compile failure is the instrument's red | n/a (the red is the compile) | **red as predicted**, exit 101: *"error[E0432]: unresolved import `slint`"*. Restored; clean |

*Behaviour rows* (no M-row is a plan criterion; each is offered because no
VA item otherwise shows a new case can fail):

| row | file | edit | must red | compiled? | result |
|---|---|---|---|---|---|
| M-1 | `args.rs`, `parse` | `(config, None) => match timeout {` → `(config, None) => match timeout.filter(\|_\| config.is_none()) {` (was: the `--timeout`-with-`--config` refusal returns the config-form `Invocation` instead) | VT-1's `--timeout`-with-`--config` row | yes | **red as predicted**: `args::tests::a_timeout_with_config_is_refused` only. Restored by copy; `git status --short` clean |
| M-2 | `args.rs`, `parse` | `config::positive_duration(TIMEOUT, text).map_err(UsageError::Timeout)` → `Ok(config::positive_duration(TIMEOUT, text).unwrap_or(DEFAULT_TIMEOUT))` (was: `positive_duration(…)` on the `--timeout` value replaced by `positive_duration(…).unwrap_or(Duration::from_secs(5))`) | VT-1's `0s` and `-1s` rows | yes | **red, beyond the prediction**: `a_zero_timeout_is_refused`, `a_negative_timeout_is_refused`, and `a_timeout_that_is_not_a_span_is_refused` (a row the sheet did not have; Findings). Restored by copy; `git status --short` clean |
| M-3 | `args.rs`, `parse` | in `program`, `Command::from_argv(argv).ok_or(UsageError::EmptyCommand)` → `{ let mut argv = argv.into_iter(); Ok(Command::new(argv.next().unwrap_or_default(), argv.collect())) }` (was: `Command::from_argv(argv)` replaced by `Some(Command::new(<first or "">, <rest>))`) | VT-1's empty-argv and empty-program rows; `an_empty_argv_is_a_usage_error` on its stdout-empty assertion (the run reaches the interim end, which is also 2 with a `goad-check: ` last line, but writes the no-view line) | yes | **red as predicted**: `an_empty_command_is_refused`, `an_empty_program_is_refused`, and `statuses::an_empty_argv_is_a_usage_error` at `assert_no_report` (stdout not empty). Restored by copy; `git status --short` clean |
| M-4 | `args.rs`, `parse` | before `Ok(Invocation::Check(Request { source, events }))`, `events.reverse();` (was: the event paths reversed (`.rev()`) before `Invocation` is built) | VT-1's `--event` order row | yes | **red as predicted**: `events_are_kept_in_the_order_given` only. Restored by copy; `git status --short` clean |
| M-5 | `main.rs`, event step | in `normalized`, `paths.iter().map(\|path\| event(path)).collect()` → `Ok(paths.iter().filter_map(\|path\| event(path).ok()).collect())` (was: a normalization failure is skipped (`filter_map(Result::ok)`-style) rather than ending the run) | `a_reserved_source_event_file_exits_2` on its stdout-empty assertion (as M-3) | yes | **red as predicted**: `statuses::a_reserved_source_event_file_exits_2` only, at `assert_no_report`. Restored by copy; `git status --short` clean |
| M-6 | `main.rs`, the interim end | in `unjudged_end`, `Ok(()) => not_judged(render::NOT_YET_IMPLEMENTED),` → `Ok(()) => ExitCode::SUCCESS,` (was: returns `ExitCode::SUCCESS` instead of going through the one 2 site) | VT-3 on status | yes | **red as predicted**: `a_run_with_no_exchange_exits_2_with_no_verdict` only, on status (`assert_not_judged`). Restored by copy; `git status --short` clean |
| M-7 | `render.rs`, the not-yet-implemented line | `NOT_YET_IMPLEMENTED`'s `"goad-check: the exchanges are …` → `"the exchanges are …` (was: its `goad-check: ` prefix dropped) | VT-3 on the last-line assertion | yes | **red, beyond the prediction**: VT-3 on the last-line assertion, and `render::tests::the_interim_end_says_nothing_was_judged_on_stderr`. Restored by copy; `git status --short` clean |
| M-8 | `render.rs`, the usage-error line | `usage_error_line`'s `EmptyCommand` arm, `"goad-check: the command after -- …` → `"the command after -- …` (was: prefix dropped) | `an_empty_argv_is_a_usage_error` | yes | **red, beyond the prediction**: `an_empty_argv_is_a_usage_error` on the last line, and `render::tests::every_usage_error_says_what_was_wrong_and_reprints_nothing`. Restored by copy; `git status --short` clean |
| M-9 | `render.rs`, the config line | `startup_error_line`'s `ConfigUnreadable` arm, `format!("goad-check: {} could not be read: {fault}", …)` → `format!("{} could not be read: {fault}", …)` (was: prefix dropped) | `an_unreadable_config_exits_2_and_says_who_spoke` | yes | **red, beyond the prediction**: `an_unreadable_config_exits_2_and_says_who_spoke` on the last line, and `render::tests::every_startup_fault_says_what_failed_and_names_its_file`. Restored by copy; `git status --short` clean |
| M-10 | `render.rs`, the event-file line | `startup_error_line`'s `EventRefused` arm, `format!("goad-check: event file {}: {fault}", …)` → `format!("event file {}: {fault}", …)` (was: prefix dropped) | `a_reserved_source_event_file_exits_2` | yes | **red, beyond the prediction**: `a_reserved_source_event_file_exits_2` on the last line, and `every_startup_fault_says_what_failed_and_names_its_file`. Restored by copy; `git status --short` clean |
| M-11 | `main.rs`, the `Help` arm | `Ok(Invocation::Help) => answer(render::USAGE),` → `Ok(Invocation::Help) => { line_to(std::io::stdout().lock(), render::USAGE); ExitCode::SUCCESS }` (was: `report::line_to` (best effort) in place of `try_line_to`, returning `ExitCode::SUCCESS`) | `a_report_that_cannot_be_written_exits_2` (`--help`) | yes | **red as predicted**: `a_report_that_cannot_be_written_exits_2` only, on status. Restored by copy; `git status --short` clean |
| M-12 | `main.rs`, the interim end | in `unjudged_end`, `match try_line_to(std::io::stdout().lock(), render::NO_VIEW) {` → `match Ok::<(), std::io::Error>(()) {` (was: the no-view line not written) | VT-3 on stdout | yes | **red as predicted**: `a_run_with_no_exchange_exits_2_with_no_verdict` only, on its stdout `assert_eq!`. Restored by copy; `git status --short` clean |
| M-13 | `main.rs`, the `Version` arm | `Ok(Invocation::Version) => answer(&version_line(…)),` → `Ok(Invocation::Version) => { let _version = version_line(…); answer(render::USAGE) }` (the call kept, so the import compiles) (was: writes `render::USAGE` in place of `version_line(…)`) | `version_prints_the_package_version_on_stdout_and_exits_0` | yes | **red as predicted**: `version_prints_the_package_version_on_stdout_and_exits_0` only. Restored by copy; `git status --short` clean |
| M-14 | `args.rs`, `parse` | `(config, None) => match timeout {` → `(config, None) => match timeout.filter(\|_\| config.is_some()) {` (was: the `--timeout`-with-neither-`--config`-nor-`--` refusal returns the default-path config-form `Invocation` instead) | VT-1's `--timeout` with neither `--config` nor `--` row | yes | **red as predicted**: `a_timeout_with_neither_config_nor_a_command_is_refused` only. Restored by copy; `git status --short` clean |
| M-15 | `args.rs`, `parse` | `(Some(_), Some(_)) => return Err(UsageError::ConfigWithCommand),` → `(Some(_), Some(command)) => Source::Argv { command: program(command)?, timeout: timeout.unwrap_or(DEFAULT_TIMEOUT) },` (was: the `--config`-with-`--` refusal returns the argv-form `Invocation` instead) | VT-1's `--config` with `--` row | yes | **red as predicted**: `config_with_a_command_is_refused` only. Restored by copy; `git status --short` clean |
| M-16 | `main.rs`, the `Help` arm | `Ok(Invocation::Help) => answer(render::USAGE),` → `Ok(Invocation::Help) => { let _answered = answer(render::USAGE); not_judged(render::NOT_YET_IMPLEMENTED) }` (was: after the usage block is written, returns through the one 2 site) | `help_prints_the_usage_block_on_stdout_and_exits_0` on status | yes | **red as predicted**: `help_prints_the_usage_block_on_stdout_and_exits_0` only, on status. Restored by copy; `git status --short` clean |

**Decisions taken during execution**
<!-- Small and local: how, within what the design already settled. A choice that
     changes the design is not one of these — stop, consult the user, and record
     it in `design-log.md`. -->

- **D-1 — `jiff` is a direct dependency, with no feature (A-V2).**
  `main`'s `DEFAULT_POLL` is `jiff::SignedDuration::from_mins(30)`, as
  `tests/support/driving.rs`' `DEFAULT_POLL` spells it. The alternative,
  `schedule::parse_span("30m")`, needs an `Err` arm that cannot arise, and
  therefore a path to 2 no test can reach. `jiff` is a workspace dependency
  already in the lockfile, so EX-5 admits it; no feature is added (A-V6).
- **D-2 — the two `--timeout` exclusions are one rule and one variant.**
  `UsageError::TimeoutWithoutCommand` covers `--timeout` with `--config`
  and `--timeout` with neither. The design gives both one reason (*"a file
  supplies the timeout, whether named or defaulted"*). Each still has its
  own VT-1 row, and M-1 and M-14 narrow the rule to one half each, each
  redding only its own row.
- **D-3 — `StartupFault` lives in `main.rs`**, as `goad-emit`'s does: it
  names what `main` reads. A refused report line has its own line,
  `render::report_unwritten_line`, distinct from `answer_unwritten_line`,
  because the report is not an answer to a question.
- **D-4 — the interim end builds no type for PHASE-12.** `prepare` returns
  `Result<(), StartupFault>`, binding `_events`, `_runtime` and `_host`.
  A struct carrying them would have fields nothing reads, which is
  `dead_code` (A-T1). PHASE-12 changes `prepare`'s return.
- **D-5 — VT-1 has five rows beyond the sheet's list**:
  `a_timeout_that_is_not_a_span_is_refused`,
  `a_help_token_in_value_position_is_a_value`,
  `a_bare_argument_before_the_separator_is_refused`,
  `an_empty_config_or_event_path_is_refused`,
  `a_command_token_that_is_not_utf8_is_refused`. Each holds a `parse` arm
  that otherwise had no case.
- **D-6 — the binary tier's file is `statuses.rs`**, with its fixtures
  `loadable.toml` and `reserved-source.json` beside it in `tests/binary/`.
  `absent.toml` is a path no file is at.
- **D-7 — `render`'s test names `EnvelopeFault::ReservedSource` directly**
  rather than normalizing a JSON fixture spelling `"source":"host"`, which
  was VA-1's one hit (VA-1 record).
- **D-8 — R-2 and R-3 were run at the scaffold**, before `main` held an
  `Option` of its own; the R-2 edit makes one
  (`std::env::args_os().next().unwrap()`). `unwrap_used` and
  `disallowed_types` fire per site, so the site does not bear on reach.

**Findings**
<!-- Things noticed in passing that are not this phase's job: a defect
     elsewhere, drift from the design, a surprise. Defects in this phase's own
     work get fixed, not recorded. These feed the audit; the ones that outlive
     the slice become Follow-ups. -->

- **Executor — five M-rows redded more than predicted, none less.** M-2 also
  redded `a_timeout_that_is_not_a_span_is_refused` (D-5's row). M-7..M-10
  each also redded a `render` unit case, which holds the same prefix one
  tier down. No row redded a case unrelated to its edit, and no predicted
  case stayed green.
- **Executor — I-1's command reads unit tests in `src`, and a fixture there
  is a hit.** VA-1's first run found `"host"` inside a JSON byte string in
  `render`'s test (D-7). The command cannot tell a source value from a side
  literal. PHASE-12's tests in `src` will build `Outcome`s and envelopes:
  its sheet should say a fixture names a variant or `HOST_SOURCE`, never
  the spelling. `tests/binary/reserved-source.json` spells it, outside the
  command's reach, and I-1 is about the checker's code, so that is not a
  breach.
- **Executor — `cargo test --workspace` at this phase reports 625 passed
  with the scaffold, while the gate's sum was 678.** Not a discrepancy: the
  gate also runs `cargo test -p goad-semantics`, whose cases are counted a
  second time. Recorded so the next reader does not chase it.

- **PLAN QUESTION 1 — resolved (a)**, `plan-log.md` 2026-10-01, *PHASE-04
  sheet questions*, Q1; VT-2 amended. **VT-2's cases cannot tell their cause
  from the interim end.** Each case asserts *"status 2 and the `goad-check: `
  prefix on the **last** stderr line"*. Until PHASE-12, a run that gets past
  every pre-exchange step also ends with status 2 and a `goad-check: ` last
  line (EX-3). So a regression that lets a bad event file or an empty argv
  *through* leaves `a_reserved_source_event_file_exits_2` and
  `an_empty_argv_is_a_usage_error` green (mutation rows M-3, M-5). This is
  `docs/memory/tests-asserting-proxies.md`'s shape. Checked: canon-delta's
  R-14 row has these cases hold the prefix only, so the plan inherited it.
  Options: (a) each VT-2 case, except the `/dev/full` one, also asserts stdout
  is empty. A failure before the first exchange writes no report line, and
  that stays true at PHASE-12. (b) Each asserts the last stderr line is not
  the not-yet-implemented line. That is weaker, and false once PHASE-12
  deletes the line. (c) Accept it: PHASE-12 removes the interim end, and the
  cases become witnesses then. **Recommendation: (a).** It is one assertion
  per case and holds in both phases. Blocked task 4's `[!]` box.
- **PLAN QUESTION 2 — resolved (a), `HashMap`**, `plan-log.md` 2026-10-01,
  *PHASE-04 sheet questions*, Q2; VA-3 amended. **VA-3's `tests` half cannot
  red.** `clippy.toml` sets `allow-unwrap-in-tests = true`. The binary tier is
  a `#[cfg(test)]` module (it must be, for `tests_outside_test_module`), so a
  planted `.unwrap()` there is exempt. Measured on `goad-emit`'s
  `exchange::code_of`: clippy exit 0 with `.unwrap()`; exit 101 with a
  `HashMap` (A-V7). Options: (a) plant a lint that is never test-exempt: a
  `std::collections::HashMap` (`disallowed_types`, measured) or `dbg!`
  (`docs/memory/clippy-toml-test-exemptions-are-a-hidden-boundary.md`); (b)
  keep `.unwrap()` and record that it does not red, leaving clippy's reach
  into `tests` unproven. **Recommendation: (a), with `HashMap`**: it is
  measured, and the `src` half keeps `.unwrap()`. Blocked row R-3.
- **PLAN QUESTION 3 — resolved as recommended**, `plan-log.md` 2026-10-01,
  *PHASE-04 sheet questions*, Q3; VA-6 amended. PHASE-12/VA-3 was not amended
  (outside PHASE-04). **VA-6's command fails as written.** `grep -n 'ExitCode'
  crates/goad-check/src` without `-r` exits 2 with *"Is a directory"* under
  the system `grep` (measured on `goad-emit/src`). In the agent's shell `grep`
  is a `ugrep` wrapper that recurses unasked and honours ignore files, so an
  agent would see it work and a person would not (A-V10). **Recommendation:**
  `grep -rn`, and every instrument command in the plan recorded as run by
  `command grep`. The same applies to PHASE-12/VA-3's `grep -rnE`, which is
  already recursive but would differ under `--ignore-files` only if an ignored
  file sat in `src`. Blocked VA-6's `[!]`.
- **PLAN QUESTION 4 — resolved as recommended**, `plan-log.md` 2026-10-01,
  *PHASE-04 sheet questions*, Q4; VT-2 and `design.md` §9 amended. **Nothing
  in the binary tier holds a written answer exiting 0.** R-11 makes a question
  answered status 0. VT-1 holds parsing only.
  `a_report_that_cannot_be_written_exits_2` holds 2 when the answer is
  refused, never 0 when it is written. So a `main` that answered `--help` and
  exited 2 would pass PHASE-04. EX-8's `version_line` call also has no case
  (row M-13). `goad` and `goad-emit` each have
  `help_prints_the_usage_block_on_stdout_and_exits_0` and
  `version_prints_the_package_version_on_stdout_and_exits_0`.
  **Recommendation:** add both to PHASE-04/VT-2 under those names. Whether
  canon-delta's R-11 row cites them is audit's call (VA-4 is *"test paths
  only"*). Blocked task 4's `[!]` box.
- **PLAN QUESTION 5 — resolved as recommended**, `design-log.md` 2026-10-01,
  *`goad-check`'s flag exclusions, stated whole*; `design.md` §5.2.1, EX-1 and
  VT-1 amended. **The forms' exclusion is half-stated.** EX-1 and VT-1 refuse
  `--timeout` with `--config`. Neither says what happens with `--timeout` and
  no `--` (the default-path config form), or with `--config` and `--`
  together. §5.2.1's synopsis admits neither, and its reason (*"the file
  already states the timeout"*) covers the default-path file too.
  **Recommendation:** both are usage errors, and each gets a VT-1 row. Blocked
  task 2's `[!]` box.
- **PLAN QUESTION 6 — resolved as recommended**, `plan-log.md` 2026-10-01,
  *PHASE-04 sheet questions*, Q6; VA-4 amended. **VA-4's re-pointing has no
  single target.** canon-delta SPEC-004 Change 5's R-11..R-13 row gives one
  prefix, `crates/goad-check/tests/binary/…`, for cases of both phases.
  PHASE-04 ships two of its names whole and one in half
  (`a_report_that_cannot_be_written_exits_2`). The R-14 row names
  `goad-check`'s cases without a path, as it does `goad-emit`'s.
  **Recommendation:** in the R-11..R-13 row, give each case this phase ships
  its own path (`crates/goad-check/tests/binary/<file>.rs::name`), leave `…::`
  on PHASE-12's, and leave the R-14 row alone. Blocked VA-4.
- **The `VT-3` id was reused.** F-13's repair removed PHASE-04/VT-3, then
  F-23's repair added a new PHASE-04/VT-3. `plan.md`'s header says ids are
  immutable and edits append. `review-plan.md` entries before F-23 that
  cite *"PHASE-04/VT-3"* mean the old case (now PHASE-12/VT-3). No action
  for this phase. For audit.
- **PHASE-12 may not add a regular dependency.** Its surfaces admit
  `crates/goad-check/Cargo.toml` *"`[dev-dependencies]` only"*. If the run
  needs a direct `[dependencies]` entry PHASE-04 did not use, PHASE-12
  cannot add it. One example would be `serde_json` to name
  `serde_json::Value`; `Submitted::to_json` returns one, though printing it
  needs no direct dependency. PHASE-04 should not add speculative
  dependencies to avoid that. For PHASE-12's sheet.
- **The gate cannot see a manifest crane refuses.** The root `Cargo.toml`'s
  `tokio` comment says crane parses every member's manifest with a TOML 1.0
  parser, and *"`just check` cannot see a re-split"*. A new member's
  manifest is a new instance of the same risk, and `nix build` is
  PHASE-05's. Offered as an unchecked task box above, not as a criterion.
- **"Both binaries" goes stale by widening.** `flake.nix`'s
  `cargoArtifacts` comment and the `justfile`'s `package` and `install`
  comments say *"both binaries"*. PHASE-05 owns those files (PL-6). For
  PHASE-05's sheet.

### PHASE-12 — `goad-check`: the run

**Written by a phase-sheet agent, not the executor**, at 3299acc (*012
PHASE-04: the gate's rerun, recorded in the sheet*). This sheet is the plan's
second reading. Where it restates a plan criterion it quotes it. It repairs
nothing: what reads as wrong in the plan is under **Findings** as a PLAN
QUESTION, and the tasks it blocks are marked `[!]`.

**Every PLAN QUESTION is resolved** (`design-log.md` 2026-10-01, *`Options::first`; R-56's condition counts `evaluate` outcomes*, for 1 and 4; `plan-log.md` 2026-10-01, *PHASE-12 sheet questions*, for 2, 3 and 5),
and the decisions are applied to `design.md` and `plan.md` in *012 PHASE-12:
decisions applied to design, plan and sheet*. Every criterion they changed is
re-quoted below from the amended plan, and no task is `[!]`.

**Objective** (quoted, `plan.md` PHASE-12): *`goad-check` sends the request
plan, answers every view and follows chains, reports every channel of every
`Outcome` with the side and requirement its kind answers, and charges R-56
only on its condition (`design.md` §5.2.2, §5.4).*

PHASE-12 replaces PHASE-04's interim end (PHASE-04/EX-3) and deletes
PHASE-04/VT-3's case `a_run_with_no_exchange_exits_2_with_no_verdict`
(PHASE-12/EX-6). Every other PHASE-04 case stays and must stay green.

**Entry**
- **EN-1** (quoted): *"PHASE-04 done."* **Discharged 2026-10-01 at 3299acc.**
  §Status has PHASE-04 `done`. `just check` exited **0**: build, both test
  tiers (**715 passed, 0 failed**, summed over all 33 `test result` lines),
  `deno check exercisers/typescript/backend.ts`, clippy with no warning,
  `cargo fmt --all --check`. `just -n check` differs from POL-001
  §Compliance's command block in one line only (`diff` of the block against
  the recipe's output):
  ```
  4c4
  < deno check examples/typescript/backend.ts
  ---
  > deno check exercisers/typescript/backend.ts
  ```
  This is PHASE-03/EX-3's departure, which audit ends (POL-001 Change 1).

**Surfaces — a closed list, copied from `plan.md`. Anything else is a STOP.**
- (quoted) *"`crates/goad-check/src/`"*.
- (quoted) *"`crates/goad-check/tests/binary/` and its bash fixtures"*. A
  committed event file, config or bash backend a binary case reads lives
  here. `tests/binary/loadable.toml`'s comment is in this surface.
- (quoted) *"`crates/goad-check/Cargo.toml` (`[dev-dependencies]` only,
  should reading a test backend's request log need one, and with it the
  comment's "No dev-dependencies" sentence; a new external dependency is
  still a STOP; `plan-log.md` 2026-10-01, *PHASE-12 sheet questions*, Q5)"*.
  The sentence is *"No dev-dependencies: the binary tier spawns the built
  binary and needs nothing the plain dependencies do not give it."*; it
  changes only in the commit that adds the entry.
- (quoted) *"`crates/goad-semantics/src/protocol/canonical.rs`
  (`Options::first` and its unit test only; `design.md` §5.2.4;
  `design-log.md` 2026-10-01, *`Options::first`; R-56's condition counts
  `evaluate` outcomes*)"*. An existing stratum-1 file: the gate's `cargo test
  -p goad-semantics`, the stratum-1 purity scan (`goad-boundary`'s
  `purity.rs`, rooted at `crates/goad-semantics/src`), the vocabulary scan,
  clippy and fmt already reach it, so no reach proof is owed for it (A-V12).
- (quoted) *"`canon-delta.md` (test paths only)"*.
- (quoted) *"`tests/support/` is read and may be included, not edited."*
- `docs/slices/012/notes.md`: this sheet, §Status, §Harvest (§Open's FU-5
  row, for VA-7). Bookkeeping (`docs/AGENTS.md` §Execute).

Not surfaces, and so a STOP if the work seems to need them: any
`[dependencies]` entry in `crates/goad-check/Cargo.toml` (PHASE-04
Findings, *PHASE-12 may not add a regular dependency*); any other
`Cargo.toml`, `[workspace.dependencies]` included; anything in
`crates/goad-semantics/` but `Options::first` and its unit test in
`canonical.rs`; `crates/goad-shell/`;
`crates/goad-emit/`; `tests/support/` and **`tests/backends/`** (a new
backend script goes under `crates/goad-check/tests/binary/`); any canon
document; `flake.nix`, the `justfile`, `README.md`; `design.md` and
`plan.md`.

**Reading list** (by symbol; `command grep -n` then `sed -n`, not whole files)
- `docs/slices/012/plan.md`: §Overview's first paragraph; *Owed to audit and
  close*; *Test names are commitments*; §Sequencing & rationale, *Why the
  checker is two phases*, *Mutation evidence* and *Invariant reads*;
  §Coverage (AC-3, AC-4 rows; the §9 test table's `12` row); §PHASE-12
  whole; §PHASE-04's EX-3, EX-4, VT-2, VT-3, VA-6 and VA-7, which this phase
  replaces, extends or reruns.
- `docs/slices/012/design.md`: §5.2.1 (the argv form; `--event`); §5.2.2
  whole (the sequence diagram, *The request plan*, *Answering*, *What is
  judged* and its table, the R-56 row's `evaluate`-only condition); §5.2.3
  (the `Requirement` constants and `R56`; *What holds the list*); §5.2.4's
  `Options::first` bullet, and the `Alternatives::first` it mirrors; §5.2.5
  whole (the report example, the no-view
  line, the status table and its bullets); §5.3; §5.4; §5.5 I-1, I-2 and
  *Edges*; §9 *`goad-check`*.
- `plan-log.md` 2026-10-01, *PHASE-12: R-56's scope gets cases;
  `UnixDatagram` in VA-3*.
- `docs/slices/012/design-log.md` 2026-10-01: *`Requirement` is built from
  named constants*; *`Options::first`; R-56's condition counts `evaluate`
  outcomes*. `plan-log.md` 2026-10-01: *plan review round 2: dispositions*
  (F-23, F-27); *plan review round 3: close* (F-29); *PHASE-04 sheet
  questions*; *PHASE-12 sheet questions*.
- `docs/slices/012/canon-delta.md`: SPEC-001 Change 3 (R-56's row, its
  `goad-check` paths); SPEC-004 Change 3 (R-11..R-13, R-15), Change 4
  (`goad-check`'s table), Change 5 (the R-14 row and the R-11..R-13 row).
- Canon: `docs/specs/001-host-backend-protocol.md` R-32, R-56..R-59 and §7's
  R-56 row; `docs/specs/004-process-exit-status.md` §3 (P-B, P-C), §5;
  `docs/policy/001-the-phase-gate.md` §Compliance, §Verification;
  `docs/adr/001-one-way-strata.md`;
  `docs/adr/003-the-host-splits-into-a-workspace-of-strata.md`.
- This sheet's predecessor, `### PHASE-04` above: its Decisions D-1..D-8,
  its Findings, its mutation tables (the format to follow).
- The shipped crate: `crates/goad-check/src/main.rs` (`main`, `prepare`,
  `configuration`, `normalized`, `event`, `unjudged_end`, `answer`,
  `not_judged`, `StartupFault`, `DEFAULT_POLL`); `src/render.rs` (`NO_VIEW`,
  `NOT_YET_IMPLEMENTED`, `report_unwritten_line`, its `mod tests`);
  `src/args.rs` (`Invocation`, `Request`, `Source`);
  `tests/binary/main.rs`; `tests/binary/statuses.rs` (`fixture`, `check`,
  `check_with_stdout_full`, `code_of`, `stderr_of`, `stdout_of`,
  `assert_not_judged`, `assert_no_report`, every case);
  `tests/binary/loadable.toml`; `Cargo.toml` and its comment.
- Stratum 2: `goad_shell::host::{Host, Outcome, Presented, Failure}`
  (`Host::evaluate`, `Host::respond`, the `Outcome` fields' docs);
  `goad_shell::backend::transport::Captured`; `goad_shell::error::{BackendError,
  CleanupFailure, StateError}` and each one's `requirement`, `fault`,
  `Display`; `goad_shell::clock::wall_clock`; `goad_shell::report::try_line_to`.
- Stratum 1: `goad_semantics::error::{Requirement, AtFault}` (`R56`, both
  `Display`s); `goad_semantics::protocol::canonical::{Stimulus, HOST_SOURCE,
  Event, UserResponse, View, Choice, Opt, Options, Fields, Field, Submitted,
  Alternatives}` (`Stimulus::event`, `Submitted::as_drawn`,
  `Submitted::to_json`, `Options::new`, `Options::as_slice`,
  `Alternatives::first`, its doc and its `#[expect]`, which `Options::first`
  mirrors); `canonical.rs`' `mod tests` (`an_empty_options_is_rejected_and_names_where`
  and the `Options::new` cases beside it, where `Options::first`'s test goes;
  `an_as_drawn_choice_submits_the_first_alternative`, the only test that
  reaches `Alternatives::first`);
  `goad_semantics::protocol::normalize::Discarded` (its `Display`).
- Test support: `tests/support/scripting.rs` (`backend`, `claim`, `marker`,
  `clear`, `logging_backend`, `invocations`, `scripted`), `driving.rs`
  (`answer_first_option`: the shape to mirror, not to include), `waiting.rs`;
  `tests/backends/answers-as-instructed.sh` (its sentinels, `@exit1`,
  `@garbage`, `@lingers`; its default past the list) and
  `tests/backends/logs-the-request-then-answers.sh`;
  `tests/backends/leaves-a-grandchild-holding-stderr.sh`'s header.
- Prior art: `crates/goad-shell/tests/integration/main.rs` (how a target
  `#[path]`-includes `scripting.rs`, with `#[cfg(test)]` and why).
- `docs/memory/`: `tests-asserting-proxies` (→ `a-green-test-can-assert-a-proxy`),
  `a-mutation-can-red-the-right-case-on-the-wrong-assertion`,
  `negative-control-must-compile`, `verify-the-proposed-instrument`,
  `a-check-that-compares-two-derived-things-holds-nothing`,
  `a-substring-scan-and-a-word-boundary-scan-are-different-instruments`,
  `shared-test-helper-lives-at-workspace-root-via-path`,
  `cargo-test-cwd-is-package-root-not-workspace-root`,
  `autotests-false-hides-an-undeclared-test-target`,
  `clippy-toml-test-exemptions-are-a-hidden-boundary`,
  `timed-test-margins-are-measured-at-the-bound`,
  `tokio-net-arrives-by-feature-unification`,
  `margin-size-is-not-margin-direction`,
  `wildcard-enum-match-arm-counts-a-named-binding`. The user's memory adds
  *a standing guard may not reach a new file* and *mutation-check the
  coverage claim*: both bind this phase's reach and mutation rows.
- §Harvest *Learned* and *Open*: I-1's grep reads unit-test fixtures in
  `src`; a struct carrying values to a later phase is dead code; run the
  exit grep before naming new tests; PHASE-04's three notes for this sheet.

**Assumptions — verified now** (at 3299acc, by reading, grep, and the runs
named)
- **A-V1 — the exchange API is what the plan names.** Read:
  `Host::evaluate(&mut self, now: Timestamp, event: Event) -> Outcome` and
  `Host::respond(&mut self, now: Timestamp, view_id: ViewId, answer:
  UserResponse) -> Outcome`, both `async`. `Outcome` has `pub` fields `view:
  Option<Presented>`, `next_check: Timestamp`, `discarded: Vec<Discarded>`,
  `stderr: Captured`, `failure: Option<Failure>`, `cleanup:
  Option<CleanupFailure>`. `Presented { view_id, view }`, both `pub`.
  `Failure::{Backend(BackendError), State(StateError)}`, with a `Display`.
  `Captured { bytes: Vec<u8>, truncated: bool }`, `pub`, in
  `goad_shell::backend::transport` (a `pub mod`). `respond` checks the
  `view_id` against host state before the transport (`State::verify`) and
  returns `Failure::State` without spawning: the R-32 check EX-1 relies on.
- **A-V2 — every channel's kind answers `requirement()` and `fault()`.**
  Read: `BackendError`, `CleanupFailure`, `StateError` (stratum 2) and
  `ProtocolError`, `BoundsError`, `ScheduleError` (stratum 1) each have both,
  total. `BackendError::Protocol(inner)` delegates to `inner`. `Discarded`
  has one variant, `Schedule { raw, reason: ScheduleError }`, and a
  `Display`; matching `Discarded::Schedule { reason, .. }` does not name
  `serde_json`. `Requirement` displays `R-N`; `AtFault` displays `backend`,
  `host`, `configuration`, `environment` through a total `match`.
- **A-V3 — the side each planned case provokes.** Read from the `fault()`
  arms: `BackendError::Spawn` is `Configuration`, R-44
  (`an_unspawnable_command_is_reported_against_the_configuration`);
  `ExitStatus` is `Backend`, R-40 (`@exit1`); `Protocol(Json)` is `Backend`,
  R-44 (`@garbage`); `ProtocolError::EmptyOptions` is `Backend`, R-13 (a
  view with `"options":[]`); every `ScheduleError` is `Backend`, R-21..R-25
  (a discarded `next_check`); `CleanupFailure::TimedOut` is `Backend`, R-48.
  **Every `ProtocolError`, `BoundsError` and `ScheduleError` arm is
  `Backend`.** The only non-backend `failure` sides are `Spawn`
  (configuration), `PipeMissing` (host) and `Io` (environment). This bears on
  the R-56 condition's side clause (Findings, *the side clause*).
- **A-V4 — `Requirement::R56` exists in stratum 1.** `pub const R56:
  Requirement = Requirement(56);` in `goad_semantics::error`. Its value is held
  by nothing yet: §5.2.3 gives that to
  `a_backend_that_fails_on_an_unrecognised_host_kind_is_reported_against_r56`,
  which must assert the literal `SPEC-001/R-56`.
- **A-V5 — the probe's `Event` can be built with no `serde_json` in
  `[dependencies]`.** `Event` has four `pub` fields; `data` is a
  `serde_json::Value`. `serde_json::Value: Default` (`Null`; read in the
  registry source, `serde_json` 1.0.150 and 1.0.151), so `Event { source:
  HOST_SOURCE.to_owned(), kind: <probe>.to_owned(), timestamp: now, data:
  Default::default() }` names no `serde_json`, and `data` is `null` as
  `Stimulus::event` builds it (§5.2.2). The values map
  (`UserResponse.values: BTreeMap<FieldId, serde_json::Value>`) is built by
  `.collect()` from `(field.id().clone(),
  Submitted::as_drawn(field.kind()).to_json())`, and a value is printed
  through `serde_json::Value`'s `Display`: neither names the crate. So PHASE-04's
  `[dependencies]` (`goad-semantics`, `goad-shell`, `jiff`, `tokio`) suffice.
- **A-V6 — `Options` has no total `first()`.** `Options::as_slice() ->
  &[Opt]`; `.first()` on it is an `Option`. `Alternatives::first` exists,
  with an `#[expect(clippy::expect_used)]` and a doc arguing that every
  consumer re-deriving non-emptiness at its call site is what it exists to
  stop. `unwrap_used`, `expect_used`, `panic`, `unreachable` and
  `indexing_slicing` are `deny` in `[workspace.lints.clippy]`, exempt only in
  tests (`clippy.toml`). PLAN QUESTION 1, resolved: `Options::first` in
  stratum 1 (`design-log.md` 2026-10-01, *`Options::first`; R-56's condition counts `evaluate` outcomes*).
- **A-V7 — `Timestamp` has no `Display`.** `Timestamp::instant()` gives a
  `jiff::Timestamp`, which does. The workspace `jiff` has
  `default-features = false`, so a report that shows a `next_check` in the
  system zone would need a feature: adding one to `jiff` is the residue
  (PHASE-04 A-V6) and a STOP. The report's format is free (§5.2.5), so the
  instant in UTC is enough.
- **A-V8 — test support at this phase.** `scripting.rs`' `scripted(case,
  instructions)` returns a `goad_shell::config::Command` running
  `tests/backends/answers-as-instructed.sh` with an invocation log at argv[2]
  and one instruction per invocation; past the list it answers
  `{"view":null,"next_check":"45 minutes"}`, which a conforming backend
  needs. `logging_backend("logs-the-request-then-answers", case)` logs each
  **raw request**, one per line, so a case reads `event.kind` and `"type"`
  off it; past its list it answers `{"view":null}`. `invocations(log)` counts
  lines. `Command`'s `program` and `arguments` are `pub`, so a case passes
  them after `--`. `backend` roots at `CARGO_MANIFEST_DIR/../../tests/backends`,
  which resolves from `crates/goad-check` as from any member. Using `scripted`
  and `invocations` reaches every item of `scripting.rs` (`scripted` →
  `logging_backend` → `marker` → `claim`, `clear`; `backend`), so the file
  can be included whole under `dead_code` (VA-7). `driving.rs` composes a
  `Host` in-process and the binary tier never does: not included.
  `waiting.rs` polls; nothing here polls: not included. This is a
  prediction; VA-7 is the read.
- **A-V9 — a cleanup failure alone is provocable.** `@lingers` (the
  behaviour of `leaves-a-grandchild-holding-stderr.sh`, whose header says
  so) answers `{"view":null}` and leaves a grandchild holding stderr for 2 s
  against `CLEANUP_LIMIT`'s 500 ms: `result` `Ok`, `cleanup`
  `Some(TimedOut)`. Load lengthens the grandchild's hold, which is the
  direction that keeps the case red-able (`margin-size-is-not-margin-direction`).
  **A prediction, read from the header, not run under load** (`plan-log.md` 2026-10-01, *PHASE-12 sheet questions*, *Also*).
  The task before the case joins the gate measures it.
- **A-V10 — the instruments, dry-run at 3299acc under the system `grep`.**
  `command grep` throughout (PHASE-04 A-V10: the agent shell's `grep` is a
  `ugrep` wrapper that recurses unasked and honours ignore files).
  - I-1: `command grep -rnE 'R-?[0-9]+|AtFault::|"(backend|host|configuration|environment)"'
    crates/goad-check/src | command grep -vE '^[^:]+:[0-9]+:[[:space:]]*//'`:
    no output, `PIPESTATUS` `0 1`.
  - I-2: `command grep -rn 'FieldKind' crates/goad-check/src
    crates/goad-check/tests`: no output, exit 1.
  - PHASE-12/VA-3, as the plan writes it but run as `command grep`:
    `command grep -rnE 'ingress::|set_var|remove_var|env_clear|env_remove|\.env\('
    crates/goad-check/src` prints two lines, exit 0:
    `src/render.rs` `use goad_shell::ingress::envelope::EnvelopeFault;` (the
    test module's import) and `src/main.rs` `use
    goad_shell::ingress::envelope::{self, EnvelopeFault};`. The call
    `envelope::normalize(&bytes)` does **not** match `ingress::`. PLAN
    QUESTION 2, resolved (`plan-log.md` 2026-10-01, *PHASE-12 sheet questions*, Q2).
  - PHASE-12/VA-3 as amended, at bc80a3d (`UnixDatagram` included):
    `command grep -rnE
    'ingress::|set_var|remove_var|env_clear|env_remove|\.envs?\(|UnixStream|UnixListener|UnixDatagram|TcpStream|TcpListener|UdpSocket'
    crates/goad-check/src` prints the same two `envelope` imports, and
    nothing else, exit 0.
  - PHASE-04/VA-6 (PHASE-12/VA-4): `command grep -rn 'ExitCode'
    crates/goad-check/src` prints the `use`, the signatures of `main`,
    `unjudged_end`, `answer`, `not_judged`, `ExitCode::SUCCESS` in `answer`
    and `ExitCode::from(2)` in `not_judged`.
- **A-V11 — what I-1 forbids in this phase's own unit tests.** The command
  reads all of `src`, `#[cfg(test)]` modules included (PHASE-04 D-7). So a
  `render` unit test may not write `AtFault::Backend`, `Requirement::R40`,
  `"R-40"`, `"SPEC-001/R-13"` or `"backend"` as a literal. A unit test that
  instead compares a line with `format!("{} SPEC-001/{}", e.fault(),
  e.requirement())` compares two derived things and holds nothing
  (`a-check-that-compares-two-derived-things-holds-nothing`). **The value of
  each side and id is asserted in the binary tier**, under `tests/`, outside
  I-1's reach, as literals. `render`'s units may hold shape only (prefix,
  that a refusal line carries the fault's `Display`). The only `AtFault::`
  and `Requirement::` spellings in `src` are VA-1's three.
- **A-V12 — guard reach for what this phase adds** (read from each
  instrument, as PHASE-04 A-V7):
  - *Vocabulary scan* (`goad-boundary`'s `domain_scan`): walks every `.rs`
    under `crates/goad-check` **excluding `tests/` and `target/`**. A new
    `src/*.rs` module is reached with no change; a report string is read
    (strings are scanned; `site`, `goal` are among `DOMAIN`'s words, so a
    phrase such as *"call site"* is a breach). Nothing under `tests/binary/`
    is reached, and no `.sh`, `.json` or `.toml` is read anywhere.
  - *I-1, I-2, VA-3*: `-r` over `src` (I-2 also `tests`), so a new module
    or test file is reached by the command with no change.
  - *Clippy* and *fmt*: reach a file only through a `mod` declaration. A new
    `tests/binary/*.rs` not named in `tests/binary/main.rs` is neither
    compiled, linted nor formatted, and its cases do not run. An included
    `tests/support/scripting.rs` is linted under this target too.
  - *Bash fixtures*: no instrument in the gate reads them. The case that
    runs a fixture is what holds it; a fixture no case runs is held by
    nothing (§Exit, *every fixture has a reader*).
  - *Crate edges / manifest allowlist / stratum-1 purity scan / `cargo test
    -p goad-semantics`*: unchanged from PHASE-04 A-V7; none bills a
    stratum-3 manifest or source.
  - *`canonical.rs`* (for `Options::first` and its test): an existing file.
    The purity scan walks `crates/goad-semantics/src` (`purity.rs`), `cargo
    test -p goad-semantics` already runs its `mod tests` (e.g.
    `an_as_drawn_choice_submits_the_first_alternative`), and the vocabulary
    scan, clippy and fmt read it through its `mod` declaration. Reach is
    already proven; the new test's name in the gate's `goad-semantics` test
    output is the record. A `[dev-dependencies]` `serde_json = {
    workspace = true }` adds no feature, so the residue is untouched.
- **A-V13 — the interim end's footprint, which EX-6 removes.** Read and
  grepped (`command grep -rn 'PHASE-12\|PHASE-04\|interim\|not yet\|NOT_YET_IMPLEMENTED\|unjudged_end\|never.spawned'
  crates/goad-check`): `render::NOT_YET_IMPLEMENTED` and its unit test
  `the_interim_end_says_nothing_was_judged_on_stderr`; `render::NO_VIEW`'s
  doc (*"Until PHASE-12 no exchange is made"*); `main`'s doc (*"Each status
  reachable until PHASE-12"*), `prepare`'s doc (*"built and not yet used"*)
  and its `_events`, `_runtime`, `_host`; `unjudged_end`; `statuses.rs`'
  `assert_no_report` doc (*"the interim end"*),
  `a_report_that_cannot_be_written_exits_2`'s doc (*"PHASE-12/VT-2 adds the
  run half"*) and `a_run_with_no_exchange_exits_2_with_no_verdict`;
  `loadable.toml`'s comment (*"never spawned at PHASE-04"*), and its command
  `bash ./never-spawned.sh`, which a run now spawns.
- **A-V14 — exit grep for the names this phase ships.** `command grep -rn
  'goad-check-unrecognised\|a_conforming_backend_is_accepted_and_exits_0\|a_backend_failing_at_startup_is_still_asked_the_rest\|event_files_are_sent_in_the_order_given\|the_probe_kind_is_none'
  crates tests`: nothing, exit 1.

**Assumptions — first tested by this phase**
- **A-T1 — the run fits one current-thread `block_on`.** `prepare` returns
  what it built (D-4 said PHASE-12 changes its return), and the run is one
  `runtime.block_on(async { … })` over the plan, each exchange awaited in
  turn (§5.4 *Concurrency*). A struct carrying the runtime, the host and the
  events is now read, so it is no longer dead code.
- **A-T2 — a fabricated respond is the only cooperating route to
  `Failure::State`, and the checker never makes one.** `ViewId` has no
  public constructor, so the checker can only pass back an id it was given.
  An id from an earlier link of a chain is stale (R-33) and would yield
  `Failure::State`: that is mutation M-17, not a test.
- **A-T3 — the binary tier can see each new case red against code that
  compiles.** With the run stubbed (`todo!()` in the run function, ignored
  parameters spelled `_name`, §Harvest *Learned*), every run case fails, on
  the panic's status 101.
- **A-T4 — the chain bound reads as at most 8 responds per planned request.**
  §5.2.2: *"keeps answering until a respond returns `view: null` or a
  failure, up to a chain bound of 8 per request"*. The bound is hit when the
  eighth respond's `Outcome` carries a view; that view is left unanswered
  and the observation is reported. The executor records this as a decision.
  The case asserts the invocation count, which makes the reading testable
  (row M-12).
- **A-T5 — every new bash fixture is run by a case.** Nothing else holds it
  (A-V12).

**STOP conditions** (consult the user; do not improvise)
- From `plan.md` PHASE-12 Notes (quoted): *"A new external dependency is a
  STOP."* Also any `[dependencies]` entry, and any feature added to a
  dependency shared with stratum 1 (`jiff`, `serde`, `serde_json`), in either
  table (the residue).
- A file outside **Surfaces**: `crates/goad-semantics/` beyond
  `Options::first` and its unit test, `crates/goad-shell/`, `tests/support/`,
  `tests/backends/`.
- An I-1 hit outside VA-1's allowed set: any requirement-id spelling or side
  literal in `src` other than the one `Requirement::R56` and the two
  `AtFault::Backend` the R-56 condition and claim need (VA-1). This includes
  unit tests in `src` (A-V11).
- Any mapping from a field kind to a JSON type or value, in `src` or `tests`
  (I-2). Values come from `Submitted::as_drawn` and `Submitted::to_json`.
- A second `ExitCode::from(2)`, a 2 that reads its cause, or a status other
  than 0, 1, 2 (EX-6; SPEC-004 R-15 in `canon-delta.md`).
- A planned exchange skipped because of what an earlier one returned, or a
  run ended early for any reason but a clock unreadable mid-run, a report
  line stdout refuses, or `Failure::State` (EX-2).
- The checker opening a socket, naming an `ingress` item but
  `envelope::normalize`, or setting, removing or clearing an environment
  variable (EX-5).
- A domain word in `crates/goad-check/src` (A-V12).
- A test name differing from `design.md` §9 or `plan.md` VT-1..VT-3. Update
  `canon-delta.md` in the same commit and say so here (*Test names are
  commitments*). Not a STOP, but never silent.
- The R-56 condition or the chain bound read otherwise than EX-3 (as
  amended for PLAN QUESTION 4) and A-T4 settle: the design did not settle it
  further.
- `git stash`, `git checkout`, `git reset`, or any history rewrite.

**Tasks** — red first. Each new case goes red against code that compiles
before its body lands. Reach is proven as soon as a new file exists.

- [x] Set PHASE-12 to `in progress` in §Status.
- [x] Print `git log -1 --oneline`. It must be this sheet's commit, or a
      descendant whose only changes since are the PLAN QUESTIONs'
      resolutions.
- [x] Exit grep first (A-V14). Record any difference.
- [x] Every PLAN QUESTION resolved, and the amended criteria re-quoted here
      before the tasks they block start. Resolved by `design-log.md` 2026-10-01, *`Options::first`; R-56's condition counts `evaluate` outcomes* and `plan-log.md` 2026-10-01, *PHASE-12 sheet questions*; applied and
      re-quoted in *012 PHASE-12: decisions applied to design, plan and
      sheet*.
- **1. Test support and fixtures (VA-7)** — `tests/binary/`
  - [x] Read each `tests/support/` file's whole exported surface and record
        the include decision for each (A-V8 predicts `scripting.rs`
        included, `driving.rs` and `waiting.rs` not).
  - [x] If included: `#[cfg(test)] #[path =
        "../../../../tests/support/scripting.rs"] mod scripting;` in
        `tests/binary/main.rs`, with the reason, as
        `crates/goad-shell/tests/integration/main.rs` gives it.
  - [x] The new cases' file(s) declared in `tests/binary/main.rs`. If the
        spawn helpers in `statuses.rs` are shared with a new file, move them
        to one helper module in `tests/binary/` rather than copy them. If a
        PHASE-04 case changes file, `canon-delta.md`'s
        `crates/goad-check/tests/binary/statuses.rs::…` paths change in the
        same commit.
  - [x] **Reach, before any behaviour**: `cargo test -p goad-check --test
        binary --no-fail-fast 2>&1 | command grep -n 'Running\|test result'`
        shows the target, and its case count rises when the first new case
        lands (A-V12). Rows R-1..R-3.
  - [x] Bash fixtures as needed under `tests/binary/`, each with a header
        saying what it does and which case runs it. Reading requests:
        `logs-the-request-then-answers.sh` already logs raw requests (A-V8).
        Committed event files for `event_files_are_sent_in_the_order_given`:
        two envelopes, distinct kinds, `source` not `"host"`.
  - [x] `loadable.toml`'s comment rewritten: a run now spawns its command.
        Its only reader after EX-6 is `a_reserved_source_event_file_exits_2`,
        which fails at normalization, before any spawn. Say that.
  - [x] Reading a request log: substring on the exact serialized key, or a
        `serde_json = { workspace = true }` `[dev-dependencies]` entry to
        parse it. Record the choice as a decision. `a_view_answered_carries_exactly_its_options_fields`
        reads `response.option` and the key set of `response.values`;
        parsing is the stronger instrument there. If the entry is added,
        the comment's *"No dev-dependencies"* sentence changes in the same
        commit (Surfaces; PLAN QUESTION 5, resolved).
- **2. The probe (EX-1, VT-3)** — `src`
  - [x] The probe kind constant (§5.2.2: `goad-check-unrecognised`) and,
        beside it, VT-3 (quoted): *"`the_probe_kind_is_none_of_the_host_s_own`,
        a unit test beside the probe constant (a binary-only crate's
        constant is not reachable from `tests/binary/`), asserting it is
        none of `Stimulus`'s kinds."* Offered: enumerate the variants
        through a `match` with no `_` arm, so a fourth `Stimulus` fails to
        compile there rather than going unchecked
        (`wildcard-enum-match-arm-counts-a-named-binding`). Red first against
        a constant equal to `Stimulus::Startup.kind()`, then green.
  - [x] The probe's `Event` is built with `source: HOST_SOURCE.to_owned()` and
        `data: Default::default()` (A-V5), never a `"host"` literal (I-1).
- **3. The binary tier, red (VT-1, VT-2)**
  - [x] VT-1 (quoted): *"binary tier, the run cases §9 names:
        `a_conforming_backend_is_accepted_and_exits_0`,
        `a_backend_that_fails_on_an_unrecognised_host_kind_is_reported_against_r56`,
        `a_refused_view_is_reported_with_its_requirement_and_the_backend_side`,
        `a_discarded_next_check_is_reported_and_exits_1`,
        `an_unspawnable_command_is_reported_against_the_configuration`,
        `a_backend_failing_identically_on_every_kind_is_not_charged_with_r56`,
        `a_backend_that_returns_no_view_is_accepted_and_says_respond_was_not_exercised`,
        `a_chained_view_is_answered_until_null`,
        `a_chain_past_its_bound_is_reported_and_does_not_change_the_status`,
        `a_view_answered_carries_exactly_its_options_fields`. The R-56 case
        asserts the line's `SPEC-001/R-56`, which holds `Requirement::R56`'s
        value (`design.md` §5.2.3). And the plan's own:
        `a_backend_failing_at_startup_is_still_asked_the_rest` (a backend
        that logs each request's kind and fails the first: the log holds
        every planned kind), and `event_files_are_sent_in_the_order_given`
        (two `--event` files, the backend's log holds their kinds in the
        order given). And the R-56 condition's scope, `evaluate` outcomes only
        (`plan-log.md` 2026-10-01, *PHASE-12: R-56's scope gets cases;
        `UnixDatagram` in VA-3*): `a_failure_in_the_probe_s_chain_is_not_charged_with_r56` (the probe's `evaluate` answers a view
        whose `respond` fails, the known kinds succeed; no `SPEC-001/R-56`)
        and `a_known_kind_s_chain_failure_does_not_excuse_the_probe` (each known kind's view's `respond` fails, the probe's
        `evaluate` fails; `SPEC-001/R-56` present). The non-zero cases SPEC-004/R-14's row names assert
        the `goad-check: ` prefix on the **last** stderr line."*
  - [x] What each case must assert so that it is not a proxy
        (`tests-asserting-proxies`). Status alone, or status plus the
        prefix, survives most regressions here: each case below names the
        assertion the mutation table leans on.
    - `a_conforming_backend_is_accepted_and_exits_0`: status 0; a view is
      returned and answered, so stdout has a verdict line and **not**
      `render`'s no-view line; **no** `SPEC-001/R-56` (M-31); stderr empty
      (AC-1's shape, §9).
    - `…_reported_against_r56`: known kinds answered, the probe
      `@exit1`; status 1; one stdout line holds both `backend` and the
      literal `SPEC-001/R-56`; the failure's own line holds `backend` and
      `SPEC-001/R-40`; the backend's stderr text (*"that answer is not to
      be trusted"*) appears verbatim in the report (EX-3, *stderr
      verbatim*; Findings).
    - `a_refused_view_is_reported_with_its_requirement_and_the_backend_side`:
      a view with `"options":[]`; status 1; one line holds `backend` and
      `SPEC-001/R-13`.
    - `a_discarded_next_check_is_reported_and_exits_1`: **exactly one**
      refusal in the run (so M-19 is seen); status 1; a line holds
      `backend` and the `ScheduleError` arm's id, as a literal; last stderr
      line begins `goad-check: ` (R-14's row).
    - `an_unspawnable_command_is_reported_against_the_configuration`: `--
      <a path no file is at>`; status 1; a line holds `configuration` and
      `SPEC-001/R-44`; **no** line holds `SPEC-001/R-56`.
    - `…_identically_on_every_kind_is_not_charged_with_r56`: every
      invocation `@exit1`; status 1; refusal lines present; **no**
      `SPEC-001/R-56` anywhere on stdout.
    - `…_returns_no_view_…`: status 0; stdout carries `render`'s no-view
      line, last before the verdict.
    - `a_chained_view_is_answered_until_null`: two views then `null`; the
      invocation count equals the planned evaluates plus the chain's
      responds, exactly; status 0.
    - `a_chain_past_its_bound_…`: a view on every answer for one request;
      status 0; the bound's observation line; **the invocation count,
      exactly** (A-T4; without it, M-12 is unseen).
    - `a_view_answered_carries_exactly_its_options_fields`: a view whose
      first option has fields of two kinds and whose second option has a
      field of its own; the respond request in the log has `option` equal
      to the first option's id and `values`' keys equal to exactly the
      first option's field ids. No expected value is written per kind in
      the test (I-2 reads `tests`).
    - `a_backend_failing_at_startup_is_still_asked_the_rest`: the first
      invocation fails; the log's `event.kind`s are **the exact sequence**
      `startup`, `requested`, `scheduled`, then the probe kind, each with
      `"source":"host"` (EX-1 says *in order*; a set comparison misses
      M-2). Spell the three host kinds through `Stimulus::kind`, which the
      target can name.
    - `event_files_are_sent_in_the_order_given`: two event files; the log's
      last two evaluates carry their kinds in the order given, **after** the
      probe (M-3, M-4).
  - [x] `a_failure_in_the_probe_s_chain_is_not_charged_with_r56` (VT-1), red first. Setup: `scripting::scripted` alone, with
        `answers-as-instructed.sh` — no new fixture. Its instructions are
        per invocation, a response body or a sentinel, and the plan fixes
        the invocation order (EX-1): the three known kinds' `evaluate`s
        answer `{"view":null,"next_check":"45 minutes"}`; the probe's
        `evaluate` answers a view with one option and no fields (the
        `@slow-view` body's shape, without the delay); its `respond` is
        `@exit1`; past the list the script behaves. Assert: status 1; a line
        holding `backend` and `SPEC-001/R-40` (the respond's failure,
        reported as any other exchange's); **no** `SPEC-001/R-56` anywhere on
        stdout; the invocation count, exactly (five, so a respond was made:
        without it the case is green with no chain at all). Row M-36.
  - [x] `a_known_kind_s_chain_failure_does_not_excuse_the_probe` (VT-1), red first. Setup: `scripting::scripted` alone, as above: each
        known kind's `evaluate` answers that view and its `respond` is
        `@exit1`, then the probe's `evaluate` is `@exit1`. **Every** known
        kind's chain fails, not one: the condition needs only one known kind
        clean, so a single failing chain leaves the mutation green. Assert:
        status 1; one stdout line holds both `backend` and the literal
        `SPEC-001/R-56`; the invocation count, exactly (seven: every chain's
        respond was made). Row M-37.
  - [x] VT-2 (quoted): *"`a_report_that_cannot_be_written_exits_2` gains its
        run half: a run with stdout on `/dev/full` exits 2, the last stderr
        line beginning `goad-check: `."* Use the argv form with a scripted
        conforming backend, not `loadable.toml` (whose command no longer
        exists to spawn), so the only fault in the run is stdout.
  - [x] **Before `a_cleanup_failure_alone_exits_1` joins the gate**, rank
        its fixture's margin by which way load moves it, not by its size
        (`docs/memory/margin-size-is-not-margin-direction.md`; `plan-log.md` 2026-10-01, *PHASE-12 sheet questions*, *Also*).
        The bound is `CLEANUP_LIMIT`'s 500 ms against `@lingers`' `sleep 2`.
        Write down, for each of the two durations, what load stretches: the
        `sleep` is wall clock, and so is the cleanup timer, but the host's
        work between the grandchild's fork and the timer's start is not, and
        it is spent out of the grandchild's 2 s. State whether load moves the
        case toward a false green (`cleanup` `None`, status 0) or away from
        it, and whether A-V9's sentence survives. Then measure at the bound
        (`docs/memory/timed-test-margins-are-measured-at-the-bound.md`): in a
        detached `git worktree`, instrument the elapsed time from the
        exchange's start to the cleanup timeout firing, run the case under
        oversubscription (busy loops well past the core count; record the
        loadavg), and record the worst elapsed against 2 s. Remove the
        instrumentation and the worktree. If load moves it toward failure
        and the measured slack is thin, the case does not join: record it
        as a Finding and leave M-21 **unseen**.
  - [x] Offered (Findings, *EX-3 and EX-4 items no named case reads*): a
        cleanup-only case, `a_cleanup_failure_alone_exits_1` (`@lingers` on
        the first invocation, A-V9: status 1, a line holding `backend` and
        `SPEC-001/R-48`), and a truncation case, `a_truncated_stderr_is_flagged`,
        with a fixture that writes past the stderr cap (read
        `floods-stderr-then-answers.sh` first; the report marks the
        truncation). Each is a sheet row, not a plan criterion; record
        whether it was added.
  - [x] Offered (Findings, *the side clause*): a case where the probe alone
        fails on a side other than the backend's, e.g.
        `a_probe_failure_on_another_side_is_not_charged_with_r56`: a copy of
        a fixture script, made executable at a `scripting::marker` path,
        that deletes itself on its third run, so the probe's spawn fails
        (`Spawn`, configuration). Spike it first; record whether it held.
  - [x] Red, against the stubbed run (A-T3), compiling: `cargo test -p
        goad-check --test binary --no-fail-fast`. Record each failure and
        its assertion.
- **4. The run (EX-1..EX-6)** — `src`
  - [x] `prepare` returns what it built (A-T1).
  - [x] EX-1 (quoted): *"the request plan of §5.2.2 in order:
        `Stimulus::Startup`, `Requested`, `Scheduled`, the R-56 probe, then
        each `--event` envelope in the order given. The probe kind is a
        `goad-check` constant, and its event's source is `HOST_SOURCE`, not a
        literal. `event.timestamp` and `now` are the wall clock at each step.
        Each view is answered through `Host::respond` with the minted
        `view_id`, its first option through `Options::first` (`design.md`
        §5.2.4), and `Submitted::as_drawn` for each of that option's fields;
        chains are followed to `view: null` or a failure, up to the bound of
        8 per request."*
  - [x] `Options::first`, red: a unit test in `canonical.rs`' `mod tests`,
        `the_first_option_is_the_one_listed_first` (`design.md` §5.2.4, §9),
        over two options, asserting the first's id. `Alternatives::first` has
        no unit test of its own (A-V6; it is reached only through
        `an_as_drawn_choice_submits_the_first_alternative`), so it goes
        beside the `Options::new` cases. Red against a compiling stub that
        returns the last option (`self.0.last().expect(…)`): `cargo test -p
        goad-semantics --no-fail-fast`, the assertion recorded.
  - [x] `Options::first(&self) -> &Opt`, green: beside `Options::as_slice`,
        mirroring `Alternatives::first` — its doc's argument (non-emptiness
        is the type's invariant, `Options::new` refuses the empty list with
        `EmptyOptions`), its `# Panics` section, its `#[expect(clippy::expect_used,
        reason = …)]` naming `Options::new` and `EmptyOptions`, and
        `#[must_use]`. Nothing else in `canonical.rs` changes. `cargo test -p
        goad-semantics`; `cargo clippy --workspace --all-targets -- -D
        warnings`.
  - [x] The answer calls `Options::first`; no `.as_slice().first()` and no
        `let…else` over the options in `goad-check`.
  - [x] EX-2 (quoted): *"every planned exchange is made whatever an earlier
        exchange did (§5.4); only a clock unreadable mid-run, a report line
        stdout refuses, or `Failure::State` ends the run early, with no
        verdict, status 2."* Each early end goes through `not_judged` with a
        `render` line: a mid-run clock line, and a `Failure::State` line
        printing the error's `fault()`, `requirement()` and `Display` (the
        §5.2.2 table's State row).
  - [x] EX-3 (quoted): *"the report: every channel of every `Outcome` as
        §5.2.2's table gives it; each refusal line prints its side and
        `SPEC-001/R-N` through `AtFault`'s and `Requirement`'s `Display`; the
        R-56 line only on its condition, which counts `evaluate` outcomes only, the probe's and the
        three known kinds', a chained `respond` being judged as any other
        exchange (`design-log.md` 2026-10-01, *`Options::first`; R-56's
        condition counts `evaluate` outcomes*); stderr verbatim with
        truncation flagged; the values sent; the chain-bound observation. The R-56
        line's id is `Requirement::R56`, from stratum 1, printed through
        `Requirement`'s `Display` as every refusal line's is; its text, "a
        backend MUST tolerate a kind it does not recognise", is the
        checker's (`design.md` §5.2.2; `design-log.md` 2026-10-01,
        *`Requirement` is built from named constants*)."* Every line's text
        in `render`; every stdout line through `report::try_line_to`.
  - [x] The R-56 condition's inputs (EX-3 as amended; PLAN QUESTION 4,
        resolved): the probe's `evaluate` `Outcome`'s `failure`, and the
        three known-kind `evaluate` `Outcome`s' `failure`s. A chained
        `respond`'s `Outcome`, from the probe's chain or a known kind's, is
        reported and feeds the status cut as any other, and the condition
        never reads it. Rows M-36, M-37.
  - [x] EX-4 (quoted): *"the run feeds the status cut (EX-6): at least one
        refusal on any channel, a cleanup failure alone included, is 1; none
        is 0; the chain bound changes no status."*
  - [x] EX-5 (quoted): *"the checker opens no socket and alters none of the
        environment it passes on (§5.2.1, §5.3): it names no `ingress` item
        but `envelope::normalize`, and sets, removes or clears no environment
        variable."*
  - [x] EX-6 (quoted): *"the verdict replaces PHASE-04/EX-3's interim end:
        the verdict line, and statuses as §5.2.5 — a delivered verdict is 0
        with no refusal and 1 with at least one, cut by one function; `main`
        gains a literal for each, and the last stderr line on 1 begins
        `goad-check: `. The not-yet-implemented stderr line and its path to 2
        are gone, and PHASE-04/VT-3's case is deleted in the commit that
        turns `a_conforming_backend_is_accepted_and_exits_0` green."*
  - [x] In that same commit: `unjudged_end`, `render::NOT_YET_IMPLEMENTED`,
        `render`'s `the_interim_end_says_nothing_was_judged_on_stderr`, and
        `a_run_with_no_exchange_exits_2_with_no_verdict` deleted. Rerun
        A-V13's grep: every remaining hit read and either true or rewritten.
  - [x] `render` unit cases for the new lines, red against `todo!()` bodies
        first, holding shape only (A-V11).
  - [x] Green: VT-1, VT-2, VT-3, the render cases, every PHASE-04 case.
        `cargo test -p goad-check --no-fail-fast`; `cargo clippy --workspace
        --all-targets -- -D warnings` clean.
- **5. Reads and records (VA-1..VA-7)**
  - [x] VA-1 (quoted): *"I-1 over the finished crate, by the command under
        *Invariant reads*: the only hits outside a comment are the R-56
        probe's own — the claim's `Requirement::R56` (EX-3), and the two
        `AtFault::Backend` uses R-56 needs: the condition's comparison of a
        probe failure's `fault()` (§5.2.2) and the probe's claim. Any other
        spelling of an id, `R-56` and `R56` included, is a hit outside that
        set. Recorded. This read is also what holds that the claim
        PHASE-08/VT-1 reads, `Requirement::R56`, is the one the report
        prints."* Run A-V10's I-1 command; read each surviving line; record
        the output verbatim. Expected: exactly those three.
  - [x] VA-2 (quoted): *"I-2 over the finished crate, `src` and `tests`.
        Recorded."* `command grep -rn 'FieldKind' crates/goad-check/src
        crates/goad-check/tests`; read each hit.
  - [x] VA-3 (quoted): *"EX-5: `command grep -rnE
        'ingress::|set_var|remove_var|env_clear|env_remove|\.envs?\(|UnixStream|UnixListener|UnixDatagram|TcpStream|TcpListener|UdpSocket'
        crates/goad-check/src` finds the `envelope` imports, and nothing
        else. Recorded (`plan-log.md` 2026-10-01, *PHASE-12 sheet
        questions*, Q2; `plan-log.md` 2026-10-01, *PHASE-12: R-56's scope
        gets cases; `UnixDatagram` in VA-3*)."* Record the output verbatim; then read the one
        call, `envelope::normalize`, which the pattern does not match, and
        record that no other `ingress` item is named. Reach row R-4.
  - [x] VA-4 (quoted): *"PHASE-04/VA-6's structural read, over `main` as this
        phase leaves it, with one literal per way to a status where
        PHASE-04/VA-6 reads one per class; still exactly one
        `ExitCode::from(2)` (`plan-log.md` 2026-10-01, *PHASE-12 sheet
        questions*, Q3)."* `command grep -rn 'ExitCode' crates/goad-check/src`,
        then trace every path to 2, 1 and 0. Expected (Findings, PLAN
        QUESTION 3): `answer`'s 0, the verdict cut's 0 and 1, `not_judged`'s
        2.
  - [x] VA-5 (quoted): *"mutation: the R-56 condition's "at least one of the
        three known-kind `evaluate`s made no failure" dropped, so any
        backend-side failure on the probe is charged, reds
        `a_backend_failing_identically_on_every_kind_is_not_charged_with_r56`.
        Recorded."* Row M-15.
  - [x] VA-6 (quoted): *"`canon-delta.md` SPEC-001 Change 3's and SPEC-004
        Change 5's `goad-check` test paths, for the cases this phase ships,
        are re-pointed from `…` to the shipped files."* Note: SPEC-001 Change
        3 lists `::the_probe_kind_is_none_of_the_host_s_own` under a
        `crates/goad-check/tests/binary/…` prefix, but VT-3 ships it in
        `src`: it gets its own `crates/goad-check/src/<file>.rs::` path.
        `crates/goad-check/tests/kit/…::each_shipped_example_is_accepted_by_the_checker`
        is PHASE-06's and keeps `…`. The R-14 row spells no `…` and is
        untouched (as PHASE-04/VA-4). `git diff --stat` on `canon-delta.md`
        recorded.
  - [x] VA-7 (quoted): *"as PHASE-04/VA-7, for any helper this phase
        adds."* Edit §Open's FU-5 bullet: the `tests/support/` file(s)
        included, and each helper copied instead, by symbol, with its
        source.
- **Refactor**
  - [x] Read the diff for a second statement of anything: a line's text
        outside `render`; a path to 2 outside `not_judged`; a refusal's side
        or id spelled rather than printed; a rule `goad-shell` or
        `goad-semantics` already states (the as-drawn value, the view_id
        check, a fault's side) restated here.
  - [x] Every doc in the crate cites by symbol and counts nothing that can
        grow (CLAUDE.md *Name, never count*). The chain bound is a constant
        with one name.
- **Verification**
  - [x] Mutation and reach rows (below), each recorded with the assertion
        that went red, not only the case
        (`a-mutation-can-red-the-right-case-on-the-wrong-assertion`).
  - [x] *Test names are commitments*: `command grep -rc 'fn <name>()'
        crates/goad-check/tests/binary crates/goad-check/src` is 1 for each
        VT-1, VT-2 and VT-3 name, and 0 for
        `a_run_with_no_exchange_exits_2_with_no_verdict`; `command grep -c
        'fn the_first_option_is_the_one_listed_first()'
        crates/goad-semantics/src/protocol/canonical.rs` is 1 (`design.md`
        §9). Any difference is
        updated in `canon-delta.md` in the same commit.
  - [x] Every bash fixture and event file under `tests/binary/` is named by
        at least one case: `command grep -rn '<file name>'
        crates/goad-check/tests/binary --include=*.rs` per file (A-T5).
  - [x] `just check` exits 0 on the final commit. Record passed and failed,
        summed over every `test result` line: 715, less the two deleted
        cases, plus this phase's.
  - [x] §Status: PHASE-12 `done`, with the date.
  - [x] Harvest updated in place (*Fresh as of*, Produced, Learned, Open).

**Exit criteria** (quoted in the tasks above; listed here to be ticked)
- [x] EX-1 *`run::plan` (host kinds, `Planned::Probe`, each `Planned::Given` in order); `Planned::event` builds the probe from `HOST_SOURCE` and `PROBE_KIND`; `now()` reads `clock::wall_clock` per request and per respond; `run::answer` uses `Options::first` and `Submitted::as_drawn`; `follow` answers with the minted `view_id` up to `run::CHAIN_BOUND`. Held by `a_backend_failing_at_startup_is_still_asked_the_rest`, `event_files_are_sent_in_the_order_given`, `a_view_answered_carries_exactly_its_options_fields`, the chain cases (M-1..M-12, M-17, M-35).*
- [x] EX-2 *`exchanges` makes every planned exchange; only a `RunFault` (`Clock`, `ReportUnwritten`, `State`) ends it, through `run_fault_line` and `not_judged`. M-5 red; M-32, M-34 unseen as predicted, held by VA-4's trace.*
- [x] EX-3 *`render::exchange_lines` writes every channel; `refusal_line` prints side and id through `AtFault`'s and `Requirement`'s `Display`; `probe_claim_line` names `Requirement::R56` on `run::charges_the_probe`, which reads evaluates only; stderr line by line with truncation flagged; `respond_line` writes the values sent; `chain_bound_line`. Every stdout line through `emit` (`try_line_to`). M-14..M-16, M-18, M-22, M-24..M-26, M-31, M-36, M-37 red.*
- [x] EX-4 *`Tally::record` counts an exchange refused on any channel (`Judged::refused`); `verdict` cuts at `> 0`; the bound adds nothing. M-13, M-19, M-20, M-21 red.*
- [x] EX-5 *VA-3 below: the `envelope` imports alone; the one call is `envelope::normalize` in `event`. R-4 shows the pattern reaching `UnixStream`, `UnixDatagram` and `.envs(`.*
- [x] EX-6 *`verdict` holds 0 and 1; `not_judged` the one 2. `unjudged_end`, `NOT_YET_IMPLEMENTED`, `the_interim_end_says_nothing_was_judged_on_stderr` and `a_run_with_no_exchange_exits_2_with_no_verdict` deleted in 139e968, the commit that turned `a_conforming_backend_is_accepted_and_exits_0` green. A-V13's grep rerun: only `loadable.toml`'s `never-spawned`, its comment rewritten to say a run spawns it. M-29 red on every status-1 case's last stderr line.*
- [x] VT-1 *every name in `crates/goad-check/tests/binary/run.rs`, `fn` count 1 each; red against the stubbed run (status 101), then green.*
- [x] VT-2 *`statuses.rs::a_report_that_cannot_be_written_exits_2` runs `--help` and a scripted conforming run on `/dev/full`: status 2, last stderr line `goad-check: `. M-30 red.*
- [x] VT-3 *`crates/goad-check/src/run.rs::the_probe_kind_is_none_of_the_host_s_own`, over `every_stimulus` (a `match` with no `_` arm); red against `"startup"`, then green. M-7 red.*
- [x] VA-1 *two lines, three uses, all allowed (Verification records).*
- [x] VA-2 *no output, exit 1.*
- [x] VA-3 *the two `envelope` imports, exit 0; nothing else.*
- [x] VA-4 *one literal per way to a status (Verification records); one `ExitCode::from(2)`.*
- [x] VA-5 *M-15 red `a_backend_failing_identically_on_every_kind_is_not_charged_with_r56` on its no-`SPEC-001/R-56` assertion.*
- [x] VA-6 *`git diff --stat docs/slices/012/canon-delta.md`: 1 file, 2 insertions, 2 deletions. SPEC-001 Change 3 → `tests/binary/run.rs` and `src/run.rs::the_probe_kind_is_none_of_the_host_s_own`; SPEC-004 Change 5's R-11..R-13 row → `tests/binary/run.rs`. `tests/kit/…` (PHASE-06) and the R-14 row untouched.*
- [x] VA-7 *§Open's FU-5 bullet extended: `scripting.rs` included whole, nothing new copied.*
- [x] `just check` exits 0 on the final commit (§Overview) *exit 0; 738 passed, 0 failed, summed over 33 `test result` lines (Verification records).*

**Mutation evidence** (`plan.md` *Mutation evidence*: copy the file to the
scratchpad and back, never `git checkout` or `git stash`; `--no-fail-fast`;
`git status --short` clean after each restore; a mutation that does not
compile is not evidence). The edits name code that does not exist yet: each
row names the symbol or behaviour it will touch, and the executor quotes the
exact edit when it runs. Command for M-rows: `cargo test -p goad-check
--no-fail-fast`. *Must red* names the case **and the assertion**. Rows marked
**unseen** are predictions that nothing reds; run them anyway (§Harvest
*Learned*) and record what holds the property instead.

*Reach rows* (A-V12; offered, as PHASE-04's R-4 was):

| row | file | edit | command | must red | compiled? | result |
|---|---|---|---|---|---|---|
| R-1 | each new `src/*.rs` module | `let _planted = "habit";` in a function in production code | `cargo test -p goad-boundary --test checks --no-fail-fast` | `vocabulary::no_workspace_member_names_the_users_domain`, naming the new file | yes | `let _planted = "habit";` first in `run::plan`. `vocabulary::no_workspace_member_names_the_users_domain` FAILED, `crates/goad-check/src/run.rs:61: forbidden token habit`. Restored. |
| R-2 | each new `tests/binary/*.rs` file | `fn _planted() { let _planted: std::collections::HashMap<u8, u8> = std::collections::HashMap::new(); }` | `cargo clippy --workspace --all-targets -- -D warnings` | `clippy::disallowed_types` in the `binary` target, naming the new file | yes | planted at the end of `process.rs` and of `run.rs`; clippy exit 101, `disallowed type std::collections::HashMap` twice in each file. Restored. |
| R-3 | each new `src/*.rs` and `tests/binary/*.rs` | a line re-indented to four spaces | `cargo fmt --all --check` | a diff naming each file | yes | the first two-space line of `src/run.rs`, `tests/binary/process.rs`, `tests/binary/run.rs` to four; `fmt --check` exit 1, `Diff in` each of the three. Restored. |
| R-4 | `crates/goad-check/src/main.rs`, a function in production code | `let _a = std::os::unix::net::UnixStream::connect("");`, `let _c = std::os::unix::net::UnixDatagram::unbound();` and `let _b = std::process::Command::new("").envs(std::iter::empty::<(&str, &str)>());` (compiled: `cargo build -p goad-check`) | VA-3's command | every planted line printed, beside the `envelope` imports (the widened pattern reaches `.envs(`, `UnixStream` and `UnixDatagram`; PLAN QUESTION 2) | yes (`cargo build -p goad-check -v`: rustc ran on `goad_check`, Finished) | planted first in `now`; VA-3 printed the two imports and all three planted lines, exit 0. Restored. |

`canonical.rs` has no reach row: it is an existing file every stratum-1
instrument already reads (A-V12).

*Behaviour rows*:

| row | file / symbol | edit | must red (case: assertion) | compiled? | result |
|---|---|---|---|---|---|
| M-1 | the plan | the probe dropped from the plan | `a_backend_failing_at_startup_is_still_asked_the_rest`: the kind sequence; `…_reported_against_r56`: status 1 and the `SPEC-001/R-56` line | yes | `Planned::Probe,` removed from the array; `Planned::Probe` kept constructed by `.chain(std::iter::empty::<Planned>().chain([Planned::Probe].into_iter().filter(\|_\| false)))` (the `unused` deny). Red: `…_still_asked_the_rest` the kind sequence; `…_reported_against_r56` status 1; and every case asserting an invocation count or the probe's kind. **As predicted.** |
| M-2 | the plan | `Requested` and `Scheduled` swapped | `a_backend_failing_at_startup_is_still_asked_the_rest`: the kind **sequence** (a set comparison would stay green) | yes | the `Requested` and `Scheduled` lines swapped. Red: `…_still_asked_the_rest` and `event_files_…` the kind sequence. **As predicted.** |
| M-3 | the event step of the run | the events iterated `.rev()` | `event_files_are_sent_in_the_order_given`: the order | yes | `.chain(given.into_iter().rev().map(Planned::Given))`. Red: `event_files_…` the sequence (`second-given` first). **As predicted.** |
| M-4 | the plan | the events sent before the host's own evaluates | `event_files_are_sent_in_the_order_given`: the events after the probe; `…_still_asked_the_rest`: the first four kinds | yes | `given.into_iter().map(Planned::Given).chain([…])`. Red: `event_files_…` the sequence (given first). `…_still_asked_the_rest` stays green: it passes no event file. **Differs in reach only**: its second clause cannot red (no event there). |
| M-5 | the run loop | `break` after the first `Outcome` whose `failure` is `Some` | `a_backend_failing_at_startup_is_still_asked_the_rest`: the kind sequence | yes | `let stop = judged.failure.is_some();` after the evaluate, `if stop { break; }` after `follow`. Red: `…_still_asked_the_rest` the sequence (`["startup"]`). **As predicted.** |
| M-6 | the probe's `Event` | `source: HOST_SOURCE.to_owned()` → `source: "probe".to_owned()` | `…_still_asked_the_rest`: `"source":"host"` on the probe's request | yes | `source: { let _ = HOST_SOURCE; "probe".to_owned() },`. Red: `…_still_asked_the_rest` the `source` assertion on the probe's request. **As predicted.** |
| M-7 | the probe constant | its value → `"requested"` | `the_probe_kind_is_none_of_the_host_s_own`: its assertion; `…_still_asked_the_rest`: the kind sequence | yes | `PROBE_KIND` = `"requested"`. Red: `the_probe_kind_is_none_of_the_host_s_own` (`Requested`); `…_still_asked_the_rest`, `event_files_…` the sequence. **As predicted.** |
| M-8 | the answer | `Options::first()` → the last option (`.as_slice().last()` with an `expect`; `cargo test` compiles it, clippy is not the command) | `a_view_answered_carries_exactly_its_options_fields`: `option` | yes | `choice.options().as_slice().last().expect("non-empty")`. Red: `…_options_fields` `option` (`"skipped"`). **As predicted.** |
| M-9 | the answer | values built from every option's fields (`flat_map`) | `a_view_answered_carries_exactly_its_options_fields`: the key set | yes | `values: choice.options().as_slice().iter().flat_map(\|option\| option.fields().as_slice())…`. Red: `…_options_fields` the key set (`reason` included). **As predicted.** |
| M-10 | the answer | the first field skipped (`.skip(1)`) | `a_view_answered_carries_exactly_its_options_fields`: the key set | yes | `.skip(1)` before `.map(\|field\| …)` in `run::answer`. Red: `…_options_fields` the key set (`["note"]`); `render`'s `a_respond_line_names_the_option_and_each_field_sent`. **As predicted**, plus the render case. |
| M-11 | the chain | answered once, not looped (`if let` for `while let`) | `a_chained_view_is_answered_until_null`: the invocation count | yes | `for _ in 0..1 {` in `follow`. Red: `a_chained_view_…` the count (5 for 6); `a_chain_past_…` the count (8 for 12). **As predicted.** |
| M-12 | the chain bound | 8 → 9 | `a_chain_past_its_bound_…`: the invocation count, and the observation line. Without the count, unseen (the 9th respond returns `null` and status is 0 either way) | yes | `CHAIN_BOUND: usize = 9`. Red: `a_chain_past_…` the count (13 for 12). **As predicted.** |
| M-13 | the status cut | the bound counted as a refusal | `a_chain_past_its_bound_…`: status 0 | yes | `tally.refused = tally.refused.saturating_add(1);` beside the bound's line. Red: `a_chain_past_…` status 0. **As predicted.** |
| M-14 | `render`, the bound's line | not written | `a_chain_past_its_bound_…`: the observation line | yes | `emit(&render::chain_bound_line())?;` → `let _unwritten = render::chain_bound_line();`. Red: `a_chain_past_…` the observation line. **As predicted.** |
| M-15 | the R-56 condition | the "at least one known-kind evaluate made no failure" clause dropped (VA-5) | `…_identically_on_every_kind_is_not_charged_with_r56`: no `SPEC-001/R-56` | yes | `(a_host_kind_made_no_failure \|\| true) && probe…` in `charges_the_probe`. Red: `…_identically_on_every_kind_…` no `SPEC-001/R-56`. **As predicted.** |
| M-16 | the R-56 condition | the `fault()` comparison with `AtFault::Backend` dropped | **unseen** by every planned case: only `Spawn` gives a non-backend probe failure, and it fails every kind, so the other clause already refuses (A-V3). Seen only by the offered side-clause case; else held by VA-1's read of the two `AtFault::Backend` and by review | yes | `failure.fault() == AtFault::Backend \|\| true`. Red: the offered `a_probe_failure_on_another_side_is_not_charged_with_r56`, no `SPEC-001/R-56`. **Seen**, by the offered case the sheet names; no planned case reds, as predicted. |
| M-17 | the chain | each respond passes the **previous** link's `view_id` (kept by `clone`) | `a_chained_view_is_answered_until_null`: status (`Failure::State` → 2) | yes | `let mut first = None;` before the loop, `let view_id = first.get_or_insert(view_id).clone();` before the respond. Red: `a_chained_view_…` status (2); `a_chain_past_…` status. **As predicted.** |
| M-18 | the R-56 claim | `Requirement::R56` → `Requirement::R53` | `…_reported_against_r56`: the literal `SPEC-001/R-56` | yes | `Requirement::R56` → `Requirement::R53` in `probe_claim_line`. Red: `…_reported_against_r56` and `a_known_kind_s_chain_failure_…` the `SPEC-001/R-56` line. **As predicted.** |
| M-19 | the status cut | `> 0` → `> 1` (or its spelling's equivalent) | `a_discarded_next_check_is_reported_and_exits_1`: status, **if** it has exactly one refusal | yes | `if refused > 1 {` in `verdict`. Red: `a_discarded_next_check_…` status 1, and every status-1 case with one refused exchange. **As predicted.** |
| M-20 | the refusal count | `discarded` not counted | `a_discarded_next_check_is_reported_and_exits_1`: status | yes | `Judged::refused` without `!self.discarded.is_empty()`. Red: `a_discarded_next_check_…` status. **As predicted.** |
| M-21 | the refusal count | `cleanup` not counted | the offered `a_cleanup_failure_alone_exits_1`: status. Else **unseen**; held by review | yes | `Judged::refused` without `self.cleanup.is_some()`. Red: the offered `a_cleanup_failure_alone_exits_1` status. **Seen**, by the offered case. |
| M-22 | `render`, a refusal line | the side omitted | `a_refused_view_…`: `backend` on the line; `an_unspawnable_…`: `configuration` | yes | `refusal_line`'s body → `{ let _side = side; format!("  REFUSED  SPEC-001/{requirement}  {what}") }`. **First run differed**: `a_refused_view_…` and `…_reported_against_r56` stayed green, because `BackendError`'s `Display` says `backend` on the same line (Findings). `a_line_names` fixed in 63db06f; rerun: red `a_refused_view_…` its side assertion, `an_unspawnable_…` `configuration`, and every case naming a side. |
| M-24 | the report | the outcome's `stderr` not written | `…_reported_against_r56`: the stderr text (the sheet's assertion; Findings) | yes | the `lines.extend(… "  stderr: {line}" …)` block removed from `exchange_lines`. Red: `…_reported_against_r56` the stderr text; `render`'s `the_backend_s_stderr_is_shown_…`. **As predicted**, plus the render case. |
| M-25 | the report | `truncated` ignored | the offered `a_truncated_stderr_is_flagged`. Else **unseen**; held by review | yes | `if false && judged.stderr.truncated {`. Red: the offered `a_truncated_stderr_is_flagged`; `render`'s `the_backend_s_stderr_is_shown_…`. **Seen**, by the offered case and a render case. |
| M-26 | the report | the values sent not written | **unseen** unless a case asserts them in the report (Findings) | yes | `.map(\|(_field, _value)\| String::new())` in `respond_line`. Red: `…_options_fields` the values line (D-8); `render`'s `a_respond_line_names_…`. **Seen**, where the sheet predicted unseen: D-8 put the assertion in. |
| M-27 | the report | the no-view line written unconditionally | `a_conforming_backend_is_accepted_and_exits_0`: no no-view line | yes | `if true \|\| !tally.viewed {`. Red: `a_conforming_…` no no-view line. **As predicted.** |
| M-28 | the report | the no-view line never written | `…_returns_no_view_…`: the no-view line | yes | `if false && !tally.viewed {`. Red: `…_returns_no_view_…` the no-view line. **As predicted.** |
| M-29 | `render`, the status-1 summary | its `goad-check: ` prefix dropped | `a_discarded_next_check_is_reported_and_exits_1`: the last stderr line | yes | `"{refused} exchange(s) refused; the report…"` (prefix dropped). Red: `a_discarded_next_check_…` the last stderr line, every status-1 case, and `render`'s `the_verdict_and_the_status_1_line_…`. **As predicted.** |
| M-30 | the report writer | one report line through `line_to` (best effort) | `a_report_that_cannot_be_written_exits_2`, run half: status 2 | yes | `emit` → `line_to(…); if false { return Err(RunFault::ReportUnwritten(std::io::Error::other("never"))); } Ok(())`: **every** report line best effort, since one line alone is unseen (the first line written already fails). Red: `a_report_that_cannot_be_written_exits_2` status (0 for 2). **As predicted.** |
| M-31 | the R-56 line | its condition replaced by `true`, so it is written even when the probe succeeds | `a_conforming_backend_is_accepted_and_exits_0`: no `SPEC-001/R-56` (and status 0, if the claim feeds the count) | yes | `Planned::Probe => true \|\| run::charges_the_probe(…)`. Red: `a_conforming_…` no `SPEC-001/R-56`; every case asserting no claim. Status unchanged: the claim does not feed the count (D-2). **As predicted.** |
| M-32 | the run loop | `Failure::State` treated as a refusal, run continues | **unseen**: no cooperating case reaches `Failure::State` (A-T2). Held by VA-4's trace | yes | `Some(Failure::State(_error)) => None,` in `run::judged`. Nothing red. **Unseen, as predicted**; held by VA-4's trace. |
| M-33 | the clock | `now` read once, before the plan, and reused | **unseen**: nothing asserts a time (R3). Held by review | yes | `now` cached in a `static OnceLock<Timestamp>`. Nothing red. **Unseen, as predicted**; held by review. |
| M-34 | the clock | a mid-run `wall_clock` error ignored | **unseen**: headless-unreachable (`canon-delta.md` R-11..R-13 row). Held by VA-4's trace | yes | `now` → `if false { return Err(RunFault::Clock(ClockError::BeforeEpoch)); } Ok(clock::wall_clock().unwrap_or(Timestamp::new(jiff::Timestamp::UNIX_EPOCH)))`. Nothing red. **Unseen, as predicted**; held by VA-4's trace. |
| M-35 | `Options::first` (`canonical.rs`) | `.first()` → `.last()` in its body; command `cargo test -p goad-semantics -p goad-check --no-fail-fast` | `the_first_option_is_the_one_listed_first`: the first's id; `a_view_answered_carries_exactly_its_options_fields`: `option` | yes | `.first()` → `.last()` in `Options::first`. Red: `the_first_option_is_the_one_listed_first` (`"second"`); `…_options_fields` `option`. **As predicted.** |
| M-36 | the R-56 condition | a failing `respond` in the probe's chain counted as a probe failure | `a_failure_in_the_probe_s_chain_is_not_charged_with_r56`: no `SPEC-001/R-56` | yes | after `follow`, `if matches!(planned, Planned::Probe) && a_host_kind_made_no_failure && tally.refused > refused_before { emit(&render::probe_claim_line())?; }`. Red: `a_failure_in_the_probe_s_chain_…` no `SPEC-001/R-56`. **As predicted.** |
| M-37 | the R-56 condition | a known kind counted as having "made a failure" when a `respond` in its chain failed | `a_known_kind_s_chain_failure_does_not_excuse_the_probe`: the `SPEC-001/R-56` line | yes | the `Host` arm sets nothing; after `follow`, `a_host_kind_made_no_failure \|= evaluate_clean && tally.refused == refused_before`. Red: `a_known_kind_s_chain_failure_…` the `SPEC-001/R-56` line. **As predicted.** |

**Verification records** (executed 2026-10-01; the code at 0fafad7)
- `git log -1 --oneline` at start: `238f26c 012 PHASE-12: R-56 scope cases
  belong to VT-1`. Exit grep (A-V14): no output, exit 1.
- **Red runs.** `the_first_option_is_the_one_listed_first` against a stub
  returning `self.0.last()`: `left: "second"`, `right: "first"`.
  `the_probe_kind_is_none_of_the_host_s_own` against `PROBE_KIND = "startup"`:
  `assertion left != right failed: Startup`. The binary tier against `run`
  stubbed `todo!()`: 15 FAILED, each on its status assertion, `left: 101`
  (the 14 cases of `tests/binary/run.rs` then, and VT-2's run half); the five
  PHASE-04 cases still passing. The five `render` unit cases against bodies
  returning `String::new()` / `Vec::new()`: all five FAILED. The three
  offered cases were written after the run was green; their reds are M-16,
  M-21 and M-25.
- **VA-1** (A-V10's I-1 command): `crates/goad-check/src/render.rs:130:
  refusal_line(AtFault::Backend, Requirement::R56, &PROBE_CLAIM)` and
  `crates/goad-check/src/run.rs:156: .is_some_and(|failure| failure.fault()
  == AtFault::Backend)`; `PIPESTATUS 0 0`. Two lines carrying the three
  allowed uses: the claim's side and id (`render::probe_claim_line`), and the
  condition's comparison (`run::charges_the_probe`). Nothing else.
- **VA-2**: `command grep -rn 'FieldKind' crates/goad-check/src
  crates/goad-check/tests`: no output, exit 1.
- **VA-3**: `src/render.rs:260: use goad_shell::ingress::envelope::EnvelopeFault;`
  (the test module) and `src/main.rs:20: use
  goad_shell::ingress::envelope::{self, EnvelopeFault};`, exit 0, nothing
  else. `command grep -rn 'envelope::' crates/goad-check/src` adds two doc
  lines and the one call, `envelope::normalize(&bytes)` in `event`: no other
  `ingress` item is named.
- **VA-4**: `command grep -rn 'ExitCode' crates/goad-check/src`: the `use`;
  the signatures of `main`, `run`, `verdict`, `answer`, `not_judged`;
  `ExitCode::from(1)` and `ExitCode::SUCCESS` in `verdict`;
  `ExitCode::SUCCESS` in `answer`; `ExitCode::from(2)` in `not_judged`.
  Traced: `main` reaches `not_judged` (usage error, `StartupFault`),
  `answer` (0, or `not_judged` on an unwritten answer) or `run`; `run`
  reaches `not_judged` on any `RunFault` (from `exchanges` or `deliver`) or
  `verdict` (0 or 1). One literal per way to a status; one
  `ExitCode::from(2)`.
- **Test names**: `command grep -rh 'fn <name>()'` over
  `crates/goad-check/tests/binary` and `crates/goad-check/src` is 1 for each
  VT-1, VT-2 and VT-3 name, 0 for
  `a_run_with_no_exchange_exits_2_with_no_verdict`;
  `the_first_option_is_the_one_listed_first` is 1 in `canonical.rs`.
- **Fixture readers**: `first-given.json`, `second-given.json` and
  `deletes-itself-on-its-third-run.sh` are named in `tests/binary/run.rs`;
  `loadable.toml` and `reserved-source.json` in `statuses.rs`.
- **Mutation rows**: every row compiled, ran with `--no-fail-fast`, was
  restored from a scratchpad copy, and `git diff --quiet` held on the file.
  All rows were rerun at 63db06f, after the M-22 repair; the table's results
  are that run's.
- **`just check`** at 0fafad7: exit 0. **738 passed, 0 failed**, summed over
  33 `test result` lines. Expected: 715, less the two deleted cases, plus 24
  new — 17 binary-tier (`run.rs`' 14 VT-1 cases and the three offered), 6
  `goad-check` unit (VT-3 and five `render` cases), and
  `the_first_option_is_the_one_listed_first` — which is 737; the 738th is
  that last case counted twice, since the gate runs `goad-semantics`' unit
  target under both `cargo test --workspace` and `cargo test -p
  goad-semantics` (its `... ok` line appears twice in the log). `just -n
  check` against POL-001 §Compliance's block: `4c4`, `deno check
  examples/typescript/backend.ts` → `deno check
  exercisers/typescript/backend.ts`, the one line expected.

**Decisions taken during execution**
<!-- Small and local: how, within what the design already settled. A choice that
     changes the design is not one of these — stop, consult the user, and record
     it in `design-log.md`. -->

- **D-1 — a pure `src/run.rs`.** The plan (`Planned`, `plan`), `PROBE_KIND`,
  `CHAIN_BOUND`, the answer (`answer`), `Judged` (an `Outcome` whose failure
  is the backend's; `judged` returns `Failure::State` as the error), the R-56
  condition (`charges_the_probe`) and the tally (`Tally`). `main` drives the
  exchanges and writes; `render` words every line. `main` stays the one
  impure file.
- **D-2 — the verdict counts exchanges refused**, as §5.2.5's example
  (*"verdict: 1 exchange refused"*): an exchange with a refusal on any
  channel counts once. The R-56 claim is a line, not a count; the probe's own
  failure already counts.
- **D-3 — the chain bound, as A-T4 reads it.** `follow` runs at most
  `CHAIN_BOUND` responds; a view still returned by the last is left
  unanswered and `chain_bound_line` is written. Held by the exact count in
  `a_chain_past_its_bound_…` (12: four evaluates and eight responds).
- **D-4 — the request log is parsed.** `serde_json = { workspace = true }` in
  `[dev-dependencies]`, and the manifest comment's *"No dev-dependencies"*
  sentence replaced, in 139e968 (PLAN QUESTION 5).
- **D-5 — the probe's `data` is `Default::default()`, under
  `#[expect(clippy::default_trait_access, reason = …)]` on `Planned::event`.**
  Clippy asks for `Value::default()`, which names `serde_json` in `src`, a
  crate `goad-check` does not depend on (A-V5; PHASE-04 Findings).
- **D-6 — I-1's *any spelling* covers identifiers.** A first draft named the
  claim's text `R56_CLAIM`, which I-1's pattern hit. Renamed `PROBE_CLAIM`;
  the lowercase `r56_line` and `charged_with_r56` became `probe_claim_line`
  and `charges_the_probe`, though the case-sensitive pattern misses them.
- **D-7 — stderr is shown through `String::from_utf8_lossy`**, one report
  line per stderr line, prefixed `  stderr: `. A non-UTF-8 byte is replaced,
  not shown raw: the report is text.
- **D-8 — the values sent are asserted by field.**
  `a_view_answered_carries_exactly_its_options_fields` also requires a report
  line naming both its fields' ids, which writes no value per kind (I-2), and
  makes M-26 seen.
- **D-9 — the spawn helpers moved** to `tests/binary/process.rs`, shared by
  `statuses.rs` and `run.rs`. `check` and `check_with_stdout_full` take
  `&[impl AsRef<OsStr>]`, so a case passes `against`'s `Vec<String>`.
  No PHASE-04 case changed file, so no `statuses.rs::` path in
  `canon-delta.md` moved.
- **D-10 — `a_cleanup_failure_alone_exits_1` joins the gate.** *Direction*:
  the grandchild's `sleep 2` runs from its fork; the 500 ms timer starts once
  the host has the body and the child's exit. A false green (`cleanup`
  `None`, status 0) needs fork-to-timer-start over 1.5 s. Load stretches that
  interval — the host's and the script's CPU work, and a late timer wake,
  since `tokio::time::timeout` polls the inner future first — so **load moves
  the case toward a false green**. Only the sleeper's own exit latency moves
  it away. A-V9's *"Load lengthens the grandchild's hold, which is the
  direction that keeps the case red-able"* does not survive (Findings).
  *Measured at the bound* in a detached worktree, `ProcessBackend::exchange`
  instrumented from before the spawn to the timer's start and its
  resolution, the binary run against `@lingers` 30 times per load: at rest,
  start-to-timer 2.2–3.1 ms; at 128 busy loops on 32 cores (131 runnable,
  1-min loadavg 38.24 and rising), worst 34.0 ms; at 256 (262 runnable,
  loadavg 133.21), worst **139.2 ms**, resolution at 644 ms. 90 of 90 under
  load timed out. The worst measured is about a tenth of the 1.5 s a false
  green needs. Worktree and instrumentation removed.
- **D-11 — the offered cases were added.** `a_cleanup_failure_alone_exits_1`
  (`@lingers`, D-10); `a_truncated_stderr_is_flagged`, through
  `scripting::backend("floods-stderr-then-answers")`, so no new fixture;
  `a_probe_failure_on_another_side_is_not_charged_with_r56`, whose spike
  held: the copy of `deletes-itself-on-its-third-run.sh`, run as the program
  itself, makes the probe's spawn fail as `Spawn` (configuration, R-44).
  M-16, M-21, M-25 are seen by them.
- **D-12 — `render`'s unit cases** (shape only, A-V11):
  `a_refusal_line_carries_the_failure_s_own_words_under_its_spec`,
  `the_backend_s_stderr_is_shown_line_by_line_and_its_truncation_flagged`,
  `a_respond_line_names_the_option_and_each_field_sent` (its view built
  through `read_response`, the path the host runs),
  `the_verdict_and_the_status_1_line_tell_accepted_from_refused`,
  `every_run_fault_says_nothing_was_judged_and_what_stopped_it`.

**Findings**
<!-- Things noticed in passing that are not this phase's job: a defect
     elsewhere, drift from the design, a surprise. Defects in this phase's own
     work get fixed, not recorded. These feed the audit; the ones that outlive
     the slice become Follow-ups. -->

- **PLAN QUESTION 1 — answering the first option needs a total `first()`
  that `Options` lacks.** EX-1 answers with *"its first option"*. `Options`
  offers only `as_slice()`, whose `.first()` is an `Option`, and every
  panicking escape is denied outside tests (A-V6). Checked:
  `Alternatives::first` is the precedent, and its doc argues that every
  consumer re-deriving non-emptiness at its call site is the defect;
  `goad`'s `draft.rs` and `driving.rs`' `answer_first_option` each use
  `.first()` with an `expect`, in test code. Options: (a) amend PHASE-12's
  surfaces to admit `Options::first` in
  `crates/goad-semantics/src/protocol/canonical.rs`, a copy of
  `Alternatives::first`'s shape and argument, with a unit test; (b) a `let
  Some(option) = … else { … }` in `goad-check`, an arm no test can reach,
  whose report line is a decision the design never took; (c)
  `.iter().take(1)`, which compiles total but silently answers nothing for
  an empty list: the same dead arm, hidden. **Recommendation: (a).** One
  small stratum-1 item, by precedent, and the checker writes no unreachable
  path. Blocks task 4's first-option box. **Resolved: (a)** (`design-log.md` 2026-10-01, *`Options::first`; R-56's condition counts `evaluate` outcomes*).
- **PLAN QUESTION 2 — VA-3's expected hits are wrong, and its pattern misses
  two routes.** Dry run (A-V10): the command prints the two `envelope`
  imports (`main`'s and `render`'s test module's), and **not** the call,
  `envelope::normalize(&bytes)`, which does not contain `ingress::`. So
  *"finds only the `envelope` import and its call"* cannot be met as
  written. Separately, `\.env\(` does not match `.envs(`, and nothing in
  the pattern sees a socket opened outside `ingress` (`std::os::unix::net`,
  or `tokio::net`, which is reachable by feature unification,
  `tokio-net-arrives-by-feature-unification`). EX-5 says *"opens no
  socket"*. **Recommendation:** expected hits restated as *"the `envelope`
  imports, and nothing else"*, the call held by reading; the pattern
  widened to `'ingress::|set_var|remove_var|env_clear|env_remove|\.envs?\(|UnixListener|UnixStream|UnixDatagram|TcpListener|TcpStream|UdpSocket'`,
  run as `command grep -rnE`. Blocks VA-3. **Resolved** (`plan-log.md` 2026-10-01, *PHASE-12 sheet questions*, Q2): the
  expected hits as recommended; the pattern widened by `\.envs?\(` and
  `UnixStream|UnixListener|TcpStream|TcpListener|UdpSocket`. The decision
  as first logged omitted this recommendation's `UnixDatagram`, a
  transcription slip; **resolved**: it joins the pattern (`plan-log.md` 2026-10-01, *PHASE-12: R-56's scope gets cases; `UnixDatagram` in VA-3*).
- **PLAN QUESTION 3 — *one literal per class* cannot hold beside EX-6.**
  VA-4 reruns PHASE-04/VA-6, which reads *"one literal per class"*. Status
  0's class (§5.2.5: *accepted*, which also covers `--help` and `--version`
  answered) already has `answer`'s `ExitCode::SUCCESS`, and EX-6 says
  *"`main` gains a literal for each"* of 0 and 1. So class 0 gets two
  literal sites. Routing `answer` through the verdict cut would couple a
  question's answer to a verdict. **Recommendation:** VA-4 reads *one
  literal per way to a status*: `answer`'s 0 (a question answered,
  SPEC-004 R-11's second clause), the cut's 0 and 1, `not_judged`'s 2; one
  `ExitCode::from(2)` still. Blocks VA-4. **Resolved** as recommended (`plan-log.md` 2026-10-01, *PHASE-12 sheet questions*,
  Q3).
- **PLAN QUESTION 4 — which exchanges the R-56 condition reads.** §5.2.2:
  *"a failure on the R-56 probe … **Only** when its `fault()` is backend
  **and** at least one of the three known-kind evaluates made no failure"*.
  The probe's evaluate can return a view, and its chain's responds can fail;
  a known kind's evaluate can succeed and its chain fail. The design does not
  say whether a chained respond's failure is *"a failure on the R-56 probe"*,
  nor whether a known kind *"made no failure"* when its chain did.
  **Recommendation:** the probe's own `evaluate` `Outcome` only, and the
  three known-kind `evaluate` `Outcome`s only; chained responds are judged as
  any other exchange. A respond answers the backend's own view, not the
  unrecognised kind, and *"evaluates"* already names the known-kind side.
  Blocks task 4's R-56 box. **Resolved** as recommended (`design-log.md` 2026-10-01, *`Options::first`; R-56's condition counts `evaluate` outcomes*).
- **PLAN QUESTION 5 — the manifest comment says *"No dev-dependencies"*.**
  The surfaces admit `Cargo.toml` *"`[dev-dependencies]` only"*. If one is
  added (task 1, reading a request log), the comment above `[dependencies]`
  becomes false, and editing it is outside *"`[dev-dependencies]` only"*.
  **Recommendation:** read the surface as admitting that comment's
  dev-dependencies sentence. Blocks task 1's log-reading box only if a
  dev-dependency is chosen. **Resolved** as recommended (`plan-log.md` 2026-10-01, *PHASE-12 sheet questions*, Q5).
- **The side clause has no planned case** (row M-16). SPEC-001 Change 3
  states it as canon (*"a failure on it whose side is the backend's"*), and
  §5.2.2's table names it. No cooperating backend fails the probe alone on a
  non-backend side, because `Spawn` fails every kind (A-V3). The sheet offers
  a self-deleting-program case. Without it, VA-1's read and review hold it.
- **EX-3 and EX-4 items no named case reads.** *"stderr verbatim"* (M-24):
  the sheet puts the assertion in the R-56 case. *"truncation flagged"*
  (M-25), *"a cleanup failure alone"* is 1 (M-21): offered cases. *"the
  values sent"* (M-26): no case; the executor may assert them in
  `a_view_answered_carries_exactly_its_options_fields`' report. EX-1's
  *"the wall clock at each step"* (M-33) is unseen by design (R3).
- **`loadable.toml`'s command, `bash ./never-spawned.sh`, is spawned once a
  run makes exchanges.** It stays safe only because its last reader,
  `a_reserved_source_event_file_exits_2`, fails before any spawn. A future
  case reusing it would run `bash` on a missing script (status 127,
  `ExitStatus`, a refusal). The task rewrites the comment to say so.
- **Test counts.** At exit the gate's sum is 715 less
  `a_run_with_no_exchange_exits_2_with_no_verdict` and
  `the_interim_end_says_nothing_was_judged_on_stderr`, plus this phase's
  cases. `goad-check`'s cases are counted once (they run under `cargo test
  --workspace` only; PHASE-04 Findings).
- **`Cargo.lock` changed, and is not on the closed surface list.** D-4's
  endorsed `[dev-dependencies]` entry adds one line, `"serde_json"`, to
  `goad-check`'s own dependency list in the lockfile; no package is added.
  Treated as part of the endorsed change rather than a STOP. Audit's call
  whether the surface list should have named it.
- **M-22's first run differed from prediction**: the side omitted from every
  refusal line left `a_refused_view_…` and `…_reported_against_r56` green,
  because the host's `Display` puts `backend` on the line whatever side it
  names (*"backend response rejected: …"*, *"backend exited with status 1"*).
  A defect in this phase's own helper, `a_line_names`, which matched the side
  anywhere on the line: fixed in 63db06f to read the side only before the id.
  The class: a substring assertion on a line that also carries a lower
  stratum's free text can be satisfied by that text.
- **M-4 reds one of its two predicted cases.**
  `a_backend_failing_at_startup_is_still_asked_the_rest` passes no event
  file, so *"the events before the host's own"* cannot change its sequence;
  `event_files_are_sent_in_the_order_given` holds M-4 alone.
- **A-V9's direction sentence is wrong** (D-10). Load moves the cleanup case
  toward a false green, not away from it. The case joins on a measured
  margin, not on direction.
- **Test names not in `design.md` §9**, shipped here, for audit (not added to
  design): `a_cleanup_failure_alone_exits_1`, `a_truncated_stderr_is_flagged`,
  `a_probe_failure_on_another_side_is_not_charged_with_r56` (binary tier),
  and D-12's five `render` unit cases.
- **`canon-delta.md` SPEC-001 Change 3's R-56 row cites neither of the scope
  cases** (`a_failure_in_the_probe_s_chain_is_not_charged_with_r56`,
  `a_known_kind_s_chain_failure_does_not_excuse_the_probe`) **nor the side
  clause's** (`a_probe_failure_on_another_side_is_not_charged_with_r56`),
  though it states the side clause as canon. VA-6 re-points paths only, so
  they were not added. Audit's call.
- **`a_probe_failure_on_another_side_is_not_charged_with_r56` writes an
  executable into the temp directory, and the checker runs it.** A
  `Text file busy` race is possible in principle if another test thread
  forks while the copy's write handle is open; `std::fs::copy` closes it
  before returning and the spawn is a grandchild's, later. Not observed.

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-10-01 · PHASE-12 done · code at 0fafad7 (*012 PHASE-12: the run cases' doc names the host's kinds rather than counting them*); this harvest in the commit after

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
- PHASE-02: in `goad_semantics::protocol::canonical` — `Stimulus` (with
  `kind`, `event`), `HOST_SOURCE`, `Finite`, `Submitted` (`to_json`, the one
  R-57 site; `as_drawn`, the one untouched-value policy) and
  `NumberRange::drawn`. In `goad`: `Edited::submitted` (a projection),
  `view_model::as_edited` (private), `view_model::as_drawn` delegating;
  `drawn_number` and `DrawnKind::Choice.first` gone. `envelope.rs` compares
  against `HOST_SOURCE`.
- PHASE-03: `goad_shell::version::version_line(version, revision)`, the one
  home of the `--version` line; `goad_shell::config::{positive_duration,
  Command::from_argv}` public, for `goad-check`; `goad-emit`'s `answer`
  (exit 2 on an unwritten `--help`/`--version`) and
  `render::answer_unwritten_line`; `exercisers/` (was `examples/`), each
  file's header pointing at `kit/`.
- PHASE-04: `crates/goad-check`, a workspace member between `goad-emit` and
  `goad-boundary`. `args::parse` (the two forms, `Invocation`, `Request`,
  `Source`, `UsageError`); `render` (`USAGE`, `NO_VIEW`,
  `NOT_YET_IMPLEMENTED`, `usage_error_line`, `startup_error_line`,
  `answer_unwritten_line`, `report_unwritten_line`); `main`'s `prepare`,
  `StartupFault`, `unjudged_end` (the interim end PHASE-12/EX-6 replaces) and
  `not_judged` (the one status 2). Binary tier `tests/binary/statuses.rs`
  with `loadable.toml` and `reserved-source.json`.
- PHASE-12: `Options::first` in `goad_semantics::protocol::canonical`.
  `goad-check`'s run: `src/run.rs` (`PROBE_KIND`, `CHAIN_BOUND`, `Planned`,
  `plan`, `answer`, `Judged`, `judged`, `charges_the_probe`, `Tally`);
  `main`'s `Prepared`, `RunFault`, `run`, `exchanges`, `follow`, `report`,
  `deliver`, `verdict` (the cut), `emit`, `now`; `render`'s
  `evaluate_line`, `respond_line`, `exchange_lines`, `probe_claim_line`,
  `chain_bound_line`, `verdict_line`, `refused_line`, `run_fault_line`. The
  interim end is gone. Binary tier: `tests/binary/process.rs` (the shared
  spawn helpers), `tests/binary/run.rs`, `scripting.rs` included, fixtures
  `first-given.json`, `second-given.json`,
  `deletes-itself-on-its-third-run.sh`. `serde_json` in `goad-check`'s
  `[dev-dependencies]`.

### Learned
<!-- Durable facts a future agent would otherwise rediscover. Candidates for
     `docs/memory/`. -->

- **A missing corpus directory does not reach a witness's vacuity guard.**
  `fixtures_of` panics on the read first. A guard's mutation must point at
  a directory that exists and is empty of fixtures (PHASE-01 mutation table,
  optional row).
- **A `todo!()` stub is a compiling red for a method, not for a `Display`
  impl.** The unused `Formatter` fails `-D unused`; `write!(f, "")` works.
  Nor for any stub whose parameter it ignores: spell it `_kind` until the
  body lands (PHASE-02).
- **A grep criterion over a deleted name also matches new names containing
  it.** EX-3's `drawn_number` grep caught the new test
  `an_as_drawn_number_…` and a comment in a file the sheet read as needing
  no edit. Run the exit grep before naming new tests (PHASE-02).
- **An "expected not to red" mutation row is a prediction, so run it.**
  PHASE-02's row 5 redded a renderer case the sheet reasoned could not
  exist.
- **A history note can trip a path grep.** Writing *"then `examples/`"* into
  a memory file put a hit in EX-2's grep. Name the old path without its
  slash, or not at all (PHASE-03).
- **I-1's grep reads unit-test fixtures in `src`.** A JSON fixture spelling
  `"source":"host"` is a hit, though it names a source, not a side. Build
  the fault, or use `HOST_SOURCE`, instead (PHASE-04, D-7).
- **A struct that carries values to a later phase is dead code now.** Bind
  them as `_name` in the function that builds them, and let the later phase
  change its return (PHASE-04, D-4).
- **The scaffold's `0 passed` `Running` line is the reach proof for a new
  test target**, before any case exists (PHASE-04, A-T4).
- **A side asserted as a substring of a refusal line can be supplied by the
  host's own text on that line.** `BackendError`'s `Display` says `backend`
  whatever side the line names; M-22 stayed green until the side was read
  only before the id (PHASE-12 Findings).
- **A margin's direction can be argued wrong from a fixture's header.** A-V9
  read `@lingers`' longer hold as the safe direction; the interval load
  stretches is the one before the timer starts, which moves toward a false
  green. Measure it (PHASE-12 D-10).
- **A mutation that removes an enum variant's only construction does not
  compile** under the workspace's `unused` deny. Keep it constructed on a
  path that never runs (`filter(|_| false)`, `if false { return Err(…) }`) —
  PHASE-12's M-1, M-30, M-34.

### Open
<!-- Still unresolved at this point. Candidates for follow-ups. -->

- **The `justfile`'s `typecheck` departs from POL-001 §Compliance** until
  audit promotes POL-001 Change 1 (PHASE-03/EX-3; PHASE-10/VA-4 checks this
  list carries it).
- **`goad-emit`'s `exchange::emit_with_stdout_full`** is a copy of `goad`'s
  `process::goad_with_stdout_full`, across crates. FU-5's class.
- **PHASE-12 Findings for audit**: `Cargo.lock`'s one-line change outside
  the surface list; test names not in `design.md` §9 (three binary-tier, five
  `render` units); `canon-delta.md` SPEC-001 Change 3's R-56 row citing
  neither scope case nor the side-clause case; A-V9's wrong direction
  sentence (the case joined on a measured margin).
- **PHASE-03 Findings for audit**: `print_usage`'s doc fragment;
  `goad-emit`'s binary-tier doc (*"Every case passes `--socket`"*); EX-8's
  parameter held by review, not a test.

- **Two `every_protocol_error` builders** (`error.rs` `mod tests`,
  `normalize.rs`). They predate the slice and cannot share across an
  integration target. Audit's call (PHASE-01 Findings).
- **PHASE-02 left three stale-but-true spellings** in imports-only files:
  `controller.rs` `dispatch`'s doc (*hard-codes `source: "host"`*),
  `glass.rs`' `DrawnKind::Choice` patterns' idle `..`, and `Finite`'s doc
  naming stratum-3 types. Audit's call (PHASE-02 Findings).

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
  manifest, so `goad-check` linking no renderer is held by
  `crates/goad-check/Cargo.toml`'s dependency comment (*"No `slint`, so a
  renderer type is `error[E0433]` here"*) and review (design.md §5.5 I-6;
  review F-26; PHASE-04/VA-5, and its row R-5 shows the crate edge refusing
  `use slint as _;`). Extend FU-7 at close.
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
  **PHASE-04 copied, into `crates/goad-check/tests/binary/statuses.rs`, from
  `crates/goad-emit/tests/binary/exchange.rs`** (another crate's target,
  which cannot be included): `check` (of `emit`), `check_with_stdout_full`
  (of `emit_with_stdout_full`), `code_of`, `stderr_of`, `stdout_of`. No
  `tests/support/` file was included: the tier spawns only `goad-check`, and
  uses no symbol of `driving.rs`, `scripting.rs` or `waiting.rs`
  (PHASE-04/VA-7).
  **PHASE-12 included `tests/support/scripting.rs` whole** in the binary
  target (`tests/binary/main.rs`), and not `driving.rs` (it composes a `Host`
  in-process, which this tier never does) or `waiting.rs` (nothing here
  polls). It moved PHASE-04's five copies, unchanged, from `statuses.rs` to
  `crates/goad-check/tests/binary/process.rs`, shared by `statuses.rs` and
  `run.rs`, and **copied nothing new**: `process.rs`' `against` and
  `assert_last_stderr_line_is_the_checker_s`, and `run.rs`' `logging`,
  `requests` and `evaluated_kinds`, are this target's own (`logging` wraps
  `scripting::logging_backend`) (PHASE-12/VA-7).

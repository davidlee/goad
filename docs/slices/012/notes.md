# Notes — Slice 012

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 | in progress | 2026-10-01 |
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
        The three tables, given their side column (`assert_row` compares
        `(id display, side)` with the row), red against `todo!()` bodies,
        then green.
  - [ ] VT-2: new `mod tests` in `goad_shell::error` with a builder per enum
        (A-V9) and `every_backend_error_names_a_requirement_and_a_side`, and
        the cleanup and state siblings, each from §5.2.3, *"expected ids
        spelled as VT-1's"*. See them red.
  - [ ] Stratum 2, in `goad_shell::error`: `requirement()`/`fault()` on
        `BackendError` (`Protocol` delegating), `CleanupFailure`,
        `StateError`, each arm naming a `Requirement` constant. VT-2 green.
  - [ ] EX-3: `grep -n 'fn requirement\|fn fault'` over
        `crates/goad-semantics/src/error.rs`, `crates/goad-shell/src/error.rs`,
        `crates/goad-shell/src/ingress/envelope.rs`; no hit in an `impl` of
        `ConfigError`, `EnvelopeFault` or `SpanFault`. Record the hits.
- **Refactor**
  - [ ] Read the diff for duplication between the witnesses and the existing
        corpus tests (the input route, `now` parsing, the walk). Docs on the
        new types cite §5.2.3 and R-59 by name, never by line.
- **Verification**
  - [ ] VA-1 (quoted): *"the §5.2.3 table and the code agree row for row, read
        side by side and recorded in the phase sheet; this is the review the
        witness's stated reach leaves (F-23). The same read holds
        `Requirement`'s constants to §5.2.3's list: none missing, none surplus
        — nothing else holds a surplus constant (§5.2.3)."* Record one line
        per row under Decisions or a VA-1 block: variant, table id/side, arm
        id/side; then the constants against §5.2.3's list. Confirm no `_` arm
        in any `requirement()`, `fault()` or `AtFault` `Display` match by
        reading each (A-V10).
  - [ ] VA-2: mutation table row 1.
  - [ ] VA-3 (quoted): *"`canon-delta.md` SPEC-001 Change 2's test names
        resolve to the shipped cases."* `grep -n` each name Change 2 cites
        (`every_protocol_error_names_a_requirement_and_a_side`, its bounds and
        schedule siblings, `every_backend_error_names_a_requirement_and_a_side`
        and its cleanup and state siblings, both witnesses) in the file Change 2
        places it. A name or file that differs is updated in `canon-delta.md` in
        the same commit and noted here (*Test names are commitments*).
  - [ ] VA-4: mutation table row 2.
  - [ ] `just check` exits 0 on the final commit. Record it.
  - [ ] §Status: PHASE-01 `done`, with the date.
  - [ ] Harvest updated in place (*Fresh as of*, Produced, Learned, Open).

**Mutation evidence** (`plan.md` *Mutation evidence*: copy the file to the
scratchpad and back, never `git checkout`/`git stash`; `--no-fail-fast`;
`git status` clean after each restore; a mutation that does not compile is not
evidence). "Cases it must red" names the plan's case first; the VT table that
also asserts the arm is expected to red with it and is listed second.

| edit | cases it must red | compiled? | redded |
|---|---|---|---|
| VA-2: `ProtocolError::requirement()`'s `NestedHints` arm R-18 → R-3 | `every_refusal_fixture_names_a_requirement_in_its_own_list`, naming `protocol/R-18-a-nested-hints-object`; also `every_protocol_error_names_a_requirement_and_a_side`. The discard witness stays green. | | |
| VA-4: `ScheduleError::requirement()`'s `NotAString` arm R-25 → R-3 | `every_discard_fixture_names_a_requirement_in_its_own_list`, its one failure naming both `protocol/R-25-next-check-of-the-wrong-type` and `schedule/R-25-not-a-string`; also the schedule sibling of `every_protocol_error_names_a_requirement_and_a_side`. The refusal witness stays green (A-T4). | | |
| optional (A-T3): a witness's corpus root pointed at a missing directory | that witness, by its non-vacuity guard | | |

**Decisions taken during execution**
<!-- Small and local: how, within what the design already settled. A choice that
     changes the design is not one of these — stop, consult the user, and record
     it in `design-log.md`. -->

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
  Not this phase's to merge; noted for audit.

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** <yyyy-mm-dd> · <phase or stage> · <commit>

### Produced
<!-- What now exists: modules, contracts, docs. -->

### Learned
<!-- Durable facts a future agent would otherwise rediscover. Candidates for
     `docs/memory/`. -->

### Open
<!-- Still unresolved at this point. Candidates for follow-ups. -->

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

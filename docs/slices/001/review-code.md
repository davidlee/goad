# Review — implementation — Slice 001

**Subject:** implementation — commit range `6489521..578bf84` on `main`: `src/`,
`tests/`, `examples/`, `Cargo.toml`, `clippy.toml`, `justfile`, root `AGENTS.md`
**Reviewer:** round 1 has two raisers — the audit agent (Claude Fable 5.1,
session 1, F-1…F-16, raised from its own evidence pass before reading any
fresh-reviewer output) and a fresh Claude subagent with no thread history
(F-17 onward, appended when its report landed)
**Opened:** 2026-09-04
**State:** resolved — round 6 (2026-09-04) returned no findings; every Outcome set, Synthesis written

Structured, append-only findings ledger for one adversarial review. Everything
needed to drive it is in this file. Narrative history — what was decided and
why, round by round — stays in the matching `-log.md`; this file holds findings
and their fate.

## Protocol

**Roles.** The **raiser** finds and states; the **responder** disposes. One agent
may hold both roles, but must switch deliberately and say which it is acting as —
disposing a finding while still wearing the raiser's hat is how a review talks
itself into `aligned`.

**Append-only.** Findings are never edited or deleted once raised, and ids
(`F-1`, `F-2`, …) are immutable across rounds. A finding raised in error is
**withdrawn**, not removed. A second round appends `F-4` onward to this same
file; it does not start a new ledger.

**Severity** — set by the raiser at raise time, not negotiated afterwards:

| | |
|---|---|
| `blocker` | Must not proceed. The only severity that gates acceptance. |
| `major` | Real defect, unsound design, or breach of canon. Recorded, does not gate. |
| `minor` | Worth fixing, survivable. |
| `nit` | Style or taste. Costs nothing to note, nothing to ignore. |

**Disposition** — set by the responder, one per finding:

| | |
|---|---|
| `aligned` | The observation is correct but nothing needs to change. Say why. |
| `fix-now` | Fix inside the current unit of work, before it closes. |
| `doc-wrong` | The artefact under review is the defect, not the thing it describes. Amend the design / plan / spec. |
| `follow-up` | Owned future work. Must land in `slice-nnn.md` Follow-ups — a disposition is not a place to put things down. |
| `tolerated` | Knowingly accepted, with a written rationale. |

**Outcome** — set by the raiser, terminal:

| | |
|---|---|
| `verified` | Disposition accepted. Done. |
| `contested` | Disagree; hands back to the responder for re-disposition. Not terminal — the finding returns to open. |
| `withdrawn` | The finding was wrong. Terminal. |

**Done** = every finding `verified` or `withdrawn`, and no `blocker` outstanding.
A ledger with no findings at all is **not** done — it means the review has not
run yet.

**Guardrails.** Do not reach for `follow-up` because the fix is large. Do not
normalise `tolerated` without a real reason. Do not downgrade a `blocker` to get
past the gate. Reject a finding on **evidence**, never on assertion. Confirm each
disposition with the user before acting on it. Fix the class, not the instance,
and do not introduce new defects repairing old ones.

## Brief

**Round 2** — 2026-09-04 — the round-1 repairs only (working tree against
`2fe3bb4`), held to the same documents. Lines of attack given to the fresh
reviewer before it looked: each Response held or not, the class fixed rather
than the instance, no new defect; the `join!` and `BrokenPipe` change on every
disposal path; the duplicate-key walk over every JSON value kind and depth;
`Object<T>` at every site the derive accepts a sequence; the `From` door as the
only classifier; the time-of-day seam; test honesty by revert.

**Round 1** — 2026-09-04 — the whole slice, held to `docs/brief.md` §3, §9,
§12–§14, `design.md` §4–§5.5 and `draft-spec.md` §4. Lines of attack, written
before the review: transport correctness (disposal on every path, the bounds,
cancellation, pipe-buffer deadlocks); normalization's refusal of ambiguity and
its handling of `null`; brief §9 against `schedule::resolve` over repeated
exchanges; the state machine on every path including a backend failure during
`respond`; config strictness and the restated duration grammar; whether each of
I1–I16 is held by its named mechanism; whether each test that carries a §7 row
can fail; anything that could live in the backend, anything narrowed to a
renderer, anything implying a sandbox; and simplicity — accretion, dead code,
comment density.

The audit agent's own findings are listed first and were **not** shaped by the
fresh reviewer's; the fresh reviewer was given the same lines of attack and no
list of prior findings, so overlap between the two is independent confirmation
rather than repetition.

## Findings

| id | severity | disposition | outcome |
|----|----------|-------------|---------|
| F-1 | major | fix-now | |
| F-2 | major | fix-now | |
| F-3 | minor | fix-now | |
| F-4 | minor | fix-now | |
| F-5 | minor | fix-now | |
| F-6 | minor | fix-now | |
| F-7 | minor | fix-now | |
| F-8 | minor | fix-now | |
| F-9 | nit | fix-now | |
| F-10 | nit | fix-now | |
| F-11 | nit | fix-now | |
| F-12 | nit | fix-now | |
| F-13 | minor | tolerated | |
| F-14 | nit | fix-now | |
| F-15 | minor | tolerated | |
| F-16 | minor | fix-now | |
| F-17 | major | fix-now | |
| F-18 | major | fix-now | |
| F-19 | minor | fix-now | |
| F-20 | minor | fix-now | |
| F-21 | minor | fix-now | |
| F-22 | minor | fix-now | |
| F-23 | minor | fix-now | |
| F-24 | minor | fix-now | |
| F-25 | minor | fix-now | |
| F-26 | minor | fix-now | |
| F-27 | minor | fix-now | |
| F-28 | minor | fix-now | |
| F-29 | nit | fix-now | |
| F-30 | nit | fix-now | |
| F-31 | nit | fix-now | |
| F-32 | nit | fix-now | |
| F-33 | nit | fix-now | |
| F-34 | major | fix-now | |
| F-35 | minor | fix-now | |
| F-36 | minor | fix-now | |
| F-37 | minor | fix-now | |
| F-38 | minor | fix-now | |
| F-39 | minor | fix-now | |
| F-40 | minor | fix-now | |
| F-41 | nit | fix-now | |
| F-42 | nit | fix-now | |
| F-43 | nit | fix-now | |
| F-44 | nit | tolerated | |

### F-1 — `resolve` keeps an elapsed check forever, so the default poll never applies after the seed

**Severity:** major
**Location:** `src/semantics/schedule.rs:146`–`:162`; `draft-spec.md` R-26; `design.md` §5.3 ("`resolved_check` is not an `Option`")

**Expected:** brief §9: "If no new `next_check` is supplied: retain an existing **valid** scheduled check if one exists; otherwise use the configured default poll interval." R-26 restates it as latest valid → previously resolved → `now + default_poll`.
**Observed:** `resolve(retained, incoming, default_poll, now)` returns `retained` whenever `incoming` is `None`, with no regard to whether `retained` has already elapsed. `Host::accept` (`host.rs:212`) always passes `Some(self.state.resolved_check())`, so the third arm is reachable only from `Host::new`. After a scheduled evaluate fires *at* the resolved instant and the backend omits `next_check`, `resolved_check` is now in the past and stays there on every subsequent such exchange. Slice 003's timer, told "next check: an instant already past", fires immediately — a busy loop until some backend response carries a `next_check`. No test covers a retained instant `<= now`; `with_no_incoming_instruction_the_retained_value_stands` (`schedule.rs:218`) uses a retained value five hours in the future.
**Evidence:** brief §9's word "valid"; the arms at `schedule.rs:152`–`:154`; `host.rs:208`–`:218`'s comment "The third is reachable only from `new`". This is the R-26 the draft would promote as canon, so it must be settled before promotion. Two readings: (a) an elapsed retained check is consumed — `resolve` treats `retained <= now` as `None` and the default applies, which also makes a backend-supplied past instant (R-28) fire once and then fall back to cadence; (b) the timer owns it and R-26 gains a sentence saying so. (a) is one comparison in a pure function with a fixture behind it; (b) pushes protocol semantics into a stratum the spec says is out of scope.

**Disposition:** fix-now — **user decision 2026-09-04, option (a).** An elapsed
retained check is consumed: `resolve` treats `retained <= now` as no retained
value, so the default poll applies. R-26 and `design.md` §5.3's "`resolved_check`
is not an `Option`" prose are reworded to match at reconciliation. Repair lands
in session 2 with a fixture for the elapsed case and a `host.rs` case across two
exchanges.
**Response:** repaired, session 2 (2026-09-04). Red first: two `schedule.rs`
resolution cases (`an_elapsed_retained_check_is_consumed_and_the_default_poll_applies`,
`a_retained_check_equal_to_now_is_consumed_too`) and one `host.rs` case across
two exchanges (`an_elapsed_check_is_consumed_and_the_default_poll_applies_from_now`:
first exchange schedules `30m`, second runs *at* that instant with no
`next_check` and must get `now + default_poll`, not the elapsed instant). All
three red at the old code. Green: `resolve`'s second arm gained the guard
`if pending.instant() > now.instant()`, and the `(None, None)` arm became
`(None, _)`. The function's doc and `host.rs`'s "reachable only from `new`"
comment rewritten to say what is now true. Not a fixture: `resolve` is VT-2's
subject and its cases are unit tests by the rationale at `schedule.rs` VT-2's
comment; the fixture corpus covers `parse`, which F-1 does not touch. `just
check` exits 0 in both columns. R-26's text and `design.md` §5.3's prose are
session 3's.

**Outcome:** `verified` (round 2)

### F-2 — `"next_check": "18:00:00"` is accepted as eighteen hours from now

**Severity:** major
**Location:** `src/semantics/schedule.rs:94`–`:107`; `tests/protocol/fixtures/schedule/R-21-bare-wall-clock-time.json`

**Expected:** brief §3.3: "Permissiveness must not mean silent guessing. Ambiguous values should fail clearly rather than acquire invented semantics." Spec P-B: "An ambiguous message MUST fail; it MUST NOT be guessed at." R-21 admits exactly two forms — an absolute instant and a relative span.
**Observed:** a bare wall-clock time fails the `jiff::Timestamp` and `civil::DateTime` parses and then parses as a `jiff::Span` of `PT18H`. A backend author writing `"18:00:00"` and meaning six this evening gets eighteen hours from now, silently and successfully. PHASE-03 measured this, could not add a variant inside its phase, and **shipped the accepted behaviour as a fixture** so the corpus documents it (`notes.md` Harvest, Open, PHASE-03). The fixture's existence means the corpus currently asserts a guess.
**Evidence:** the fixture file; `MissingOffset`'s own rationale at `design.md` §5.2 ("the single most likely backend mistake deserves a name") applies verbatim to a bare time. A fix is a `jiff::civil::Time` parse attempt before the span parse, rejecting on success; whether that is a new `ScheduleError` variant or `MissingOffset` broadened to "a wall-clock form without date and offset" is the user's call. Either way the fixture flips from `instant` to `error`.

**Disposition:** fix-now — **user decision 2026-09-04.** Rejected with a new `ScheduleError::TimeOfDay { raw }` — a `jiff::civil::Time` parse attempt before the span parse — rather than a broadened `MissingOffset`, because adding an offset to `18:00:00+10:00` still yields no instant, so "no offset" would mislead. Lands through F-4's shared span parser, so config refuses the same form. Fixture flips from `instant` to `error`.
**Response:** repaired. `schedule::parse_span` tries `jiff::civil::Time` before the span parse and refuses on success with `SpanFault::TimeOfDay`; `parse_instruction` maps it to `ScheduleError::TimeOfDay { raw }`. Fixture `R-21-bare-wall-clock-time` flipped to `{"error": "TimeOfDay"}` and its description rewritten; `runner.rs::schedule_error_name` and both `every_schedule_error` lists gained the arm. Config refuses `timeout = "09:00:00"` by the same call (`config.rs::a_duration_the_grammar_refuses_is_rejected_naming_the_key`).

**Outcome:** `verified` (round 2)

### F-3 — two host-side impossibilities are reported as the backend's fault

**Severity:** minor
**Location:** `src/shell/backend/process.rs:60`–`:63` and `:67`–`:72`

**Expected:** root `AGENTS.md`: "Every refusal is reported and says which side was wrong." R-47 governs refusals of backend-supplied values; `BackendError` is documented as "why an exchange produced no response body" from the backend.
**Observed:** a `Request` that fails to serialize — host-authored, infallible — is returned as `BackendError::Protocol(ProtocolError::Json(_))`, the variant a caller reads as "the backend's bytes did not mean anything". An empty `command` — rejected at config load, so unreachable through `Config` — is returned as `BackendError::Spawn(InvalidInput)`. Both paths exist because the lint table forbids `unwrap`. Raised at PHASE-05 and left for audit (`notes.md` Harvest, Open).
**Evidence:** the two `match`/`let … else` arms cited. D53 (as amended) built `#[expect(clippy::unwrap_used, reason = …)]` for exactly F-35's case — "a value the *host* created, where an `unwrap` is a statement about our own code" — and this is that case twice. The alternative for the second is making the empty vector unrepresentable (`ProcessBackend::new(program, args, timeout)`), which is what `Config` already splits.

**Disposition:** fix-now — **user decision 2026-09-04.** The empty command is made unrepresentable: `BackendConfig` splits into `program` and `arguments` at load, `ProcessBackend::new(program, arguments, timeout)`, and the `else` arm goes. Serialization of a host-authored `Request` moves under `#[expect(clippy::unwrap_used, reason = …)]`, the case D53 (as amended) exists for. Closes F-30.
**Response:** repaired. `shell::config::Command { program, arguments }` replaces `Vec<String>` past the config boundary; `Command::from_argv` returns `None` for the empty vector and `Config::parse` maps that to `EmptyCommand`, so `ProcessBackend` holds a `Command` and the `split_first` arm is gone. The request serialization is `serde_json::to_vec(request).unwrap()` under `#[expect(clippy::unwrap_used, reason = …)]`. The test harness's `backend`, `example`, `logging_backend`, `scripted`, `config` and `host` all take or return `Command`.

**Outcome:** `verified` (round 2)

### F-4 — the duration grammar is implemented twice

**Severity:** minor
**Location:** `src/shell/config.rs:120`–`:134` against `src/semantics/schedule.rs:94`–`:107`

**Expected:** `design.md` §5.2 Config: "one duration syntax across the product". User's standing code standard (`~/.claude/CLAUDE.md`): "No parallel implementation!"
**Observed:** `config::signed` parses a `jiff::Span` and converts it with `SpanRelativeTo::days_are_24_hours()`; `schedule::parse_instruction` does the same two calls. The duplication was a user decision at PHASE-07 (`plan-log.md` 2026-09-03), taken because `schedule.rs` was not that phase's surface and because the existing function is private, tries the absolute form first, and reports `ScheduleError`. Surfaces no longer constrain the audit, and the other two objections are answered by extracting only the span half: a `pub fn` in `semantics::schedule` returning `Result<SignedDuration, SpanFault>` with two variants (`Unparseable`, `CalendarUnit`), which `parse_instruction` maps into `ScheduleError` and `config::signed` into `ConfigError::Duration`. The "must not diverge" comment at `config.rs:110` then has nothing to guard.
**Evidence:** the two cited ranges are the same two jiff calls in the same order.

**Disposition:** fix-now — The span half is extracted as `pub fn schedule::parse_span(&str) -> Result<SignedDuration, SpanFault>`; `parse_instruction` maps it into `ScheduleError` and `config::signed` into `ConfigError::Duration`. The "must not diverge" comment at `config.rs:110` is deleted with the duplicate it guarded. Carries F-2's rejection and F-28's tests.
**Response:** repaired. `pub fn schedule::parse_span(&str) -> Result<SignedDuration, SpanFault>` is the one grammar; `SpanFault { TimeOfDay, CalendarUnit(jiff::Error), Unparseable(jiff::Error) }` lives in `semantics::error` with `Display` and `Error`. `config::signed` calls it and maps into `ConfigError::Duration { key, raw, fault }`, whose `source()` now chains the fault. The restatement comment is gone. A side finding closed with it: `config::unsigned`'s `Duration::try_from` failure was documented as "a magnitude std cannot hold", which is false — a non-negative `SignedDuration` always converts — so it now reports `NonPositive`, the only refusal that can reach it.

**Outcome:** `verified` (round 2)

### F-5 — a config file's unknown keys are ignored silently

**Severity:** minor
**Location:** `src/shell/config.rs:52`–`:67`

**Expected:** brief §3.3's permissiveness is about the *wire* — a backend written against a newer host (I10, R-4). A config file is the user's own, read once, and `design.md` §5.4 makes a malformed one "fatal at construction". Brief §5's illustrative file carries `socket` and `[logging]`, both out of scope for this slice by the OQ-4 decision.
**Observed:** `File`, `FileBackend` and `FileSchedule` carry no `deny_unknown_fields`, so a user who copies brief §5's example verbatim — the file a reader will actually copy — gets no report that `socket` and `[logging]` did nothing. A misspelled *required* key still fails; only extra keys vanish. Raised at PHASE-07, "no criterion asks for it".
**Evidence:** the three structs cited. `toml`'s error for an unknown key names the key and its line. Slice 005 adds `socket` when it implements it; until then silence is the wrong answer for a file whose author is sitting at the keyboard.

**Disposition:** fix-now — `deny_unknown_fields` on `File`, `FileBackend`, `FileSchedule`, with a test that brief §5's `socket` key is refused and the refusal names it.
**Response:** repaired. `#[serde(deny_unknown_fields)]` on `File`, `FileBackend`, `FileSchedule`; `config.rs::an_unknown_key_is_refused_and_named` asserts brief §5's `socket` and `[logging]`, and a misspelled optional key, each refused as `Syntax` with the key in the message.

**Outcome:** `verified` (round 2)

### F-6 — `Outstanding.issued_at` is written and never read

**Severity:** minor
**Location:** `src/shell/state.rs:30`–`:45`

**Expected:** user's code standard: write less code. `design.md` §5.3 names the field; nothing in the design, the spec or any criterion reads it.
**Observed:** kept under `#[expect(dead_code, reason = …)]` by user decision at PHASE-07, "dropping it would be a design change made to satisfy a lint". The design is not canon and the audit is the place to record drift; a field with no reader is a claim about a future the brief has not named (P3's second half).
**Evidence:** the `#[expect]` at `state.rs:39`. Removing it is a two-line deletion and a *Design drift* entry; keeping it is a comment that will outlive the reason.

**Disposition:** fix-now — **user decision 2026-09-04**, after checking canon: `issued_at` appears only in `design.md:1172`'s struct sketch — no prose, not in the brief, not in the draft spec. Removed with its `#[expect]`. Design-drift entry for session 3.
**Response:** repaired. `Outstanding` is `{ view_id }`; the field and its `#[expect]` are gone. Design-drift entry for session 3 (`design.md:1172`).

**Outcome:** `verified` (round 2)

### F-7 — R-34 across a backend failure during `respond` has no direct test

**Severity:** minor
**Location:** `src/shell/host.rs:177`–`:182` (`no_action` on a failed `respond` exchange); `tests/integration/host.rs`

**Expected:** R-34: rejecting a stale response must not clear the outstanding interaction — and by the same rule (`host.rs:245`–`:251`'s comment), a *failed* exchange must not either. `draft-spec.md` §7's R-34 row rests the backend-failure half on `failure_matrix.rs::one_host_survives_every_misbehaving_backend_and_still_works`, where it is incidental to the reuse witness.
**Observed:** `host.rs` asserts R-34 for a *refusal* (`a_superseded_id_is_refused_and_the_outstanding_interaction_survives`) and never for a backend failure on the `respond` path. If the one-`Host` test is ever simplified, the rule loses its only test. Raised at PHASE-10.
**Evidence:** grep of `tests/integration/host.rs` for `respond(` followed by a `failing(` script: none. One fake-backed case — view issued, `respond` meets `failing(Timeout)`, then `respond` again succeeds — closes it at no spawn cost.

**Disposition:** fix-now — One fake-backed `host.rs` case: view issued, `respond` meets `failing(Timeout)`, the outstanding interaction survives, and a second `respond` with the same id succeeds.
**Response:** repaired. `host.rs::a_backend_failure_during_respond_leaves_the_interaction_answerable`: view issued, `respond` meets `failing(Timeout)`, a second `respond` with the same id succeeds. Broken and re-run: closing the interaction on a failed exchange fails it.

**Outcome:** `verified` (round 2)

### F-8 — the example backend's types are narrower than the protocol

**Severity:** minor
**Location:** `examples/typescript/backend.ts:71`–`:73` (`body?: string`), `:85` (`kind: "text" | "boolean" | "datetime" | "number"`)

**Expected:** root `AGENTS.md` and brief §22.3: do not narrow the protocol to the current consumer. `draft-spec.md` R-16 names five field kinds including `choice`; R-19 names four content forms, three of them tagged objects. The file's own header says "Copy this file. It is meant to be edited."
**Observed:** `Field.kind` omits `"choice"`; `View.body` is typed `string` while its doc comment says "an object may tag markdown". An agent copying this file (brief §3.7's intended author) inherits a subset of the protocol as if it were the protocol — the narrowing risk R4 (`design.md` §8) names, arriving through the example rather than the renderer.
**Evidence:** the two type declarations against R-16 and R-19. The fix is either the full union types or one sentence saying the types are a subset and pointing at the spec.

**Disposition:** fix-now — `backend.ts` gains the protocol's full unions: tagged `Content`, `"choice"` among the field kinds, `min`/`max`/`options`, open hints. `deno check` in the gate holds it. Closes F-26.
**Response:** repaired. `backend.ts` declares `Content` (bare string or tagged `text`/`markdown`/`html`/`uri`), `Field` as a discriminated union over the five kinds with `min`/`max` on `number` and `options: Alternative[]` on `choice`, an open index signature for hints, and `Alternative`. The doc says the types are the whole of what the host accepts. Broken and re-run: `kind: "slider"` in the example fails `deno check`.

**Outcome:** `verified` (round 2)

### F-9 — `read_capped` can hold roughly twice the stated bound before it checks it

**Severity:** nit
**Location:** `src/shell/backend/process.rs:234`–`:243`

**Expected:** I11: every stream read from a backend is capped; `STDOUT_LIMIT` is the cap.
**Observed:** `out.reserve(READ_CHUNK)` guarantees *at least* 4 KiB of spare capacity, and `Vec` growth doubles, so `read_buf` — which fills all spare capacity — can read up to the full doubled capacity in one call before `out.len() > limit` is checked. The buffer is bounded, so I11 holds, but the bound is nearer 16 MiB than 8.
**Evidence:** `Vec::reserve` semantics plus `AsyncReadExt::read_buf` reading into `spare_capacity_mut()`. `reader.take(remaining)` or a fixed-size chunk read is the exact form.

**Disposition:** fix-now — `reader.take(limit + 1).read_to_end(&mut out)` — exact bound, fewer lines, same ownership.
**Response:** repaired. `read_capped` is `reader.take(limit + 1).read_to_end(&mut out)` then one length check; `READ_CHUNK` remains for `drain_capped` only. The stdout-flood case still passes and still sees the close.

**Outcome:** `verified` (round 3) — the repair held from round 2; the bound is pinned by F-43's test

### F-10 — the request write is not concurrent with the stdout read

**Severity:** nit
**Location:** `src/shell/backend/process.rs:173`–`:179`

**Expected:** `design.md` §5.4 step 3: "Drain stdout and stderr concurrently … reading them in sequence deadlocks whenever a backend writes more than a pipe buffer to the stream we are not yet reading."
**Observed:** `body` writes the whole request to stdin *before* reading stdout. A backend that writes more than 64 KiB to stdout before consuming a request larger than 64 KiB deadlocks both sides until `config.timeout`. Stderr is not affected (its drain runs in the `select!`). Requests are host-authored and small, and `Event.data` is the only way to make one large, so the case is unlikely; but it is bounded by the timeout rather than absent, and the design's sentence claims the class.
**Evidence:** the sequential `await`s cited; `harness::padded_evaluate` already builds the >64 KiB request that would exercise it.

**Disposition:** fix-now — Inside `body`, the write-then-close of stdin and `read_capped(stdout)` run under `tokio::join!`, so a full stdout pipe no longer blocks the request write. Closes F-23. Repaired together with F-24 because both are the same `body`.
**Response:** repaired. `body` runs `deliver(stdin, payload)` — write, then drop — under `tokio::join!` with `read_capped(stdout)`. New script `floods-stdout-then-reads-then-answers.sh` (200 000 bytes of whitespace, then reads stdin, then answers) and `transport.rs::a_backend_that_floods_stdout_before_reading_its_request_is_still_answered`, which timed out at the old code and receives the answer at the new.

**Outcome:** `verified` (round 2)

### F-11 — `Fields` and `Hints` derive `Default`, a second door past `new`

**Severity:** nit
**Location:** `src/semantics/protocol/canonical.rs:126`, `:379`

**Expected:** I1 / D30: outside `semantics::protocol` a canonical value comes only from normalization.
**Observed:** `Fields::default()` and `Hints::default()` are public and construct empty values from anywhere. Both empties are valid, so no invariant is breached — but the derive exists only to serve two test helpers (`canonical.rs:616`) inside the module, where a struct literal is available.
**Evidence:** the two derives. Harmless today; worth removing so the rule "only `new`" is true rather than nearly true.

**Disposition:** fix-now — The two `Default` derives are removed; the test helpers use `new`. Closes F-29.
**Response:** repaired. Both derives removed; `canonical.rs`'s test helper builds through `Fields::new`.

**Outcome:** `verified` (round 2)

### F-12 — `Cargo.toml` says the doc-comment lints are paused; `pedantic = "deny"` enables three of them

**Severity:** nit
**Location:** `Cargo.toml:414`–`:417`

**Expected:** a manifest comment that states what the table does.
**Observed:** `missing_errors_doc`, `missing_panics_doc` and `missing_safety_doc` are commented out under "paused alongside `missing_docs`", but all three are pedantic lints and `pedantic = { level = "deny" }` above enables them. PHASE-02 met `missing_errors_doc` as a hard error. Raised at PHASE-02, left for audit.
**Evidence:** clippy's lint groups; the four `# Errors` sections in `canonical.rs` that exist because the lint fired.

**Disposition:** fix-now — The manifest comment now says the three lints are enabled by `pedantic`.
**Response:** repaired. The comment now states that the three lints are pedantic and enabled by `pedantic = "deny"`.

**Outcome:** `verified` (round 2)

### F-13 — `src/` cites 46 review-finding ids and 61 design-decision ids, none of which survive the slice

**Severity:** minor
**Location:** every module under `src/`; `process.rs` is 86 comment lines in 274

**Expected:** brief §3.7 and §15: the repository is written for agents that read the line in front of them. `docs/AGENTS.md`: canon is evergreen; `review-*.md` and `design.md` are slice-local records. R-N ids survive promotion (AC-13); F-N and D-N do not resolve to anything outside `docs/slices/001/`.
**Observed:** `grep -oE 'F-[0-9]+' -r src | wc -l` → 46; `D[0-9]+` → 61; `design.md` by name → 36. A reader of `process.rs` meets D44, F-49, F-41, I13, R-48, D15, R-40, F-59, D34, R-43 in the first hundred lines. The reasoning is good and belongs somewhere; the question is whether the code should point at a slice folder that will be one of many. R-N citations (102) are the durable ones.
**Evidence:** the counts. Not a defect in behaviour; a maintainability question the user's simplicity standard asks the audit to raise. The cheap form: keep R-N and spec-section citations, and for F/D ids either keep them as-is (they are greppable) or reduce them to the *reason* without the id.

**Disposition:** tolerated — **user decision 2026-09-04.** The ids stay. `docs/slices/001/` is a permanent record and every id greps to its ledger entry; a sweep would trade a greppable pointer for a paraphrase and risks losing the reasoning. Convention forward, recorded for `docs/memory/` at close: new code cites `R-N` and spec sections, not slice-local `F-N`/`D-N` ids.
**Response:** no change, per disposition. Convention recorded for `docs/memory/` at close.

**Outcome:** `tolerated` — no change, by user decision

### F-14 — the domain-vocabulary scan matches substrings

**Severity:** nit
**Location:** `tests/protocol/boundary.rs:136`–`:147`, token list `:168`–`:179`

**Expected:** AC-11: no domain vocabulary in host *types or module names*.
**Observed:** the scan lowercases each line and tests `contains`, so "call sites" trips `site` (it did, at PHASE-02) and any prose containing "goals" or "reminders" will. Stricter than the AC, and the false positives land in comments. Raised at PHASE-02.
**Evidence:** `line.contains(token)` at `:139`. A word-boundary match keeps the AC's strength on identifiers and stops failing on English.

**Disposition:** fix-now — Word-boundary match on the token.
**Response:** repaired. `boundary.rs::mentions` splits a line on non-alphanumerics and at lower-to-upper case boundaries and compares each word case-insensitively; `a_token_matches_a_word_and_not_a_substring_of_one` asserts `SiteView`, `site_id`, `mod site;` caught and `call sites`, `websites`, `offsite` clean. The scan caught one of this session's own comments ("at every site") the moment it landed, which is the AC working.

**Outcome:** `superseded` by F-36 (round 2), whose repair keeps the word rule; F-45 and F-49 finished it

### F-15 — `PipeMissing` and `cleanup_only` are reachable by no test

**Severity:** minor
**Location:** `src/shell/backend/process.rs:91`–`:95`, `:209`–`:216`; `src/shell/error.rs:38`–`:40`

**Expected:** AC-6: each failure mode maps to a distinct typed error; PHASE-05/VT-3 asks for a case per variant "this phase can reach".
**Observed:** `PipeMissing` fires only when `child.stdin/stdout/stderr.take()` returns `None` after a spawn the host itself configured with all three piped, which no backend can arrange. `cleanup_only` exists only to serve it. Both are untested; raised at PHASE-05 and left for audit.
**Evidence:** tokio's `Child` populates the three handles exactly when `Stdio::piped()` was requested, which `:76`–`:78` does unconditionally. The honest options are (a) tolerate as a guard with the argument on the page — it already is, at `error.rs:38` — or (b) collapse it: `let … else { unreachable }` is forbidden by the lint table, so (a).

**Disposition:** tolerated — tokio hands back `Option`s for the three handles and populates them exactly when piped, which `process.rs` requests unconditionally. Unreachable by construction; the argument is on the page at `error.rs:38`, and `let … else { unreachable!() }` is forbidden by the lint table. Option (a) of the finding.
**Response:** no change, per disposition.

**Outcome:** `tolerated` — no change, by user decision

### F-16 — the README's config, the one a reader copies, is exercised by nothing

**Severity:** minor
**Location:** `examples/typescript/README.md:8`–`:15`

**Expected:** brief §15.2: examples should make the intended experience obvious and be complete. AC-1 is a claim about a clean clone.
**Observed:** the README's TOML uses a relative path resolved against goad's working directory; every test roots paths at `CARGO_MANIFEST_DIR`, so the file a user copies is parsed and run by nothing. `config.rs` parses `design.md`'s example instead, which differs only in the path. Raised at PHASE-08.
**Evidence:** `harness::example()` at `tests/integration/harness.rs:206`. A test that reads the README's fenced block, parses it with `Config::parse`, and runs one exchange with `current_dir` set to the crate root would make the README a fixture rather than prose.

**Disposition:** fix-now — A test reads the README's fenced TOML, parses it with `Config::parse`, and runs one exchange. cargo runs integration tests with the crate root as working directory, so the README's relative path resolves as written.
**Response:** repaired. `round_trip.rs::the_readme_s_own_config_loads_and_runs_the_example` reads the README's fenced TOML with `include_str!`, parses it with `Config::parse`, builds the host through the new `harness::host_from(config, now)` — which `harness::host` now also calls — and runs one quiet exchange.

**Outcome:** `verified` (round 2)

### Round 1, fresh reviewer — F-17…F-33

Raised by a fresh Claude subagent (general-purpose, no thread history) given the
Brief above and no list of prior findings. Its own numbering ran F-1…F-17;
renumbered here by +16 so ids stay unique and append-only. Six of its findings
independently confirm audit-agent findings and are marked so; the rest are new.
Every probe output quoted below is the reviewer's own run.

### F-17 — A retained check that has already fired is kept forever, so a backend that omits `next_check` after the first poll turns the timer into a busy loop

**Severity:** major
**Location:** `src/semantics/schedule.rs:152-154`; `src/shell/host.rs:209-218`

**Expected:** brief §9: "If no new `next_check` is supplied: retain an existing **valid** scheduled check if one exists; otherwise use the configured default poll interval." `design.md` §5.5 edge table (line 1707): "A past instant means the next wake is due, which slice 003's timer expresses by firing immediately." `config.rs` refuses `default_poll = "0s"` because "a zero poll is a busy loop" (`src/shell/error.rs:130-133`). A check that has fired is not an existing scheduled check; after it fires with no new instruction the default interval should apply.
**Observed:** `resolve` returns `retained` unconditionally whenever `incoming` is `None` (`schedule.rs:153-154`), and `Host::accept` always passes `Some(self.state.resolved_check())` as `retained` (`host.rs:212-218`), so the `(None, None)` arm — the only one that adds `default_poll` — "is reachable only from `new`" by the code's own comment (`host.rs:209-211`). Once `resolved_check` ≤ `now`, every subsequent accepted `{"view": null}` leaves it in the past.
**Evidence:** probe driving a `Host` (seeded 04:00Z, default poll 30m) with a backend that always answers `{"view":null}`:
```
now=2026-08-23T04:30:00Z  next_check=2026-08-23T04:30:00Z  (past: true)
now=2026-08-23T04:31:00Z  next_check=2026-08-23T04:30:00Z  (past: true)
now=2026-08-23T06:00:00Z  next_check=2026-08-23T04:30:00Z  (past: true)
now=2026-08-24T06:00:00Z  next_check=2026-08-23T04:30:00Z  (past: true)
```
A slice-003 timer that "fires immediately" on a past instant would spawn the backend in a tight loop for as long as it omits `next_check`, which R-21 says it MAY. No unit test in `schedule.rs` (lines 192-243) or `host.rs` exercises a retained value at or before `now`.

*Independently raises the same defect as F-1 (audit agent). Disposition follows F-1's.*

**Disposition:** fix-now — follows F-1.
**Response:** see F-1.

**Outcome:** `verified` (round 2) — with F-1

### F-18 — A bare wall-clock time is accepted as a span of hours, and the corpus pins the invented meaning as R-21 behaviour

**Severity:** major
**Location:** `src/semantics/schedule.rs:94-107`; `tests/protocol/fixtures/schedule/R-21-bare-wall-clock-time.json`; `src/shell/config.rs:126-129`

**Expected:** brief §3.3: "Permissiveness must not mean silent guessing. Ambiguous values should fail clearly rather than acquire invented semantics." `draft-spec.md` P-B; R-21: `next_check` is "either an absolute instant or a relative span"; R-25: "An invalid `next_check` MUST be discarded and reported." The fixture's own description concedes the value "is neither of the two forms next_check offers".
**Observed:** after the `Timestamp` and `civil::DateTime` parses fail, `raw.parse::<jiff::Span>()` accepts jiff's `hh:mm:ss` friendly form, so `"18:00:00"` becomes eighteen hours from `now` and is returned as `Ok` with no discard. The fixture asserts `{"instant": "2026-08-23T22:12:00Z"}` under `"requirement": ["R-21"]`, so the corpus that is to be promoted as canon documents a guess as the contract. `config.rs:126-129` uses the same grammar, so `timeout = "09:00:00"` is nine hours.
**Evidence:** probe over `schedule::parse` with `now = 2026-08-23T04:12:00Z`:
```
18:00:00        => ACCEPTED 2026-08-23T22:12:00Z
09:30:00        => ACCEPTED 2026-08-23T13:42:00Z
23:59:59        => ACCEPTED 2026-08-24T04:11:59Z
1 day 18:00:00  => ACCEPTED 2026-08-24T22:12:00Z
18:00           => ERR unparseable schedule: 18:00
```
The same shape that motivated `MissingOffset` (`design.md` §5.2: "the single most likely backend mistake") is here silently resolved instead of named. A `jiff::civil::Time` parse attempt before the span parse would refuse it.

*Independently raises the same defect as F-2 (audit agent). Disposition follows F-2's.*

**Disposition:** fix-now — follows F-2.
**Response:** see F-2.

**Outcome:** `verified` (round 2) — with F-2

### F-19 — Duplicate keys inside a view, option or field are resolved last-wins, while the same duplicate on the envelope is refused

**Severity:** minor
**Location:** `src/semantics/protocol/wire.rs:90-94` (`WireView.rest: serde_json::Value`); `wire.rs:136-152` (`#[serde(flatten)]` on `WireField`); `tests/protocol/normalize.rs:267-270`

**Expected:** P-B: "An ambiguous message MUST fail; it MUST NOT be guessed at." `draft-spec.md` §5 "Ambiguity is failure". Two `id` keys on one option is exactly an ambiguity about an identifier a response names (R-52). `serde_json`'s default for a struct is to refuse a duplicate field, which is what the envelope gets.
**Observed:** everything under `view` passes through a `serde_json::Value` (`rest`), whose map collapses duplicates before any struct is read; `WireField`'s flatten uses the buffered-content path, which also does not check duplicates. The envelope refuses; the payload guesses.
**Evidence:** probe through `serde_json::from_str::<WireResponse>` then `normalize_response`:
```
{"view":{"kind":"choice","title":"a","title":"b",…}}                      => ACCEPTED title "b"
{"view":{…"options":[{"id":"x","id":"y","label":"X"}]}}                  => ACCEPTED id "y"
{"view":{…"fields":[{"id":"f","id":"g","kind":"text","label":"L"}]}}     => ACCEPTED id "g"
{"view":{…"fields":[{…"kind":"number","min":1,"min":2}]}}                => ACCEPTED min 2.0
{"view":null,"view":{…}}                                                  => ERR malformed JSON: duplicate field `view`
{"view":null,"next_check":"1h","next_check":"2h"}                         => ERR malformed JSON: duplicate field `next_check`
```
The fixture corpus cannot state this either way: `check_protocol` deserializes from a `serde_json::Value` (`tests/protocol/normalize.rs:268`), which has already collapsed duplicates, so no fixture can carry one.

**Disposition:** fix-now — **user decision 2026-09-04.** A pre-pass over the raw bytes — a serde visitor that walks the document and refuses a duplicate key at any depth — runs before `from_slice`, in `semantics::protocol`. Pure, and the one read site calls it.
**Response:** repaired. `wire::reject_duplicate_keys(bytes)` walks the raw document with a `DeserializeSeed` visitor that carries the offending key out through a slot, and `normalize::read_response` calls it before `from_slice`; a duplicate at any depth, envelope included, is `ProtocolError::DuplicateKey { key }`. Two fixtures in the text corpus (`protocol-text/R-52-a-duplicate-key-inside-an-option`, `R-44-a-duplicate-key-on-the-envelope`), whose runner now goes through `read_response` too.

**Outcome:** `verified` (round 2)

### F-20 — JSON arrays are accepted positionally as the envelope, an option and an alternative

**Severity:** minor
**Location:** `src/semantics/protocol/wire.rs:41-60`, `:112-118`, `:161-165`

**Expected:** P-B; `draft-spec.md` §6.2 gives every message and every option as an object. A response that is an array has not said what its fields are; accepting `[null, null]` as `{"view": null}` is the host manufacturing the backend's assertion that R-11 exists to prevent.
**Observed:** the derived `Deserialize` on plain structs accepts a sequence and binds elements by declaration order. Nothing opts out, so field order in a Rust struct becomes wire syntax.
**Evidence:** probe:
```
[null,null]                                                         => ACCEPTED view=None schedule=None
[1,{"kind":"choice","title":"a","options":[{"id":"x","label":"X"}]},"1h"] => ACCEPTED view=Choice… schedule=2026-08-23T05:12:00Z
{"view":{…"options":[["x","X"]]}}                                  => ACCEPTED Opt{id:"x",label:"X"}
{…"kind":"choice","label":"L","options":[["a","A"]]…}               => ACCEPTED Alternative{id:"a",label:"A"}
```
`WireField` and `WireContent` refuse arrays only as a side effect of `flatten` and the two-pass read, so the behaviour differs by level.

**Disposition:** fix-now — An `Object<T>` newtype whose `Deserialize` requires a JSON object and then delegates, applied to the envelope, `WireOpt` and `WireAlternative` — the three sites the derive accepts a sequence at.
**Response:** repaired. `wire::Object<T>` deserializes a `Value`, refuses anything but an object naming the JSON type found, and delegates to `T`; applied to the envelope in `read_response`, to `WireChoice.options`, and to the alternative read in `normalize_alternative`. Three fixtures (`R-11-an-envelope-written-as-an-array`, `R-13-an-option-written-as-an-array`, `R-52-an-alternative-written-as-an-array`), each `Shape`.

**Outcome:** `verified` (round 2)

### F-21 — A `protocol` value of the wrong JSON type is refused as malformed JSON, not as an unsupported version

**Severity:** minor
**Location:** `src/semantics/protocol/wire.rs:43-44`; `src/semantics/protocol/normalize.rs:84-88`

**Expected:** R-3: "reject, with a distinct error naming the version found, a response declaring a `protocol` value it does not implement." `design.md` §5.2 argues for `next_check` that a loose wire type "is what makes a precise error possible"; the version field is the other value the host reads to decide something.
**Observed:** `protocol: Option<u32>` means `"1"`, `1.0`, `-1` and out-of-range integers fail inside serde and reach the caller as `ProtocolError::Json` with serde's message; only an in-range unsigned integer reaches `UnsupportedProtocolVersion`.
**Evidence:** probe:
```
{"protocol":"1","view":null}   => ERR malformed JSON: invalid type: string "1", expected u32 at line 1 column 15
{"protocol":1.0,"view":null}   => ERR malformed JSON: invalid type: floating point `1.0`, expected u32 …
{"protocol":-1,"view":null}    => ERR malformed JSON: invalid value: integer `-1`, expected u32 …
{"protocol":0,"view":null}     => ERR unsupported protocol version 0
```

**Disposition:** fix-now — **user decision 2026-09-04**: closed by F-22's classification rather than by widening `protocol` to a `Value`. `"1"`, `1.0` and `-1` become `ProtocolError::Shape`, whose message names what was found; a non-integer is not a version the host lacks, it is not a version.
**Response:** repaired through F-22: fixture `R-3-protocol-declared-as-a-string` asserts `Shape`, whose message is serde's `invalid type: string "1", expected u32`.

**Outcome:** `verified` (round 2) — with F-22

### F-22 — "Malformed JSON" and "protocol-invalid message" are one variant for every shape error the taxonomy does not name

**Severity:** minor
**Location:** `src/semantics/protocol/normalize.rs:136`, `:191-195`, `:350`, `:364`; `src/shell/host.rs:282-285`

**Expected:** R-44: "Each of these MUST map to its own distinct error: … malformed JSON; a protocol-invalid message; …". Brief §13 lists "malformed JSON" and "protocol-invalid response" as separate failure modes. AC-6 repeats the pair.
**Observed:** a well-formed document with a missing `title`, a numeric `label`, a `body` object without `value`, or an option written as a string is reported as `ProtocolError::Json`, the same variant as unparseable bytes, because each nested shape is re-read with `serde_json::from_value` and the error is wrapped as `Json`. A caller can only separate the two by calling `classify()` on the inner `serde_json::Error`, which neither the taxonomy nor `Display` exposes.
**Evidence:** probe:
```
{"view":{"kind":"choice","options":[…]}}            => ERR malformed JSON: missing field `title`
{"view":{"kind":"choice","title":5,…}}              => ERR malformed JSON: invalid type: integer `5`, expected a string
{"view":{…"body":{"kind":"text"},…}}                => ERR malformed JSON: missing field `value`
{"view":"choice"}                                   => ERR malformed JSON: invalid type: string "choice", expected struct WireView
```
`failure_matrix.rs:479-493` asserts `Json` for the garbage body, and no test asserts anything different for a well-formed but invalid document, so the collapse is untested rather than chosen.

**Disposition:** fix-now — One `ProtocolError::from(serde_json::Error)` classifying by `Category`: `Data` → new `Shape(serde_json::Error)`, the rest → `Json`. Every `map_err(ProtocolError::Json)` goes through it, so the split holds at every site rather than at the ones a test happened to reach.
**Response:** repaired. `ProtocolError::Shape(serde_json::Error)` added; `impl From<serde_json::Error> for ProtocolError` classifies `Category::Data` as `Shape` and the rest as `Json`, and every former `map_err(ProtocolError::Json)` in `normalize.rs` is now `?`. The one read site is `normalize::read_response(bytes, now)`, which `host.rs` calls and both corpora run through — `host::read` is gone. `R-15-a-misspelled-required-key` flipped to `Shape`; `R-44-a-title-that-is-not-a-string` added. `error.rs::a_serde_failure_enters_the_taxonomy_by_category` pins the door; `failure_matrix.rs`'s garbage body still asserts `Json`.

**Outcome:** `verified` (round 2)

### F-23 — The request is written to stdin before stdout is read, so a backend that emits more than a pipe buffer before consuming a request larger than a pipe buffer deadlocks until the timeout

**Severity:** minor
**Location:** `src/shell/backend/process.rs:173-179`

**Expected:** `design.md` §5.4 step 3: "Reading them in sequence deadlocks whenever a backend writes more than a pipe buffer (64 KiB on Linux) to the stream we are not yet reading, and it deadlocks only for chatty backends, which is the worst possible failure distribution." The same reasoning applies to the write: the host must not hold a blocking write while not reading stdout. R-9 makes `event.data` and `response.values` opaque, so request size is not the host's to bound.
**Observed:** `body` awaits `stdin.write_all(payload)` to completion, and only then calls `read_capped(stdout)`. Stderr is drained concurrently; stdout is not.
**Evidence:** probe with a 200 KB `event.data` against a bash backend that writes 200 KB to stdout, then reads stdin, then answers `{"view":null}` (timeout 2 s):
```
elapsed=2.004116697s result=Err(Timeout { after: 2s }) cleanup=None stderr_len=0
```
Both sides were blocked on a full pipe; the backend's valid response was never read and it was killed.

*Independently raises the same defect as F-10 (audit agent). Disposition follows F-10's.*

**Disposition:** fix-now — follows F-10.
**Response:** see F-10.

**Outcome:** `verified` (round 2) — with F-10

### F-24 — A failed stdin write is fatal even when the backend answers correctly and exits zero

**Severity:** minor
**Location:** `src/shell/backend/process.rs:173`; pinned by `tests/integration/transport.rs:180-205`

**Expected:** R-37 obliges the host to write one request and close stdin; nothing obliges the backend to read it, and AC-12 says "a bash script that ignores its request and emits a canned response is sufficient". R-40 makes the exit status the arbiter of a response. A backend that ignores stdin, writes a valid document and exits 0 has done nothing the protocol forbids.
**Observed:** `write_all(...).map_err(BackendError::Io)?` returns from `body` on `EPIPE` before stdout is read, so the response is discarded and the exchange is reported as `Io`. Whether this happens depends on request size: under 64 KiB the write lands in the pipe before the child exits; over it, the write blocks until the child exits and then fails. `a_backend_that_exits_before_reading_breaks_the_pipe` asserts the failure as the intended outcome.
**Evidence:** the test's own arrangement (a 1 MiB `data` padding, `transport.rs:189`) is what makes the case fail, and a probe of the same transport against a compiled backend that prints `{"view":null}` and never reads stdin succeeds 200/200 with a small request. The same backend is accepted or refused by the size of a payload it never looks at. Treating `BrokenPipe` on stdin as non-fatal and letting the status and body decide would close it; the deadlock in F-7 is a separate defect with a separate fix.

**Disposition:** fix-now — **user decision 2026-09-04.** `BrokenPipe` on the request write is not a failure of the exchange; the exit status and body decide, per R-40 and AC-12. Every other write error stays `Io`. `a_backend_that_exits_before_reading_breaks_the_pipe` flips to assert success. Repaired with F-10.
**Response:** repaired. In `body`, a `BrokenPipe` from `deliver` is not a failure; any other write error is `Io`. `exits-without-reading-stdin.sh` now prints `{"view":null}` before exiting, and `transport.rs::a_backend_that_answers_without_reading_its_request_is_still_answered` sends the padded request and asserts the answer — it received `Io(BrokenPipe)` at the old code.

**Outcome:** `verified` (round 2)

### F-25 — A nested `hints` object is silently absorbed as a hint named `hints`

**Severity:** minor
**Location:** `src/semantics/protocol/wire.rs:150-151`; `src/semantics/protocol/normalize.rs:237-239`

**Expected:** `design.md` §5.2 rejected accepting both a nested `hints` object and flat keys because "two spellings for one thing is exactly the ambiguity brief §3.3 says must fail rather than be guessed at", and called silently discarding presentation information "the worst available outcome". R-47: "The host MUST NOT absorb an invalid value silently."
**Observed:** the flatten collects every unmodelled key, so the spelling the design discussed and refused does not fail: `{"hints": {"multiline": true}}` becomes `Hints({"hints": {"multiline": true}})`, reported nowhere, and a renderer looking for `multiline` finds nothing.
**Evidence:** probe:
```
{…"fields":[{"id":"f","kind":"text","label":"L","hints":{"multiline":true}}]…}
   ACCEPTED … hints: Hints({"hints": Object {"multiline": Bool(true)}})
```
No fixture covers a `hints` key; `R-50-a-misspelled-optional-key-becomes-a-hint` covers `minn`, which the design named as the accepted cost, not this spelling, which it named as the refused alternative.

**Disposition:** fix-now — `normalize_field` refuses a hint named `hints` with a new `ProtocolError::NestedHints { at }`, whose message says hints are flat keys on the field.
**Response:** repaired. `normalize_field` refuses a hint named `hints` with `ProtocolError::NestedHints { at }`; fixture `R-18-a-nested-hints-object` asserts it at `view.options[0].fields[0]`.

**Outcome:** `verified` (round 2)

### F-26 — The example backend's TypeScript types admit a strict subset of the protocol

**Severity:** minor
**Location:** `examples/typescript/backend.ts:83-109`

**Expected:** brief §3.7 makes this file what an agent copies; brief §22.3 and root `AGENTS.md` ("Do not narrow wire compatibility merely because the current renderer implements only a subset"); `draft-spec.md` §6.2 gives tagged `body` objects, `choice` fields, `min`/`max`, and `options` on a field.
**Observed:** `View.body?: string` (line 92) refuses every tagged content form while its own comment says "an object may tag markdown"; `Field.kind` (line 105) omits `"choice"`; `Field` declares `multiline` as the only hint and no `min`, `max` or `options`; `Response.next_check?: string` is right but `Instant` is a bare `string` alias. A backend author extending this file gets a type error for protocol the host accepts, and `deno check` is in the gate to enforce exactly those types.
**Evidence:** lines cited; compare with `R-16-a-choice-field.json` and `R-19-a-body-tagged-as-markdown.json`, both accepted by the host.

*Independently raises the same defect as F-8 (audit agent). Disposition follows F-8's.*

**Disposition:** fix-now — follows F-8.
**Response:** see F-8.

**Outcome:** `verified` (round 2) — with F-8

### F-27 — The protocol version is two constants in two modules

**Severity:** minor
**Location:** `src/semantics/protocol/canonical.rs:535`; `src/semantics/protocol/normalize.rs:59`

**Expected:** R-1 (the version the host writes) and R-3 (the version it accepts) are one number. `design.md` §9 names restatement drift as the slice's recurring defect class; `canonical.rs:292-297` refuses three copies of a uniqueness walk for that reason.
**Observed:** `const PROTOCOL_VERSION: u32 = 1;` is declared privately in both files. A bump in one leaves the host emitting one version and refusing responses that echo it, and nothing ties the two: the R-1 tests read the emitted value and the R-3 fixture hard-codes `2`.
**Evidence:** `grep -rn PROTOCOL_VERSION src/` returns the two declarations and one use of each.

**Disposition:** fix-now — One `pub const PROTOCOL_VERSION` in `canonical.rs`; `normalize.rs` imports it.
**Response:** repaired. `pub const PROTOCOL_VERSION` in `canonical.rs`, imported by `normalize.rs`; the private copy is gone.

**Outcome:** `verified` (round 2)

### F-28 — Config's negative-duration and unparseable-duration branches have no test

**Severity:** minor
**Location:** `src/shell/config.rs:126-131`; tests at `config.rs:174-253`

**Expected:** `config.rs:117-119`: "Both `\"0s\"` and `\"-1s\"` parse … so each rejection EX-1 asks for is a check that had to be written." `design.md` §5.5: `timeout = "0s"` or `default_poll = "0s"` rejected at load; `ConfigError::Duration` exists for `"1 month"` and non-durations (`error.rs:114-127`).
**Observed:** the test module covers the example loading, a missing file, a missing section, an empty command and the two zero cases. Nothing exercises `resolved.is_negative()` or either `map_err(named)` path, so `ConfigError::Duration` is constructed by no test and `"-30m"` being refused is asserted nowhere.
**Evidence:** `grep -n '"-1s"\|Duration' src/shell/config.rs` finds the comment on line 117 and the two constructors on lines 121-129; no `#[test]` body mentions either.

**Disposition:** fix-now — Lands with F-4: `"-30m"`, `"1 month"` and prose each refused at load, asserted by name.
**Response:** repaired with F-4: `config.rs::a_duration_the_grammar_refuses_is_rejected_naming_the_key` (`1 month`, prose, a time of day) and `::a_negative_duration_is_rejected_as_non_positive`.

**Outcome:** `verified` (round 2) — with F-4

### F-29 — `Fields` and `Hints` derive `Default`, a public constructor beside the checked one

**Severity:** nit
**Location:** `src/semantics/protocol/canonical.rs:126-127`, `:379-380`

**Expected:** `canonical.rs:1-8` and I1: "Outside this module a canonical value can only have come out of `normalize_response`." `design.md` §5.2: canonical types are "constructible only via normalize".
**Observed:** `Fields::default()` and `Hints::default()` are public and construct canonical values from nothing. Both are empty and hold the invariants trivially, so the leak is of the rule rather than of a bad value; but `Fields::new` exists precisely to be the door, and `Default` is a second one the module doc says does not exist.
**Evidence:** the derives cited; `Options`, `Alternatives` and `NumberRange` do not derive it, so the asymmetry is unexplained.

*Independently raises the same defect as F-11 (audit agent). Disposition follows F-11's.*

**Disposition:** fix-now — follows F-11.
**Response:** see F-11.

**Outcome:** `verified` (round 2) — with F-11

### F-30 — Two host-side impossibilities are reported in the backend's voice

**Severity:** nit
**Location:** `src/shell/backend/process.rs:60-63`, `:67-72`; `src/shell/error.rs:192`, `:203`

**Expected:** root `AGENTS.md`: "Every refusal is reported and says which side was wrong." `error.rs:43-46` defines `Protocol` as "the bytes arrived and did not mean anything the protocol admits".
**Observed:** a host-authored request that fails to serialize is returned as `BackendError::Protocol(ProtocolError::Json(_))`, whose `Display` is "backend response rejected: malformed JSON: …"; an empty command vector is returned as `BackendError::Spawn`, whose `Display` is "backend could not be spawned: …". Both name a backend that did nothing.
**Evidence:** lines cited. Both are unreachable in practice (config refuses an empty command; `Request` serializes infallibly), which is why this is a nit rather than a defect.

*Independently raises the same defect as F-3 (audit agent). Disposition follows F-3's.*

**Disposition:** fix-now — follows F-3.
**Response:** see F-3.

**Outcome:** `verified` (round 2) — with F-3

### F-31 — The broken-pipe fixture and its test explain the kernel wrongly

**Severity:** nit
**Location:** `tests/backends/exits-without-reading-stdin.sh:4-7`; `tests/integration/transport.rs:183-185`

**Expected:** a comment that states a measured mechanism should state the right one. `write(2)` on a pipe whose read end is closed fails with `EPIPE` regardless of the write's size.
**Observed:** the comment claims "a request smaller than the pipe buffer is accepted by the kernel and sits there whether or not anyone will ever read it". The 20/20 successes it reports are because the host's write lands before `bash` has finished starting, not because the kernel accepts writes to a reader-less pipe.
**Evidence:** probe: spawn `true` with a piped stdin, sleep 300 ms, then write 13 bytes:
```
write of 13 bytes after the reader exited: Err(Os { code: 32, kind: BrokenPipe, message: "Broken pipe" })
```
The test's outcome is unaffected; the recorded reason is what is wrong, and a reader who relied on it to size a future fixture would be misled.

**Disposition:** fix-now — Rewritten with F-24, whose repair changes that test's assertion anyway.
**Response:** repaired with F-24: the script's comment and the test's doc now say that `EPIPE` depends on whether the write lands before the child exits, and that padding past the pipe buffer makes the failure certain rather than being what the kernel does.

**Outcome:** `verified` (round 2) — with F-24

### F-32 — The `option_option` expectation's reason string carries runs of embedded whitespace

**Severity:** nit
**Location:** `src/semantics/protocol/wire.rs:49-52`

**Expected:** a multi-line string literal in an attribute uses a trailing `\` to elide the newline and indentation, as `state.rs:41-42` does in the same crate.
**Observed:** the reason is one physical line whose wrapped segments were joined with their indentation intact, so the string contains fifteen-space runs mid-sentence; `cargo fmt` leaves it because attributes are not reformatted.
**Evidence:** `sed -n 51p src/semantics/protocol/wire.rs | cat -A` shows `asserts               there is nothing to show`.

**Disposition:** fix-now — Trailing `\` continuations.
**Response:** repaired. The reason is a `\`-continued literal with single spaces.

**Outcome:** `verified` (round 2)

### F-33 — `Outcome`, `Failure` and `Discarded` carry errors that implement `Display`, but none of the three does

**Severity:** nit
**Location:** `src/shell/host.rs:44-48`, `:57-84`; `src/semantics/protocol/normalize.rs:49-55`

**Expected:** brief §13 asks that the host "log enough information to debug the backend"; the crate denies `use_debug`, so the intended rendering of a refusal is `Display`, which every leaf type provides (`semantics/error.rs:95-146`, `shell/error.rs:136-221`).
**Observed:** the values a caller is actually handed — `Failure`, `Discarded`, `Outcome` — derive `Debug` only. The test harness had to write `describe_outcome` (`tests/integration/harness.rs:369-376`) to render one, and slice 002's renderer will have to do the same.
**Evidence:** the derives cited; no `impl fmt::Display` exists for any of the three (`grep -n "impl fmt::Display" src/shell/host.rs src/semantics/protocol/normalize.rs` is empty).

**Disposition:** fix-now — **user decision 2026-09-04**: `Display` for `Failure` and `Discarded`, delegating to the leaf types. `Outcome` stays `Debug`-only — rendering an outcome is the renderer's job, and a `Display` here would be a second renderer.
**Response:** repaired. `impl Display for Failure` delegates to the leaf; `impl Display for Discarded` renders `next_check {raw} discarded: {reason}`. `harness::describe_outcome` uses the former in place of its own two arms, and `host.rs::a_failure_and_a_discard_render_as_their_leaves_do` asserts both.

**Outcome:** `verified` (round 2)

### Round 2, fresh reviewer — F-34…F-44

Raised by a fresh Claude subagent (general-purpose, no thread history) against
the working tree since `2fe3bb4`, given the ledger with every round-1
Disposition and Response, the brief, the design, the draft spec, and the
diff. Method, in its own words: ran `just check` (exit 0, both columns); wrote
probe binaries under the scratchpad against the crate by path; copied the repo
to the scratchpad and reverted each repair in turn to see whether its test goes
red. Every probe output quoted is the reviewer's own run. No repository file
was edited by it.

### F-34 — A failed exchange at or after the scheduled instant keeps the elapsed check forever — F-1's class is fixed on the accept path only

**Severity:** major
**Location:** `src/shell/host.rs:267`–`:281` (`no_action`); `src/semantics/schedule.rs:189`–`:198` (`resolve`, the repaired arm)
**Expected:** brief §9: "If parsing fails, report the backend protocol error and preserve a **sensible** existing/default schedule rather than disabling the application." F-1/F-17's rationale, now in `resolve`'s doc: a retained instant at or before `now` handed to a timer that "fires immediately" (design §5.5) is a busy loop. R-29 says a failed exchange must not *alter* the resolved check.
**Observed:** `no_action` returns `self.state.resolved_check()` untouched and never calls `resolve`, so F-1's guard is bypassed on every failure path. Once the scheduled instant has arrived, a backend that fails — spawn failure, non-zero exit, timeout, malformed JSON — leaves `next_check` in the past on every subsequent failed exchange. For a spawn failure (command not found, brief §13's first item) each exchange is instantaneous, so slice 003's timer would respawn in a tight loop until the backend recovers. R-29 as written and brief §9's "sensible" pull apart here; the accept path resolved that in favour of §9 (option (a) of F-1) and the failure path did not.
**Evidence:** probe driving `Host` (seeded 04:00Z, default poll 30m) with a scripted backend — one good exchange scheduling `30m`, then `Spawn(NotFound)` on every call:
```
t=04:00 ok        next_check=2026-08-23T04:30:00Z
t=04:30 FAIL  next_check=2026-08-23T04:30:00Z  past=true
t=04:31 FAIL  next_check=2026-08-23T04:30:00Z  past=true
t=04:31 FAIL  next_check=2026-08-23T04:30:00Z  past=true   (a day later)
```
No test covers a failure at or after the resolved instant; `stderr_and_the_cleanup_verdict_survive_a_failed_exchange` and EX-5's case run before it. Two readings, as with F-1: (a) `no_action` also goes through `resolve(Some(retained), None, default_poll, now)`, so an elapsed check is consumed on failure too and R-29 is reworded to "must not accept a new instruction"; (b) R-29 stands literally and the timer owns it. (a) is one call; (b) is the reading F-1 already rejected.

**Disposition:** fix-now — **user decision 2026-09-04.** `no_action` reports `resolve(Some(retained), None, default_poll, now)` rather than the retained value bare, so an elapsed check is consumed on the failure path too; a failure *before* the instant still reports it unchanged. Host state is not written on failure — the reported instant is the caller's, the stored one moves only on acceptance — so R-29's letter about state holds and its wording gains "must not accept a new instruction" at reconciliation.
**Response:** repaired. `Host::no_action` takes `now` and reports `resolve_from(None, now)`, a new private method both the accept path and the failure path call — `schedule::resolve(Some(self.state.resolved_check()), incoming, default_poll, now)` written once. State is not written on failure. Held by `tests/integration/host.rs::a_failure_at_an_elapsed_check_reports_the_default_poll_from_now`: a good exchange scheduling `30m`, a failure *at* that instant reporting `now + 30m`, and a failure eight minutes later reporting from its own `now`. Red before the change (reported `04:42`), green after. `no_failure_moves_the_schedule` and `a_successful_exchange_does_move_the_schedule` still hold the before-the-instant case.

**Outcome:** `verified` (round 3)

### F-35 — F-20 is not closed at the content block: `"body": ["text"]` is accepted positionally

**Severity:** minor
**Location:** `src/semantics/protocol/wire.rs:302`–`:319` (`WireContent`, `WireContentValue`); `src/semantics/protocol/normalize.rs:224`–`:228` (`normalize_content`)
**Expected:** F-20's disposition: an `Object<T>` at every site the derive accepts a sequence. `Object`'s doc says `WireView` and `WireField` need no wrapper because `flatten` forces map access — true — and says nothing about the content pair, which have no `flatten`.
**Observed:** a one-element array is read twice by declaration order: `["text"]` binds `kind = "text"` in `WireContent` and then `value = "text"` in `WireContentValue`, so the body becomes `Content::Text("text")`; `["markdown"]` becomes `Content::Markdown("markdown")`. A two-element array is refused only because `serde_json`'s `Value` deserializer counts leftover elements.
**Evidence:** probe through `read_response`:
```
body array ["text"]       ACCEPTED … body: Some(Text("text")) …
body array ["markdown"]   ACCEPTED … body: Some(Markdown("markdown")) …
body array ["html","v"]   ERR protocol-invalid message: invalid length 2, expected fewer elements in array [Shape]
view as array             ERR … invalid type: sequence, expected struct WireView [Shape]
field as array            ERR … invalid type: sequence, expected struct WireField [Shape]
```
No fixture carries an array body. Fix is `Object<WireContent>` / `Object<WireContentValue>` at the two `from_value` calls, plus one fixture.

**Disposition:** fix-now — `Object<WireContent>` and `Object<WireContentValue>` at the two `from_value` calls, plus a fixture.
**Response:** repaired. `normalize_content` reads `Object<WireContent>` and `Object<WireContentValue>`; `Object`'s doc now lists the content pair among its sites. Held by fixture `R-19-a-body-written-as-an-array.json` (`"body": ["text"]` → `Shape`), red before the change (accepted as `Text("text")`).

**Outcome:** `verified` (round 3)

### F-36 — The word matcher misses plural and suffixed domain identifiers that the substring scan caught

**Severity:** minor
**Location:** `tests/protocol/boundary.rs:164`–`:184` (`mentions`, `camel_segments`); token list `:200`–`:208`
**Expected:** AC-11 on *names*: a `Habits` type or a `mod habits;` names the domain as plainly as `Habit`. F-14's repair was to stop matching English prose, not to stop matching identifiers.
**Observed:** the tokens are singular and the match is whole-word, so every plural identifier now passes, as does any suffixed one. `camel_segments` splits only at lower→upper, so an all-caps prefix hides a word: `HTTPSite` is one segment. The prior `contains` caught all of these.
**Evidence:** probe over a verbatim copy of `mentions`:
```
pub struct Habits        token=habit     mentions=false
mod habits;              token=habit     mentions=false
struct Streaks;          token=streak    mentions=false
fn goals()               token=goal      mentions=false
Reminders                token=reminder  mentions=false
pub struct HTTPSite      token=site      mentions=false
let habit2 = 1;          token=habit     mentions=false
SITE_ID                  token=site      mentions=true
pub struct SiteView      token=site      mentions=true
call sites               token=site      mentions=false
```
Cheapest repair that keeps F-14's prose fix: a word matches when it equals the token or the token plus `s`/`es`; and split camel at an upper→upper-then-lower boundary as well (`HTTPSite` → `HTTP`, `Site`). "call sites" then trips again, which is the price of the plural; if that is unwanted, restrict the plural rule to identifier-shaped lines. The test `a_token_matches_a_word_and_not_a_substring_of_one` should gain `Habits`, `mod habits;` and `HTTPSite` so the class is pinned.

**Disposition:** fix-now — **user decision 2026-09-04.** The scan strips comment text (`//`, `///`, `//!`) before matching, so prose cannot trip it; a word then matches the token or the token plus `s`/`es`; `camel_segments` also splits at an upper-upper-lower boundary. `Habits`, `mod habits;`, `HTTPSite` pinned in the test alongside the clean cases.
**Response:** repaired. `mentions` runs on `code_of(line)` — the line cut at its first `//` — then splits identifier segments and matches `is_singular_or_plural_of` (token, token+`s`, token+`es`); `camel_segments` also splits where an upper-case run ends before a lower-case letter (`HTTPSite` → `HTTP`, `Site`). The `//`-in-a-string-literal blind spot is documented on the function. Held by `boundary.rs::a_token_matches_a_word_and_not_a_substring_of_one`, now eight caught and seven clean cases including `Habits`, `mod habits;`, `Sites`, `HTTPSite`, `SITE_ID`, and `habitat`/`websites`/`offsite` plus comment-only lines as clean. Red on `Habits` before the change. `no_host_source_file_names_the_user_s_domain` still passes over `src/`.

**Outcome:** `verified` (round 3)

### F-37 — The time-of-day refusal is jiff's `civil::Time` grammar, so the same span is refused zero-padded and accepted unpadded

**Severity:** minor
**Location:** `src/semantics/schedule.rs:135`–`:143` (`parse_span`); `tests/protocol/fixtures/schedule/R-21-bare-wall-clock-time.json`
**Expected:** F-2's rule: a bare wall-clock time is refused as ambiguous. A rule whose boundary a backend author can predict, and a fixture that pins it.
**Observed:** `civil::Time` accepts only a two-digit hour, so the seam between "time of day" and "span" is the leading zero: `01:30:00` and `00:00:05` are `TimeOfDay`, `1:30:00` and `0:00:05` are spans of 1h30m and 5s; `24:00:00` is a span of 24h. Config inherits it: `timeout = "0:00:05"` loads, `"00:00:05"` is refused. The corpus pins `18:00:00` only.
**Evidence:** probe over `parse_span`:
```
18:00:00     civil::Time=true  Span=true   parse_span=Err(a time of day is not a span)
01:30:00     civil::Time=true  Span=true   parse_span=Err(a time of day is not a span)
1:30:00      civil::Time=false Span=true   parse_span=Ok(PT1H30M)
00:00:05     civil::Time=true  Span=true   parse_span=Err(a time of day is not a span)
0:00:05      civil::Time=false Span=true   parse_span=Ok(PT5S)
24:00:00     civil::Time=false Span=true   parse_span=Ok(PT24H)
1 day 18:00:00  civil::Time=false Span=true parse_span=Ok(PT42H)
18:00        civil::Time=true  Span=false  parse_span=Err(a time of day is not a span)
T18:00:00    civil::Time=true  Span=false  parse_span=Err(a time of day is not a span)
```
Nothing that was a legitimate span *and not* a time of day is lost (`PT18H`, `1h30m`, `90m`, `1 hour 30 minutes`, `1d 2h` all parse), so the user's decision holds; the seam is the question. Options: refuse the bare `H:MM:SS` colon form outright as ambiguous whatever the padding (the `1 day 18:00:00` form still parses because it is not bare), or keep jiff's line and pin `1:30:00` accepted / `01:30:00` refused as fixtures so the seam is documented rather than latent.

**Disposition:** fix-now — **user decision 2026-09-04.** Every bare colon form — a string of ASCII digits and colons containing a colon — is `TimeOfDay`, whatever the padding; the `civil::Time` parse goes, since this rule subsumes it. `1 day 18:00:00`, `90m`, `1h30m`, `PT1H30M` are unaffected. Fixture for the unpadded form.
**Response:** repaired. `parse_span` refuses `looks_like_a_time_of_day(raw)` — ASCII digits and colons only, containing a colon — and the `civil::Time` parse is gone. Held by fixture `schedule/R-21-bare-wall-clock-time-unpadded.json` (`1:30:00` → `TimeOfDay`), red before the change (parsed as `PT1H30M`). The padded fixture, `1 day 18:00:00`, `90m`, `1h30m` and the config duration tests all still hold.

**Outcome:** `verified` (round 3)

### F-38 — `"hints": null` is refused as a nested hints object, against the crate's own `null` rule

**Severity:** minor
**Location:** `src/semantics/protocol/normalize.rs:268`–`:272`; compare `:396`–`:401` (`normalize_alternative`'s exemption of `"fields": null`)
**Expected:** D50 / R-51 and design §5.5: an explicit `null` asserts nothing and is read as omission everywhere but `view`; `normalize_alternative` applies exactly that to `fields`, a key it otherwise refuses, with the comment "nothing is lost by reading it as omission". A `null` under `hints` carries no hints to lose.
**Observed:** `hints.contains_key("hints")` refuses any value, `null` included, with a message that names "a nested `hints` object" — also for a string or a number, which are not objects.
**Evidence:** probe:
```
hints null     ERR hints are the field's own keys, not a nested `hints` object, at view.options[0].fields[0] [NestedHints]
hints string   ERR hints are the field's own keys, not a nested `hints` object, at view.options[0].fields[0] [NestedHints]
fields null on alt   ACCEPTED … alternatives: [Alternative { id: "a", label: "A" }] …
```
Either `null` is exempted as it is for `fields`, or R-51's exception list grows a second entry. The first is the one line the alternative path already has.

**Disposition:** fix-now — `null` exempted, as `normalize_alternative` exempts `"fields": null` (R-51); the message says "key" rather than "object". Fixture for the nulled key, accepted.
**Response:** repaired. `normalize_field` removes the `hints` key and refuses only a non-`null` value; the message and the variant's doc say "key" rather than "object". Held by fixture `R-51-a-nulled-hints-key-on-a-field.json` (accepted, `hints: {}`), red before the change (`NestedHints`). `R-18-a-nested-hints-object.json` still refuses the object.

**Outcome:** `verified` (round 3)

### F-39 — `json_type_name` is now implemented twice in stratum 1

**Severity:** minor
**Location:** `src/semantics/protocol/wire.rs:65`–`:74`; `src/semantics/schedule.rs:21`–`:30`
**Expected:** user's standing code standard: "No parallel implementation"; design §9 names restatement drift as the slice's recurring defect class.
**Observed:** the F-20 repair added a `Value` → type-name function to `wire.rs` that differs from `schedule.rs`'s only by an article ("a boolean" vs "boolean"). Same stratum, same crate, same six arms.
**Evidence:** `grep -rn json_type_name src` → two definitions, one call each. One `pub(crate)` function in `wire.rs` (or `semantics::error`) with the article added at the one call site that wants it.

**Disposition:** fix-now — One `pub(crate) fn json_type_name` in `semantics::error`; `wire.rs`'s copy goes and its message drops the article.
**Response:** repaired. One `pub(crate) fn json_type_name` in `semantics::error`; `schedule.rs` and `wire.rs` import it. `Object`'s message reads "expected an object, found a JSON array" so the shared table's bare noun still reads. No test asserted either message; the corpus's `Shape` fixtures hold the refusal.

**Outcome:** `verified` (round 3)

### F-40 — The example's `Field` type admits keys the host refuses, so `deno check` cannot hold what its comment claims

**Severity:** minor
**Location:** `examples/typescript/backend.ts:94`–`:104`
**Expected:** F-8/F-26's Response: "The doc says the types are the whole of what the host accepts." The `Field` comment: "`min`, `max` and `options` belong to the kinds that give them meaning and are refused on any other." R-50 refuses them; F-25 refuses a nested `hints`.
**Observed:** the open index signature `[hint: string]: unknown` admits every key on every kind, so `options` on a `text` field, `min` on a `choice` field and a nested `hints` object all typecheck, and the host rejects each (`InapplicableKey`, `InapplicableKey`, `NestedHints`). Only the kind discriminant is held.
**Evidence:** `deno check` over a copy of the file with these appended:
```
const misplacedOptionsOnText: Field = { id: "f", kind: "text", label: "L", options: [{ id: "a", label: "A" }] };
const minOnChoice: Field = { id: "f", kind: "choice", label: "L", options: [{ id: "a", label: "A" }], min: 3 };
const nestedHints: Field = { id: "f", kind: "text", label: "L", hints: { multiline: true } };
→ Check backend.ts   (exit 0)
```
and with `kind: "slider"` added: `TS2322 Type '"slider"' is not assignable …` (exit 1), which confirms the Response's own probe while showing it was the only one that could fail. `min?: never; max?: never; options?: never` on the kinds that lack them, and `hints?: never` on the base, close it — the index signature stays, and `never` narrows the intersection.

**Disposition:** fix-now — `min?: never; max?: never; options?: never` on the kinds that lack them and `hints?: never` on the base, as the reviewer suggests. Break-tested with `deno check`.
**Response:** repaired. `Field`'s base carries `hints?: never` and each kind carries `never` for the modelled keys it lacks. Break-tested with `deno check` over scratch copies: `options` on `text`, `min` on `choice`, `options` on `number` and a nested `hints` object each fail with TS2322; a hint on `text`, bounds on `number` and alternatives on `choice` still typecheck, and `just check`'s `deno check` of the file itself passes.

**Outcome:** `verified` (round 3)

### F-41 — `command = [""]` passes config; the empty program is the empty command one level down

**Severity:** nit
**Location:** `src/shell/config.rs:56`–`:61` (`Command::from_argv`)
**Expected:** F-3: "the empty command is made unrepresentable". An argv whose program is `""` has nothing to spawn either.
**Observed:** `from_argv` refuses only the empty vector. `[""]` loads, and the failure surfaces at spawn as `BackendError::Spawn(ENOENT)` — R-44's "command not spawnable", so the voice is defensible, but it is a config defect reported at every exchange rather than once at load.
**Evidence:** probe: `Config::parse` with `command=[""]` → `Ok("")`.

**Disposition:** fix-now — `Command::from_argv` refuses an empty program as it refuses the empty vector, so `EmptyCommand` covers both at load.
**Response:** repaired. `Command::from_argv` filters an empty program as it filters the empty vector; `EmptyCommand`'s doc and `Display` name both spellings. Held by `config.rs::an_empty_command_is_rejected_because_there_is_nothing_to_spawn`, now over `[]`, `[""]` and `["", "./backend.ts"]`; red on `[""]` before the change.

**Outcome:** `verified` (round 3)

### F-42 — `Discarded` renders `raw` twice; `ConfigError::Duration` now renders its fault in `Display` and returns it from `source()`

**Severity:** nit
**Location:** `src/semantics/protocol/normalize.rs:61`–`:67`; `src/shell/error.rs:160`–`:165`, `:178`
**Expected:** F-33's line of attack: nothing rendered twice.
**Observed:** every `ScheduleError` already ends with `: {raw}`, and `Discarded` prefixes `next_check {raw}`. `ConfigError::Duration` embeds `{fault}` and chains it as `source()`, so a logger that walks the chain prints the fault twice. The second follows the crate's existing convention for `Json`/`Bounds`/`Schedule`, so it is consistent rather than new; noted because F-4 extended it.
**Evidence:**
```
Discarded: next_check "18:00:00" discarded: schedule is a time of day, which is neither an instant nor a span: 18:00:00
ConfigError: backend.timeout = "1 month" is not a duration this host can resolve: a calendar unit has no fixed length: using unit 'month' …
  caused by: a calendar unit has no fixed length: using unit 'month' …
```

**Disposition:** fix-now — `Discarded` renders `next_check discarded: {reason}`; the reason already ends with the raw value. The `source()` chain on `ConfigError::Duration` stays — it is the crate's convention for every wrapping variant, and a chain-walking logger printing a wrapped message twice is that convention's known cost, not this finding's.
**Response:** repaired. `Discarded` renders `next_check discarded: {reason}`; `raw` stays on the variant for a caller that wants the value. Held by `host.rs::a_failure_and_a_discard_render_as_their_leaves_do`, which now asserts the raw value appears exactly once; red before the change (twice). `ConfigError::Duration`'s `source()` chain untouched, as dispositioned.

**Outcome:** `verified` (round 3)

### F-43 — F-9's exact bound is asserted by no test

**Severity:** nit
**Location:** `src/shell/backend/process.rs:249`–`:262` (`read_capped`); `tests/integration/transport.rs` flood cases
**Expected:** a repair whose claim is "the bound is now exact" carries a test that the old code fails.
**Observed:** with `read_capped` reverted to the growing-buffer loop in the scratch copy, all 57 integration tests pass, the two flood cases included. The Response's "the stdout-flood case still passes" is true of both implementations.
**Evidence:** revert run: `F-9 revert (whole integration) :: test result: ok. 57 passed; 0 failed`. A unit test inside `process.rs` with a counting `AsyncRead` (one byte per poll, total bytes served recorded) asserting `served == limit + 1` on the refused path would pin it.

**Disposition:** fix-now — A unit test in `process.rs` with a counting `AsyncRead` that serves one byte per poll and records the total, asserting exactly `limit + 1` bytes are read on the refused path.
**Response:** repaired. `process.rs::tests::a_refused_read_takes_exactly_one_byte_past_the_bound`: a `Counting(&AtomicUsize)` `AsyncRead` that **fills every buffer it is offered** and counts, against `limit = 1000`, asserting `served == 1001` on the refused path. Confirmed by revert to the growing-buffer loop: `left: 4096, right: 1001`. A first draft served one byte per poll and passed against both implementations — a trickle cannot expose a reader that takes its spare capacity — which is why the reader fills the buffer. The counter is borrowed, not `Arc`, because `transport_shape.rs`'s share-nothing scan covers the test module too.

**Outcome:** `verified` (round 3)

### F-44 — The repairs add some twenty new slice-local `F-N` citations to `src/` under a disposition that records the convention as R-N and spec sections

**Severity:** nit
**Location:** `src/` throughout (`F-1`, `F-2`, `F-3`, `F-9`, `F-10`, `F-19`, `F-20`, `F-22`, `F-23`, `F-24`, `F-25`, `F-27`, `F-33`, …)
**Expected:** F-13's disposition: "Convention forward … new code cites `R-N` and spec sections, not slice-local `F-N`/`D-N` ids."
**Observed:** the code written in the same session cites the round-1 ids heavily. Whether "forward" starts at close or now is the responder's call; recorded so it is a decision rather than drift.
**Evidence:** `grep -oE 'F-[0-9]+' src -r | wc -l` against `2fe3bb4` and the working tree.

**Disposition:** tolerated — **user decision 2026-09-04.** This slice's code cites this slice's ledger, consistently with what was already there; the convention F-13 records applies from slice 002 and is lifted into `docs/memory/` at close.
**Response:**

**Outcome:** `tolerated` — no change, by user decision

### Round 2 — confirmed round-1 repairs

The reviewer's verification of each round-1 Response, verbatim:

- **F-1** — accept path holds: `<=` at the boundary (equal-to-now consumed); a backend-supplied past instant is stored as given and then falls back to cadence on the next omission (probe: `03:00:00Z` stored, next exchange `04:30:01Z`). Failure path is F-34. Tests red on revert (2 unit, 1 host).
- **F-2 / F-18** — `18:00:00` refused as `TimeOfDay` on the wire and in config; seam is F-37. Tests red on revert (config case and schedule corpus).
- **F-3 / F-30** — `Command { program, arguments }` everywhere past config; no `Vec<String>` command in a public type; `split_first` arm gone; `unwrap` under `#[expect]`. Empty *string* is F-41.
- **F-4** — one grammar; `Duration::try_from(SignedDuration)` succeeds for `ZERO`, `MAX` and `1s` (probe), so the `.or(Err(NonPositive))` claim is true.
- **F-5** — `deny_unknown_fields` refuses keys inside `[backend]` and `[schedule]` too (`poll_interval` case); `[schedul]` reports "unknown field `schedul`, expected `backend` or `schedule`". Test red on revert; the missing-section test still green.
- **F-6** — `Outstanding { view_id }`; no `issued_at` anywhere.
- **F-7** — test red when a failed `respond` closes the interaction (revert applied in `Host::exchange`).
- **F-8 / F-26** — `Content`, `choice`, `min`/`max`, `Alternative` present; `"slider"` fails `deno check`. Width is F-40.
- **F-9** — `take(limit+1).read_to_end`: bound exact by construction, stream dropped at the bound; behaviour of both flood cases unchanged. Testability is F-43.
- **F-10 / F-23** — `join!` cannot deadlock the host against itself: the timeout drops `body`, which drops `deliver`'s `ChildStdin` and `read_capped`'s `ChildStdout`, then disposal runs (`a_cancelled_exchange_leaves_nothing_of_the_host_behind` still green). A refused flood from a backend that traps `SIGPIPE`, never reads stdin, and got a >64 KiB request still waits for the timeout — identical to the old sequential code, so no regression. Flood test times out at 5.01 s on revert.
- **F-11 / F-29** — no `Default` derives in `canonical.rs`.
- **F-12** — `pedantic = { level = "deny" }` at `Cargo.toml:123`; comment now true.
- **F-16** — `include_str!` reads the README's fenced block; cargo documents the test cwd as the package root (`cargo help test`, "Working directory of tests"); test red with the README path broken.
- **F-19** — every JSON value kind serde_json can emit without `arbitrary_precision` is visited (feature not enabled: `cargo tree -e features -i serde_json` shows `default`, `std` only); 5000-deep nesting → `Json` (recursion limit), no stack overflow; escaped-key duplicates caught; envelope, option, hints and nested-array duplicates all `DuplicateKey`; syntax/EOF/invalid-UTF-8 from the pre-pass classify as `Json`, matching `from_slice`. Corpus test red on revert.
- **F-20** — envelope, option and alternative arrays refused as `Shape` with "expected an object, found an array"; `Object`'s `custom` error classifies as `Data` → `Shape`. Content block is F-35. Corpus red on revert.
- **F-21 / F-22** — `"1"`, `1.0`, `-1`, `4294967296` → `Shape`; `2` → `UnsupportedProtocolVersion`; `null` → omission; no `ProtocolError::Json(` construction remains outside the door (`grep`); door test and corpus red on revert.
- **F-24 / F-31** — `EPIPE` tolerated, other write errors `Io`; script and doc now describe the kernel correctly; test red on revert.
- **F-25** — nested object refused with the path; `null` is F-38. Corpus red on revert.
- **F-27** — one `pub const PROTOCOL_VERSION`, one import.
- **F-28** — both tests red on revert (grammar cases; negative → `NonPositive`).
- **F-32** — continuation literal, single spaces.
- **F-33** — `Failure` delegates to the leaf; `Discarded` renders; `Outcome` `Debug`-only as decided. Double rendering is F-42.

Not verified by the reviewer: F-13 and F-15 (tolerated, no change); F-14 (superseded by F-36); the R-26 / design §5.3 rewording (session 3).

### Round 3, fresh reviewer — F-45…F-51

Raised by a fresh Claude subagent (general-purpose, no thread history) against `3ee96f9`, the round-2 repairs only, given the ledger, the brief, the design, the draft spec and the diff. Method, in its own words: Ran `just check` on `3ee96f9`: exit 0. Default column 42 lib + 58 integration + 16 protocol tests; `--no-default-features` column 25 lib + 16 protocol; `deno check`, clippy in both columns and `cargo fmt --check` all clean. Copied the repo (minus `target`, `.git`) to `scratchpad/r3/repo` and worked only there: probe tests appended to the protocol and integration targets and to `process.rs`, a compiled-out `use crate::shell::host::Host;` in `src/semantics/mod.rs` for the scanner, five `deno check` scratch copies of `backend.ts`, and a script that reverts each round-2 repair in turn, runs the test its Response names, and restores the file from the working tree. Every output quoted below is my own run. No repository file was edited.

### F-45 — The stratum-direction scan cannot match `crate::shell` or `crate::bin`, so two of its three tokens are vacuous

**Severity:** minor
**Location:** `tests/protocol/boundary.rs:166`–`:171` (`mentions`), `:225`–`:229` (`STRATUM_1_LOOKS_ONLY_DOWN`)
**Expected:** the scan's own doc: "AC-15's direction half … this catches the `use crate::shell::…` a feature flag cannot see." F-14's disposition was a word-boundary match to stop matching prose, not to blind the stratum scan; F-36 rewrote `mentions` and its test and left this standing.
**Observed:** words are produced by splitting on every non-alphanumeric byte, so no word can ever equal a token containing `::`. `tokio` still matches. The vacuity predates the round-2 repair — `ad811c6`'s matcher has the same property, and `2fe3bb4`'s `contains` did not — and round 2 recorded F-14 as "not verified". The build gate (`--no-default-features`) still holds the underlying property, which is why this is minor and not major.
**Evidence:** probe over a verbatim copy of `mentions`, and the real test with a `use` line in stratum 1 (under `#[cfg(any())]` so it compiles):
```
F36-vacuity use crate::shell::host::Host;        token=crate::shell  mentions=false
F36-vacuity use crate::bin::x;                   token=crate::bin    mentions=false
F36-vacuity use tokio::io;                       token=tokio         mentions=true
### boundary vacuity: stratum scan with a real 'use crate::shell' line in src/semantics/mod.rs
10:use crate::shell::host::Host;
test boundary::stratum_1_names_neither_the_shell_a_binary_nor_the_runtime ... ok
```
Cheapest repair: a path token (one containing `::`) matches by `contains` on `code_of(line)`, a word token by the word rule; `a_token_matches_a_word_and_not_a_substring_of_one` gains `use crate::shell::host::Host;` as a caught case. There is no `crate::bin` module in `src/`, so that token has never had anything to catch.

**Disposition:** fix-now — **user decision 2026-09-04.** A token containing `::` is a path and matches by `contains` on the comment-cut line; a word token keeps the word rule. `use crate::shell::host::Host;` pinned as caught. `crate::bin` stays as a guard against a future binary module.
**Response:** repaired. `mentions` matches a token containing `::` by `contains` on `code_of(line)`; word tokens keep the word rule. Held by `boundary.rs::a_token_matches_a_word_and_not_a_substring_of_one`, which now catches `use crate::shell::host::Host;` and `crate::shell::config::Command::new(x)` for `crate::shell` and keeps `// see crate::shell …` clean; red on the `use` line before the change. `stratum_1_names_neither_the_shell_a_binary_nor_the_runtime` still green over `src/semantics`.

**Outcome:** `verified` (round 4)

### F-46 — F-37's shape rule does not subsume `civil::Time`: fractional-second wall-clock forms F-2 refused now parse as spans

**Severity:** minor
**Location:** `src/semantics/schedule.rs:126`–`:143` (`parse_span`, `looks_like_a_time_of_day`)
**Expected:** F-37's disposition: "the `civil::Time` parse goes, since this rule subsumes it." F-2: a bare wall-clock time is refused as ambiguous, and the function's own doc: "a seam … is one no backend author could predict."
**Observed:** `civil::Time` accepts fractional seconds and the shape rule (digits and colons only) does not, so every such form was `TimeOfDay` before the repair and is a span after it. The seam has moved rather than gone: it is now on a fractional tail and on a sign (`+1:30:00` is a span, `1:30:00` is not). The config grammar inherits it (`timeout = "18:00:00.000"` loads as eighteen hours). No fixture pins any of these.
**Evidence:** probe over `parse_span` on `3ee96f9`:
```
F37frac 18:00:00.5             civil::Time=true  parse_span=Ok(PT18H0.5S)
F37frac 18:00:00.000           civil::Time=true  parse_span=Ok(PT18H)
F37frac 18:00:00.123456789     civil::Time=true  parse_span=Ok(PT18H0.123456789S)
F37frac 06:30:00.5             civil::Time=true  parse_span=Ok(PT6H30M0.5S)
F37frac 18:00:00,5             civil::Time=true  parse_span=Ok(PT18H0.5S)
F37 -1:30:00               civil::Time=false parse_span=Ok(-PT1H30M)
F37 +1:30:00               civil::Time=false parse_span=Ok(PT1H30M)
F37 1:30:00                civil::Time=false parse_span=Err(TimeOfDay)
```
Either the `civil::Time` parse comes back beside the shape rule (`||`, one line), or the shape admits an optional sign and a `.`/`,` fraction after the last group; a fixture for `18:00:00.000`, refused, pins whichever is chosen.

**Disposition:** fix-now — **user decision 2026-09-04.** With F-50: `TimeOfDay` is the shape `digits(:digits)+` with an optional `.`/`,` fraction, every group non-empty, **or** whatever `civil::Time` parses — the two rules together, since neither subsumes the other. Fixtures: `18:00:00.000` refused as `TimeOfDay`, `T18:00:00` refused as `TimeOfDay`.
**Response:** repaired, with F-50. `looks_like_a_time_of_day` is `has_the_shape_of_a_time_of_day(raw) || raw.parse::<jiff::civil::Time>().is_ok()`; the shape is two or more non-empty digit groups separated by `:` with an optional `.`/`,` digit fraction. Held by fixtures `schedule/R-21-wall-clock-time-with-a-fraction.json` (`18:00:00.000` → `TimeOfDay`) and `R-21-wall-clock-time-with-the-iso-designator.json` (`T18:00:00` → `TimeOfDay`), both red before the change. Probed after: `1:30:00.5`, `18:00:00,5`, `24:00:00` → `TimeOfDay`; `-1:30:00`, `+1:30:00` → signed spans, since a sign is not something a time of day carries; `1 day 18:00:00`, `PT1H30M`, `90m`, `1.5h` parse.

**Outcome:** `verified` (round 4)

### F-47 — F-42 dropped the raw value from the rendered discard for `NotAString`, the one variant whose reason does not carry it

**Severity:** minor
**Location:** `src/semantics/protocol/normalize.rs:63`–`:68` (`Discarded`'s `Display`); `src/semantics/error.rs:119`, `:197` (`NotAString`)
**Expected:** F-42's disposition rests on "the reason already ends with the raw value". That is true of the five `raw: String` variants and false of `NotAString { found: &'static str }`, which by design names only the type. R-47 and brief §13 ("log enough information to debug the backend"): before the repair the line read `next_check 45 discarded: …`.
**Observed:** a wrong-typed `next_check` now renders with no trace of what was sent. `Discarded` still carries `raw`, so the value is not lost, only unrendered; `a_failure_and_a_discard_render_as_their_leaves_do` asserts only the calendar-unit case, where the reason does carry it.
**Evidence:**
```
F42 next_check=45                       discard renders: next_check discarded: schedule must be a string, found number
F42 next_check=true                     discard renders: next_check discarded: schedule must be a string, found boolean
F42 next_check=[1]                      discard renders: next_check discarded: schedule must be a string, found array
F42 next_check={"a":1}                  discard renders: next_check discarded: schedule must be a string, found object
F42 next_check="1 month"                discard renders: next_check discarded: schedule uses a calendar unit, which has no fixed length: 1 month
```
One arm: render `raw` when `reason` is `NotAString` (or let `NotAString` carry the value and keep `Discarded` as it is). The existing test should gain the `45` case asserting the value appears exactly once.

**Disposition:** fix-now — **user decision 2026-09-04.** `Discarded`'s `Display` renders `raw` when the reason is `NotAString`, the one variant whose message does not carry it; the other five stay as F-42 left them. The rendering test gains the `45` case asserting the value appears exactly once.
**Response:** repaired. `Discarded`'s `Display` has one extra arm: a `NotAString` reason renders `next_check {raw} discarded: {reason}`; every other reason renders as F-42 left it. Held by `host.rs::a_failure_and_a_discard_render_as_their_leaves_do`, whose `next_check: 45` case asserts `45` appears exactly once; red before the change (zero).

**Outcome:** `verified` (round 4)

### F-48 — After F-34 the stored check no longer tracks what the host reported, so a no-instruction success after an elapsed-instant failure resolves from a stale value and every no-action call drifts the reported wake forward

**Severity:** minor
**Location:** `src/shell/host.rs:258`–`:265` (`resolve_from`), `:282`–`:297` (`no_action`); `src/shell/state.rs:51`–`:54` (`resolved_check`'s doc)
**Expected:** brief §9: "retain an existing valid scheduled check if one exists"; design §5.5: a broken backend "still gets polled on its existing cadence"; `State::resolved_check`'s doc: "The instant the host will next ask the backend for something." F-34's disposition chose not to write state on failure; this records what that choice does downstream.
**Observed:** once the retained instant has elapsed, a failed exchange reports `now + default_poll` but leaves `resolved_check` at the elapsed instant. Two consequences. (1) The next *successful* exchange that omits `next_check` resolves against the elapsed value, consumes it, and reports its own `now + default_poll` — not the instant the caller was already told to wake at. (2) Every further no-action call (a failure, or a stale `respond` that never touches the backend) reports a fresh `now + default_poll`, so a caller that re-arms on each `Outcome` has its routine check pushed later by each one. Had state been written on failure, the retained `05:12` would have stood in both cases. Nothing before the instant changed: a failure and a stale `respond` before it still report the retained value unchanged. `resolved_check`'s doc is now false after such a failure.
**Evidence:** probe driving `Host` (seeded `04:12`, `default_poll = 30m`):
```
F34 t=04:12 ok(30m)            reported=Timestamp(2026-08-23T04:42:00Z)
F34 t=04:42 FAIL                reported=Timestamp(2026-08-23T05:12:00Z)
F34 t=04:50 ok(no next_check)   reported=Timestamp(2026-08-23T05:20:00Z)  failure=false
F34 t=04:55 FAIL                reported=Timestamp(2026-08-23T05:20:00Z)
F34b t=04:12 ok(view,30m)       reported=Timestamp(2026-08-23T04:42:00Z)
F34b t=04:20 FAIL (before)      reported=Timestamp(2026-08-23T04:42:00Z)
F34b t=04:30 stale respond      reported=Timestamp(2026-08-23T04:42:00Z) calls=2
F34b t=04:42 stale respond (at) reported=Timestamp(2026-08-23T05:12:00Z) calls=2
F34b t=04:50 stale respond      reported=Timestamp(2026-08-23T05:20:00Z) calls=2
```
Two readings: (a) `no_action` also calls `state.resolve_to` with what it reports, so stored and reported agree and R-29 is reworded exactly as the disposition already plans ("must not accept a new instruction"); (b) the drift is accepted and `resolved_check`'s doc says it is the last *accepted* resolution rather than the next wake. (a) is one line and no new test shape; (b) is a doc change. Either way `a_failure_at_an_elapsed_check_reports_the_default_poll_from_now` should gain the 04:50 success step so the choice is pinned.

**Disposition:** fix-now, reading (a) — **user decision 2026-09-04.** `no_action` writes what it reports, so stored and reported never differ; R-29 is reworded at reconciliation to "must not accept a new instruction", as F-34's disposition already planned. The test gains the later failure and the no-instruction success, both reporting the retained `05:12`.
**Response:** repaired, reading (a). `no_action` takes `&mut self`, resolves through `resolve_from(None, now)` and writes the result with `state.resolve_to` before reporting it, so stored and reported never differ; its doc says so and says why. Held by `host.rs::a_failure_at_an_elapsed_check_reports_the_default_poll_from_now`, which now continues: a failure at `04:50` and a no-instruction success at `04:50` both report the `05:12` the caller was told; red on the second failure before the change (`05:20`). R-29's rewording is on session 3's reconciliation list.

**Outcome:** `verified` (round 4)

### F-49 — F-36 leftovers: block comments are not cut, and digit-glued or all-caps compounds still pass

**Severity:** nit
**Location:** `tests/protocol/boundary.rs:155`–`:176` (`mentions`, `code_of`), `:193`–`:213` (`camel_segments`)
**Expected:** the doc: "Comment text is cut off first"; F-36's own evidence listed `let habit2 = 1;` as a miss and the disposition did not address it.
**Observed:** only `//` is cut, so `/* … */` prose trips the scan; digits are alphanumeric so `habit2` is one word; an all-caps compound has no case boundary to split on. Nothing in `src/` hits any of these today (no block comment, no `//` inside a string literal — checked by grep — so the documented blind spot is latent). The plural rule also admits non-words (`habites`, `Sitees`) and misses `Habitss`, which is harmless.
**Evidence:**
```
F36 /* a habit */ let x = 1;                         token=habit       mentions=true
F36 let x = 1; /* call sites */                      token=site        mentions=true
F36 habit2                                           token=habit       mentions=false
F36 habits2                                          token=habit       mentions=false
F36 SITEID                                           token=site        mentions=false
F36 HTTPSITE                                         token=site        mentions=false
F36 SITEs                                            token=site        mentions=false
F36 let s = "http://x"; let site_id = 1;             token=site        mentions=false
F36 camel … SITEs=["SIT", "Es"] … Site1Habit=["Site1Habit"]
```
Splitting at a letter/digit boundary closes the digit case for one extra arm; the rest is documentation or accepted.

**Disposition:** fix-now for the digit boundary only — **user decision 2026-09-04.** `camel_segments` also splits between a letter and a digit, so `habit2` is caught. Block comments, all-caps compounds and the `//`-in-a-string blind spot are tolerated as documented and latent (nothing in `src/` has any of them).
**Response:** repaired for the digit boundary. `camel_segments` splits where a digit follows a letter or a letter follows a digit, so `habit2` → `habit`, `2`. Held by the same boundary test's `let habit2 = 1;` case, red before the change. Block comments, all-caps compounds and the string-literal blind spot are named in `mentions`'s doc as accepted and latent.

**Outcome:** `verified` (round 4)

### F-50 — The shape rule reports strings that are not times as "a time of day"

**Severity:** nit
**Location:** `src/semantics/schedule.rs:140`–`:142`
**Expected:** brief §13: a diagnostic a backend author can act on. F-2's rule is about `hh:mm[:ss]`; `ScheduleError::TimeOfDay`'s doc gives `"18:00:00"`.
**Observed:** any string of digits and colons with a colon is `TimeOfDay`, including ones with no digits at all. These were `Unparseable` before the repair.
**Evidence:**
```
F37 ::                     civil::Time=false parse_span=Err(TimeOfDay)
F37 :                      civil::Time=false parse_span=Err(TimeOfDay)
F37 123:                   civil::Time=false parse_span=Err(TimeOfDay)
F37 :30                    civil::Time=false parse_span=Err(TimeOfDay)
F37 1::30                  civil::Time=false parse_span=Err(TimeOfDay)
```
rendered as `schedule is a time of day, which is neither an instant nor a span: ::`. Requiring `\d+(:\d+)+` — every group non-empty — keeps the disposition's rule and the message honest.

**Disposition:** fix-now — **user decision 2026-09-04.** Closed by F-46's rule: every colon-separated group must be non-empty digits, so `::`, `:`, `123:` and `:30` are `Unparseable` again. Fixture for `::`.
**Response:** repaired with F-46: every group must be non-empty digits, so `::`, `:`, `123:`, `:30`, `1::30` fall through to the span parse and are `Unparseable` with jiff's own message. Held by fixture `schedule/R-25-colons-with-nothing-between-them.json` (`::` → `Unparseable`), red before the change (`TimeOfDay`).

**Outcome:** `verified` (round 4)

### F-51 — The example's `never` members refuse `null`s the host reads as omission, including the `hints: null` F-38 accepted in the same commit

**Severity:** nit
**Location:** `examples/typescript/backend.ts:105`–`:115`
**Expected:** the file's own claim: "The types below are the whole of what the host accepts, not the subset this file happens to use." R-51: an explicit `null` on a modelled key is omission; fixtures `R-51-a-nulled-hints-key-on-a-field` and `R-51-a-nulled-modelled-key-on-a-field` accept `hints: null` and `min: null`.
**Observed:** `hints?: never` and `min?: never` admit only `undefined`, so a backend author writing what the host accepts is refused by `deno check`. This follows the pre-existing `next_check?: string`, which also refuses `null`, so it is a consistency note rather than new drift.
**Evidence:**
```
== hints_null      TS2322 [ERROR]: Type 'null' is not assignable to type 'undefined'.   exit=1
== min_null_text   TS2322 [ERROR]: Type 'null' is not assignable to type 'number | undefined'.   exit=1
== options_undef   exit=0
```
Either `?: never | null` where the host reads `null` as omission, or one sentence in the comment saying the types are stricter than the host about `null` on purpose.

**Disposition:** fix-now as documentation — **user decision 2026-09-04.** One sentence in the example's comment: the types are stricter than the host about `null` on purpose — the host reads a nulled modelled key as omission, and a backend written from these types simply omits it.
**Response:** repaired as documentation. `Field`'s comment in `examples/typescript/backend.ts` says the `never` members are stricter than the host about `null` on purpose, and that a backend written from these types omits the key instead. `deno check` passes.

**Outcome:** `verified` (round 4)

### Round 3 — confirmed round-2 repairs

Each Response's claim, checked by reverting the repair in the scratch copy and running the test it names. Results verbatim.

- **F-34** — repair as dispositioned: `no_action` takes `now` and reports `resolve_from(None, now)`; state unwritten on failure (`resolve_to` is called only in `accept`). Revert (`next_check: self.state.resolved_check()`): `test host::a_failure_at_an_elapsed_check_reports_the_default_poll_from_now ... FAILED — left: Timestamp(2026-08-23T04:42:00Z) right: Timestamp(2026-08-23T05:12:00Z)`. Before-the-instant behaviour unchanged (probe: 04:20 failure and 04:30 stale respond both report `04:42`). The stale-`respond` path reporting a resolved instant is consistent with the disposition; its downstream drift is F-48.
- **F-35** — `Object<WireContent>` and `Object<WireContentValue>` at both reads. Probe: `["text"]`, `["markdown","x"]`, `1`, `true` → `Shape` "expected an object, found a JSON array/number/boolean"; `{"kind":["text"],"value":"x"}` → `Shape`; `{"kind":"text","value":"x"}` accepted. Revert: `every_protocol_fixture_states_what_a_wire_document_means ... FAILED` naming `R-19-a-body-written-as-an-array.json`.
- **F-36** — comment cut, plural, acronym split all present. Full revert to `ad811c6`'s matcher: `FAILED — "pub struct Habits" names the domain and was missed`. Partial reverts: plural only → same; acronym split only → `"pub struct HTTPSite" … was missed`; comment cut only → `"// the call sites" does not name the domain and was caught`. So each of the three parts is pinned by its own case. `no_host_source_file_names_the_user_s_domain` green. The `//`-in-a-string blind spot is real (four probe lines hidden) and latent (grep finds no such literal in `src/`). Leftovers are F-49; the stratum scan's vacuity is F-45.
- **F-37** — `civil::Time` parse gone, shape rule in. Probe: `1:30`, `1:30:00`, `01:30:00`, `18:00`, `00:00:05`, `24:00:00`, `0:0` → `TimeOfDay`; `1 day 18:00:00` → `PT42H`; `90m`, `1h30m`, `PT1H`, `PT1H30M`, `1 hour 30 minutes` → parse; `1h:30m`, ` 1:30`, `1.5:00`, `T18:00:00`, `٣:٣٠` → `Unparseable`. Revert: `every_scheduling_fixture_states_what_the_protocol_does ... FAILED` naming `R-21-bare-wall-clock-time-unpadded.json`. Fractional/signed seam is F-46; non-time strings named as times is F-50.
- **F-38** — `hints: null` accepted with `hints: {}`; `"x"`, `1`, `[]`, `false`, `{}`, `{"multiline":true}` refused as `NestedHints … at view.options[0].fields[0]`, and on a second field `… fields[1]`; message says "key". Revert (`contains_key`): `every_protocol_fixture_states_what_a_wire_document_means ... FAILED` naming `R-51-a-nulled-hints-key-on-a-field.json`.
- **F-39** — `grep -rn json_type_name src`: one definition (`semantics/error.rs:18`), two imports, two calls. `Object`'s message reads "expected an object, found a JSON array".
- **F-40** — `deno check`: the four negatives each `TS2322` (exit 1); the positives (a hint on `text`, bounds on `number`, `max` alone, alternatives on `choice`, `options: undefined`) exit 0. `null` handling is F-51.
- **F-41** — Revert (`argv.next()?`): `shell::config::tests::an_empty_command_is_rejected_because_there_is_nothing_to_spawn ... FAILED` (panicked at the `[""]` case). `EmptyCommand` renders "names no program".
- **F-42** — Revert (`next_check {raw} discarded: {reason}`): `host::a_failure_and_a_discard_render_as_their_leaves_do ... FAILED — left: 2 right: 1`. `NotAString`'s lost value is F-47.
- **F-43** — Assumptions checked against tokio 1.53.1 source: `Take::poll_read` clamps every buffer to its remaining limit (`buf.take(limit_)`), and `read_to_end` grows its offer adaptively — measured offers on the refused path `[32, 32, 64, 128, 256, 489]`, total 1001; with `limit = 100000`, `[32, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, …]`, total 100001. The assertion therefore holds by `Take`'s clamp and is independent of how much `read_to_end` offers. Revert to the `2fe3bb4` growing loop: `a_refused_read_takes_exactly_one_byte_past_the_bound ... FAILED — left: 4096 right: 1001`.

Not verified: F-44 (tolerated, no change).

### Round 4, fresh reviewer — F-52…F-54

Raised by a fresh Claude subagent (general-purpose, no thread history) against `83d1b77`, the round-3 repairs only, given the ledger, the brief, the design, the draft spec and the diff. Method, in its own words: Ran `just check` on `83d1b77`: exit 0 (default column 42 lib + 58 integration + 16 protocol; `--no-default-features` 25 lib + 16 protocol; `deno check`, clippy in both columns, `cargo fmt --check` clean). Copied the repo minus `target` and `.git` to `scratchpad/r4/repo` and worked only there: a probe test in the protocol target over `parse_span` and `read_response`, one in the integration target over `Config::parse` and a stale-`respond` sequence against `Host`, a probe appended to `boundary.rs` over `mentions` and `camel_segments`, and `revert.sh`, which reverts each round-3 repair in turn, runs the test its Response names, and restores the file from the working tree (`cmp` clean afterwards). jiff is 0.2.35. Every output quoted below is my own run. No repository file was edited.

### F-52 — Trailing whitespace defeats the time-of-day refusal: `"18:00:00 "` is an eighteen-hour span on the wire and in config

**Severity:** minor
**Location:** `src/semantics/schedule.rs:126`–`:134` (`parse_span`), `:145`–`:163` (`looks_like_a_time_of_day`, `has_the_shape_of_a_time_of_day`, `all_digits`)
**Expected:** F-2: a bare wall-clock time is refused as ambiguous rather than read as hours; F-37 and F-46's own standard, "a seam … is one no backend author could predict"; `parse_span`'s doc: jiff "would read `\"18:00:00\"` as eighteen hours, and a backend author who wrote it almost certainly meant six this evening".
**Observed:** both rules see the raw string and both fail on a trailing space or tab — the last group `"00 "` is not all digits, and `civil::Time` refuses it — but jiff's span grammar tolerates trailing whitespace, so the value falls through to the parse F-2 exists to keep it from. Leading whitespace is refused by jiff too, so the hole is one-sided. Pre-existing under every rule since F-2's repair (the `civil::Time` rule and the round-2 shape rule both fail on the same byte); round 3 probed leading whitespace only. The config grammar inherits it.
**Evidence:**
```
R4span 18:00:00               civil=false parse_span=Ok(18h)   wire=Ok(schedule=Some("2026-08-23T22:12:00Z"))   ← "18:00:00 " (trailing space)
R4span 18:00:00	              civil=false parse_span=Ok(18h)   wire=Ok(schedule=Some("2026-08-23T22:12:00Z"))   ← "18:00:00\t"
R4span 1:30:00                civil=false parse_span=Ok(1h 30m) wire=Ok(schedule=Some("2026-08-23T05:42:00Z"))  ← "1:30:00 "
R4span  18:00:00              civil=false parse_span=Err(Unparseable(expected duration to start with a unit value …))   ← " 18:00:00" (leading)
R4config timeout=18:00:00         Ok(timeout=64800s)   ← timeout = "18:00:00 "
R4config timeout=18:00:00	        Ok(timeout=64800s)   ← timeout = "18:00:00\t"
```
Cheapest repair: apply both rules to `raw.trim_end()` (or `trim()`), since the question is what the author wrote and whitespace is not part of it; fixture `18:00:00 ` refused as `TimeOfDay`.

**Disposition:** fix-now — **user decision 2026-09-04.** `parse_span` trims the string before either time-of-day rule or the span parse sees it: whitespace is not part of what the author wrote. Fixture `"18:00:00 "` refused as `TimeOfDay`.
**Response:** repaired. `parse_span` binds `raw = raw.trim()` before either time-of-day rule or the span parse. Held by fixture `schedule/R-21-wall-clock-time-with-trailing-whitespace.json` (`"18:00:00 "` → `TimeOfDay`), red before the change (parsed as eighteen hours). Probed after: `"18:00:00\t"` and `" 18:00:00"` → `TimeOfDay`; `"90m "` and `" 90m"` → `PT1H30M`, where the leading form was `Unparseable` before — a widening, and the one the disposition asked for.

**Outcome:** `verified` (round 5)

### F-53 — The `civil::Time` arm names unitless integers as times of day, and the diagnostic for `"18"` got worse than it was

**Severity:** minor
**Location:** `src/semantics/schedule.rs:146`
**Expected:** brief §13: a diagnostic a backend author can act on; F-50's principle, "the message honest"; F-46's disposition reads "whatever `civil::Time` parses" as a clock form beside the colon shape, and its fixtures are `18:00:00.000` and `T18:00:00`.
**Observed:** jiff's `civil::Time` also reads a bare hour and ISO basic forms, so `"18"`, `"00"`, `"1200"`, `"1800"`, `"180000"` and `"T18"` are now `TimeOfDay`, while `"5"`, `"24"`, `"123"` are `Unparseable` with jiff's "expected to find unit designator suffix" — the message that tells the author what was missing. Before F-46, `"18"` got that message too. So a two-digit integer under 24 is told it wrote a time of day and one of 24 or more is told it forgot a unit. Nothing is guessed, but the refusal names something the author did not write, which is the F-50 defect on the other arm. The config path inherits it: `timeout = "18"` says "a time of day is not a span".
**Evidence:**
```
R4span 18                     civil=true  parse_span=Err(TimeOfDay)   wire=Discard[… schedule is a time of day, which is neither an instant nor a span: 18]
R4span 00                     civil=true  parse_span=Err(TimeOfDay)
R4span 1200                   civil=true  parse_span=Err(TimeOfDay)
R4span 1800                   civil=true  parse_span=Err(TimeOfDay)
R4span 180000                 civil=true  parse_span=Err(TimeOfDay)
R4span T18                    civil=true  parse_span=Err(TimeOfDay)
R4span 5                      civil=false parse_span=Err(Unparseable(expected to find unit designator suffix (e.g., `years` or `secs`) after parsing integer))
R4span 24                     civil=false parse_span=Err(Unparseable(expected to find unit designator suffix (e.g., `years` or `secs`) after parsing integer))
R4config timeout=18               Err(backend.timeout = "18" is not a duration this host can resolve: a time of day is not a span)
R4config timeout=5                Err(backend.timeout = "5" is not a duration this host can resolve: expected to find unit designator suffix (e.g., `years` or `secs`) after parsing integer)
```
One conjunct: consult `civil::Time` only when the string carries a `:` or a `T`/`t` designator (`T18:00:00` stays caught; `T18` and `1800` become `Unparseable`), plus a fixture for `18` asserting `Unparseable`.

**Disposition:** fix-now — **user decision 2026-09-04.** The clock parser is consulted only when the string carries a `:` or begins with `T`/`t`, so a unitless integer reaches the span parse and gets jiff's "expected unit designator" message; `T18:00:00` stays a time of day. Fixture `"18"` asserting `Unparseable`.
**Response:** repaired. `looks_like_a_time_of_day` consults `civil::Time` only when `could_be_a_clock_form` — the string contains `:` or starts with `T`/`t`. Held by fixture `schedule/R-25-unitless-integer.json` (`"18"` → `Unparseable`), red before the change (`TimeOfDay`). Probed after: `18`, `1800`, `180000`, `5`, `24` → jiff's "expected to find unit designator suffix"; `T18`, `T18:00:00` → `TimeOfDay`.

**Outcome:** `verified` (round 5)

### F-54 — F-48's repair left `State::resolve_to`'s doc and design §5.4 stating the rule it replaced

**Severity:** nit
**Location:** `src/shell/state.rs:56`–`:58`; `docs/slices/001/design.md:1611`, `:1615`–`:1619`
**Expected:** F-48's Response: "`no_action` … writes the result with `state.resolve_to` … its doc says so and says why." The disposition puts R-29's rewording on the reconciliation list; the same rule is stated in two other places the disposition does not name.
**Observed:** `resolve_to`'s doc still reads "Only a *successful* exchange may call this: every failure path leaves the resolved check exactly as it was (R-29, P2)", and `no_action` now calls it on every failure path including the stale-`respond` refusal. Design §5.4's paragraph "**Failure does not move the schedule.** Every failure path leaves `resolved_check` exactly as it was" and its state-diagram row "respond(stale id) — rejected, no backend call, state untouched" say the same. The behaviour is as dispositioned; the words around it are not.
**Evidence:**
```
src/shell/state.rs:56:  /// Move the schedule. Only a *successful* exchange may call this: every
src/shell/state.rs:57:  /// failure path leaves the resolved check exactly as it was (R-29, P2).
src/shell/host.rs:290:    self.state.resolve_to(next_check);          ← inside no_action
R4f48 t=04:12 ok(view,30m)      reported=Timestamp(2026-08-23T04:42:00Z)
R4f48 t=04:42 stale respond      reported=Timestamp(2026-08-23T05:12:00Z) calls=1
R4f48 t=04:50 ok(no next_check)  reported=Timestamp(2026-08-23T05:12:00Z) failure=false
```
The `resolve_to` doc is one sentence ("Every path that reports a schedule writes it — a failure resolves with no instruction, F-48"); §5.4 and the diagram row join R-29 on the reconciliation list.

**Disposition:** fix-now for the code doc — **user decision 2026-09-04.** `State::resolve_to`'s doc says every path that reports a schedule writes it. `design.md` §5.4's paragraph and the diagram row join R-29 on session 3's reconciliation list; `design.md` is not touched mid-audit.
**Response:** repaired for the code doc. `State::resolve_to`'s doc reads: every path that reports a schedule writes it, so the stored check is always the one the caller was last told. `design.md` §5.4's paragraph and the `respond(stale id)` diagram row are on the session-3 reconciliation list in `notes.md` beside R-29.

**Outcome:** `verified` (round 5) — by caller walk, no test names a doc

### Round 4 — confirmed round-3 repairs

Each Response's claim, checked by reverting the repair in the scratch copy and running the test it names (`revert.sh`). Results verbatim.

- **F-45** — path tokens matched by `contains` on `code_of(line)`. Revert (drop the `::` branch): `test boundary::a_token_matches_a_word_and_not_a_substring_of_one ... FAILED — "use crate::shell::host::Host;" names the domain and was missed`. Probes: `crate::shell`, `use crate::shell::x` → caught; `// crate::shell` → clean. Substring consequences, all latent (no such module or literal in `src/`): `crate::shells`, `crate::shell_x`, `mycrate::shell` and `let s = "use crate::shell::x";` are all caught — over-eager, so they fail loudly rather than pass silently; `let s = "// crate::shell"; use crate::shell::x;` is hidden by the documented `//`-in-a-string blind spot, which now covers path tokens too.
- **F-46** — shape with fraction, `||` `civil::Time`. Full revert to the round-2 rule: `runner::every_scheduling_fixture_states_what_the_protocol_does ... FAILED` naming `R-21-wall-clock-time-with-a-fraction.json: expected TimeOfDay, got 2026-08-23T22:12:00Z`, `R-21-wall-clock-time-with-the-iso-designator.json: expected TimeOfDay, got Unparseable`, and `R-25-colons-with-nothing-between-them.json: expected Unparseable, got TimeOfDay`. Reverting the `civil::Time` arm alone: `FAILED` naming the iso-designator fixture only — so each half is pinned by its own fixture. Probes after: `1:30`, `1:30:00.5`, `18:00:00,5`, `18:00:00.000`, `T18:00`, `T18:00:00`, `18:00:00+10:00`, `0:0` → `TimeOfDay`; `1:30:00.`, `.5:00`, `1:30:00.5.5`, `18:00:00Z`, `18:00:00.5Z`, `1:30:00+1h`, `٣:٣٠`, `1 :30`, ` 1:30`, `::` → `Unparseable` with jiff's message; `-18:00:00` → `18h ago`, `+18:00:00` → `18h` (signed spans, per the Response — and `+18:00:00` loads as a config timeout, `-18:00:00` is refused as non-positive); `2026-08-23` → `Unparseable` from `parse_span`, `MissingOffset` on the wire (`civil::DateTime` reads a bare date); `1,5h` and `1.5h` → `1h 30m`. Coherent with F-2 as dispositioned, with two exceptions raised above: trailing whitespace (F-52) and unitless integers (F-53). `1:2:3:4:5` and `99:99` are `TimeOfDay` by the disposition's own `digits(:digits)+`; noted, not raised.
- **F-47** — `NotAString` renders `raw`. Revert (drop the arm): `host::a_failure_and_a_discard_render_as_their_leaves_do ... FAILED — left: 0 right: 1`. Renders: `next_check 45 discarded: schedule must be a string, found number`; `true`, `[1,2,3]`, `{"a":1,"b":[1,{"c":null}]}` compact, `4.5`, `-0.0`, `1e+300` as serde prints them; an object holding a 200-byte string renders it whole. Unbounded, as the five `raw: String` variants already are; the line is sane.
- **F-48** — `no_action` writes what it reports. Revert (drop `resolve_to`): `host::a_failure_at_an_elapsed_check_reports_the_default_poll_from_now ... FAILED — left: Timestamp(2026-08-23T05:20:00Z) right: Timestamp(2026-08-23T05:12:00Z)` at the second failure. `&mut self` changes nothing in `exchange`: the transport's result is destructured before either `no_action` call, so no borrow overlaps. The stale-`respond` path writes too (probe above: stale at `04:42` reports and stores `05:12`, a later no-instruction success stands on it). Neither `no_failure_moves_the_schedule` nor `a_successful_exchange_does_move_the_schedule` nor the R-29 rows of `failure_matrix.rs` becomes vacuous: every one runs its failure before the retained instant, where `resolve_from(None, now)` returns the retained value and writing it back is a no-op — `failure_matrix.rs:40`–`:44` already records that the seed cannot distinguish "unchanged" from "recomputed" and moves the check first. What is stale is the prose (F-54).
- **F-49** — letter/digit boundary. Revert: `boundary::a_token_matches_a_word_and_not_a_substring_of_one ... FAILED — "let habit2 = 1;" names the domain and was missed`. `camel_segments`: `Site2Habit` → `Site, 2, Habit` (both tokens caught); `2site` → `2, site`; `site2site` → `site, 2, site`; `x86Site` → `x, 86, Site`; `H2O` → `H, 2, O`. `SITEID` → not caught, `/* a habit */` → caught, as the doc now says.
- **F-50** — every group non-empty. Revert (`all_digits` without the emptiness check): `runner::every_scheduling_fixture_states_what_the_protocol_does ... FAILED` naming `R-25-colons-with-nothing-between-them.json: expected Unparseable, got TimeOfDay`. `::` renders `unparseable schedule: ::`.
- **F-51** — documentation. `examples/typescript/backend.ts:104`: "They are stricter than the host about `null` on purpose: the host reads a nulled modelled key as omission, and a backend written from these types omits it." `deno check examples/typescript/backend.ts` exit 0. The claim it rests on — a nulled `hints`/`min` is omission — is held by the `R-51` fixtures round 3 already confirmed.

Not verified beyond the above: nothing. The round is clean apart from F-52…F-54.

### Round 5, fresh reviewer — F-55…F-56

Raised by a fresh Claude subagent (general-purpose, no thread history) against `d6cc44a`, the round-4 repairs only, given the ledger, the brief, the draft spec and the diff. Method, in its own words: Ran `just check` on `d6cc44a`: exit 0 (default column 42 lib + 58 integration + 16 protocol; `--no-default-features` 25 lib + 16 protocol; `deno check`, clippy in both columns, `cargo fmt --check` clean). Copied the repo minus `target` and `.git` to `scratchpad/r5/repo` and worked only there: a probe test in the protocol target over `parse_span`, `civil::Time` and `schedule::parse` for 44 strings, one in the integration target over `Config::parse` with `timeout` set to 19 of them, and three edit-run-restore cycles on `schedule.rs` (revert the F-52 trim; revert the F-53 conjunct; try one candidate repair for F-55), each ending with `cmp` clean against the working tree. jiff is 0.2.35. Every output quoted below is my own run. No repository file was edited (`git status`: only the pre-existing `flake.lock` change).

Probe results on `d6cc44a`, by group. Right, and as the dispositions say: `"\n18:00:00\n"`, `"18:00:00\u{a0}"` (NBSP is Unicode `White_Space`, so `trim` strips it), `"18:00:00\u{2003}"`, `"18:00:00\u{85}"`, `"1:30 "`, `"t18:00"`, `"T18:00"`, `"T18"`, `"T18:00:00"`, `"18:00:00+10:00"`, `"18:00:00[Australia/Sydney]"` → `TimeOfDay`. `"T"`, `"T:"`, `"t"`, `"Tuesday"`, `"today"`, `"tomorrow"`, `"true"`, `"ten minutes"` → `Unparseable` with jiff's "expected duration to start with a unit value … no integer was found". `"18"`, `"1800"` → `Unparseable` with "expected to find unit designator suffix". `"18:00:00Z"`, `"18:00:00 +10:00"`, `"18:00:00\u{200b}"` → `Unparseable` with "parsed value, but unparsed input remains"; `"\u{feff}18:00:00"` (BOM, not whitespace) → `Unparseable`. `" 90m "`, `"12h "`, `"PT1H30M "`, `" 1h 30m "`, `"-18:00:00 "`, `" +18:00:00"` → spans, the last two signed per F-46. `""`, `" "`, `"\t"` → jiff's "an empty string is not a valid duration", which after the trim is what they are. Config: `timeout = "18:00:00 "`, `"18:00:00\n"`, `"18:00:00 "`, `"1:30 "`, `"T18"`, `"18:00:00+10:00"` → "a time of day is not a span"; `"18"`, `"1800"` → "expected to find unit designator suffix"; `"T"`, `"Tuesday"`, `"today"` → "no integer was found"; `" 90m "` → 5400 s, `"12h "` → 43200 s; `""`, `" "` → "an empty string is not a valid duration". Observed and not raised because pre-existing at `83d1b77` and untouched by either repair: `"T1:30"` is `Unparseable` where `"T18:00"` is `TimeOfDay` (F-37's padding seam survives on the designator form, which the disposition's two rules do not cover); a `raw` holding newlines is rendered verbatim into the diagnostic (F-47's territory); config `timeout = "2026-08-23T05:00:00"` says "a time of day is not a span" because `civil::Time` reads the time out of a datetime string (F-46's arm, unchanged by the conjunct since the string carries a colon).

### F-55 — The trim forgives whitespace on R-21's span form only, and turns an absolute instant with a stray space into a false "time of day"

**Severity:** minor
**Location:** `src/semantics/schedule.rs:72`–`:84` (`parse_instruction`, the two absolute arms), `:127`–`:130` (`parse_span`, the trim), `:153`–`:159` (`looks_like_a_time_of_day`, `could_be_a_clock_form`)
**Expected:** F-52's disposition: "whitespace is not part of what the author wrote". R-21 admits two forms, an absolute instant and a relative span; R-22: an offsetless instant "MUST be rejected with an error distinct from a general parse failure". Brief §13: a diagnostic the backend author can act on; F-50's principle, "the message honest".
**Observed:** the trim lives in `parse_span`, so only the span arm sees the trimmed string. The two absolute arms in `parse_instruction` still see the raw one and refuse it on the whitespace; the value then falls through to `parse_span`, which trims it, finds a colon, and hands it to `civil::Time` — and `civil::Time` reads the time out of a full datetime string. So `"2026-08-23T05:00:00+10:00 "`, a valid instant with a trailing space, is reported as "a time of day, which is neither an instant nor a span", and `"2026-08-23T05:00:00 "` — R-22's own case with a trailing space — is reported as a time of day instead of as missing its offset. `" 90m "` is accepted while `" 2026-08-23T05:00:00Z"` is `Unparseable`: the same stray byte is forgiven on one of R-21's forms and not the other. Before `d6cc44a` both datetime cases were `Unparseable`, which was uninformative but not false; the trim plus the retained `civil::Time` arm made the diagnostic name something the author did not write, which is the F-53 defect on another shape. The config path is not affected by the whitespace half (an instant is not a config duration either way), but see the pre-existing note in Method.
**Evidence:**
On `d6cc44a`:
```
R5 "2026-08-23T05:00:00Z"       civil=Err                    parse_span=Err(Unparseable(expected to find unit designator suffix (e.g., `years` or   wire=Ok(Timestamp(2026-08-23T05:00:00Z))
R5 "2026-08-23T05:00:00Z "      civil=Err                    parse_span=Err(Unparseable(expected to find unit designator suffix (e.g., `years` or   wire=Err[unparseable schedule: 2026-08-23T05:00:00Z ]
R5 " 2026-08-23T05:00:00Z"      civil=Err                    parse_span=Err(Unparseable(expected to find unit designator suffix (e.g., `years` or   wire=Err[unparseable schedule:  2026-08-23T05:00:00Z]
R5 "2026-08-23T05:00:00 "       civil=Err                    parse_span=Err(TimeOfDay)                                                              wire=Err[schedule is a time of day, which is neither an instant nor a span: 2026-08-23T05:00:00 ]
R5 "2026-08-23T05:00:00"        civil=Ok(05:00:00)           parse_span=Err(TimeOfDay)                                                              wire=Err[schedule has no UTC offset: 2026-08-23T05:00:00]
R5 "2026-08-23T05:00:00+10:00 " civil=Err                    parse_span=Err(TimeOfDay)                                                              wire=Err[schedule is a time of day, which is neither an instant nor a span: 2026-08-23T05:00:00+10:00 ]
R5 " 90m "                      civil=Err                    parse_span=Ok(1h 30m)                                                                  wire=Ok(Timestamp(2026-08-23T05:42:00Z))
```
Same probe with the F-52 trim reverted (the `83d1b77` behaviour for these two):
```
R5 "2026-08-23T05:00:00 "       civil=Err                    parse_span=Err(Unparseable(expected to find unit designator suffix (e.g., `years` or   wire=Err[unparseable schedule: 2026-08-23T05:00:00 ]
R5 "2026-08-23T05:00:00+10:00 " civil=Err                    parse_span=Err(Unparseable(expected to find unit designator suffix (e.g., `years` or   wire=Err[unparseable schedule: 2026-08-23T05:00:00+10:00 ]
```
Candidate repair, tried in the scratch copy: trim once at the top of `parse_instruction` so both of R-21's forms see the same string (`parse_span` keeps its own trim for the config path). All 17 protocol tests pass and the probe reads:
```
R5 "2026-08-23T05:00:00Z "      … wire=Ok(Timestamp(2026-08-23T05:00:00Z))
R5 " 2026-08-23T05:00:00Z"      … wire=Ok(Timestamp(2026-08-23T05:00:00Z))
R5 "2026-08-23T05:00:00 "       … wire=Err[schedule has no UTC offset: 2026-08-23T05:00:00]
R5 "2026-08-23T05:00:00+10:00 " … wire=Ok(Timestamp(2026-08-22T19:00:00Z))
R5 "2026-08-23 "                … wire=Err[schedule has no UTC offset: 2026-08-23]
```
One caution for whoever repairs it: my one-line trial shadowed `raw`, so the `raw` carried in every `ScheduleError` became the trimmed value (`"\n18:00:00\n"` rendered as `18:00:00`). The error should keep quoting what was sent; bind the trimmed string under another name. Fixtures: `"2026-08-23T05:00:00+10:00 "` accepted as the instant, `"2026-08-23T05:00:00 "` refused as `MissingOffset`.

**Disposition:** fix-now — **user decision 2026-09-04.** `parse_instruction` trims once, so both of R-21's forms see the same string; every `ScheduleError` keeps quoting the untrimmed value as sent. `parse_span` keeps its own trim for the config path. Fixtures: an instant with a trailing space accepted; an offsetless instant with a trailing space refused as `MissingOffset`.
**Response:** repaired. `parse_instruction` binds `written = raw.trim()` and parses `written` on all three arms, while every `ScheduleError` still carries `raw` as sent; `parse_span` keeps its own trim for the config path, and its comment says why there are two. Held by fixtures `schedule/R-21-absolute-with-trailing-whitespace.json` (`"2026-08-23T05:00:00+10:00 "` → the instant) and `R-22-absolute-without-offset-with-trailing-whitespace.json` (`"2026-08-22T18:00:00 "` → `MissingOffset`), both red before the change (`TimeOfDay`).

**Outcome:** `verified` (round 6)

### F-56 — F-48's and F-54's Responses say three items are on `notes.md`'s reconciliation list, and none is

**Severity:** minor
**Location:** `docs/slices/001/review-code.md:1032` (F-48 Response), `:1188` (F-54 Response); `docs/slices/001/notes.md:84`–`:105` (the session-3 reconciliation list); `docs/slices/001/audit.md:201`–`:209` (the Reconciliation table, still the template)
**Expected:** F-48's Response: "R-29's rewording is on session 3's reconciliation list." F-54's Disposition: "`design.md` §5.4's paragraph and the diagram row join R-29 on session 3's reconciliation list"; its Response: they "are on the session-3 reconciliation list in `notes.md` beside R-29." `AGENTS.md`'s review protocol: a Response states what was done, and a claim in it is checkable.
**Observed:** `notes.md` was last changed at `ad811c6` (round 2); `d6cc44a` touches only `review-code.md`, `schedule.rs`, `state.rs` and two fixtures, and the working tree has no uncommitted change to it. Its reconciliation list carries R-26 and §5.3 for F-1 and, on R-29's row, session 2's elapsed-instant note — nothing about "must not accept a new instruction" (F-34, F-48), nothing about §5.4's "Failure does not move the schedule" paragraph, nothing about the `respond(stale id)` diagram row. `audit.md`'s Reconciliation table is the unfilled template. The only record of the three items is the Disposition and Response lines themselves, so a session 3 that works from the list `notes.md` names as the list will miss them. The code doc repair itself is as dispositioned (see the confirmation below); this is the ledger claim, not the code.
**Evidence:**
```
$ git show --stat d6cc44a | grep -i notes            → (no output)
$ git log --oneline -3 -- docs/slices/001/notes.md
ad811c6 audit 001.2: round 1 dispositioned and repaired, round 2 raised and dispositioned
2fe3bb4 audit 001.1: fresh reviewer's round-1 findings appended, F-17..F-33
41af112 audit 001.1: brief, evidence, code review round 1 opened
$ git status --short                                  →  M flake.lock
$ grep -rn -i 'must not accept a new instruction' docs/slices/001/   → only review-code.md:723, :725, :1029, :1031 (the F-34 and F-48 finding text)
$ grep -n -i 'diagram row\|stale.id\|5\.4.*R-29\|R-29.*5\.4' docs/slices/001/notes.md docs/slices/001/audit.md   → (no output)
notes.md:86-90:  … Add to §7 or R-29's row: a *failed* exchange at an elapsed check still reports the
                 elapsed instant (R-29 holds; slice 003's timer must retry on cadence, not spin) — noted in session 2, not changed.
audit.md:207-208: | document | change | reason | done |
                  | `specs/NNN-…md §4` | | code diverged at `path:line`; code is right | [ ] |
```
Repair: three lines under `notes.md:84`'s list — R-29's rewording to "must not accept a new instruction" (F-34, F-48); `design.md:1615`–`:1619`'s paragraph (F-54); the `respond(stale id)` diagram row at `design.md:1611` (F-54) — or the Responses corrected to say where the items actually are.

**Disposition:** fix-now — **user decision 2026-09-04.** The three lines are added to `notes.md`'s session-3 reconciliation list now, ahead of the handover rewrite, so F-48's and F-54's Responses become true.
**Response:** repaired. `notes.md`'s session-3 reconciliation list now carries R-29's rewording (F-34, F-48), `design.md` §5.4's "Failure does not move the schedule" paragraph (F-54) and the `respond(stale id)` diagram row (F-54), each with the line numbers the reviewer gave. F-48's and F-54's Responses are true as written.

**Outcome:** `verified` (round 6)

### Round 5 — confirmed round-4 repairs

Each Response's claim, checked by reverting the repair in the scratch copy and running the test it names. Results verbatim.

- **F-52** — `parse_span` trims before either rule. Revert (delete `let raw = raw.trim();`): `test runner::every_scheduling_fixture_states_what_the_protocol_does ... FAILED` — `R-21-wall-clock-time-with-trailing-whitespace.json: … expected TimeOfDay, got 2026-08-23T22:12:00Z`. Restored, `cmp` clean. The Response's probes hold: after, `"\n18:00:00\n"`, `"18:00:00\u{a0}"`, `"1:30 "` → `TimeOfDay`; `" 90m "`, `"12h "`, `"PT1H30M "` → spans, and in the reverted copy `" 90m "` is `Unparseable` ("expected duration to start with a unit value"), as the Response says of the leading form. Config `timeout = "18:00:00 "` and `"18:00:00\n"` → "a time of day is not a span". What the trim newly does to the absolute form is F-55.
- **F-53** — `civil::Time` consulted only behind `could_be_a_clock_form`. Revert (drop the conjunct and the function, back to `|| raw.parse::<jiff::civil::Time>().is_ok()`): `test runner::every_scheduling_fixture_states_what_the_protocol_does ... FAILED` — `R-25-unitless-integer.json: … expected Unparseable, got TimeOfDay (schedule is a time of day, which is neither an instant nor a span: 18)`. Restored, `cmp` clean. In the reverted copy `18`, `1800`, `T18` are all `TimeOfDay`; on `d6cc44a` `18` and `1800` are `Unparseable` with "expected to find unit designator suffix", `T18`, `T18:00:00`, `t18:00` are `TimeOfDay`, and `T`, `T:`, `Tuesday`, `today`, `tomorrow`, `true` reach the span parse and get "no integer was found" — the designator gate admits a word beginning with `t` to the clock parser and the clock parser refuses it, so nothing English is called a time of day. Config `timeout = "18"` → "expected to find unit designator suffix"; `"T18"` → "a time of day is not a span".
- **F-54** — `State::resolve_to`'s doc. No test names it; checked against the callers instead. `grep -n resolve_to src`: `host.rs:229` inside `accept`, `host.rs:290` inside `no_action`, `state.rs:60` the definition. `Outcome` is constructed at exactly those two sites (`host.rs:246`, `:291`), `evaluate` and `respond` both return through `exchange` → `accept`/`no_action` or straight through `no_action` (`host.rs:162`, the stale-id refusal), and `Host::new` seeds through `State::new` without reporting. "Every path that reports a schedule writes it" is true of every caller. The `notes.md` half of the Response is F-56.

Not verified beyond the above: nothing. `just check` exit 0.

### Round 6, fresh reviewer — no findings

Raised by a fresh Claude subagent (no thread history) against `24b1c3e`, the round-5 repairs only: `git diff d6cc44a..24b1c3e -- src tests docs/slices/001/notes.md`, the F-55 and F-56 Disposition and Response lines, draft-spec R-21…R-28 and brief §13.

**Method.** Ran `just check` on the working tree at `24b1c3e`: exit 0 (default column 42 lib + 58 integration + 16 protocol; `--no-default-features` 25 lib + 16 protocol; `deno check`, clippy in both columns, `cargo fmt --check` clean). The protocol count is 16 in both columns because the two new fixtures join the one corpus test rather than adding tests. Copied the repo minus `target` and `.git` to `scratchpad/r6/repo` and worked only there: one edit-run-restore cycle on `schedule.rs` (revert the `written` trim), ending with `cmp` clean against the working tree, and a probe test in the protocol target over `schedule::parse` (through `serde_json::Value`), `Discarded`'s `Display`, and the real wire path `read_response` with `{"view": null, "next_check": <string>}` for 15 strings. Every output quoted below is my own run. No repository file was edited by me. `git status` shows `flake.lock` (pre-existing) and an uncommitted change to `review-code.md` that fills earlier rounds' `**Outcome:**` lines — not mine, not in scope, and not what I reviewed.

Probe results on `24b1c3e`, wire path. Accepted as the instant: `"2026-08-23T05:00:00+10:00 "` → `2026-08-22T19:00:00Z`; `" 2026-08-23T05:00:00Z"`, `"\t2026-08-23T05:00:00Z\t"`, `"\r\n2026-08-23T05:00:00Z\r\n"`, `"\u{a0}2026-08-23T05:00:00Z"` → `2026-08-23T05:00:00Z`. `" 90m "` → `2026-08-23T05:42:00Z`. `MissingOffset`: `"2026-08-22T18:00:00 "`, `"2026-08-23 "`, `"2026-08-23 05:00:00 "`. `TimeOfDay`: `"\n18:00:00\n"`. `CalendarUnit`: `" 1 month "`. `Unparseable`: `"  "`, `""`, `"2026-08-23T05:00:00 +10:00"` (interior space, not trimmed — correct), `"\n100000000 days\n"` (the grammar's unit bound, as R-25's fixture says). In every error case the `raw` carried by the `ScheduleError` is byte-equal to the string as sent, untrimmed — the probe asserts it and passed — and `Discarded`'s rendering carries the same string, e.g. `next_check discarded: schedule has no UTC offset: 2026-08-22T18:00:00 ` with the trailing space. A `raw` holding newlines renders verbatim: `next_check discarded: schedule is a time of day, which is neither an instant nor a span: ⏎18:00:00⏎` — one diagnostic across three lines, not broken, and identical to `d6cc44a` (round 5 observed and did not raise it; F-47's territory).

Observed and not raised, because pre-existing and untouched by the repair: `""` and `"  "` render as `unparseable schedule: ` and `unparseable schedule:   `, the value invisible — a consequence of F-42's verbatim rendering plus the untrimmed `raw`, and already so at `d6cc44a`. `parse`'s `# Errors` doc (`schedule.rs:39`–`:47`) lists five variants and omits `TimeOfDay`, which F-2 added in round 1. The new comment at `schedule.rs:69`–`:70` and both new fixture descriptions cite `F-52`/`F-55` where F-13's disposition records "new code cites `R-N`… not slice-local `F-N`" as the convention forward; every round-2…4 repair does the same and was confirmed, so this reads as the audit's own convention until close, and is for the user to say. `draft-spec.md:377`'s corpus list names neither new fixture, nor any fixture added since round 1; `notes.md:105`–`:111` already schedules re-running PHASE-09's two-direction script before promotion.

No findings.

### Round 6 — confirmed round-5 repairs

Each Response's claim, checked against the code and by reverting the repair in the scratch copy. Results verbatim.

- **F-55** — `parse_instruction` binds `written = raw.trim()` and parses `written` on all three arms (`schedule.rs:71`, `:72`, `:79`, `:84`); every `ScheduleError` constructor still takes `raw` (`:81`, `:85`, `:98`). `parse_span` keeps its own trim (`:134`) and its comment says why there are two (`:132`–`:133`). Revert (`let written = raw;`), `cargo test --test protocol every_scheduling`: `test runner::every_scheduling_fixture_states_what_the_protocol_does ... FAILED` — `R-21-absolute-with-trailing-whitespace.json: … expected 2026-08-22T19:00:00Z, got TimeOfDay (schedule is a time of day, which is neither an instant nor a span: 2026-08-23T05:00:00+10:00 )` and `R-22-absolute-without-offset-with-trailing-whitespace.json: … expected MissingOffset, got TimeOfDay (schedule is a time of day, which is neither an instant nor a span: 2026-08-22T18:00:00 )`. Both red before the change with `TimeOfDay`, as the Response says. Restored, `cmp` clean. Probe: the eight strings the brief asked for behave as listed in Method; `raw` is the untrimmed string as sent in all nine error cases probed; round 5's caution (a shadowed `raw` trimming the diagnostic) did not recur — `"\n18:00:00\n"` renders with both newlines.
- **F-56** — `grep -n "must not accept\|1615\|1611" docs/slices/001/notes.md` → `91:- \`draft-spec.md\` R-29: reword to "a failed exchange must not accept a new`, `97:  (\`design.md:1615\`–\`:1619\`): stale for the same reason (F-54) — *Design`, `99:- \`design.md\` §5.4 state diagram, \`respond(stale id)\` row (\`design.md:1611\`):`. The three bullets sit at `notes.md:91`–`:102`, directly under R-29's session-2 bullet, and cite F-34/F-48, F-54, and F-48/F-54 respectively. The line numbers they carry are right: `design.md:1611` is the `Outstanding --> Outstanding: respond(stale id) — rejected, no backend call, state untouched` row and `:1615`–`:1619` is the `**Failure does not move the schedule.**` paragraph. F-48's Response ("R-29's rewording is on session 3's reconciliation list") and F-54's ("are on the session-3 reconciliation list in `notes.md` beside R-29") are now true as written. `git show --stat 24b1c3e`: `notes.md` +12, `review-code.md` +81, `schedule.rs` +8/−4, two fixtures — the five files the Responses account for, nothing else.

Not verified beyond the above: nothing. `just check` exit 0.

### F-57 — `schedule::parse`'s `# Errors` doc lists five variants and omits `TimeOfDay`

**Severity:** nit
**Location:** `src/semantics/schedule.rs:39`–`:47`
**Expected:** the doc enumerates "one `ScheduleError` per way of not being an instant"; F-2 added a sixth in round 1.
**Observed:** `TimeOfDay` is absent from the list. Raised by the audit agent from the round-6 reviewer's "observed and not raised" note, so that the ledger records it rather than a silent tidy.
**Evidence:** `sed -n 39,47p src/semantics/schedule.rs` on `24b1c3e` — five variants named.

**Disposition:** fix-now — self-raised nit, the doc gains the sixth variant with its R-21 / brief §3.3 reason. No user decision needed: a doc restating what the code and F-2's disposition already say.
**Response:** repaired. The list names `TimeOfDay` between `MissingOffset` and `CalendarUnit`. `just check` exit 0, both columns.

**Outcome:** `repaired` (round 6, self-raised) — a doc; no test names it

### Round 7, fresh reviewer — one nit

Raised by a fresh Claude subagent (no thread history) against the uncommitted working tree over `cbc7570`, scoped to `git diff -- src tests` only: the six comment lines changing `draft-spec.md` to `SPEC-001` at promotion (audit session 3), held to `docs/specs/001-host-backend-protocol.md` §4, §6.1, §7.

**Method.** `git diff --stat -- src tests`: four files, 6 insertions, 6 deletions; `git diff -w --stat` identical. Every changed line inside a `///`, `//!` or `//` comment; the only token changed is the literal. Each rewritten citation checked against the promoted spec — `canonical.rs:117`, `:511`, `:721` (§6.1's RFC 3339 form and its two request literals, byte-for-byte the test literals at `:745`–`:747` and `:766`–`:768`), `schedule.rs:2` (R-21…R-28 is the range the module implements; R-29 is the host's and appears nowhere in the module), `normalize.rs:21` (§7 names both corpus tests), `runner.rs:48` (§4 is the `R-N` table) — all true. Sweep `grep -rn "draft-spec\|spec.*:[0-9]\{2,\}" src tests examples` → empty: no citation by path or by line number remains. Broader `grep -rni 'draft'` → `host.rs:29` (an earlier draft of the code, not the spec) and the one below. Not verified: the gate — run by the audit agent, exit 0 in both columns.

### F-58 — `state.rs` names the spec as "the draft spec" in prose

**Severity:** nit
**Location:** `src/shell/state.rs:28`–`:30`
**Expected:** after promotion, code names the spec `SPEC-001`; the diff fixed the hyphenated `draft-spec.md` form everywhere and this is the one unhyphenated instance the grep did not match.
**Observed:** `/// draft spec or this slice reads when a view was issued, so the field is not` — the claim holds (`grep -n issued docs/specs/001-host-backend-protocol.md` → empty; `design.md:1172` is still the `Outstanding { view_id, issued_at }` line); only the name is stale.
**Evidence:** `sed -n 28,30p src/shell/state.rs`; `grep -rni 'draft' src tests examples`.
**Disposition:** fix-now — **user decision 2026-09-04.** Same class as the six lines round 7 confirmed; no further round.
**Response:** repaired. `state.rs:28`–`:30` now reads "nothing in the brief, SPEC-001 or this slice reads when a view was issued". `grep -rni 'draft' src tests examples` → `host.rs:29` only, which is about the code. `just check` exit 0 in both columns.

**Outcome:** `verified` — by the raiser's own grep; a comment names no test

## Synthesis

Written by the raiser once the ledger resolved, 2026-09-04. Fifty-eight
findings over seven rounds: five major, thirty-three minor, twenty nit, no
blocker. Fifty-five fix-now and repaired, three tolerated by user decision
(F-13, F-15, F-44). Round 6 came back with no findings; its one aside became
F-57, a doc nit the raiser repaired itself. Round 7 was opened at
reconciliation over the six comment lines that renamed the spec at promotion,
and found one nit of the same class beside them (F-58). Every repair was confirmed by a later fresh reviewer
reverting it and watching the named test go red; F-9 alone needed a third
round to be pinned (F-43), and one first-draft test for it passed against
both implementations before it was made to fill the buffer it was offered.
`just check` exits 0 in both feature columns at every round's commit.

**What the review changed.** Three things of substance, all coherence with
the brief rather than with any execution-time decision:

1. *The schedule survives a backend that stops instructing it* (F-1, F-17,
   F-34, F-48). `schedule::resolve` consumes a retained check at or before
   `now` and falls back to the default poll; the host resolves through that
   one arm on every path, failure included, and writes what it reports. Slice
   003's timer will never be handed an elapsed instant. R-26 and R-29 must be
   reworded at reconciliation: a failed exchange "must not accept a new
   instruction", not "must not alter the resolved check".
2. *A time of day is refused, not read as hours* (F-2, F-18, F-37, F-46,
   F-50, F-52, F-53, F-55). The rule took five rounds to converge and is now
   two conjuncts and a trim: the bare colon shape with non-empty groups and an
   optional fraction, or jiff's clock grammar behind a colon-or-`T` gate,
   applied to the trimmed string on both of R-21's forms while every error
   quotes the value as sent. Signed colon forms are spans; unitless integers
   get jiff's "unit missing" message. Twelve schedule fixtures pin the seam.
3. *Shape refusals are typed and total* (F-19, F-20, F-22, F-25, F-35, F-38).
   Duplicate keys are refused at every depth by a pre-pass; every object the
   protocol reads through its own type is an `Object<T>` so arrays are never
   bound positionally; `Shape` and `Json` are distinct variants entered through
   one `From`; a nested `hints` object is refused and a nulled one is omission.

Around those: one duration grammar shared by the wire and the config (F-4,
F-28); `Command { program, arguments }` so the empty command — and the empty
program (F-41) — cannot reach the transport (F-3, F-30); the request write
concurrent with the stdout read and a broken pipe tolerated (F-10, F-23,
F-24); the stdout bound exact and pinned (F-9, F-43); config refusing unknown
keys (F-5) and its README example run by a test (F-16); the example's
TypeScript types the whole protocol, with `never` members so `deno check`
refuses what the host refuses (F-8, F-26, F-40, F-51); `Display` on every
handed-over value rendering each fact once (F-33, F-42, F-47); and the
boundary scans made to match words, plurals, acronyms, digits and paths while
ignoring comment prose (F-14, F-36, F-45, F-49).

**What it confirmed.** The strata hold in both compile columns and by scan;
the transport disposes on every path; host state survives every failure mode
in the matrix; the fixture corpora run through the same read site the host
uses. No finding touched canon, `design.md` or the draft's promotion.

**Risks knowingly left standing.**
- The boundary scanner is a text scan, not a parser: a `//` inside a string
  literal hides the rest of its line, `/* */` blocks are not cut, all-caps
  compounds do not split, and path tokens match as substrings. All latent —
  nothing in `src/` has any of them — and the build gate holds the stratum
  property independently (F-45, F-49).
- `1:2:3:4:5` and `99:99` are "a time of day" by the shape rule; `T1:30` is
  unparseable where `T18:00` is a time of day; a config `timeout` written as a
  full datetime is told it is a time of day. None is a value a backend author
  writes on purpose; all are noted in round 5's method paragraph.
- A `raw` value renders unbounded and verbatim in a discard line, newlines
  included (F-47, round 5 note). The five string variants always did.
- `ConfigError::Duration` renders its fault and chains it as `source()`, so a
  chain-walking logger prints it twice — the crate's convention (F-42).
- `PipeMissing` and `cleanup_only` are reachable by no test (F-15).
- This slice's code cites its own ledger ids; from slice 002 the convention is
  R-N and spec sections (F-13, F-44), lifted to `docs/memory/` at close.

**For reconciliation** — the record is now behind the code in the places
listed under "Handover — session 2b" in `notes.md`: R-26, R-29, R-21/R-25,
R-44, R-51, R-19, R-36, §7's test-name rows, and `design.md` §5.2/§5.3/§5.4.

# Review — implementation — Slice 001

**Subject:** implementation — commit range `6489521..578bf84` on `main`: `src/`,
`tests/`, `examples/`, `Cargo.toml`, `clippy.toml`, `justfile`, root `AGENTS.md`
**Reviewer:** round 1 has two raisers — the audit agent (Claude Fable 5.1,
session 1, F-1…F-16, raised from its own evidence pass before reading any
fresh-reviewer output) and a fresh Claude subagent with no thread history
(F-17 onward, appended when its report landed)
**Opened:** 2026-09-04
**State:** open

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
| F-2 | major | | |
| F-3 | minor | | |
| F-4 | minor | | |
| F-5 | minor | | |
| F-6 | minor | | |
| F-7 | minor | | |
| F-8 | minor | | |
| F-9 | nit | | |
| F-10 | nit | | |
| F-11 | nit | | |
| F-12 | nit | | |
| F-13 | minor | | |
| F-14 | nit | | |
| F-15 | minor | | |
| F-16 | minor | | |
| F-17 | major | | |
| F-18 | major | | |
| F-19 | minor | | |
| F-20 | minor | | |
| F-21 | minor | | |
| F-22 | minor | | |
| F-23 | minor | | |
| F-24 | minor | | |
| F-25 | minor | | |
| F-26 | minor | | |
| F-27 | minor | | |
| F-28 | minor | | |
| F-29 | nit | | |
| F-30 | nit | | |
| F-31 | nit | | |
| F-32 | nit | | |
| F-33 | nit | | |

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
**Response:** pending — session 2.

**Outcome:**

### F-2 — `"next_check": "18:00:00"` is accepted as eighteen hours from now

**Severity:** major
**Location:** `src/semantics/schedule.rs:94`–`:107`; `tests/protocol/fixtures/schedule/R-21-bare-wall-clock-time.json`

**Expected:** brief §3.3: "Permissiveness must not mean silent guessing. Ambiguous values should fail clearly rather than acquire invented semantics." Spec P-B: "An ambiguous message MUST fail; it MUST NOT be guessed at." R-21 admits exactly two forms — an absolute instant and a relative span.
**Observed:** a bare wall-clock time fails the `jiff::Timestamp` and `civil::DateTime` parses and then parses as a `jiff::Span` of `PT18H`. A backend author writing `"18:00:00"` and meaning six this evening gets eighteen hours from now, silently and successfully. PHASE-03 measured this, could not add a variant inside its phase, and **shipped the accepted behaviour as a fixture** so the corpus documents it (`notes.md` Harvest, Open, PHASE-03). The fixture's existence means the corpus currently asserts a guess.
**Evidence:** the fixture file; `MissingOffset`'s own rationale at `design.md` §5.2 ("the single most likely backend mistake deserves a name") applies verbatim to a bare time. A fix is a `jiff::civil::Time` parse attempt before the span parse, rejecting on success; whether that is a new `ScheduleError` variant or `MissingOffset` broadened to "a wall-clock form without date and offset" is the user's call. Either way the fixture flips from `instant` to `error`.

**Disposition:**
**Response:**

**Outcome:**

### F-3 — two host-side impossibilities are reported as the backend's fault

**Severity:** minor
**Location:** `src/shell/backend/process.rs:60`–`:63` and `:67`–`:72`

**Expected:** root `AGENTS.md`: "Every refusal is reported and says which side was wrong." R-47 governs refusals of backend-supplied values; `BackendError` is documented as "why an exchange produced no response body" from the backend.
**Observed:** a `Request` that fails to serialize — host-authored, infallible — is returned as `BackendError::Protocol(ProtocolError::Json(_))`, the variant a caller reads as "the backend's bytes did not mean anything". An empty `command` — rejected at config load, so unreachable through `Config` — is returned as `BackendError::Spawn(InvalidInput)`. Both paths exist because the lint table forbids `unwrap`. Raised at PHASE-05 and left for audit (`notes.md` Harvest, Open).
**Evidence:** the two `match`/`let … else` arms cited. D53 (as amended) built `#[expect(clippy::unwrap_used, reason = …)]` for exactly F-35's case — "a value the *host* created, where an `unwrap` is a statement about our own code" — and this is that case twice. The alternative for the second is making the empty vector unrepresentable (`ProcessBackend::new(program, args, timeout)`), which is what `Config` already splits.

**Disposition:**
**Response:**

**Outcome:**

### F-4 — the duration grammar is implemented twice

**Severity:** minor
**Location:** `src/shell/config.rs:120`–`:134` against `src/semantics/schedule.rs:94`–`:107`

**Expected:** `design.md` §5.2 Config: "one duration syntax across the product". User's standing code standard (`~/.claude/CLAUDE.md`): "No parallel implementation!"
**Observed:** `config::signed` parses a `jiff::Span` and converts it with `SpanRelativeTo::days_are_24_hours()`; `schedule::parse_instruction` does the same two calls. The duplication was a user decision at PHASE-07 (`plan-log.md` 2026-09-03), taken because `schedule.rs` was not that phase's surface and because the existing function is private, tries the absolute form first, and reports `ScheduleError`. Surfaces no longer constrain the audit, and the other two objections are answered by extracting only the span half: a `pub fn` in `semantics::schedule` returning `Result<SignedDuration, SpanFault>` with two variants (`Unparseable`, `CalendarUnit`), which `parse_instruction` maps into `ScheduleError` and `config::signed` into `ConfigError::Duration`. The "must not diverge" comment at `config.rs:110` then has nothing to guard.
**Evidence:** the two cited ranges are the same two jiff calls in the same order.

**Disposition:**
**Response:**

**Outcome:**

### F-5 — a config file's unknown keys are ignored silently

**Severity:** minor
**Location:** `src/shell/config.rs:52`–`:67`

**Expected:** brief §3.3's permissiveness is about the *wire* — a backend written against a newer host (I10, R-4). A config file is the user's own, read once, and `design.md` §5.4 makes a malformed one "fatal at construction". Brief §5's illustrative file carries `socket` and `[logging]`, both out of scope for this slice by the OQ-4 decision.
**Observed:** `File`, `FileBackend` and `FileSchedule` carry no `deny_unknown_fields`, so a user who copies brief §5's example verbatim — the file a reader will actually copy — gets no report that `socket` and `[logging]` did nothing. A misspelled *required* key still fails; only extra keys vanish. Raised at PHASE-07, "no criterion asks for it".
**Evidence:** the three structs cited. `toml`'s error for an unknown key names the key and its line. Slice 005 adds `socket` when it implements it; until then silence is the wrong answer for a file whose author is sitting at the keyboard.

**Disposition:**
**Response:**

**Outcome:**

### F-6 — `Outstanding.issued_at` is written and never read

**Severity:** minor
**Location:** `src/shell/state.rs:30`–`:45`

**Expected:** user's code standard: write less code. `design.md` §5.3 names the field; nothing in the design, the spec or any criterion reads it.
**Observed:** kept under `#[expect(dead_code, reason = …)]` by user decision at PHASE-07, "dropping it would be a design change made to satisfy a lint". The design is not canon and the audit is the place to record drift; a field with no reader is a claim about a future the brief has not named (P3's second half).
**Evidence:** the `#[expect]` at `state.rs:39`. Removing it is a two-line deletion and a *Design drift* entry; keeping it is a comment that will outlive the reason.

**Disposition:**
**Response:**

**Outcome:**

### F-7 — R-34 across a backend failure during `respond` has no direct test

**Severity:** minor
**Location:** `src/shell/host.rs:177`–`:182` (`no_action` on a failed `respond` exchange); `tests/integration/host.rs`

**Expected:** R-34: rejecting a stale response must not clear the outstanding interaction — and by the same rule (`host.rs:245`–`:251`'s comment), a *failed* exchange must not either. `draft-spec.md` §7's R-34 row rests the backend-failure half on `failure_matrix.rs::one_host_survives_every_misbehaving_backend_and_still_works`, where it is incidental to the reuse witness.
**Observed:** `host.rs` asserts R-34 for a *refusal* (`a_superseded_id_is_refused_and_the_outstanding_interaction_survives`) and never for a backend failure on the `respond` path. If the one-`Host` test is ever simplified, the rule loses its only test. Raised at PHASE-10.
**Evidence:** grep of `tests/integration/host.rs` for `respond(` followed by a `failing(` script: none. One fake-backed case — view issued, `respond` meets `failing(Timeout)`, then `respond` again succeeds — closes it at no spawn cost.

**Disposition:**
**Response:**

**Outcome:**

### F-8 — the example backend's types are narrower than the protocol

**Severity:** minor
**Location:** `examples/typescript/backend.ts:71`–`:73` (`body?: string`), `:85` (`kind: "text" | "boolean" | "datetime" | "number"`)

**Expected:** root `AGENTS.md` and brief §22.3: do not narrow the protocol to the current consumer. `draft-spec.md` R-16 names five field kinds including `choice`; R-19 names four content forms, three of them tagged objects. The file's own header says "Copy this file. It is meant to be edited."
**Observed:** `Field.kind` omits `"choice"`; `View.body` is typed `string` while its doc comment says "an object may tag markdown". An agent copying this file (brief §3.7's intended author) inherits a subset of the protocol as if it were the protocol — the narrowing risk R4 (`design.md` §8) names, arriving through the example rather than the renderer.
**Evidence:** the two type declarations against R-16 and R-19. The fix is either the full union types or one sentence saying the types are a subset and pointing at the spec.

**Disposition:**
**Response:**

**Outcome:**

### F-9 — `read_capped` can hold roughly twice the stated bound before it checks it

**Severity:** nit
**Location:** `src/shell/backend/process.rs:234`–`:243`

**Expected:** I11: every stream read from a backend is capped; `STDOUT_LIMIT` is the cap.
**Observed:** `out.reserve(READ_CHUNK)` guarantees *at least* 4 KiB of spare capacity, and `Vec` growth doubles, so `read_buf` — which fills all spare capacity — can read up to the full doubled capacity in one call before `out.len() > limit` is checked. The buffer is bounded, so I11 holds, but the bound is nearer 16 MiB than 8.
**Evidence:** `Vec::reserve` semantics plus `AsyncReadExt::read_buf` reading into `spare_capacity_mut()`. `reader.take(remaining)` or a fixed-size chunk read is the exact form.

**Disposition:**
**Response:**

**Outcome:**

### F-10 — the request write is not concurrent with the stdout read

**Severity:** nit
**Location:** `src/shell/backend/process.rs:173`–`:179`

**Expected:** `design.md` §5.4 step 3: "Drain stdout and stderr concurrently … reading them in sequence deadlocks whenever a backend writes more than a pipe buffer to the stream we are not yet reading."
**Observed:** `body` writes the whole request to stdin *before* reading stdout. A backend that writes more than 64 KiB to stdout before consuming a request larger than 64 KiB deadlocks both sides until `config.timeout`. Stderr is not affected (its drain runs in the `select!`). Requests are host-authored and small, and `Event.data` is the only way to make one large, so the case is unlikely; but it is bounded by the timeout rather than absent, and the design's sentence claims the class.
**Evidence:** the sequential `await`s cited; `harness::padded_evaluate` already builds the >64 KiB request that would exercise it.

**Disposition:**
**Response:**

**Outcome:**

### F-11 — `Fields` and `Hints` derive `Default`, a second door past `new`

**Severity:** nit
**Location:** `src/semantics/protocol/canonical.rs:126`, `:379`

**Expected:** I1 / D30: outside `semantics::protocol` a canonical value comes only from normalization.
**Observed:** `Fields::default()` and `Hints::default()` are public and construct empty values from anywhere. Both empties are valid, so no invariant is breached — but the derive exists only to serve two test helpers (`canonical.rs:616`) inside the module, where a struct literal is available.
**Evidence:** the two derives. Harmless today; worth removing so the rule "only `new`" is true rather than nearly true.

**Disposition:**
**Response:**

**Outcome:**

### F-12 — `Cargo.toml` says the doc-comment lints are paused; `pedantic = "deny"` enables three of them

**Severity:** nit
**Location:** `Cargo.toml:414`–`:417`

**Expected:** a manifest comment that states what the table does.
**Observed:** `missing_errors_doc`, `missing_panics_doc` and `missing_safety_doc` are commented out under "paused alongside `missing_docs`", but all three are pedantic lints and `pedantic = { level = "deny" }` above enables them. PHASE-02 met `missing_errors_doc` as a hard error. Raised at PHASE-02, left for audit.
**Evidence:** clippy's lint groups; the four `# Errors` sections in `canonical.rs` that exist because the lint fired.

**Disposition:**
**Response:**

**Outcome:**

### F-13 — `src/` cites 46 review-finding ids and 61 design-decision ids, none of which survive the slice

**Severity:** minor
**Location:** every module under `src/`; `process.rs` is 86 comment lines in 274

**Expected:** brief §3.7 and §15: the repository is written for agents that read the line in front of them. `docs/AGENTS.md`: canon is evergreen; `review-*.md` and `design.md` are slice-local records. R-N ids survive promotion (AC-13); F-N and D-N do not resolve to anything outside `docs/slices/001/`.
**Observed:** `grep -oE 'F-[0-9]+' -r src | wc -l` → 46; `D[0-9]+` → 61; `design.md` by name → 36. A reader of `process.rs` meets D44, F-49, F-41, I13, R-48, D15, R-40, F-59, D34, R-43 in the first hundred lines. The reasoning is good and belongs somewhere; the question is whether the code should point at a slice folder that will be one of many. R-N citations (102) are the durable ones.
**Evidence:** the counts. Not a defect in behaviour; a maintainability question the user's simplicity standard asks the audit to raise. The cheap form: keep R-N and spec-section citations, and for F/D ids either keep them as-is (they are greppable) or reduce them to the *reason* without the id.

**Disposition:**
**Response:**

**Outcome:**

### F-14 — the domain-vocabulary scan matches substrings

**Severity:** nit
**Location:** `tests/protocol/boundary.rs:136`–`:147`, token list `:168`–`:179`

**Expected:** AC-11: no domain vocabulary in host *types or module names*.
**Observed:** the scan lowercases each line and tests `contains`, so "call sites" trips `site` (it did, at PHASE-02) and any prose containing "goals" or "reminders" will. Stricter than the AC, and the false positives land in comments. Raised at PHASE-02.
**Evidence:** `line.contains(token)` at `:139`. A word-boundary match keeps the AC's strength on identifiers and stops failing on English.

**Disposition:**
**Response:**

**Outcome:**

### F-15 — `PipeMissing` and `cleanup_only` are reachable by no test

**Severity:** minor
**Location:** `src/shell/backend/process.rs:91`–`:95`, `:209`–`:216`; `src/shell/error.rs:38`–`:40`

**Expected:** AC-6: each failure mode maps to a distinct typed error; PHASE-05/VT-3 asks for a case per variant "this phase can reach".
**Observed:** `PipeMissing` fires only when `child.stdin/stdout/stderr.take()` returns `None` after a spawn the host itself configured with all three piped, which no backend can arrange. `cleanup_only` exists only to serve it. Both are untested; raised at PHASE-05 and left for audit.
**Evidence:** tokio's `Child` populates the three handles exactly when `Stdio::piped()` was requested, which `:76`–`:78` does unconditionally. The honest options are (a) tolerate as a guard with the argument on the page — it already is, at `error.rs:38` — or (b) collapse it: `let … else { unreachable }` is forbidden by the lint table, so (a).

**Disposition:**
**Response:**

**Outcome:**

### F-16 — the README's config, the one a reader copies, is exercised by nothing

**Severity:** minor
**Location:** `examples/typescript/README.md:8`–`:15`

**Expected:** brief §15.2: examples should make the intended experience obvious and be complete. AC-1 is a claim about a clean clone.
**Observed:** the README's TOML uses a relative path resolved against goad's working directory; every test roots paths at `CARGO_MANIFEST_DIR`, so the file a user copies is parsed and run by nothing. `config.rs` parses `design.md`'s example instead, which differs only in the path. Raised at PHASE-08.
**Evidence:** `harness::example()` at `tests/integration/harness.rs:206`. A test that reads the README's fenced block, parses it with `Config::parse`, and runs one exchange with `current_dir` set to the crate root would make the README a fixture rather than prose.

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

### F-24 — A failed stdin write is fatal even when the backend answers correctly and exits zero

**Severity:** minor
**Location:** `src/shell/backend/process.rs:173`; pinned by `tests/integration/transport.rs:180-205`

**Expected:** R-37 obliges the host to write one request and close stdin; nothing obliges the backend to read it, and AC-12 says "a bash script that ignores its request and emits a canned response is sufficient". R-40 makes the exit status the arbiter of a response. A backend that ignores stdin, writes a valid document and exits 0 has done nothing the protocol forbids.
**Observed:** `write_all(...).map_err(BackendError::Io)?` returns from `body` on `EPIPE` before stdout is read, so the response is discarded and the exchange is reported as `Io`. Whether this happens depends on request size: under 64 KiB the write lands in the pipe before the child exits; over it, the write blocks until the child exits and then fails. `a_backend_that_exits_before_reading_breaks_the_pipe` asserts the failure as the intended outcome.
**Evidence:** the test's own arrangement (a 1 MiB `data` padding, `transport.rs:189`) is what makes the case fail, and a probe of the same transport against a compiled backend that prints `{"view":null}` and never reads stdin succeeds 200/200 with a small request. The same backend is accepted or refused by the size of a payload it never looks at. Treating `BrokenPipe` on stdin as non-fatal and letting the status and body decide would close it; the deadlock in F-7 is a separate defect with a separate fix.

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

### F-26 — The example backend's TypeScript types admit a strict subset of the protocol

**Severity:** minor
**Location:** `examples/typescript/backend.ts:83-109`

**Expected:** brief §3.7 makes this file what an agent copies; brief §22.3 and root `AGENTS.md` ("Do not narrow wire compatibility merely because the current renderer implements only a subset"); `draft-spec.md` §6.2 gives tagged `body` objects, `choice` fields, `min`/`max`, and `options` on a field.
**Observed:** `View.body?: string` (line 92) refuses every tagged content form while its own comment says "an object may tag markdown"; `Field.kind` (line 105) omits `"choice"`; `Field` declares `multiline` as the only hint and no `min`, `max` or `options`; `Response.next_check?: string` is right but `Instant` is a bare `string` alias. A backend author extending this file gets a type error for protocol the host accepts, and `deno check` is in the gate to enforce exactly those types.
**Evidence:** lines cited; compare with `R-16-a-choice-field.json` and `R-19-a-body-tagged-as-markdown.json`, both accepted by the host.

*Independently raises the same defect as F-8 (audit agent). Disposition follows F-8's.*

**Disposition:**
**Response:**

**Outcome:**

### F-27 — The protocol version is two constants in two modules

**Severity:** minor
**Location:** `src/semantics/protocol/canonical.rs:535`; `src/semantics/protocol/normalize.rs:59`

**Expected:** R-1 (the version the host writes) and R-3 (the version it accepts) are one number. `design.md` §9 names restatement drift as the slice's recurring defect class; `canonical.rs:292-297` refuses three copies of a uniqueness walk for that reason.
**Observed:** `const PROTOCOL_VERSION: u32 = 1;` is declared privately in both files. A bump in one leaves the host emitting one version and refusing responses that echo it, and nothing ties the two: the R-1 tests read the emitted value and the R-3 fixture hard-codes `2`.
**Evidence:** `grep -rn PROTOCOL_VERSION src/` returns the two declarations and one use of each.

**Disposition:**
**Response:**

**Outcome:**

### F-28 — Config's negative-duration and unparseable-duration branches have no test

**Severity:** minor
**Location:** `src/shell/config.rs:126-131`; tests at `config.rs:174-253`

**Expected:** `config.rs:117-119`: "Both `\"0s\"` and `\"-1s\"` parse … so each rejection EX-1 asks for is a check that had to be written." `design.md` §5.5: `timeout = "0s"` or `default_poll = "0s"` rejected at load; `ConfigError::Duration` exists for `"1 month"` and non-durations (`error.rs:114-127`).
**Observed:** the test module covers the example loading, a missing file, a missing section, an empty command and the two zero cases. Nothing exercises `resolved.is_negative()` or either `map_err(named)` path, so `ConfigError::Duration` is constructed by no test and `"-30m"` being refused is asserted nowhere.
**Evidence:** `grep -n '"-1s"\|Duration' src/shell/config.rs` finds the comment on line 117 and the two constructors on lines 121-129; no `#[test]` body mentions either.

**Disposition:**
**Response:**

**Outcome:**

### F-29 — `Fields` and `Hints` derive `Default`, a public constructor beside the checked one

**Severity:** nit
**Location:** `src/semantics/protocol/canonical.rs:126-127`, `:379-380`

**Expected:** `canonical.rs:1-8` and I1: "Outside this module a canonical value can only have come out of `normalize_response`." `design.md` §5.2: canonical types are "constructible only via normalize".
**Observed:** `Fields::default()` and `Hints::default()` are public and construct canonical values from nothing. Both are empty and hold the invariants trivially, so the leak is of the rule rather than of a bad value; but `Fields::new` exists precisely to be the door, and `Default` is a second one the module doc says does not exist.
**Evidence:** the derives cited; `Options`, `Alternatives` and `NumberRange` do not derive it, so the asymmetry is unexplained.

*Independently raises the same defect as F-11 (audit agent). Disposition follows F-11's.*

**Disposition:**
**Response:**

**Outcome:**

### F-30 — Two host-side impossibilities are reported in the backend's voice

**Severity:** nit
**Location:** `src/shell/backend/process.rs:60-63`, `:67-72`; `src/shell/error.rs:192`, `:203`

**Expected:** root `AGENTS.md`: "Every refusal is reported and says which side was wrong." `error.rs:43-46` defines `Protocol` as "the bytes arrived and did not mean anything the protocol admits".
**Observed:** a host-authored request that fails to serialize is returned as `BackendError::Protocol(ProtocolError::Json(_))`, whose `Display` is "backend response rejected: malformed JSON: …"; an empty command vector is returned as `BackendError::Spawn`, whose `Display` is "backend could not be spawned: …". Both name a backend that did nothing.
**Evidence:** lines cited. Both are unreachable in practice (config refuses an empty command; `Request` serializes infallibly), which is why this is a nit rather than a defect.

*Independently raises the same defect as F-3 (audit agent). Disposition follows F-3's.*

**Disposition:**
**Response:**

**Outcome:**

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

**Disposition:**
**Response:**

**Outcome:**

### F-32 — The `option_option` expectation's reason string carries runs of embedded whitespace

**Severity:** nit
**Location:** `src/semantics/protocol/wire.rs:49-52`

**Expected:** a multi-line string literal in an attribute uses a trailing `\` to elide the newline and indentation, as `state.rs:41-42` does in the same crate.
**Observed:** the reason is one physical line whose wrapped segments were joined with their indentation intact, so the string contains fifteen-space runs mid-sentence; `cargo fmt` leaves it because attributes are not reformatted.
**Evidence:** `sed -n 51p src/semantics/protocol/wire.rs | cat -A` shows `asserts               there is nothing to show`.

**Disposition:**
**Response:**

**Outcome:**

### F-33 — `Outcome`, `Failure` and `Discarded` carry errors that implement `Display`, but none of the three does

**Severity:** nit
**Location:** `src/shell/host.rs:44-48`, `:57-84`; `src/semantics/protocol/normalize.rs:49-55`

**Expected:** brief §13 asks that the host "log enough information to debug the backend"; the crate denies `use_debug`, so the intended rendering of a refusal is `Display`, which every leaf type provides (`semantics/error.rs:95-146`, `shell/error.rs:136-221`).
**Observed:** the values a caller is actually handed — `Failure`, `Discarded`, `Outcome` — derive `Debug` only. The test harness had to write `describe_outcome` (`tests/integration/harness.rs:369-376`) to render one, and slice 002's renderer will have to do the same.
**Evidence:** the derives cited; no `impl fmt::Display` exists for any of the three (`grep -n "impl fmt::Display" src/shell/host.rs src/semantics/protocol/normalize.rs` is empty).

**Disposition:**
**Response:**

**Outcome:**

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

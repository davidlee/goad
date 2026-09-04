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

## Synthesis

<!-- Written when the ledger resolves. The closure story: what the review
     changed, what it confirmed, and the risks it knowingly leaves standing. A
     reader who trusts this section should not need to read the findings. -->

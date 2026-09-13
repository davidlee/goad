# Notes — Slice 005: `goad emit`

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 | done | 2026-09-11 |
| PHASE-02 | done | 2026-09-14 |
| PHASE-03 | pending | |
| PHASE-04 | pending | |

## Phase sheets

<!-- One block per phase, written at phase-plan time, immediately before
     execution. Disposable detail — it exists to get one agent through one
     phase. -->

### PHASE-01 — four lifts, and nothing else

**Objective:** the configuration-path rule, the output sink, the wall clock and
the reply's wire type are all reachable from stratum 2, with no behaviour changed
anywhere.

**Entry criteria: met.** EN-1 — `just check` exit 0 on a clean tree at
`9471053`, run before anything was edited. Transcript:
`/tmp/claude-1000/-home-david-dev-goad/b49ddf85-…/scratchpad/gate-EN1-phase01.txt`.

**Reading list**

Binding design: `design.md` §5.2 (the four signatures and `wire::Reply`'s
fields), D-9 (why the clock moves rather than being copied), D-11 (`protocol`
and `accepted` optional on read), §8/R-1 and R-2 (the two regression risks this
phase carries). Binding plan: `plan.md` PHASE-01 EX-1..EX-7, VT-1..VT-3, VA-1,
and S-2/S-5.

Lift A — the configuration path:
- `crates/goad/src/startup.rs:85-130` — `arguments`, its doc table, and the `[]`
  arm whose XDG/`HOME` logic moves. `StartupError::NoConfigPath` at `:26-28`,
  its sentence at `:52-54`.
- `crates/goad-shell/src/config.rs:1-13` — the module's doc comment and its
  note on why the arithmetic lint is not crate-wide here; new code joins this.

Lift B — the sink:
- `crates/goad/src/diagnostics.rs:306-317` — `line_to` and the comment on why
  both outcomes are matched (`design.md` §5.4 of slice 003). `USAGE` and
  `print_usage` sit immediately below and **do not move** (EX-2).

Lift C — the clock:
- `crates/goad/src/clock.rs` — whole; 70 lines. The `Clock` **type alias** at
  `:16`, `ClockError` at `:19`, `wall_clock` at `:59`, and the comment at
  `:43-47` recording why `jiff::Timestamp::now()` is refused.
- The ten sites EX-7 enumerates, all confirmed present at the stated lines:
  `src/lib.rs:3`, `src/startup.rs:34`, `src/main.rs:7` (`:53` and `:116` are
  uses through that one import), `src/controller.rs:18` (the alias) and `:769`
  (the test module's `ClockError` and `wall_clock`), plus five test files —
  `tests/event_loop/closing.rs:16`, `tests/event_loop_schedule/scheduling.rs:21`,
  `tests/renderer/harness.rs:13`, `tests/renderer/scheduling.rs:13`,
  `tests/renderer/startup.rs:17`.
- Three prose sites that break nothing: `tests/renderer/harness.rs:30`,
  `tests/renderer/scheduling.rs:609`, `crates/goad/Cargo.toml:11`.

Lift D — the reply's wire type:
- `crates/goad-shell/src/ingress/mod.rs:574-583` — the private `Wire<'a>`;
  `:585-591` its doc comment on the newline terminator, which stays with
  `reply`; `:592-616` — `reply`, the three lines EX-5 changes and the
  `#[expect(clippy::unwrap_used, …)]` that stays.
- `crates/goad-shell/src/ingress/mod.rs:936-945` — the existing
  `a_reply_is_one_newline_terminated_line` test, which VT-3's exact-byte case
  joins.
- `crates/goad-shell/src/ingress/mod.rs:1-13` — the module doc and
  `#![deny(clippy::arithmetic_side_effects)]`; `pub mod wire;` joins
  `pub mod envelope;` at `:26`.

**Assumptions & STOP conditions**

Verified before starting, not taken on faith:
- A-1 — `goad-shell` already carries `jiff` (`crates/goad-shell/Cargo.toml:14`)
  and `jiff` is on stratum 2's allowlist (`allowlist.rs:19-26`). `wall_clock`
  uses `from_nanosecond` and `jiff::Error` only, both available under
  `default-features = false` (`Cargo.toml:35`). **The clock lift therefore adds
  no manifest entry and enables no feature — S-5 does not fire.**
- A-2 — `crates/goad` already depends on `goad-shell`
  (`crates/goad/Cargo.toml:18`), so lifts A, B and C add no dependency edge.
- A-3 — 004's wire tests assert over **parsed JSON**, not bytes
  (`tests/integration/ingress.rs:185-186`). **VT-3 is therefore a new
  assertion, not an edit to an existing one** — EX-6 is not breached by adding
  it, and without it nothing in the workspace would catch
  `{"protocol":null,"accepted":null}`.
- A-4 — `crates/goad` keeps `jiff` after the lift: `diagnostics.rs:293-296`
  names `jiff::Unit` and `jiff::RoundMode` in production code. Only the
  *comment* at `Cargo.toml:11` goes stale.
- A-5 — serde serializes struct fields in declaration order, so preserving the
  field order of `Wire` in `Reply` preserves the host's bytes. VT-3 is what
  makes this an assertion rather than an assumption.

STOP and consult, per `plan.md` §Sequencing:
- S-1 — a normative sentence is needed in `docs/specs|policy|adr`. Tier-2 signal.
- S-2 — an existing **assertion or fixture** must change to stay green. A `use`
  line following a moved module is the bounded, enumerated exception (EX-7);
  anything else in a test diff is S-2.
- S-5 — the clock lift turns out to need a feature on a shared dependency.
  Believed dead by A-1; if A-1 is wrong, stop.
- Local: if `line_to` cannot move without dragging `USAGE`/`print_usage` with
  it, that is a design question (OQ-4 argued the move is worth it for four
  lines), not a thing to improvise past.

**Tasks**
<!-- [ ] todo · [~] in progress · [x] done · [!] blocked -->
- [x] EN-1 — gate green on a clean tree at `9471053`, transcript kept.
- [x] Lift A — `config::default_path` + its doc table (EX-1) at
      `crates/goad-shell/src/config.rs:22-51`; `startup::arguments`' `[]` arm
      delegates in three lines; VT-1's five rows are tests in that module;
      `startup.rs`'s own tests untouched and green.
- [x] Lift B — `goad_shell::report::line_to` (EX-2) with its comment;
      `diagnostics.rs` imports it and defines no sink; `USAGE`/`print_usage`
      stayed. `report.rs` gained two cases of its own — a `Vec<u8>` sink and a
      sink whose every write fails — so "best effort" is asserted rather than
      only described.
- [x] Lift C — `clock.rs` moved whole by `git mv` (EX-3), `pub mod clock;` added
      to `goad-shell` and removed from `crates/goad/src/lib.rs`; all ten EX-7
      sites follow, plus the three prose sites.
- [x] Lift D — `ingress/wire.rs` with `pub struct Reply` (EX-4); `reply()`
      builds it, three lines changed (EX-5); VT-2 across both modules; VT-3's
      exact-byte case.
- [x] VA-1 — `git diff` over `crates/goad` is the two call sites, seven import
      or comment lines, the deletion of `line_to` and of `pub mod clock;`, and
      the file move. Nothing else.
- [x] `just check` exit 0. EX-6 confirmed by reading the test-file diff: five
      `use` lines and two doc-comment lines across the five files, no assertion
      and no fixture touched.

**Decisions taken during execution**

- **VT-3 is written as a characterization test, and it was green before the
  lift.** It asserts `reply`'s bytes on the pre-lift code, then again after —
  which is the only ordering in which a regression net for a lift means
  anything. Red/green would have meant writing a test that *wanted* the bytes
  to change.
- **No field of `wire::Reply` carries `#[serde(default)]`**, where EX-4 said
  three of them would. Measured: serde reads a missing `Option` field as `None`
  without it, so the attribute is inert on all five — and inert on three fields
  but absent on two would imply a difference in read behaviour that does not
  exist. The struct's doc comment now says this, and
  `an_empty_object_parses_and_carries_nothing` holds it. EX-4's *purpose*
  (absent-tolerant on read, D-11) is discharged; only its spelling differs.
- **The three prose sites were tidied**, which EX-7 permits and does not
  require. `crates/goad/Cargo.toml`'s comment is the one that mattered: it
  justified the `jiff` entry by `clock.rs`, which is no longer there, and it now
  names `diagnostics.rs`'s `TimestampRound` — still true, and still in
  production code rather than a test.
- **The clock module's own prose moved with it.** Two sentences were about its
  stratum, and both would have been false at stratum 2: the module doc now says
  why it sits there (005/D-9), and `wall_clock`'s comment says features unify
  across the *workspace* build rather than naming stratum 3.

**Findings**

- `StartupError::NoConfigPath`'s sentence and `diagnostics::USAGE` both still
  name `XDG_CONFIG_HOME` and `HOME` in prose. That is host-facing text rather
  than the rule, and EX-1's "appears once in the workspace" is about the logic,
  which does. Worth an eye at audit: three places now describe one rule, and
  only one of them is executable.

### PHASE-02 — the client half of SPEC-003

**Objective:** a caller in stratum 2 can send an envelope to a real listener and
get a normalized answer, with every failure shape named.

**Entry criteria: met.** EN-1 — PHASE-01's exit criteria discharged (`notes.md`
above, all seven ticked) and `just check` exit 0 on a clean tree at `4dc9bb5`,
re-run before anything was edited. Transcript:
`…/scratchpad/gate-EN1-phase02.txt`.

**Reading list**

Binding design: `design.md` §5.2 (the two signatures, `Answered`, `SendFault`
verbatim), §5.5 (which reply shape is which fault — the `{}` ruling, `protocol`
optional, the unknown token, `retry_after_ms` reported not obeyed, `source:
"host"` not pre-empted), D-1, D-2, D-3, D-11, and §9's tier list (**this tier is
where R-6's framing is held**). Binding plan: `plan.md` PHASE-02 EX-1..EX-5,
VT-1..VT-5, VA-1, and S-1/S-3/S-4.

Normative: `SPEC-003` §6.2 (the envelope's four keys), §6.3 (the reply's five
fields, and what makes one non-conforming), §6.4 (the unbounded wait), R-8
(a close with nothing on it), R-13 (the reserved source), R-14 (rounding).

What the client is answering:
- `crates/goad-shell/src/ingress/wire.rs:30-40` — `Reply`, all five fields
  `Option`, permissive on read. Its four tests say what already parses.
- `crates/goad-shell/src/ingress/mod.rs:581-605` — `reply()`, the bytes this
  client reads. `:448-472` — `Refusal`, the seven variants; `:479-490` —
  `reason()`, the closed set of eight tokens.
- `crates/goad-shell/src/ingress/mod.rs:156` — `bind`, and the `# Errors`
  section every public `-> Result` here carries (`clippy::pedantic` denies
  `missing_errors_doc`).

Prior art to copy rather than re-invent:
- `crates/goad/tests/renderer/ingress.rs:128-152` — `write_one`/`send`: the
  blocking `std::os::unix::net::UnixStream` writer on `spawn_blocking`, with the
  module doc at `:14-19` explaining why the writer's side is `std` and not
  `tokio`. **That is the shape `client::send` takes**, minus the panics.
- `crates/goad-shell/tests/integration/ingress.rs:80-123` — `judge`, and `:56-67`
  `Verdict`/`Seen`. Extend; do not mint a second (EX-2's F-7 note names the
  collision that would follow).
- `crates/goad-shell/tests/integration/ingress.rs:29-47` — `socket_path` and
  `cleanup`, which every case here needs too.
- `crates/goad-shell/tests/integration/ingress.rs:22-26` — `GOOD`, the one-line
  envelope; `:206-223` — `padded_envelope`, for `too_large`.

**Assumptions & STOP conditions**

Verified before starting, not taken on faith:
- A-1 — `goad-semantics::protocol::canonical::Event` derives `Serialize`
  (`canonical.rs:489-496`), so EX-4's "no envelope struct of its own" is
  `serde_json::to_string(event)` and nothing more. Its key set is PHASE-03/VT-5's
  to pin, not this phase's.
- A-2 — **no new dependency.** `std::os::unix::net` is std; `serde_json` and
  `goad-semantics` are already `goad-shell`'s (`Cargo.toml:12-18`). S-3 does not
  fire, and the manifest allowlist is untouched.
- A-3 — `tokio`'s `rt` feature is on workspace-wide (`Cargo.toml:36-37`), so
  `spawn_blocking` is available in the integration target. It is **required**,
  not stylistic: `#[tokio::test]` is a current-thread runtime and the `judge` is
  a task on it, so a blocking `send` called inline deadlocks rather than fails.
- A-4 — `timed_out` is **not reachable through a conforming `send`**: the
  listener's `ENVELOPE_DEADLINE` fires only on a writer that connects and
  completes nothing, and `send` always writes a whole envelope and shuts the
  write half. So VT-2's "every remaining shape the fixture can produce" is
  `malformed`, `invalid_envelope`, `too_large` and `unavailable` from the
  listener, plus `engaged` and `too_soon` scripted through `Verdict::Refuse`.
  Its absence is the phase's R-6 evidence, not a gap: a client that framed
  wrongly would draw `timed_out` and red VT-1 (`design.md` §9).
- A-5 — `SendFault` cannot derive `PartialEq`: `io::Error` and
  `serde_json::Error` are not `PartialEq`. Cases over it match on the variant.

STOP and consult:
- S-1 — a normative sentence is wanted in `docs/specs|policy|adr`. §6.3 already
  says everything `read_reply` decides; needing more is the tier-2 signal.
- S-3 — anything at all wants adding to `crates/goad-shell/Cargo.toml`.
  Believed dead by A-2.
- S-4 — an assertion has to settle for emit's own bytes where a normalized
  `Event` was the subject. VT-1 reads the `Event` the **real** listener produced,
  off `Seen::Event`; if that route does not work, raise it.
- Local — if `read_reply`'s ruling on a shape is not already settled by §6.3 or
  §5.5, that is a design question. The four settled rulings are in Tasks below.

**Tasks**
<!-- [x] todo · [~] in progress · [x] done · [!] blocked -->
- [x] EN-1 — gate green on a clean tree at `4dc9bb5`, transcript kept.
- [x] `read_reply` first, red: the pure half, with VT-5's cases beside it in
      `client.rs`'s own `mod tests`. The four rulings, all from §6.3/§5.5:
      `accepted: None` ⇒ `NonConforming` (a `&'static str` saying which field);
      `Some(true)` ⇒ `Accepted` whatever `protocol` says (D-11);
      `Some(false)` with `reason: None` ⇒ `NonConforming`;
      `Some(false)` with a token ⇒ `Refused`, token verbatim, unknown or not.
- [x] `Answered` and `SendFault` (EX-2), `Debug` on both, every variant
      constructed by something before the phase ends.
- [x] `send` (EX-1, EX-5): `UnixStream::connect` ⇒ `Unreachable` on error;
      write the line and `\n`, `shutdown(Shutdown::Write)`; `BufReader::read_line`
      ⇒ zero bytes is `NoReply` (R-8), an `io::Error` is `Faulted`;
      `serde_json::from_str::<wire::Reply>` ⇒ `Unreadable`; then `read_reply`.
      No `tokio` import in the file.
- [x] `pub mod client;` beside `pub mod wire;` (`ingress/mod.rs:28`).
- [x] Integration cases (VT-1..VT-4) in `tests/integration/ingress.rs`, each
      through `spawn_blocking`, against the extended `judge`.
- [x] VT-3's second half — the unknown token — needs a **fake** listener, since
      the host cannot produce a ninth token: a blocking `std` `UnixListener` on
      its own thread, reading one line and writing one canned reply.
- [x] VA-1 — inject the defect each negative case guards, watch it red for its
      own reason, revert. Two injections where a case asserts two absences.
- [x] Refactor pass, then `just check` exit 0.

**Decisions taken during execution**

- **`envelope_line` returns a `String`, not a `Result`.** `clippy::unwrap_in_result`
  is denied and is *not* scoped away by `clippy.toml`, so `ingress::reply`'s
  precedent — a host-authored value of primitives serializes infallibly, and a
  failure is a defect rather than a caller's mistake — only transfers to a
  function that does not return a `Result`. Extracting the serialization is
  what makes the claim local: that function is the whole of what is claimed
  infallible. The alternative was a sixth `SendFault` variant, which EX-2
  forbids twice over — the five are `design.md` §5.2's, and a variant nothing
  constructs is not "constructed somewhere".
- **`answer(&str)` is a private step between the wire and the rule.**
  `SendFault::Unreadable` is a property of *bytes*, and `read_reply(Reply)` —
  EX-3 fixes that signature — never sees bytes. VT-5 asks for `[1,2]` and
  `not json` at the no-socket tier, so the parse needs a testable home that is
  not `send`.
- **`Answered` derives `PartialEq`; `SendFault` cannot.** `design.md` §5.2
  shows no derives and EX-2 asks for `Debug` on both. `PartialEq` on `Answered`
  is what lets a case assert a refusal *whole* — token, advice and detail in
  one assertion rather than three. `SendFault` carries `io::Error` and
  `serde_json::Error`, neither of which is `PartialEq`; its cases match on the
  variant.
- **No `Display` or `Error` impl on `SendFault`.** `design.md` §5.2 gives the
  rendering to `goad-emit`'s `render::fault_line`, and a second rendering here
  would drift from it. PHASE-03 is where a fault becomes a line.
- **VT-2's coverage is by script, and A-4 undercounted.** The sheet said one
  token has no real trigger a conforming `send` can pull. It is **three**:
  `timed_out` (framing), and `malformed` and `invalid_envelope` (shape) — a
  client that serializes an `Event` cannot author a mis-shaped envelope at all.
  `reserved_source` is the exception that proves it: that rule is about a
  *value*, so a real `source: "host"` reaches the host and is refused there
  (AC-5). The other six tokens are scripted through `Verdict::Refuse`, which is
  how `retry_after_ms_is_absent_from_every_reason_but_too_soon` already covers
  the set.

**Findings**

- **VT-1 holds `SPEC-003/R-6`'s framing jointly, not the terminator.** VA-1
  measured it: `send` writes the `\n` *and* shuts the write half, and this
  listener admits either — dropping the terminator leaves the case green,
  dropping the shutdown leaves it green, dropping both reds it with `timed_out`
  and `nothing complete arrived within 500ms`. The doc comment said "the only
  thing holding R-6" before the injection and now says what was measured. Each
  mechanism is pinned separately by `mod.rs`'s own two envelope-termination
  cases; writing both is the permissive-wire invariant pointed at the *writing*
  side, and that reasoning is now recorded on `send`.
- **`SendFault::Faulted` is constructed in production and exercised by no
  case.** A connection that breaks mid-exchange has no deterministic trigger at
  this tier — a peer that closes early races the client's write. `plan.md`
  VT-4 asks only for `Unreachable` and `NoReply`, so this is stated residue
  rather than a missed case. Worth an eye at audit.
- **A mutation that leaves an unused import reds on the lint, not the
  assertion.** The shutdown-only injection first reported red because
  `use std::net::Shutdown;` became unused and `unused = "deny"` fired. A false
  red is worse than no injection: it says a case guards something it does not.
  Every VA-1 injection has to be lint-clean before its colour means anything.

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-09-14 · PHASE-02 complete · gate green

### Produced

- `goad_shell::config::default_path` — the XDG rule, with its table, tested at
  five rows. `startup::arguments` is its only caller today.
- `goad_shell::report::line_to` — the sink, with two cases of its own.
- `goad_shell::clock` — `wall_clock`, `ClockError`, the `Clock` alias; moved
  whole from `crates/goad`.
- `goad_shell::ingress::wire::Reply` — public, both directions, no
  `deny_unknown_fields`. `ingress::reply` builds it; the host's bytes are
  pinned by `the_reply_s_bytes_are_exactly_these`.
- `goad_shell::ingress::client` — `send`, `read_reply` (pure, public),
  `Answered`, `SendFault`. Blocking `std::os::unix::net` throughout, no
  `tokio`, no new dependency. Eight unit cases over §6.3 with no socket, seven
  integration cases against the real listener.
- `tests/integration/ingress.rs` gained `event_of`, `send_event` (the
  `spawn_blocking` wrapper every client case needs) and `fake_listener` — a
  blocking `std` listener on its own thread, for the two replies the host
  cannot produce: a ninth reason token and silence.

### Learned

- **serde reads a missing `Option` field as `None` with no `#[serde(default)]`.**
  Three plan lines assumed otherwise. The attribute is not wrong, it is inert.
- **004's wire assertions all go through `serde_json::from_str`**
  (`tests/integration/ingress.rs:185`), so until this phase nothing in the
  workspace could tell `"protocol":1` from `"protocol":null`. A wire contract
  wants at least one assertion over bytes.
- A lift's regression net is written *before* the lift and must be green when
  written. If it is red, it is not a net — it is a specification of a change.
- **A client that serializes a canonical type cannot author a bad envelope.**
  Three of the eight reason tokens have no real trigger `send` can pull; only
  `reserved_source` does, because R-13 is a rule about a value rather than a
  shape. That is why the host's rules are not duplicated client-side.
- **`clippy::unwrap_in_result` is live and unscoped**, so "this value
  serializes infallibly" only buys an `#[expect]` in a function that does not
  return a `Result`. Extract the infallible part; do not widen an error type
  for a case that cannot happen.
- **Injections must be lint-clean.** A mutation that orphans an import reds on
  `unused`, not on the assertion, and reports a case as load-bearing when it is
  not.

### Open

- Three places state the configuration-path rule: `default_path`'s table,
  `StartupError::NoConfigPath`'s sentence, `diagnostics::USAGE`. One is
  executable. See Findings.
- `SendFault::Faulted` has no case. No deterministic trigger at this tier; the
  plan did not ask for one. See PHASE-02 Findings.
- `SendFault` has no rendering. PHASE-03's `render::fault_line` owes each of
  the five variants a line, including the two that carry an error whose text
  is the only useful part.

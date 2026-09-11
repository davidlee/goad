# Notes — Slice 005: `goad emit`

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 | done | 2026-09-11 |
| PHASE-02 | pending | |
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

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-09-11 · PHASE-01 complete · gate green

### Produced

- `goad_shell::config::default_path` — the XDG rule, with its table, tested at
  five rows. `startup::arguments` is its only caller today.
- `goad_shell::report::line_to` — the sink, with two cases of its own.
- `goad_shell::clock` — `wall_clock`, `ClockError`, the `Clock` alias; moved
  whole from `crates/goad`.
- `goad_shell::ingress::wire::Reply` — public, both directions, no
  `deny_unknown_fields`. `ingress::reply` builds it; the host's bytes are
  pinned by `the_reply_s_bytes_are_exactly_these`.

### Learned

- **serde reads a missing `Option` field as `None` with no `#[serde(default)]`.**
  Three plan lines assumed otherwise. The attribute is not wrong, it is inert.
- **004's wire assertions all go through `serde_json::from_str`**
  (`tests/integration/ingress.rs:185`), so until this phase nothing in the
  workspace could tell `"protocol":1` from `"protocol":null`. A wire contract
  wants at least one assertion over bytes.
- A lift's regression net is written *before* the lift and must be green when
  written. If it is red, it is not a net — it is a specification of a change.

### Open

- Three places state the configuration-path rule: `default_path`'s table,
  `StartupError::NoConfigPath`'s sentence, `diagnostics::USAGE`. One is
  executable. See Findings.

# Notes — Slice 005: `goad emit`

Durable per-slice scratchpad and the only record of progress. Phase sheets are
expanded here just before execution and left in place; anything worth keeping
after the slice closes is lifted into the Harvest section.

## Status

| phase | state | as of |
|-------|-------|-------|
| PHASE-01 | in progress | 2026-09-11 |
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
- [ ] Lift A — `config::default_path` + its doc table (EX-1); `startup::arguments`'
      `[]` arm delegates; VT-1's five rows as tests; `startup.rs`'s own tests
      untouched and still green.
- [ ] Lift B — `goad_shell::report::line_to` (EX-2) with its comment;
      `diagnostics.rs` calls it and defines no sink; `USAGE`/`print_usage` stay.
- [ ] Lift C — `clock.rs` moved whole to `goad-shell` (EX-3), `pub mod clock;`
      added there and removed from `crates/goad/src/lib.rs`; all ten EX-7 sites
      follow; `crates/goad/src/clock.rs` deleted.
- [ ] Lift D — `ingress/wire.rs` with `pub struct Reply` (EX-4); `reply()` builds
      it, three lines changed (EX-5); VT-2's round-trip cases; VT-3's exact-byte
      cases.
- [ ] VA-1 — `git diff` over `crates/goad`: call sites, their imports, and the
      deletion of `clock.rs`, and nothing else.
- [ ] `just check` exit 0; EX-6 confirmed by reading the test-file diff — `use`
      lines only.

**Decisions taken during execution**
<!-- Small and local: how, within what the design already settled. -->

**Findings**
<!-- Things noticed in passing that are not this phase's job. -->

## Harvest

<!-- Updated in place, not appended. Ids and one-line hooks only — never
     restate content that lives elsewhere. -->

**Fresh as of:** 2026-09-11 · PHASE-01 phase plan · 9471053

### Produced
<!-- What now exists: modules, contracts, docs. -->

### Learned
<!-- Durable facts a future agent would otherwise rediscover. -->

### Open
<!-- Still unresolved at this point. Candidates for follow-ups. -->

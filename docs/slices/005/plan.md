# Plan — Slice 005: `goad emit`

The executable phase plan. Read with `design.md` — the plan never overrides the
design or canon; if it seems to, the plan is wrong.

## Overview

Four phases, in strict dependency order, each one agent-session sized.

The work is deliberately **bottom-up**: everything the CLI needs exists and is
tested before the CLI exists, so the binary itself is the smallest phase. Two of
the four touch no new behaviour — PHASE-01 moves four things down a stratum,
PHASE-04 spends most of its budget on evidence.

```
PHASE-01  four lifts, no new behaviour    goad-shell, goad
PHASE-02  the client half of SPEC-003     goad-shell
PHASE-03  the crate and the binary        goad-emit (new)
PHASE-04  the binary's evidence, the demo goad-emit tests, examples, justfile
```

## Sequencing & rationale

- **01 before 02** because the client needs `wire::Reply` public and
  deserializable and needs a clock it can reach; doing those inside the client
  phase mixes lifts with new behaviour in one diff.
- **02 before 03** because every client behaviour is testable against the real
  listener with no binary. If a wire question is first asked in the CLI phase,
  the design was wrong.
- **03 before 04** trivially: nothing spawns a binary that does not exist.
- **Could be reordered:** nothing. **Could be dropped:** nothing, though
  PHASE-04's `examples/` and `justfile` edits could slip to 006 — they are the
  only documentation-rather-than-behaviour part, and AC-8 needs them.

**STOP conditions**, at any phase:

- S-1 — a phase needs a normative sentence written in `docs/specs/`,
  `docs/policy/` or `docs/adr/`. That is the tier-2 signal (`design.md` §10).
- S-2 — an existing test's **assertion or fixture** must change to stay green in
  PHASE-01. That means a lift was a rewrite; stop and re-plan rather than adjust
  the assertion. **An import path changing is not that**, and is expressly
  allowed: a lift moves a module, so every `use` naming its old path must follow
  it. The allowance is bounded and enumerated — the seven sites in PHASE-01/EX-7,
  `use` lines only, `git diff` over them showing nothing but the import. A file
  outside that list, or a diff touching anything but a `use`, is S-2 again.
- S-3 — the client needs `tokio`, or `goad-emit` needs any dependency —
  including a **dev-dependency** — beyond `goad-shell`, `goad-semantics` and
  `serde_json`. A runtime in `dev-dependencies` is still a runtime
  (`crates/goad-boundary/tests/checks/allowlist.rs:49-53`).
- S-4 — AC-6's case cannot read a **normalized `Event`**, and the nearest
  available assertion is over emit's own bytes. That is R-3's proxy; raise it
  rather than settle for it.
- S-5 — PHASE-01's clock lift cannot be done without enabling a feature on a
  dependency shared with stratum 1. That is POL-001's residue, it reverses a
  prior slice's decision (`crates/goad/src/clock.rs:43-47`), and it is a design
  question, not an implementation one.

## Coverage

| AC | discharged by |
|----|---------------|
| AC-1 | PHASE-02/VT-1 (the exchange), PHASE-04/VT-1 (exit 0 from the binary) |
| AC-2 | PHASE-02/VT-2, VT-3 (refusals, `retry_after_ms`, unknown token), PHASE-03/VT-4 (the rendered line), PHASE-04/VT-2 (exit 1). **Its last clause — "nothing branches on `detail`" — is review-held, not tested**, in the form SPEC-003 §7 uses for such a clause: `detail` is an absence of code, and a test that renders a line containing it cannot tell appending from branching. What review checks: `refused_line` is the one function reading `detail`, it interpolates the string and no arm matches on it |
| AC-3 | PHASE-02/VT-4, VT-5 (unreachable, no reply, unreadable, non-conforming), PHASE-03/VT-2 (usage errors), PHASE-04/VT-3 (exit 2) |
| AC-4 | PHASE-01/VT-1 (`default_path`'s rows), PHASE-03/VT-3 (`--socket` wins and reads no config), PHASE-03/VT-6 (an absent, unreadable or unparseable config, and one with no `[ingress]`, are exit 2 naming the path) |
| AC-5 | PHASE-02/VT-2 (`reserved_source` reported, not pre-empted) |
| AC-6 | PHASE-04/VT-4 |
| AC-7 | PHASE-03/VA-1 (the manifest), PHASE-04/VA-1 (`cargo tree`) |
| AC-8 | PHASE-04/VH-1 |

---

## PHASE-01 — four lifts, and nothing else

**Objective:** the configuration-path rule, the output sink, the wall clock and
the reply's wire type are all reachable from stratum 2, with no behaviour changed
anywhere.

**Surfaces:** `crates/goad-shell/src/{config.rs, report.rs (new), clock.rs (new),
lib.rs}`, `crates/goad-shell/src/ingress/{mod.rs, wire.rs (new)}`,
`crates/goad/src/{lib.rs, startup.rs, diagnostics.rs, clock.rs (removed), main.rs,
controller.rs}`, and — **import lines only** — `crates/goad/tests/{event_loop/closing.rs,
event_loop_schedule/scheduling.rs, renderer/harness.rs, renderer/scheduling.rs,
renderer/startup.rs}`.

**Entry**
- EN-1 — `just check` exit 0 on a clean tree at the slice's base commit, re-run
  before anything is edited, transcript kept.

**Exit**
- EX-1 — `goad_shell::config::default_path(env) -> Option<PathBuf>` exists with
  the doc table of its rows. `startup::arguments`' `[]` arm calls it and maps
  `None` to `StartupError::NoConfigPath`; the XDG and `HOME` logic appears once
  in the workspace.
- EX-2 — `goad_shell::report::line_to` exists, with the comment explaining why
  both outcomes are matched. `crates/goad/src/diagnostics.rs` calls it and
  defines no sink of its own; its `*_line` functions do not move.
- EX-3 — `goad_shell::clock::{wall_clock, ClockError, Clock}` exists, moved
  whole, including the comment recording why `jiff::Timestamp::now()` is not used
  and the `Clock` **type alias** that appears in four `controller.rs` signatures
  (`:274, :477, :532, :581`). **No feature is added to any dependency** (S-5).
- EX-4 — `goad_shell::ingress::wire::Reply` is public and derives `Debug`,
  `PartialEq`, `Serialize`, `Deserialize`. `protocol` and `accepted` are
  `Option`, written unconditionally and permissive on read (`design.md` D-11);
  `reason`, `retry_after_ms` and `detail` keep `skip_serializing_if` and gain
  `#[serde(default)]`. No `deny_unknown_fields`, with the reason in its doc
  comment citing CLAUDE.md's permissive-wire invariant.
- EX-5 — `ingress::mod::reply` builds a `wire::Reply`. **Three lines change**,
  and the first two are what keep the host's bytes identical: `protocol: 1`
  becomes `protocol: Some(1)`, `accepted` becomes `Some(accepted)`, and
  `reason: refusal.map(Refusal::reason)` becomes an owning form now the field is
  `String` (`mod.rs:598-600`). Nothing else in that function moves, the `\n`
  terminator included.
- EX-7 — `crates/goad/src/clock.rs` is deleted and **every site naming it
  follows the module**, enumerated so the sweep cannot stop early
  (`docs/memory/a-repair-sweep-misses-the-binding-site.md`): `src/lib.rs:3`
  (`pub mod clock;` removed), `src/main.rs:7`, `src/controller.rs:18` (the `Clock`
  alias) and `:769` (the test module's `ClockError`), `src/startup.rs:34`
  (`StartupError::Clock`), and the five test files in Surfaces. A re-export is not
  the escape: `clippy::pub_use = "deny"`.
- EX-6 — every existing test passes with **no assertion and no fixture changed**;
  the only test-file edits in this phase are the `use` lines EX-7 enumerates
  (S-2's bounded allowance).

**Verification**
- VT-1 — `default_path`'s rows: `XDG_CONFIG_HOME` absolute; set but relative
  (ignored, falls to `HOME`); unset with `HOME` set; `HOME` empty; both absent.
  `startup.rs`'s existing tests stay as they are and still pass.
- VT-2 — a `wire::Reply` serialized by `reply()` parses back to an equal value;
  one carrying an unknown key parses; one carrying neither `protocol` nor
  `accepted` parses (both are `Option` now) and is left for PHASE-02 to judge.
- VT-3 — the host's reply bytes are unchanged: `reply(true, None)` and one
  refusal still produce exactly the strings 004's tests assert.
- VA-1 — `git diff` shows `crates/goad` changed only at the call sites and their
  imports, plus the deletion of `clock.rs`.

**Notes for the implementer**

`line_to` is four lines; the temptation is to inline it and skip the move. Don't —
OQ-4 recorded the argument.

**The clock lift is the one with reach, and EX-7 is the list to work from rather
than a grep you trust.** Two sites are easy to miss: `controller.rs:18` imports
the `Clock` **type alias** — a different symbol from the `ClockError` its test
module imports at `:769` — and `src/lib.rs:3` declares the module at all. Five
test files then name `goad::clock::…` by path and must follow it; that is the
bounded allowance S-2 carves out, not a breach of it. (`renderer/startup.rs:179`
mentions `line_to` in a doc comment only: stale prose, same allowance, no compile
break.)

`Wire`'s doc comment explains why the reply is built with `serde_json` rather
than interpolated; keep it, it is still true.

---

## PHASE-02 — the client half of SPEC-003

**Objective:** a caller in stratum 2 can send an envelope to a real listener and
get a normalized answer, with every failure shape named.

**Surfaces:** `crates/goad-shell/src/ingress/client.rs` (new),
`crates/goad-shell/src/ingress/mod.rs` (the `mod` line),
`crates/goad-shell/tests/integration/ingress.rs`.

**Entry**
- EN-1 — PHASE-01's exit criteria discharged, `just check` exit 0.

**Exit**
- EX-1 — `client::send(socket: &Path, event: &Event) -> Result<Answered, SendFault>`:
  connect, write `serde_json::to_string(event)` plus `\n`, shut down the write
  half, read to the first newline or EOF, parse, then delegate to `read_reply`.
- EX-2 — `Answered` and `SendFault` exactly as `design.md` §5.2 gives them, every
  variant constructed somewhere, `Debug` on both. The name is `Answered`, not
  `Verdict`, which the fixture in the test file already owns (F-7).
- EX-3 — `read_reply(Reply) -> Result<Answered, SendFault>` is **pure and
  public**, so every §6.3 conformance rule is testable with no socket. Absent
  `accepted`, and `accepted: false` with no `reason`, are
  `SendFault::NonConforming` with a `&'static str` saying which. An unknown
  `reason` token is **not** a fault.
- EX-4 — `send` writes `Event`'s own `Serialize` output and builds no envelope
  struct of its own (D-3).
- EX-5 — no `tokio` import in `client.rs`; blocking `std::os::unix::net`
  throughout.

**Verification**
- VT-1 — accepted: a real `bind`, a judge that accepts, `send` returns
  `Answered::Accepted`, and the `Event` the listener normalized equals the one
  sent, `data` included.
- VT-2 — refusals: `reserved_source` from a real `source: "host"` envelope
  (AC-5, not pre-empted client-side), plus one scripted refusal per remaining
  shape the fixture can produce.
- VT-3 — `too_soon` carries `retry_after_ms` through to `Answered::Refused`; an
  unknown token (`"a_ninth_reason"`, from a fake listener rather than the host)
  yields `Refused` with the token verbatim.
- VT-4 — `SendFault::Unreachable` for a path with nothing listening;
  `SendFault::NoReply` for a listener that accepts and closes silently.
- VT-5 — over `read_reply`, no socket. **`{}` parses** — every field is `Option`
  — and is **`NonConforming`**, a JSON object breaching §6.3 by carrying no
  `accepted` (measured, F-13); `{"accepted":true}` is `Accepted`, because
  requiring `protocol` would narrow what emit takes (D-11);
  `{"protocol":1,"accepted":false}` is `NonConforming`; an unknown *field* beside
  the five is not a fault. `Unreadable`'s own cases are bytes that are not one
  JSON object: `[1,2]` and `not json`.
- VA-1 — each negative case has been seen to fail for its own reason: inject the
  defect it guards, run, read the message, revert
  (`docs/memory/a-green-test-can-assert-a-proxy.md`). Where a case asserts two
  absences, inject twice.

**Notes for the implementer**

004's `judge` fixture in `tests/integration/ingress.rs` already drives a real
`Ingress` and can accept or refuse on script; extend it rather than minting a
second. Tokio tests calling blocking `send` must use `spawn_blocking` — the shape
004's renderer tier already uses for the writer's side. A fake listener (for the
unknown-token case) is a blocking `std` `UnixListener` on its own thread.

---

## PHASE-03 — the crate, and the binary

**Objective:** `cargo run -p goad-emit -- …` performs the exchange; every part of
it that can be pure is.

**Surfaces:** `Cargo.toml` (one member entry), `crates/goad-emit/` (new, whole).

**Entry**
- EN-1 — PHASE-02's exit criteria discharged, `just check` exit 0.

**Exit**
- EX-1 — `crates/goad-emit/Cargo.toml`: `goad-shell`, `goad-semantics`,
  `serde_json`, workspace lints, and nothing else — **no `jiff`** (the clock
  comes from stratum 2 after PHASE-01) and no dev-dependencies (S-3, AC-7).
- EX-2 — `args::parse` is pure over `impl Iterator<Item = OsString>`, returns
  `Invocation` or `UsageError`, and carries the doc table of every row.
- EX-3 — `render`'s four functions are pure and return `String`. Success renders
  nothing at all (D-7).
- EX-4 — `main` is the only file reading env, clock, filesystem or socket. It
  returns `ExitCode`; `std::process::exit` is disallowed and is not used.
- EX-5 — `--socket` skips configuration loading entirely. Without it:
  `config::default_path`, then `Config::load`. `StartupFault` names the **four**
  ways that road ends at exit 2, split as AC-4 and VT-6 split them because the
  remedies differ: no path discoverable at all; the file absent or unreadable;
  the file unparseable; a config with `ingress: None`.
- EX-6 — `--help` on stdout exit 0; a usage error on stderr exit 2, naming the
  flag and not reprinting the usage block (`crates/goad`'s principle 4). The
  help text states that emit waits as long as the host takes (D-10) and that
  discovery covers the default configuration path only.

**Verification**
- VT-1 — `args::parse`: one case per doc-table row, plus missing `--source`,
  missing `--kind`, empty `--source`, unparseable `--data`, repeated flag,
  unknown flag.
- VT-2 — every `UsageError` renders a line naming the offending flag.
- VT-3 — `--socket` wins over a configuration naming a different path, and is
  honoured when no configuration file exists at all.
- VT-4 — `render::refused_line` shows the reason token, and `retry_after_ms`
  exactly when present; `detail` is interpolated (see the Coverage note on what
  review holds here rather than a test).
- VT-5 — the serialized envelope's key set is exactly
  `{source, kind, timestamp, data}` (`design.md` §5.5's assumption, pinned).
- VT-6 — each `StartupFault` renders a line naming the path and the fault:
  no path discoverable, file absent, file unparseable, no `[ingress]` section.
- VA-1 — `crates/goad-emit/Cargo.toml` names no `slint`, no `tokio`, no `jiff`,
  and no dev-dependency (AC-7, S-3).

**Notes for the implementer**

Copy `startup::arguments`' shape, including the doc table and the
argument-vector handling — it already solves `OsString`, the program-name skip
and `-h`. `--version` comes from `env!("CARGO_PKG_VERSION")`.

---

## PHASE-04 — the evidence, and the demo a person runs

**Objective:** the built binary's three exit codes are demonstrated against a
real socket, and a person has prompted a real evaluation with it.

**Surfaces:** `crates/goad-emit/tests/` (new), `examples/demo.toml`, `justfile`.

**Entry**
- EN-1 — PHASE-03's exit criteria discharged, `just check` exit 0.

**Exit**
- EX-1 — the integration target spawns the binary via
  `env!("CARGO_BIN_EXE_goad-emit")` against a **fake listener built on blocking
  `std::os::unix::net::UnixListener`** on its own thread, which reads one line,
  writes one canned reply and closes. No runtime, no `tokio`, no
  `ingress::bind` — which S-3 forbids here and which PHASE-02 exercises properly
  one tier up (F-2). What this tier holds is the **binary**: its exit codes, its
  stderr, and the bytes it puts on a socket. The module doc states the split
  precisely (F-15): **R-6's framing is held by PHASE-02/VT-1**, where a
  mis-framed write draws `timed_out` from the real listener; **R-7's byte and
  time bounds are held by 004's listener cases** and are not emit's to hold —
  emit does not second-guess them, exactly as it does not pre-empt R-13.
- EX-2 — `examples/demo.toml` shows the `goad-emit --socket ./goad-demo.sock`
  line; the raw `socat` one-liner **stays**, labelled as the no-CLI worked
  example, because SPEC-003 is what a second implementation is held to. The
  `justfile` gains an `emit` recipe wrapping the same invocation if it earns its
  place; `examples/shell/backend.sh` and `README.md` are **not** surfaces —
  neither contains a one-liner to change (F-12).
- EX-3 — `notes.md`'s Harvest carries anything durable; `audit.md`'s Evidence
  names AC-8's observation in the user's own account.

**Verification**
- VT-1 — exit 0 and empty stdout when the fake listener replies
  `{"protocol":1,"accepted":true}`.
- VT-2 — exit 1 and the reason token on stderr for a refusal reply; the
  `too_soon` case also shows `retry_after_ms`.
- VT-3 — exit 2 for a path with nothing listening, for a usage error, and for a
  reply that breaches §6.3.
- VT-4 — **AC-6**: the binary sends `--source w --kind k --data '{"n":4}'`, and
  the fake listener's bytes, passed through
  `goad_shell::ingress::envelope::normalize`, yield an `Event` carrying all three
  as sent. The assertion's subject is the **normalized `Event`**, never the raw
  bytes (S-4).
- VA-1 — `cargo tree -p goad-emit` contains no `slint` (AC-7).
- VA-2 — every AC in `slice-005.md` has a named passing test or a named
  argument, and the audit's closing walk re-runs them.
- VH-1 — **a person**: `just demo`, then, from another terminal,
  `cargo run -p goad-emit -- --socket ./goad-demo.sock --source hand --kind poke`,
  and the view changes. The `--socket` is not optional here: the demo starts the
  host on an explicit configuration whose socket is `./goad-demo.sock`, which
  emit's default-path discovery never finds (F-6). AC-8.

**Notes for the implementer**

`CARGO_BIN_EXE_<name>` is set only for test targets **in the binary's own
package**, which is why these tests live in `crates/goad-emit/tests/`. The
backend leg — that the host actually invoked the backend with this event — is
VH-1's, deliberately: no test target links both the CLI and a running host
(`slice-005.md` AC-6).

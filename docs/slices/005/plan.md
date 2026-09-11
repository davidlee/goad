# Plan — Slice 005: `goad emit`

The executable phase plan. Read with `design.md` — the plan never overrides the
design or canon; if it seems to, the plan is wrong.

## Overview

Four phases, in strict dependency order, each one agent-session sized.

The shape of the work is deliberately **bottom-up**: everything the CLI needs
exists and is tested before the CLI exists, so the binary itself is the smallest
and least interesting phase. Two of the four touch no new behaviour at all —
PHASE-01 moves three things down a stratum, PHASE-04 spends most of its budget
on evidence.

```
PHASE-01  three lifts, no new behaviour   goad-shell, goad
PHASE-02  the client half of SPEC-003     goad-shell
PHASE-03  the crate and the binary        goad-emit (new)
PHASE-04  the binary's evidence, the demo goad-emit tests, examples, justfile
```

## Sequencing & rationale

- **01 before 02** because the client needs `wire::Reply` to be public and
  deserializable, and doing that as part of the client phase would mix a lift
  with new behaviour in one diff — the shape that makes a review argue about two
  things at once.
- **02 before 03** because every client behaviour is testable against the real
  listener with no binary in the picture. If the CLI phase is where a wire
  question first gets asked, the design was wrong.
- **03 before 04** trivially: nothing can spawn a binary that does not exist.
- **Could be reordered:** nothing. **Could be dropped:** nothing, though
  PHASE-04's `examples/` and `justfile` edits could slip to 006 if a session
  runs out of room — they are the only part of the slice that is documentation
  rather than behaviour, and AC-8 needs them.

**STOP conditions**, at any phase:

- S-1 — a phase needs a normative sentence written anywhere in `docs/specs/`,
  `docs/policy/` or `docs/adr/`. That is the tier-2 signal (`design.md` §10).
- S-2 — an existing 004 ingress test must be edited to stay green in PHASE-01.
  That means a lift was a rewrite; stop and re-plan rather than adjusting the
  assertion.
- S-3 — the client needs `tokio` (`design.md` §5.5), or `goad-emit` needs any
  dependency beyond `goad-shell`, `goad-semantics`, `serde_json`, `jiff`.
- S-4 — AC-6's case cannot read the normalized `Event`, and the nearest
  available assertion is over emit's own bytes. That is `R-3`'s proxy; raise it
  rather than settle for it.

## Coverage

| AC | discharged by |
|----|---------------|
| AC-1 | PHASE-02/VT-1 (the exchange), PHASE-04/VT-1 (exit 0 from the binary) |
| AC-2 | PHASE-02/VT-2, VT-3 (refusals, `retry_after_ms`, unknown token), PHASE-03/VT-4 (the rendered line), PHASE-04/VT-2 (exit 1) |
| AC-3 | PHASE-02/VT-4, VT-5 (unreachable, no reply, unreadable, ambiguous), PHASE-03/VT-2 (usage errors), PHASE-04/VT-3 (exit 2) |
| AC-4 | PHASE-01/VT-1 (`default_path`'s rows), PHASE-03/VT-3 (`--socket` wins and reads no config; a config with no `[ingress]` is exit 2) |
| AC-5 | PHASE-02/VT-2 (`reserved_source` reported, not pre-empted) |
| AC-6 | PHASE-04/VT-4 |
| AC-7 | PHASE-03/VA-1 (the manifest), PHASE-04/VA-2 (`cargo tree`) |
| AC-8 | PHASE-04/VH-1 |

---

## PHASE-01 — three lifts, and nothing else

**Objective:** the configuration-path rule, the output sink and the reply's wire
type are all reachable from stratum 2, with no behaviour changed anywhere.

**Surfaces:** `crates/goad-shell/src/config.rs`, `crates/goad-shell/src/report.rs`
(new), `crates/goad-shell/src/ingress/mod.rs`, `crates/goad-shell/src/ingress/wire.rs`
(new), `crates/goad-shell/src/lib.rs`, `crates/goad/src/startup.rs`,
`crates/goad/src/diagnostics.rs`.

**Entry**
- EN-1 — `just check` exit 0 on a clean tree at the slice's base commit,
  re-run before anything is edited, transcript kept.

**Exit**
- EX-1 — `goad_shell::config::default_path(env) -> Option<PathBuf>` exists and
  carries the doc table of its rows. `startup::arguments`' `[]` arm calls it and
  maps `None` to `StartupError::NoConfigPath`; the XDG and `HOME` logic appears
  once in the workspace.
- EX-2 — `goad_shell::report::line_to` exists, with the comment explaining why
  both outcomes are matched. `crates/goad/src/diagnostics.rs` calls it and
  defines no sink of its own; its `*_line` functions do not move.
- EX-3 — `goad_shell::ingress::wire::Reply` is public, derives `Serialize` and
  `Deserialize`, drops the `'a` lifetime (`reason: Option<String>`), keeps every
  `skip_serializing_if`, and adds `#[serde(default)]` on the three optional
  fields. It carries **no** `deny_unknown_fields` — the reason is one sentence
  in its doc comment, citing CLAUDE.md's permissive-wire invariant.
- EX-4 — `ingress::mod::reply` builds a `wire::Reply` and is otherwise
  unchanged, terminator included.
- EX-5 — every existing test in the workspace passes **unedited**. If one needs
  editing, S-2 applies.

**Verification**
- VT-1 — `crates/goad-shell/tests/…` — `default_path`'s rows: `XDG_CONFIG_HOME`
  absolute; set but relative (ignored, falls to `HOME`); unset with `HOME` set;
  `HOME` empty; both absent. These are the cases `startup.rs`'s own tests assert
  today through `arguments` — the new ones assert the extracted rule directly,
  and the old ones stay as they are.
- VT-2 — a round-trip case: a `wire::Reply` serialized by `reply()` parses back
  to an equal value, and one carrying an unknown key still parses.
- VA-1 — `git diff --stat` shows no line of `crates/goad` changed except the two
  call sites and their imports.

**Notes for the implementer**

`line_to` is four lines; the temptation is to inline it and skip the move. Don't
— `design.md` D-2's argument is about one definition per judgement, and OQ-4
recorded it. `Wire`'s doc comment explains why the reply is built with
`serde_json` rather than interpolated; keep it, it is still true.

---

## PHASE-02 — the client half of SPEC-003

**Objective:** a caller in stratum 2 can send an envelope to a real listener and
get a normalized verdict, with every failure shape named.

**Surfaces:** `crates/goad-shell/src/ingress/client.rs` (new),
`crates/goad-shell/src/ingress/mod.rs` (the `mod` line),
`crates/goad-shell/tests/integration/ingress.rs`.

**Entry**
- EN-1 — PHASE-01's exit criteria discharged, `just check` exit 0.

**Exit**
- EX-1 — `client::send(socket: &Path, event: &Event) -> Result<Verdict, SendFault>`:
  connect, write `serde_json::to_string(event)` plus `\n`, shut down the write
  half, read to the first newline or EOF, parse, normalize.
- EX-2 — `Verdict` and `SendFault` exactly as `design.md` §5.2 gives them, all
  variants constructed somewhere, `Debug` on both.
- EX-3 — normalization is a **named, pure** function over the parsed
  `wire::Reply` — `Verdict` or `SendFault::Ambiguous` — so the ambiguity rules
  are testable with no socket. `accepted: false` with no `reason` is ambiguous;
  an unknown `reason` token is not.
- EX-4 — `send` writes the envelope as `Event`'s own `Serialize` output and
  builds no envelope struct of its own (D-3).
- EX-5 — no `tokio` import in `client.rs`; blocking `std::os::unix::net`
  throughout (S-3).

**Verification**
- VT-1 — accepted: a real `bind`, a judge that accepts, `send` returns
  `Verdict::Accepted`, and the `Event` the listener normalized equals the one
  sent, `data` included.
- VT-2 — refusals: `reserved_source` from a real `source: "host"` envelope
  (AC-5, not pre-empted client-side), and one scripted refusal per remaining
  shape the fixture can produce.
- VT-3 — `too_soon` carries `retry_after_ms` through to `Verdict::Refused`, and
  an unknown token (`"a_ninth_reason"`, written by a fake listener rather than
  the host) still yields `Refused` with the token verbatim.
- VT-4 — `SendFault::Unreachable` for a path with nothing listening;
  `SendFault::NoReply` for a listener that accepts and closes silently.
- VT-5 — `Unreadable` for bytes that are not JSON; `Ambiguous` for `{}` and for
  `{"protocol":1,"accepted":false}`; an unknown *field* beside the five parses
  fine and is **not** a fault.
- VA-1 — each negative case has been seen to fail for its own reason: inject the
  defect it guards, run, read the message, revert
  (`docs/memory/a-green-test-can-assert-a-proxy.md`).

**Notes for the implementer**

004's `judge` fixture in `tests/integration/ingress.rs` already drives a real
`Ingress` and can accept or refuse on script; extend it rather than minting a
second fixture. Tokio tests calling blocking `send` must use `spawn_blocking` —
the same shape 004's renderer tier uses for the writer's side.

---

## PHASE-03 — the crate, and the binary

**Objective:** `cargo run -p goad-emit -- …` performs the exchange; every part of
it that can be pure is.

**Surfaces:** `Cargo.toml` (one member entry), `crates/goad-emit/` (new, whole).

**Entry**
- EN-1 — PHASE-02's exit criteria discharged, `just check` exit 0.

**Exit**
- EX-1 — `crates/goad-emit/Cargo.toml`: `goad-shell`, `goad-semantics`,
  `serde_json`, `jiff`, workspace lints, and nothing else (S-3, AC-7).
- EX-2 — `args::parse` is pure over `impl Iterator<Item = OsString>`, returns
  `Invocation` or `UsageError`, and carries the doc table of every row.
- EX-3 — `render`'s four functions are pure and return `String` (or `None` for
  silence on success, D-7).
- EX-4 — `main` is the only file reading env, clock, filesystem or socket. It
  returns `ExitCode`; `std::process::exit` is disallowed and is not used.
- EX-5 — `--socket` skips configuration loading entirely; without it,
  `config::default_path` then `Config::load`, and `ingress: None` is exit 2 with
  a message saying the host is not configured to listen.
- EX-6 — `--help` on stdout exit 0; a usage error on stderr exit 2, naming the
  flag and not reprinting the usage block (`crates/goad`'s principle 4).

**Verification**
- VT-1 — `args::parse`: one case per doc-table row, plus missing `--source`,
  missing `--kind`, empty `--source`, unparseable `--data`, repeated flag,
  unknown flag, `--` handling if any.
- VT-2 — every `UsageError` renders a line naming the offending flag.
- VT-3 — `--socket` wins over a configuration that names a different path, and
  is honoured when no configuration file exists at all.
- VT-4 — `render::refused_line` shows the reason token, and `retry_after_ms`
  exactly when present; `detail` is appended but never parsed.
- VT-5 — the serialized envelope's key set is exactly
  `{source, kind, timestamp, data}` (`design.md` §5.5's assumption, pinned).
- VA-1 — `crates/goad-emit/Cargo.toml` names no `slint` and no `tokio` (AC-7).

**Notes for the implementer**

Copy `startup::arguments`' shape, including the doc table and the argument-vector
handling — it already solves `OsString`, the program-name skip, and `-h`.
`goad-emit`'s own `--version` comes from `env!("CARGO_PKG_VERSION")`.

---

## PHASE-04 — the evidence, and the demo a person runs

**Objective:** the built binary's three exit codes are demonstrated against a
real listener, and a person has prompted a real evaluation with it.

**Surfaces:** `crates/goad-emit/tests/` (new), `examples/demo.toml`,
`examples/shell/backend.sh`, `justfile`, `README.md` if it names the one-liner.

**Entry**
- EN-1 — PHASE-03's exit criteria discharged, `just check` exit 0.

**Exit**
- EX-1 — the integration target spawns the binary via
  `env!("CARGO_BIN_EXE_goad-emit")` against a listener bound in-process.
- EX-2 — `examples/demo.toml` and the `justfile` show the `goad-emit` line; the
  raw `socat` one-liner **stays**, labelled as the no-CLI worked example, because
  SPEC-003 is what a second implementation is held to.
- EX-3 — `notes.md`'s Harvest carries anything durable, and `audit.md`'s
  Evidence names AC-8's observation in the user's own account.

**Verification**
- VT-1 — exit 0 and empty stdout when the listener accepts.
- VT-2 — exit 1 and the reason token on stderr when it refuses.
- VT-3 — exit 2 for a path with nothing listening, and for a usage error.
- VT-4 — **AC-6**: the binary sends `--source w --kind k --data '{"n":4}'`, and
  the `Event` the listener normalizes carries all three as sent. This is the
  case S-4 protects: assert the normalized `Event`, never emit's own bytes.
- VA-1 — `cargo tree -p goad-emit` contains no `slint` (AC-7).
- VA-2 — every AC in `slice-005.md` has a named, passing test or a named
  argument, and the audit's closing walk re-runs them.
- VH-1 — **a person**: `just demo`, then
  `goad-emit --source hand --kind poke` from another terminal, and the view
  changes. AC-8.

**Notes for the implementer**

`CARGO_BIN_EXE_<name>` is set only for test targets **in the binary's own
package**, which is why this phase's tests live in `crates/goad-emit/tests/` and
not in `crates/goad`. The backend leg — that the host actually invoked the
backend with this event — is VH-1's, deliberately, because no test target links
both the CLI and a running host (`slice-005.md` AC-6).

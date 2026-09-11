# Slice 005: `goad emit`

**Stage:** design
**Tier:** 1 (thin) — see `docs/AGENTS.md` §Tiers. It writes no canon: SPEC-003
already specifies the envelope, the reply and the closed reason set, and this
slice is a *client* of that contract rather than an amendment to it. **The one
thing that could have raised it is settled**: OQ-3 is answered *no allowlist
row*, so no ADR-001 instrument and no POL-001 wording changes — see OQ-3 below
for why a row would have been new policy rather than applied policy.
**Depends on:** 004 (event ingress) — closed 2026-09-11.

## Purpose

004 gave the host a socket. Prompting an evaluation today means a `socat` or
`nc` one-liner that hand-writes the envelope, so every caller must know the wire
format, the reserved `source`, the RFC 3339 offset rule and the reply's shape.
That knowledge does not belong in a cron line.

Once this lands, a cron job, a shell hook or another program prompts an
evaluation with one command and reads the outcome from an exit code:

```zsh
goad-emit --source reddit-watcher --kind reddit-opened --data '{"count_last_hour":4}'
```

It is the last piece before 006, the slice after which the software is used
daily. Thin by construction: argument parsing, an envelope, a socket write, one
reply read, an exit code. If this needs a large design, the fault is in 004's
socket, not here.

## Scope

- **`crates/goad-emit/`** — a new workspace member with one binary. It depends
  on `goad-shell` (config, and the envelope's canonical types by re-export) and
  on nothing that links Slint. This is where all new code goes.
- **`Cargo.toml`** — one `workspace.members` entry, enumerated, never a glob.
- **`crates/goad-shell/`** — the production edits outside the new crate, all of
  them moves rather than new behaviour (OQ-2, OQ-4): `config::default_path`, the
  configuration-path rule lifted out of `crates/goad/src/startup.rs`; the
  reply's wire type made `pub` and given `Deserialize` beside the `Serialize`
  the host already writes with; the client half of SPEC-003's contract —
  connect, write, read one reply, normalize it — beside the listener it talks
  to; and `report::line_to`, lifted from `crates/goad/src/diagnostics.rs`.
- **`crates/goad/`** — the two call sites those moves leave behind.
- **`justfile`, `examples/demo.toml`, `examples/shell/backend.sh`** — the demo's
  documented one-liner becomes `goad-emit`; the raw `socat` line stays as the
  no-CLI worked example, because SPEC-003's contract is what a second
  implementation is held to.
- **Tests** — an integration tier in the new crate against a fake listener, and
  the end-to-end case that goes through a real host.

## Non-goals

- **Retry, backoff, queueing or coalescing.** SPEC-003/R-12 says the host holds
  no pending event and brief §7 puts coalescing in the watcher. `emit` reports
  `retry_after_ms` and exits; a caller that wants to wait is a shell loop.
- **Watching anything.** No inotify, no polling, no timers. `emit` is a one-shot
  write, invoked by something that already knows an event happened.
- **Reading events from stdin, or a stream of them.** One invocation, one
  envelope (R-6). `--data -` was considered and dropped; it returns if quoting
  large JSON on a command line actually hurts in 007.
- **A configuration file of its own.** It reads the host's, so there is one
  place the socket path is written.
- **Packaging, install paths, shell completions, a man page.** 006 and 009.
- **Interpreting an event.** Same boundary as the host's: `kind` and `data` are
  the caller's vocabulary, carried, never read.

## Acceptance criteria

- [ ] AC-1 — `goad-emit --source S --kind K [--data JSON]` writes one
      SPEC-003/§6.2 envelope to the socket and exits **0** when the reply is
      `accepted: true`. `timestamp` is the moment of invocation, RFC 3339 with
      an explicit offset (R-10).
- [ ] AC-2 — a reply with `accepted: false` exits **1** and writes the reason
      *token* to stderr; a `too_soon` refusal also writes `retry_after_ms`.
      Nothing branches on `detail` (R-14), and the reason set the CLI can print
      is SPEC-003's closed eight — an unknown token is reported verbatim rather
      than mapped or dropped, because a newer host is not a malformed one.
- [ ] AC-3 — no socket at the path, a connection that fails or faults, a reply
      that is not readable as one JSON object, and a usage error each exit
      **2**, with a message naming which of those happened and the path
      involved. Exit 2 never means "the host refused it".
- [ ] AC-4 — the socket path comes from the host's own configuration, found by
      the same rule the host uses (`$XDG_CONFIG_HOME/goad/config.toml`, else
      `$HOME/.config/goad/config.toml`); `--socket` overrides it and consults no
      configuration at all. A configuration with no `[ingress]` section is an
      exit-2 error saying the host is not configured to listen.
- [ ] AC-5 — a `source` of `"host"` is refused **by the host**, and `emit`
      reports that refusal like any other (exit 1, reason `reserved_source`).
      The CLI does not pre-empt the check: SPEC-003/R-13 is the host's
      requirement, and a client that duplicated it would make the host's own
      refusal untested from this side.
- [ ] AC-6 — the bytes a real invocation of the **built binary** puts on a real
      socket normalize to an `Event` carrying the `source`, `kind` and `data` as
      sent. Deliberately *not* phrased as "reaches the backend": no test target
      links both the CLI binary and a running host, and a case that claimed the
      backend leg while asserting something upstream of it would be exactly the
      proxy `docs/memory/a-green-test-can-assert-a-proxy.md` warns about. **The
      backend leg is AC-8's**, where a person sees the view change.
- [ ] AC-7 — `crates/goad-emit` links no renderer, and **the crate edge is what
      holds it**: its manifest names `goad-shell`, `goad-semantics`,
      `serde_json` and `jiff`, none of which names `slint`, so a `use slint::…`
      in the CLI is `error[E0433]`. No new gate instrument — see OQ-3. Evidence
      at audit is `cargo tree -p goad-emit` with no `slint` in it.
- [ ] AC-8 — **a person runs it** (`docs/AGENTS.md` §Tiers): `just demo`, then
      an `emit` from another terminal, and the view changes. Recorded in
      `audit.md` under Evidence.

## Governing canon

**Binding:**

- **SPEC-003** (host event ingress) — the whole client-facing half. §6.1 the
  socket and its path, §6.2 the envelope's four fields, §6.3 the reply, the
  closed reason set and `retry_after_ms`; R-6 (one connection, one envelope,
  read to newline or EOF), R-8 (exactly one reply), R-9/R-10 (the envelope's
  shape and its timestamp), R-13 (`"host"` reserved), R-14 (a reader may act on
  `retry_after_ms` and MUST NOT branch on `detail`).
- **SPEC-001** — R-7 (the event's four fields), R-9 (the host interprets
  neither `data` nor a submitted value), R-22 (the explicit-offset rule),
  R-56 (`"host"` is reserved to evaluations the host originates).
- **SPEC-002** — R-12 only, and indirectly: the minimum spacing is what a
  `too_soon` reply reports, and this slice changes nothing about it.
- **ADR-001** (one-way strata) — the new member is **stratum 3**, which that
  ADR's §Decision names in terms: *"entry points — the Slint renderer,
  command-line binaries"*. It may name both strata below it and does; nothing
  may name it. Its Slint-freedom is a fact about its manifest (AC-7), not a
  rule an instrument enforces (OQ-3).
- **ADR-003** (the host as a workspace of strata) — governs what a member is
  and how one is added; it supersedes ADR-002, whose **T2** ("a second binary is
  required") is what this slice fires.
- **POL-001** (the phase gate) — `just check` is the gate, unchanged. A new
  member joins the one column; it does not get a column of its own.

**Checked and not applicable:**

- **ADR-004** (the scheduled-firing anchor) — a host-side decision about time.
  `emit` sends one envelope and exits; it anchors nothing and the spacing
  reaches it only as a `too_soon` reply to report.

**Binding as precedent rather than as a rule:**

- **ADR-005** (the event envelope normalizes in stratum 2) — its *reasoning* is
  what places this slice's reply normalization: what decides a normalization's
  stratum is **which contract it holds, not which shape it has**. The reply is
  the other half of the contract the envelope belongs to, so reading one belongs
  beside the listener that writes it, in `goad-shell`. This slice follows that
  ADR; it does not amend it.

## Open questions

- ~~OQ-1 — **Argument parsing: hand-rolled or a dependency.**~~ **Hand-rolled**,
  2026-09-11. `crates/goad/src/startup.rs::arguments` is the prior art and the
  shape to copy: a pure function over `impl Iterator<Item = OsString>` plus an
  env closure, one doc-table row per behaviour, one test per row. `clap` would
  be the first dependency no stratum needs, paid for in every `just check`
  build.
- ~~OQ-2 — **Where configuration-path discovery lives.**~~ **Extract the rule,
  not the parser**, 2026-09-11. `startup::arguments` stays where it is — it is
  coupled to `Launch` and `StartupError` and it is the *host's* CLI. What both
  binaries share is one rule, and it becomes
  `goad_shell::config::default_path(env) -> Option<PathBuf>`; `arguments` calls
  it and maps `None` to `StartupError::NoConfigPath`. The env stays a closure
  because `std::env::var` is a `disallowed-method` (`clippy.toml`).
- ~~OQ-3 — **Does the new member get a `goad-boundary` allowlist row?**~~ **No,
  and the tier stays 1**, 2026-09-11. The instrument is scoped to the two strata
  whose value is what they *cannot* reach: POL-001 §Verification says it holds a
  dependency entry *"in a stratum 1 or 2 manifest"*, and `allowlist.rs`'s own
  module doc says *"`goad` and `goad-boundary` carry no allowlist here: `goad`
  is unconstrained (stratum 3 may name both strata below it)."* `goad-emit` is
  stratum 3. A row for it would **extend** the instrument to a stratum it was
  never scoped to — new policy, amending POL-001's Verification table and its
  counting rule, which is the tier-2 move — and it would buy little, because
  what the row would prevent (Slint reaching the CLI) is already impossible by
  crate edge (AC-7). **The gap it would close is real and is stratum 3's,
  not this crate's**: nothing but review stops a future edit adding `slint` to
  `crates/goad-emit/Cargo.toml`, exactly as nothing has stopped it for
  `crates/goad` since 002. Closing it means one allowlist row for stratum 3
  covering both members, argued on its own terms; **Follow-ups** carries it, and
  after 006 is the earliest it is worth taking.
- ~~OQ-4 — **How the CLI writes to stderr and stdout.**~~ **Reuse the shape, and
  move the helper down**, 2026-09-11. `clippy::print_stdout` and `print_stderr`
  are denied workspace-wide, so emit needs the same route `crates/goad` uses: a
  pure `*_line` function that a test can assert with no sink to fake, plus
  `line_to` to put it somewhere. `line_to` is four lines but encodes a judgement
  — best effort, *both outcomes considered*, the exit code carries the fact — and
  two copies of a judgement is parallel implementation. It moves to
  `goad_shell::report`; `crates/goad`'s pure line functions stay where they
  are.
- ~~OQ-5 — **What `data` is when `--data` is absent.**~~ **`null`, and emit
  parses `--data` locally**, 2026-09-11. R-9 requires the key present and §6.2
  admits any JSON value. Emit builds the envelope with `serde_json` either way,
  so the parse is free, and a bad `--data` becomes a usage error (exit 2) naming
  the JSON fault instead of a round trip returning `invalid_envelope`. It
  duplicates no host check — R-9 stays the host's and stays tested from
  `goad-shell`'s own integration tier. The caveat is SPEC-003 §6.2's existing
  one: emit re-serializes, so key order and float precision normalize, and the
  *spelling* was never promised.
- ~~OQ-6 — **Whether `--timestamp` exists at all.**~~ **No flag**, 2026-09-11.
  Replay is speculative, every caller can already lie by other means, and
  `jiff::Timestamp::now()` through `goad_semantics::protocol::canonical::Timestamp`
  gives R-10's explicit offset for free — that type's `Serialize` is
  `collect_str` over jiff's `Display`, which is already the RFC 3339 form.
  Revisit in 007 if use asks.

- OQ-7 — **Raised here, 2026-09-11, and answered in the same breath: no `tokio`
  in emit.** One connection, one write, one read, exit — `std::os::unix::net::UnixStream`
  does it with no runtime, so the manifest is `goad-shell`, `goad-semantics`,
  `serde_json`, `jiff` and nothing else. The client code itself lives in
  `goad-shell` beside the listener (ADR-005's reasoning), where it is blocking
  code in a crate that also has a runtime; tests call it on `spawn_blocking`,
  which is the shape slice 004's renderer tier already uses for the writer's
  side.

## Summary

<!-- Written at close: what actually landed, in three or four lines. -->

## Follow-ups

<!-- Deferred work surfaced by this slice. Each becomes a future slice or a
     line in a spec. -->

- **Stratum 3 carries no manifest allowlist row** (OQ-3). `crates/goad` has been
  unbilled since 002 and `crates/goad-emit` joins it. Nothing but review stops a
  future edit naming `slint` or `tokio` in either. Closing it is one row plus a
  POL-001 Verification amendment — tier 2 by construction, and worth taking on
  its own terms rather than inside a slice that merely adds the second member.
  Earliest sensible point: after 006, when daily use has shown whether stratum 3
  drifts at all.

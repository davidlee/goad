# Slice 005: `goad emit`

**Stage:** scoping
**Tier:** 1 (thin) — see `docs/AGENTS.md` §Tiers. It writes no canon: SPEC-003
already specifies the envelope, the reply and the closed reason set, and this
slice is a *client* of that contract rather than an amendment to it. **Raise to
tier 2 if OQ-3 turns out to need an ADR-001 instrument change** — a new
workspace member that carries a dependency allowlist row is a policy surface,
not a file move.
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
- **`crates/goad-shell/src/config.rs`** — read-only use of `IngressConfig.path`,
  unless OQ-2 moves configuration-path discovery down into this stratum, which
  is the only production edit outside the new crate this slice anticipates.
- **`crates/goad-boundary/`** — an allowlist row for the new member if OQ-3 says
  it gets one.
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
- [ ] AC-6 — end to end against a real host: an `emit` invocation reaches the
      backend as an `evaluate` whose event carries the `source`, `kind` and
      `data` as sent, and whose `event.kind` is not `"scheduled"`.
- [ ] AC-7 — `crates/goad-emit` links no renderer. Held by an instrument, not by
      inspection: its dependency closure contains no `slint`.
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
- **ADR-001** (one-way strata) — the new member sits at stratum 2's level: a
  filesystem and a socket, no renderer, and it never names `crates/goad`.
- **ADR-003** (the host as a workspace of strata) — governs what a member is
  and how one is added; it supersedes ADR-002, whose **T2** ("a second binary is
  required") is what this slice fires.
- **POL-001** (the phase gate) — `just check` is the gate, unchanged. A new
  member joins the one column; it does not get a column of its own.

**Checked and not applicable:**

- **ADR-004** (the scheduled-firing anchor) and **ADR-005** (the envelope
  normalizes in stratum 2) — both are host-side placement decisions. `emit`
  writes bytes and reads a reply; it normalizes nothing and anchors nothing.

## Open questions

- OQ-1 — **Argument parsing: hand-rolled or a dependency.** Four flags,
  `--help`, `--version`. `clap` is the idiom and costs a dependency the
  allowlist must admit; a hand-rolled parser costs perhaps 60 lines and a test
  file. Decide against the manifest bill, not taste.
- OQ-2 — **Where configuration-path discovery lives.** It is
  `crates/goad/src/startup.rs::config_path` today, in a crate `emit` may not
  depend on. Move it to `goad-shell` (one rule, two callers) or restate it
  (two rules that must not drift). Moving is the DRY answer and touches the
  host's startup path; that is the cost to weigh.
- OQ-3 — **Does the new member get a `goad-boundary` allowlist row?** The
  allowlist covers `goad-semantics` and `goad-shell` and fails closed by
  design. A new member that carries a socket, a filesystem and possibly an
  argument parser is exactly the surface it exists to bill. **If answering this
  amends ADR-001 or POL-001, the slice is tier 2.**
- OQ-4 — **How the CLI writes to stderr and stdout.** `clippy::print_stdout`
  and `print_stderr` are denied workspace-wide. `crates/goad` solves it with a
  pure line function plus a sink (`diagnostics::report_startup_line` and
  `line_to`), which is testable and is prior art to reuse rather than a lint to
  except.
- OQ-5 — **What `data` is when `--data` is absent.** R-9 requires the key to be
  present; §6.2 admits any JSON value, so `null` is legal and `{}` is legal.
  Also: does `emit` parse the `--data` argument as JSON before sending, or send
  the bytes and let the host refuse? Parsing locally gives a better message and
  duplicates a check; not parsing keeps the CLI honest about who validates.
- OQ-6 — **Whether `--timestamp` exists at all.** A caller replaying an old
  event has a use for it; every other caller has a way to send a lie. Default
  is no flag.

## Summary

<!-- Written at close: what actually landed, in three or four lines. -->

## Follow-ups

<!-- Deferred work surfaced by this slice. Each becomes a future slice or a
     line in a spec. -->

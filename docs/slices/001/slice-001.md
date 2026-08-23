# Slice 001: Protocol core and process backend transport

**Stage:** scoping
**Depends on:** —

## Purpose

Nothing exists yet: no crate, no types, no canon. This slice lands the semantic
core of the host — canonical protocol types plus one working backend transport —
so that an `evaluate`/`respond` round trip completes against a real user-written
backend with no GUI involved.

The ordering is deliberate. The GUI risk is already retired: a throwaway spike
confirmed Slint 1.17.1 builds and opens a Wayland window in this dev shell. The
outstanding risk is the protocol, which must admit capabilities the v0 renderer
will not implement (option-scoped fields, richer content forms, natural-language
schedules) without narrowing to what a first renderer happens to need. Settling
that contract headless, under fixtures, means slice 002's renderer is a view
over a decided contract rather than the force that shapes it.

Once this lands: a backend written in any language can be invoked, can answer
"nothing to show", can return a choice, can receive the user's answer, and can
fail in every way user-owned code fails — each mapping to a typed host error
rather than a crash.

## Scope

Surfaces this slice may touch.

- Cargo manifest and crate skeleton; `src/`, `tests/`.
- Protocol types: request and response envelopes, protocol version field,
  `view`, `choice`, options, contextual content, `next_check`.
- Normalization: permissive wire forms to strict canonical internal types.
- Schedule resolution as a pure function (latest valid instruction wins).
- Backend transport abstraction — async at its boundary — plus the
  spawn-per-invocation implementation (stdin JSON, stdout JSON, stderr capture,
  timeout) on tokio.
- Backend error model and diagnostics surface.
- TOML configuration loading: backend command, backend timeout, default poll
  interval. Nothing else.
- JSON fixture corpus and integration tests.
- `examples/` — a minimal TypeScript backend, run under deno.
- `flake.nix` — adding deno to `devToolPkgs`. Added to scope by the OQ-9
  answer; the dev shell is otherwise untouched.
- Root `AGENTS.md` (currently empty; `CLAUDE.md` symlinks to it).
- Declared verification commands.

## Non-goals

- **Any GUI.** No Slint, no rendering, no window. Slice 002.
- **The timer.** `next_check` is parsed, normalized and resolved here; nothing
  wakes on it. Wall-clock scheduling is slice 003.
- **External event ingress.** No host event socket, no `goad emit` CLI. The
  request types must be able to carry an opaque event, but nothing produces one
  outside tests. Slice 004.
- **Persistent socket transport.** The transport abstraction must not assume
  spawn-per-invocation, but only that implementation lands. Slice 005.
- **Rendering option fields, Markdown, HTML or URI content.** Admitted by the
  types, implemented by nobody yet — brief §22.3.
- **Natural-language time parsing.** `"tomorrow morning"` must remain
  addable; it is not added. Brief §9.1 forbids making it a blocker.
- **Multiple backends, discovery, manifests, daemon lifecycle management.**
- **Any domain concept.** Brief §21.16.

## Acceptance criteria

- [ ] AC-1 — From a clean clone in the nix dev shell: build, test, lint (zero
      warnings) and format check all pass. The commands are named in
      `AGENTS.md`.
- [ ] AC-2 — Canonical Rust types exist for the `evaluate` and `respond`
      requests and their responses. The envelope carries a protocol version.
      Unknown optional fields are ignored; an unknown *required* semantic
      primitive is rejected with a named error. Brief §13.
- [ ] AC-3 — `next_check` accepts an RFC 3339 timestamp and a simple relative
      duration, normalizing both to one canonical instant. An ambiguous or
      unparseable value yields a protocol error and never an invented instant.
      Brief §3.3, §9.1.
- [ ] AC-4 — Schedule resolution is a pure function over (existing schedule,
      incoming instruction, default interval) implementing latest-valid-wins;
      an invalid instruction preserves the existing or default schedule rather
      than disabling anything. Brief §9.
- [ ] AC-5 — The process transport spawns the configured command, writes one
      JSON request to stdin, reads one JSON response from stdout, enforces a
      timeout, and captures stderr into diagnostics. Brief §6.2.
- [ ] AC-6 — Each failure mode in brief §13 reachable by this transport —
      command not found, timeout, non-zero exit, malformed JSON,
      protocol-invalid response, invalid scheduling value, unsupported required
      primitive — maps to a distinct typed error. No path panics. Per ADR-001,
      the taxonomy splits at the stratum seam: parse and validation errors
      belong to the pure core, transport errors to the I/O shell wrapping the
      core's. Not one flat enum spanning both.
- [ ] AC-7 — Round trip, driven by an integration test with no GUI: an example
      backend returns `view: null`; then returns a choice; the host assigns a
      `view_id` and records it; a `respond` carrying that id reaches the
      backend; the backend's reply is accepted.
- [ ] AC-8 — A response bearing an unknown or stale `view_id` is rejected with
      a named error. Brief §12.
- [ ] AC-9 — A JSON fixture corpus covers the protocol-level cases in brief
      §15.3 that fall inside this slice: valid evaluate request and response,
      `view: null`, a simple choice, response round trip, scheduling
      replacement, process transport, timeout and failure, malformed backend
      output.
- [ ] AC-10 — Root `AGENTS.md` is a map plus invariant sheet per brief §15.1:
      it states that the host does not understand the user's domain, states the
      permissive-wire / canonical-internal rule, warns against narrowing the
      protocol to the current renderer, points at the authoritative documents,
      and names the verification commands.
- [ ] AC-11 — No domain vocabulary (habit, streak, journal, site, goal,
      reminder, compliance) appears in host types or module names. Brief
      §21.16. Grep-checkable.

## Governing canon

- **ADR-001 — Host code flows one way through three strata.** Binding. The
  pure semantic core (protocol types, normalization, schedule resolution) must
  build and test with no renderer and no async runtime in its dependency graph;
  the I/O shell (transport, config, host operational state, ingress) may depend
  on it; entry points depend on the shell. No import may point upward.
- **ADR-002 — The host stays one crate until a renderer or a second binary
  arrives.** Binding. This slice creates a single crate. The strata above are
  modules within it, not crates. None of ADR-002's triggers fires in this
  slice: no renderer, one binary at most, no renderer build to dominate test
  time. Adding tokio does not fire T1 — tokio is a stratum 2 dependency, and
  the constraint is only that stratum 1 must not acquire it.
- `docs/policy/` is empty; `docs/specs/` is empty. Checked, nothing to apply.
- `docs/AGENTS.md` — methodology, not canon by its own definition, but the
  process this slice follows.
- `docs/brief.md` — the initial project brief. Cited throughout above as
  intent, **not** as normative canon.

This slice also *adds* canon. ADR-001 and ADR-002 above were raised by it and
accepted during its design. Per the OQ-1 decision a protocol specification is
written into `docs/specs/` during audit, copied from `docs/templates/spec.md`
and derived from what shipped rather than from intent. That obligation lives in
`audit.md`'s reconciliation and needs explicit user endorsement before it is
written.

## Open questions

- ~~OQ-1 — Does this slice produce `docs/specs/` canon for the protocol, or does
  the protocol live only as Rust types plus fixtures until a later slice? Brief
  §15 treats a protocol document as authoritative; nothing in `docs/specs/`
  exists to be authoritative yet.~~ **Answered:** yes, written during audit, so
  the spec describes what shipped rather than what was intended. During
  execution the Rust types plus the AC-9 fixture corpus are the only contract;
  nothing may cite a spec that does not yet exist.
- ~~OQ-2 — One crate, or a workspace (core library, plus binaries later)? The
  answer binds slices 002 and 004, which add a GUI binary and a CLI.~~
  **Answered:** one crate, with the strata as modules. Promoted to canon as
  ADR-001 (the strata) and ADR-002 (the crate count and its split triggers).
- ~~OQ-3 — Does the process transport need an async runtime, or do
  `std::process` plus a thread suffice until the socket transport in slice 005?
  Brief §4.1 asks for the smallest reasonable integration; adopting one now is
  cheaper than retrofitting, adopting one needlessly is a dependency this slice
  cannot justify.~~ **Answered:** async from the start, so the transport
  boundary needs no signature change when slice 005 adds the socket. Raises
  OQ-8.
- ~~OQ-8 — Which async runtime? Measured for the features this slice needs,
  tokio resolves 14 unique dependencies against 31 for the smol family, so
  brief §4.1's "smallest reasonable" favours tokio despite first appearances.
  Recommended, not yet decided.~~ **Answered:** tokio. Features for this slice
  are `process`, `time`, `rt`, `io-util`; `net` waits for slice 005. It is a
  stratum 2 dependency and stratum 1 must not acquire it.
- ~~OQ-4 — How much configuration lands now: only what this slice reads
  (backend command, timeout), or the whole illustrative file in brief §5
  including socket path, poll interval and log path?~~ **Answered:** only what
  this slice needs — backend command, timeout, default poll interval. AC-4 takes
  the default interval as a parameter, so it must exist even though nothing
  wakes on it until slice 003. No socket path, no log path.
- ~~OQ-5 — Which language for the example and fixture backends? Brief §15.2
  wants several eventually; this slice needs at least one, and the fixture
  backend has different requirements from a showcase example.~~ **Answered:**
  TypeScript. Raises OQ-9 and OQ-10.
- ~~OQ-9 — Which TypeScript runtime? The choice reaches into `flake.nix` and
  determines whether `cargo test` needs a compile step or a `node_modules`
  before it can run a backend.~~ **Answered:** deno, invoked with `-A`. It runs
  `.ts` with no build step and no `node_modules`, and unlike bun it typechecks
  rather than stripping types — which is the point of choosing TypeScript when
  brief §3.7 makes agents the authors. The example must not present deno's
  default-deny permissions as a security boundary; brief §14 is explicit that
  backends are trusted.
- OQ-10 — Are the deliberately-misbehaving fixtures behind AC-6 (timeout,
  non-zero exit, malformed stdout, command not found) also TypeScript, or
  minimal non-TypeScript helpers? They need to fail precisely, which is a
  different job from demonstrating that no SDK is required.
- ~~OQ-6 — Does host operational state (outstanding `view_id`, resolved
  schedule) persist to disk in this slice, or stay in memory? Brief §20 phase 4
  says "if required", which defers the question rather than answering it.~~
  **Answered:** in memory. AC-8's stale-`view_id` rejection is therefore scoped
  to one process lifetime — a restart forgets the outstanding interaction rather
  than rejecting a response to it. Design must state this as an assumption.
- ~~OQ-7 — Is the Slint spike committed now as a skeleton, held for slice 002,
  or discarded? It currently lives outside the repository.~~ **Answered:**
  committed at `spikes/slint/` in `99404f8` purely to give the findings a
  referent, then deleted in `c8ab319`, which adds
  `docs/memory/slint-build-mechanics.md`. Never entered `src/`; ADR-002's T1 did
  not fire.

## Summary

<!-- Written at close. -->

## Follow-ups

<!-- Written at close. -->

# Slice 002: The workspace split, and the first renderer

**Stage:** design
**Depends on:** slice 001 (closed) — SPEC-001, the canonical protocol types, the
process transport, and `Host`.
**Research:** `research.md` — 5 survey threads, 3 spikes, 2 adversarial
verifiers, all three spike verdicts confirmed and none refuted.
**Decisions:** `design-log.md`. Design: `design.md`.

## Purpose

Slice 001 settled the contract and proved it headless. Nothing draws it. The host
can be run by a test and by nothing else: there is no window, no way for a person
to answer a view, and no way to see that a backend failed.

This slice puts the protocol on the glass. A backend that returns a `choice` gets
it rendered; a person picks an option; the answer goes back as a `respond` and
the exchange completes. When there is nothing to show, goad is a tray icon and
no window. When the backend misbehaves, the failure is visible and named, and
the host stays up and stays able to invoke the backend again.

It also cashes a bet slice 001 deliberately deferred. ADR-002 said the crate
stays whole until a trigger fires, and named T1 — a dependency stratum 1 must not
need — as the one expected here. It fires. The split lands first, as its own
change, before Slint enters the tree.

Once this lands, brief §21's AC-5 and AC-13 have real observables for the first
time, and the two invariants that have so far been held by argument — that the
protocol is not narrowed to what the renderer draws, and that a backend failure
never takes the host down — are held by a renderer that exists.

## Scope

Surfaces this slice may touch.

**The split** (first, and alone):

- `Cargo.toml` → a workspace root; `crates/goad-semantics/`, `crates/goad-shell/`.
- `src/**` → relocated under those two crates. Relocation, not redesign.
- `tests/**` → retiered against the new members; fixtures to `tests/fixtures/`.
- `justfile` and the gate's command list.
- `clippy.toml`, `[workspace.dependencies]`, `[workspace.lints]`.

**The renderer**:

- `crates/goad/` — stratum 3: `ui/**.slint`, `build.rs`, the `slint` dependency,
  the binary, and a lint set of its own.
- The view mapper: canonical types → row structs the markup consumes.
- The runtime seam: the tokio runtime, the Slint event loop, and the task that
  owns `Host`.
- The tray component, the prompt window, and the diagnostic surface.

**Supporting**:

- `flake.nix` — a font package for the devshell, and nothing else.
- `tests/protocol/boundary.rs` — its configuration extended to `.slint`.
- `canon-delta.md` — drafted here, promoted at audit.

## Non-goals

- **Scheduling.** No timer, no `next_check` consumption, no default poll. Slice
  003 owns the clock. This slice takes its stimulus from process start and from
  an explicit tray action, which is a stimulus, not a schedule.
- **Fields.** SPEC-001 admits option-scoped fields; this renderer draws options
  and their labels and nothing else. That is a renderer subset, and the design
  states in writing how the mapper stays a subset rather than becoming a
  narrowing — the failure CLAUDE.md invariant 3 names.
- **HTML and URI content.** Slint has no element for either and no WebView
  (brief §11.2 defers it). They stay admitted by the protocol and undrawn.
- **Event ingress** (slice 004) and **the socket transport** (slice 005).
- **Persistence.** Nothing survives a process restart, which is what keeps
  SPEC-001 OQ-3 shut.
- **Pixel or layout assertions.** Determinism requires a test-only font family a
  production UI must not carry. Structure and semantics are asserted; geometry
  is not.
- **Getting a prompt in front of the user by force.** Placement, always-on-top
  and focus-stealing are verified no-ops on Wayland. The slice sets a stable xdg
  app id and documents a compositor window rule; it does not pretend to more.

## Acceptance criteria

- [ ] AC-1 — The workspace builds and `just check` exits 0 from a clean clone
      under `nix develop`, at the split commit and again at slice close. The
      gate is the six commands in `design.md` §5.6; the pre-split two-column
      matrix no longer exists, because the split retires the `shell` feature
      that created it.
- [ ] AC-2 — Every file that moved in the split moved unchanged, or its content
      change is named in `audit.md` with a reason. Content changes beyond import
      paths and manifest entries are evidence of redesign and must be argued.
- [ ] AC-3 — Stratum 1's purity is enforced two ways, both inside the gate:
      `cargo test -p goad-semantics` passes, and a test reads
      `crates/goad-semantics/Cargo.toml` and fails on a runtime, renderer or
      filesystem-shaped dependency. `boundary.rs`'s `tokio` source grep is
      retired in the same change — the fact it checked now lives in a manifest,
      where no compiler and no source scan can see it.
- [ ] AC-4 — A backend returning a `choice` view has it drawn: title, body, and
      one activatable control per option, in the order the backend sent them.
- [ ] AC-5 — Activating a control sends a `respond` carrying that option's
      `OptionId` and the `ViewId` the host assigned, and the exchange completes.
- [ ] AC-6 — A backend returning `view: null` produces no window, and goad
      remains running and answerable.
- [ ] AC-7 — Every failure in SPEC-001's taxonomy leaves the process running,
      the diagnostic surface showing the failure, and the backend invocable
      again — asserted by a successful exchange after the sequence, through one
      retained `Host`. No failure ends the loop, and no failure hides a view the
      host still considers outstanding: a failed `respond` leaves the question
      on screen, because the answer failed to deliver and the question did not
      go away.
- [ ] AC-8 — Both arbitrary values are bounded at the glass: captured stderr and
      a discarded scheduling instruction's verbatim `raw`. Each fact the
      diagnostic surface reports is rendered exactly once (F-42, F-47).
- [ ] AC-9 — A `markdown` body that the renderer's parser rejects is still
      shown, degraded, and the degradation is reported. No legal view is refused
      because the renderer cannot draw part of it.
- [ ] AC-10 — The renderer's tests run under `cargo test` with no display
      server, and a guard test proves the query API is live — so a `build.rs`
      regression fails loudly instead of turning the suite into vacuous passes.
- [ ] AC-11 — The empty state is asserted by presence *and* absence: a test
      names both what must be there and what must not, and each assertion is
      shown to fail against a deliberately broken implementation.
- [ ] AC-12 — Closing the window cancels the in-flight exchange — dropped, not
      awaited; `kill_on_drop` is what disposes of the child — the host task ends,
      and only then does the loop quit. No backend child outlives the process.
- [ ] AC-13 — `boundary.rs` scans `.slint` and `.rs` across every workspace
      member, and its comment cut respects string literals in both languages —
      so a URL in markup cannot hide the user-visible strings after it on the
      same line. Positive controls plant a forbidden word in a component name,
      an accessible label, an ordinary string, and a string after a URL.
- [ ] AC-14 — No domain vocabulary appears in any crate name, module name, type,
      markup component, accessible label, or user-visible string.
- [ ] AC-15 — `canon-delta.md` accounts for every canon movement this slice
      obliges, and each is promoted or abandoned in writing at audit.

## Governing canon

**Binding:**

- **SPEC-001** — the host/backend protocol. §2 puts drawing out of scope, so the
  renderer inherits no direct requirement — but R-9 (values as sent), R-12
  (refuse an unsupported primitive clearly), R-14 (two options may share a
  label), R-19 (the host must not dereference a `uri`), R-20 (no silent dropping
  of a view part) and R-32 (interaction identity) all constrain it at one
  remove. `research.md` Thread 1 enumerates the set.
- **ADR-001** — one-way strata. Stratum 1 is pure and never names stratum 2. The
  renderer is stratum 3 and may name both.
- **ADR-002** — single crate until triggered. T1 has fired; this slice supersedes
  it (`design-log.md` 2026-09-05).

**Checked, not applicable:**

- `docs/policy/` — empty.

## Open questions

- ~~OQ-1 — Does ADR-002's T1 fire, and does the crate split?~~ **It fires, and
  it splits, first and alone.** `design-log.md` 2026-09-05; evidence in
  `research.md` Thread 6. ADR-002's stated *reason* is false and the superseding
  ADR must say so.
- ~~OQ-2 — What is goad when it has nothing to show?~~ **A tray icon and no
  window.** `design-log.md` 2026-09-05.
- ~~OQ-3 — Is Markdown "straightforward" in the sense brief §11.1 conditions
  on?~~ **Yes** — stable API, zero extra dependencies (`research.md` Thread 5).
  It is in.
- OQ-4 — What a `markdown` body the parser rejects does. Design takes it; the
  answer needs somewhere to report a degradation, and `Discarded` has one
  variant.
- OQ-5 — Whether the choice view scrolls. If it does, a count assertion counts
  the viewport rather than the model, and the assertions change shape.
- OQ-6 — Whether activating a link inside a rendered body counts as the host
  dereferencing a `uri` under R-19.
- OQ-7 — What stimulus drives an evaluation in a slice with no clock.
- OQ-8 — What the diagnostic surface actually says, given a palette of a tray
  icon, a tooltip, a title, a menu, and a window title, and no notification and
  no urgency hint.

## Summary

<!-- Written at close. -->

## Follow-ups

<!-- Written at close. -->

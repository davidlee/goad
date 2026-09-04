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

- `Cargo.toml` → a workspace root; `crates/goad-semantics/`, `crates/goad-shell/`,
  and `crates/goad-boundary/` — a test-only member owning the workspace-wide
  invariant checks, which belong to no stratum (`design.md` D17).
- `src/**` → relocated under those two crates. Relocation, not redesign.
- `tests/**` → retiered against the new members; fixtures to `tests/fixtures/`.
  The host-driving half of slice 001's `harness.rs` becomes one file both test
  crates include by `#[path]` (`design.md` §9 item 12.8).
- `justfile` and the gate's command list.
- `clippy.toml`, `[workspace.dependencies]`, `[workspace.lints]`.

**The renderer**:

- `crates/goad/` — stratum 3: `ui/**.slint`, `build.rs`, the `slint` dependency,
  a **library plus a thin binary** (`main`, `run` and `start` are the whole of
  `main.rs`; everything a test reaches is a `pub mod` of the library —
  `design.md` §5.1), and `README.md` carrying the recommended compositor window
  rule. It inherits `[workspace.lints]` unchanged; the only laxity is the
  module-scoped `#![expect(...)]` around `include_modules!()`.
- The view mapper: canonical types → row structs the markup consumes.
- The reception seam: the one function that consumes an `Outcome`.
- The runtime seam: the tokio runtime, the Slint event loop, and the one
  `serve` future that owns `Host`, the controller and the glass.
- The tray component, the prompt window, and the diagnostic surface.

**Supporting**:

- `flake.nix` — a font package for the devshell, and nothing else.
- `tests/protocol/boundary.rs` — re-homed into `crates/goad-boundary` and its
  configuration extended to `.slint`, with members read from `workspace.members`.
- `canon-delta.md` — drafted here, promoted at audit.
- `draft-policy.md` — the phase gate as new canon, drafted here from
  `docs/templates/policy.md` and promoted separately at audit. New canon is not
  drafted in `canon-delta.md` (`docs/AGENTS.md`; review `F-24`).

## Non-goals

- **Scheduling.** No timer, no `next_check` consumption, no default poll. Slice
  003 owns **scheduling and timers** — when to evaluate, how `next_check` is
  consumed, what happens on failure. This slice takes its stimulus from process
  start and from an explicit tray action, which is a stimulus, not a schedule.
  It **is** authorized to read wall time, solely to stamp the two events it
  sends and the calls that carry them: every `Host` entry point has always
  required a caller-supplied `Timestamp`, including slice 001's tests, and
  supplying one is not owning a schedule. The adapter that does it is one
  function wide and is not a timer (`design.md` D15, F-23).
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
      gate is the **six** commands in `design.md` §5.6; the pre-split two-column
      matrix no longer exists, because the split retires the `shell` feature
      that created it, and the renderer adds no column back.
- [ ] AC-2 — Every file that moved in the split moved unchanged, or its content
      change is named in `audit.md` with a reason. Content changes beyond import
      paths and manifest entries are evidence of redesign and must be argued.
- [ ] AC-3 — Four instruments hold four different parts of ADR-001's stratum 1
      rule, and the slice states each one's boundary rather than claiming their
      sum is "purity, enforced" (F-6, third raising). Every one of them is inside
      the gate:
      1. **Cargo resolution** — a `goad-semantics` source naming `goad_shell` or
         `tokio` is `error[E0433]`. Crate edges only.
      2. **The manifest allowlist test** — reads the dependency tables of
         `crates/goad-semantics/Cargo.toml` against an allowlist and fails on
         anything else, including in `dev-` and `build-dependencies`, in a
         target-specific table, and behind a `package` rename. **Names only** —
         not versions, not features, and not what a permitted dependency does.
      3. **The stratum 1 purity scan** — the same walk as the vocabulary scan,
         over `crates/goad-semantics/src` with a path-token list, failing on a
         direct `std` reach for the filesystem, processes, sockets, threads, the
         environment, or a clock. This is the half no manifest entry and no
         compiler can see: `std::fs` needs neither. Its limits are named, not
         assumed away — a brace-grouped `use std::{fs}`, an alias, and I/O
         performed on stratum 1's behalf by a permitted dependency all pass it.
      4. **`cargo test -p goad-semantics`** — the only command that builds
         stratum 1 with the features its own manifest asks for, because
         `--workspace` unifies features across members. It **rejects nothing**
         and is not a purity check.

      And one residue, recorded as a review obligation rather than as an
      enforced rule: a feature switched on in a shared dependency by stratum 2
      or 3 unifies into stratum 1's build, and no instrument above sees it
      (`design.md` §5.6, D25). `boundary.rs`'s `tokio` source grep is retired in
      the same change — the fact it checked now lives in a manifest, where
      instrument 2 reads it.
- [ ] AC-4 — A backend returning a `choice` view has it drawn: title, body, and
      one activatable control per option, in the order the backend sent them.
- [ ] AC-5 — Activating a control sends a `respond` carrying that option's
      `OptionId` and the `ViewId` the host assigned, and the exchange completes.
      The control carries the view token it was drawn with, and a click whose
      token names a presentation that has since been replaced is refused
      locally, with no backend contact.
- [ ] AC-6 — `view: null` means what `Host` means by it, and the window follows
      the interaction rather than the message. A successful `respond` returning
      `view: null` closes the interaction and leaves goad with no window. A
      successful `evaluate` returning `view: null` leaves any outstanding
      interaction — and therefore the question on screen — exactly as it was;
      with nothing outstanding it leaves goad a tray icon and no window. In every
      case goad remains running and answerable.
- [ ] AC-7 — Every failure in SPEC-001's taxonomy leaves the process running,
      the diagnostic surface showing the failure, and the backend invocable
      again — asserted by a successful exchange after the sequence, through one
      retained `Host`. No failure ends the loop, and no failure hides a view the
      host still considers outstanding: a failed `respond` leaves the question
      on screen, because the answer failed to deliver and the question did not
      go away.

      **One cohort is exempt from "one retained `Host`", and the exemption is
      stated rather than improvised** (F-9): `BackendError::Spawn` needs a
      command that does not exist, and a `Host` owns exactly one command
      (`config.rs:47-51`, `process.rs:46-49`), so a command that cannot be
      spawned cannot first have succeeded. That row runs in its own `Host`, as
      slice 001's does, and for it "the backend is invocable again" means the
      weaker, checkable thing: a **second** `evaluate` on that same `Host`
      attempts a second spawn and fails the same way, so the host neither died
      nor latched. Every other row is in the retained-`Host` cohort, and the
      trailing success is theirs.
- [ ] AC-8 — Both arbitrary values are bounded at the glass: captured stderr and
      a discarded scheduling instruction's verbatim `raw`. The bound is applied
      to the **displayed** form — after lossy decoding and escaping — and counted
      in characters, so a truncation cannot split a codepoint; the display
      truncation and the transport's own capture truncation are two distinct
      statements. Each fact the diagnostic surface reports is rendered exactly
      once (F-42, F-47).
- [ ] AC-9 — A `markdown` body that the renderer's parser rejects is still
      shown, degraded, and the degradation is reported. No legal view is refused
      because the renderer cannot draw part of it.
- [ ] AC-10 — The renderer's tests run under `cargo test` with no display
      server, and a guard test proves the query API is live — so a `build.rs`
      regression fails loudly instead of turning the suite into vacuous passes.
- [ ] AC-11 — The empty state is asserted by presence *and* absence: a test
      names both what must be there and what must not, and each assertion is
      shown to fail against a deliberately broken implementation.
- [ ] AC-12 — A stop request while an exchange is in flight ends the exchange
      rather than waiting for it: the in-flight future is dropped — not awaited —
      inside the still-entered tokio runtime, `serve` returns, and only then is
      the event loop asked to quit. What is asserted is what the host holds: the
      task ended far inside the backend's configured timeout, `serve` returned,
      and the renderer leaves behind no task or handle a drop would fail to
      cancel (SPEC-001 R-48). Disposal of the child past that drop is
      `kill_on_drop`'s, which R-48 concedes is best-effort and which no test
      asserts. Stopping travels out of band, not as a queued command, because a
      loop awaiting an exchange cannot receive one.
- [ ] AC-13 — The vocabulary scan reads `workspace.members` and scans `.slint`
      and `.rs` across every member it finds, so a new member cannot arrive
      unscanned; and its comment cut respects string literals in both languages —
      so a URL in markup cannot hide the user-visible strings after it on the
      same line. Positive controls plant a forbidden word in a component name,
      an accessible label, an ordinary string, a string after a URL, a string
      after an escaped quote, and a string after a raw string.
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
  of a view part), R-32 (interaction identity), R-33 (a new view replaces the
  outstanding one and the replaced id is stale at once), R-34 (a refusal closes
  nothing), R-42 (stderr travels with every outcome), R-48 (cancellation is
  best-effort, and what the host must instead guarantee) and R-54 (cleanup is a
  channel of its own) all constrain it at one remove. `research.md` Thread 1
  enumerates the set.
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
- ~~OQ-4 — What a `markdown` body the parser rejects does.~~ **Shown as plain
  text and reported as `Undrawn::MarkdownUnsupported`** — the renderer's own
  diagnostic surface, not an extension of `Discarded`. `design.md` §5.2's content
  table and OQ-4.
- ~~OQ-5 — Whether the choice view scrolls.~~ **Yes**, and counts therefore come
  from `accessible-item-count`. `design.md` E-2.
- ~~OQ-6 — Whether activating a link inside a rendered body counts as the host
  dereferencing a `uri` under R-19.~~ **The question does not arise: no URL is
  opened.** A scheme policy is a decision, and it is a follow-up rather than an
  implementation nobody took. `design.md` E-5, D11.
- ~~OQ-7 — What stimulus drives an evaluation in a slice with no *schedule*.~~
  **Two, as values an agent can write** — `Stimulus::Startup` and
  `Stimulus::Requested`, a wall-clock adapter one function wide that is not a
  timer, and a config-path rule with no guessing in it. The question was
  originally written "with no clock", which the non-goal above now corrects:
  slice 003 owns scheduling, and stamping a call is not scheduling it (F-23).
  `design.md` §5.4, OQ-7, D15.
- ~~OQ-8 — What the diagnostic surface actually says.~~ **Written out
  literally** — every string, three character-counted bounds applied after
  escaping, two distinguishable truncations, a deterministic order, and two tray
  states drawn by a pure function rather than shipped as assets. `design.md`
  §5.4 "The diagnostic surface".

## Summary

<!-- Written at close. -->

## Follow-ups

<!-- Written at close. -->

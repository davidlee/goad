# Roadmap

**Status:** advisory. This document is **not canon** — it binds nobody and
amends nothing. `docs/specs/`, `docs/policy/` and `docs/adr/` govern; a slice's
own `slice-nnn.md` is the truth about that slice's scope. This file exists to
answer one question: *what is the next slice, and why that one?*

Revise it in place whenever the answer changes. No changelog.

## Where this stands

**2026-09-08.** Slices 001, 002 and 003 are closed. 001 produced SPEC-001 and
the semantic core: canonical protocol types, permissive normalization, pure
schedule resolution, the spawn-per-invocation process transport, and the failure
taxonomy. 002 split the single crate into a workspace of one member per stratum
(ADR-003) and drew the first renderer. 003 gave the host a clock: a resolved
next check now makes it evaluate, in both directions, bounded by a three-second
floor (SPEC-002, ADR-004).

`just check` is **six commands and one column** — the crate split retired the
feature matrix (POL-001). Purity is held by four ADR-001 instruments plus the
domain-vocabulary scan, plus one residue nothing enforces.

The host renders, keeps time, and can be run by a person. **Nothing listens**:
there is no way for anything outside the host to prompt an evaluation, which is
slice 004.

## Sequence

Brief §20 suggests eight implementation phases; slices 001–007 carry them, with
§20's phases 1 and 2 both landed by slice 001.

```mermaid
graph LR
  S1["001 ✔<br/>protocol core<br/>+ process transport"]
  S2["002 ✔<br/>minimal renderer"]
  S3["003 ✔<br/>scheduling"]
  S4["004<br/>event ingress"]
  S5["005<br/>socket transport"]
  S6["006<br/>starter experience"]
  S7["007<br/>v0.1.0 polish"]

  T1{{"ADR-002 T1<br/>Slint build-dep<br/>→ workspace split"}}
  T2{{"ADR-002 T2<br/>second binary<br/>goad emit"}}

  S1 --> S2 --> S3 --> S4 --> S5 --> S6 --> S7
  T1 -.-> S2
  T2 -.-> S4

  classDef done fill:#2d5016,stroke:#4a7c26,color:#fff
  classDef trigger fill:#5c4317,stroke:#8a6620,color:#fff
  class S1,S2,S3 done
  class T1,T2 trigger
```

The chain is mostly hard dependency, not preference:

- **002 before 003.** A timer with nothing to show has no observable behaviour,
  so its acceptance criteria would be unwritable.
- **003 before 004.** Ingress is a second stimulus into a scheduled evaluation
  path. Building the second stimulus first means building the path twice.
- **005 anywhere after 001**, in principle — the transport abstraction is
  already in place. It is placed fifth because it is the least user-visible
  remaining item, not because it is blocked.
- **006 last but one.** The starter experience documents what exists. Writing it
  earlier documents intentions.

## The slices

### 002 — minimal renderer ✔

Brief §20 phase 3, §10.1, §11.1. **Closed 2026-09-05.**

A Slint window that draws a `choice` view, collects an option, and shows the
empty and diagnostic states. Host-generated `view_id` reaches the screen.

- **Fires ADR-002 T1.** Slint's build-dependency cannot be feature-gated
  cleanly, so this slice splits the crate into a workspace along the ADR-001
  strata — or supersedes ADR-002 with a decision not to. That split is a
  relocation of files; if it cannot be, ADR-001 was not being honoured, and
  *that* is the slice's first finding.
- **Carries from 001:** diagnostics retention and bounded rendering (F-42,
  F-47). `Outcome` is per-call and forgotten; whatever surfaces it must bound
  what it prints. A discarded `next_check`'s `raw` renders verbatim and
  unbounded, newlines included, and `ConfigError::Duration` both renders its
  fault and chains it as `source()` — a chain-walking logger prints it twice.
- **The standing hazard:** SPEC-001 admits option-scoped fields, richer content
  forms, and natural-language schedules that this renderer will not implement.
  Not implementing them is correct. *Narrowing the protocol to match* is the
  failure this project exists to avoid — CLAUDE.md invariant 3, brief §22.3.

### 003 — scheduling ✔

Brief §20 phase 4, §9. **Closed 2026-09-08.**

`serve` gained a third `select!` arm holding a pinned `tokio::time::Sleep`, so a
resolved next check now makes the host evaluate without being asked. An
instruction from either an `evaluate` or a `respond` moves the wait, in both
directions; an elapsed instant fires once without underflowing or spinning; a
failing backend keeps its cadence and no faster; an unreadable clock loses
neither the schedule nor its liveness. A three-second minimum spacing, anchored
to the previous scheduled firing on the monotonic clock and cleared by nothing,
is the only thing between the host and a backend that instructs the past on
every response. It adjusts nothing the host stores or reports. The next check is
one line in the diagnostic surface, and a scheduled `evaluate` is
distinguishable on the wire as `event.kind` = `"scheduled"`.

It produced **SPEC-002** (the host's scheduling behaviour) and **ADR-004** (the
floor's anchor), and added **R-56** to SPEC-001 — the three event kinds, their
meanings fixed, the set left open and a backend required to tolerate a kind it
does not know. All twelve acceptance criteria met on evidence re-run at audit;
one code review over four rounds, 22 findings, none outstanding.

- **The 001 carry is discharged.** The timer re-resolves nothing: two structural
  scans in `crates/goad-boundary` hold it, one asserting the identifier
  `resolve` names no production line in stratum 3 and one asserting
  `schedule::resolve` is called from exactly two places, both in `host.rs`. The
  busy-loop failure mode F-1, F-34 and F-48 kept raising is bounded by
  SPEC-002/R-4.
- **Nothing persists** — decided by the user at design. SPEC-001 OQ-3 stays
  shut, and SPEC-002/R-11 records the catch-up rule for the slice that changes
  that.
- **Left open:** a scheduled firing can supersede a view a person is
  mid-answering (SPEC-002 OQ-4). Both candidate repairs put domain judgement in
  the host, so the likely answer is a backend affordance and therefore a
  protocol question.

### 004 — external event ingress

Brief §20 phase 5, §7, §19.

A Unix socket accepting an opaque event envelope, and a `goad emit` CLI. The
event is forwarded verbatim; the host interprets nothing beyond the envelope.

- **Fires ADR-002 T2** (a second binary) if 002 has not already split the crate.
- **Read ADR-004 before touching the floor.** An ingested event is the first
  stimulus that is not a person and not a due check. SPEC-002/R-5 says in terms
  that the scheduled-firing spacing bounds it in no way and that this slice must
  decide separately how it is bounded; ADR-004 says why the anchor is written so
  that an event cannot clear it.
- The whole risk is scope: event *interpretation*, debouncing, and filtering are
  the backend's and the user's watcher's, per brief §7. A host that learns what
  an event means has crossed the boundary.

### 005 — persistent socket transport

Brief §20 phase 6, §6.1, §6.3.

JSONL over a configured Unix socket, one request in flight, process fallback
when the socket is absent or unusable, and defined reconnect behaviour. The
semantic protocol is identical across transports — SPEC-001 already says so.

- **Carries from 001:** `BackendError::PipeMissing` and `cleanup_only` are
  reachable by no test (F-15, tolerated at audit). Either a unit test that
  fabricates the state, or removal, when the transport is reworked.
- **Also cheap here:** no end-to-end case exists for a backend that writes
  nothing, or for brief §10.1/§10.2 through a real process. Both are held at
  other tiers today. If this slice rebuilds the failure matrix, add them.

### 006 — starter experience

Brief §20 phase 7, §15.

`README`, the backend author's guide, minimal backends in several languages, and
the interstitial-journal example. Discharges brief §21 AC-14 and AC-15 — an
agent reads repository-local material and writes a working backend without
touching the host.

This is where **capability declaration (OQ-1)** and **validation feedback
(OQ-2)** most plausibly land — see *Open decisions*.

### 007 — v0.1.0 polish

Brief §20 phase 8, §17.

Configuration validation, diagnostics and logging, Markdown context if cheap,
Linux packaging and run instructions, and the complete acceptance suite walked
end to end.

## v0.1.0 acceptance coverage

Brief §21. Where each criterion is discharged.

| # | criterion | slice |
|---|---|---|
| 1 | clone and run the native Linux GUI | 002 runs it; 007 packages it |
| 2 | configuration points at a trivial scripting backend | 001 ✔ (config + example); observable at 002 |
| 3 | host periodically asks the backend | 003 ✔ |
| 4 | backend returns no view without error | 001 ✔ |
| 5 | simple choice rendered correctly | 002 |
| 6 | selection delivers a response to the backend | 002 |
| 7 | `next_check` from evaluation and from response | 001 ✔ |
| 8 | a later valid `next_check` supersedes an earlier one | 001 ✔ as semantics; 003 ✔ observable over time, in both directions |
| 9 | an external script sends an opaque event | 004 |
| 10 | the event reaches the backend uninterpreted | 004 |
| 11 | backend may run as a persistent JSONL socket service | 005 |
| 12 | fallback to process invocation when it is unavailable | 005 |
| 13 | crashes, timeouts, invalid JSON do not crash the GUI | 001 ✔ taxonomy; 002 surfaces it |
| 14 | example backend implements the journal with no host change | 006 |
| 15 | an agent implements a backend from repository material alone | 006 |
| 16 | no domain concepts enter the host model | 001 ✔ boundary test; **standing, every slice** |

## Not on the sequence

Carried from slice 001's follow-ups, deliberately unscheduled. Each has a
condition rather than a position.

- **The boundary scanner is a text scan.** A `//` inside a string literal hides
  the rest of its line, `/* */` is not cut, all-caps compounds do not split, and
  path tokens match as substrings. Latent today; the build gate holds the
  stratum property independently. Revisit the first time `src/` acquires one of
  those forms (F-45, F-49).
- **Time-of-day strings no author writes on purpose** — `1:2:3:4:5` and `99:99`
  parse as a time of day, `T1:30` is unparseable where `T18:00` is not, and a
  config `timeout` written as a full datetime is told it is a time of day.
  Recorded, not acted on. A fixture per case if any is ever reported.
- **Slice 001's design drift**, listed in its `audit.md`. Left as written by
  user decision: SPEC-001 is the living truth, and each departure is documented
  at its site in the code.

## Open decisions

- **Where capability declaration (OQ-1) and validation feedback (OQ-2) land.**
  Both are additive fields on a view — `field.value`, `field.error`, a
  form-level message — and slice 001 confirmed neither needs a breaking
  restructure. Both need **a protocol version bump or a capability
  declaration**, because an older host that ignores `field.error` shows a form
  with no sign anything was rejected: tolerating a field is not honouring it
  (F-7 corrected the original analysis, which claimed otherwise). Per-field
  errors are semantics and must be typed fields, never keys in `hints`.
  *Recommendation:* slice 006, alongside the field-capable renderer work the
  examples will want. They could equally be their own slice after 002.

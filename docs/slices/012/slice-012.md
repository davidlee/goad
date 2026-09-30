# Slice 012: the backend author's kit

**Stage:** plan
**Tier:** 2 (full) — a new binary, the checker, enters SPEC-004's scope, and
whatever the checker reports may need canon of its own.
**Depends on:** —

## Purpose

A goad backend is written against the wire contract, and that contract has
stood still since 007. Today the only way to learn it is to read this
repository: SPEC-001..003, the two examples, and whatever the host's normalizer
happens to accept. The next backend author is, in practice, a coding agent
working in someone else's repository — and nothing is packaged for it.

Once this lands, that agent gets everything needed to write a goad backend
**without reading this repo**, and a way to find out — before a person runs the
host — whether what it wrote is accepted, and if not, which side is wrong.
Discharges brief §21 AC-15.

## Scope

Decided before scoping (roadmap §012, 2026-09-26):

- **A plugin the repo exposes**, agent-generic: a Claude Code manifest and a
  Codex manifest over one skill. The skill carries a backend-facing reference
  and new examples; no scripts, since none has a job of its own
  (`design.md` §6 OQ-4).
- **A headless checker binary.** It drives a backend command with requests and
  judges each reply with the host's own normalizer and failure taxonomy, saying
  which side was wrong. It is a new binary, so SPEC-004 names its exit status.
- **A reference that cannot drift silently.** A thinner, backend-facing view of
  SPEC-001..003 — not a second spec. Every wire example it ships is a fixture
  the gate runs through normalization.

**A walk capsule** — an oubliette capsule whose target is `goad-walk`, a
sibling repo whose flake exports the goad packages and the kit as its tool set:
a fresh home per walk, credentials pushed in, egress through an allowlisting
proxy (`design-log.md`, 2026-09-30, superseding the consumer jails). It proves
the kit stands alone; it is not shipped to anyone.

**Examples, split by job** (`design-log.md`, 2026-09-26, superseding the
roadmap's "moved, not copied"). The current `examples/` are host exercisers —
`just demo` and `goad-shell`'s integration tests run them — and stay in the
repo, renamed so they do not read as the file to copy. The skill ships **new**
examples, written to be both teaching examples and demo backends — each
silent (`view: null`) unless it has a reason to speak, owning its own state,
and demonstrable in under a minute from **Check now** or `goad-emit`:

| example | language | behaviour |
|---|---|---|
| focus check | Python | a choice view titled with the current focus: *continue for 10 min* (with a progress-log text field) / *switch focus to ___* (text field) / *take a quick break* |
| Downloads triage | shell + `jq` | a user-owned `inotifywait` watcher emits each new file; the backend asks where it goes (a choice field) and moves it |
| breadcrumbs | TypeScript (deno) | a context switch (shell `chpwd` or workspace hook) emits *leaving X*; the backend asks where you were, and shows the note back on return |

`python3` and `jq` join the devshell so the gate can run them (`design-log.md`,
2026-09-26, 2026-09-30).

Surfaces (settled in the design conversation, `design-log.md`; exact files
are design's):

- `crates/goad-check` — new, stratum 3, driving `goad_shell::host::Host`; its
  tests carry every kit check (OQ-9).
- `crates/goad-semantics`, `crates/goad-shell` — `requirement()` and `fault()`
  beside each error taxonomy; the host kind names and a pure R-57
  value-per-kind lifted into stratum 1; fixture `requirement` lists corrected
  (OQ-8, OQ-4); `config::Command::from_argv` made public for the checker's
  argv form (review F-33).
- `crates/goad` — `Stimulus::kind` and `draft.rs::submitted` delegate to the
  lifted code (OQ-4).
- `crates/goad-emit` — an answer to `--help`/`--version` that stdout refuses
  exits 2 (`design-log.md`, 2026-09-30, U6).
- `crates/goad-boundary` — the allowlist module doc's member list, which
  `goad-check` joins.
- `crates/goad-check/Cargo.toml` — a comment arguing that it links no
  renderer, as `goad-emit`'s does (FU-7).
- `kit/` and the root marketplace manifests — the plugin (OQ-6).
- `examples/` renamed; its referencing sites, including `goad-shell`'s
  integration tests and `justfile`.
- `flake.nix` — `goad-kit` and `goad-check` packages, `python3`, `jq`.
- `~/dev/goad-walk` — a new sibling repo: the walk capsule's target, with
  `ruby` in its tool set only. Its registration with oubliette is
  oubliette-side configuration.
- **Canon**, drafted in `canon-delta.md`: SPEC-001 (requirement id and side
  at fault on each refusal; R-16's non-empty clause for a `choice` field's
  `options`; R-45 reworded to cover a failure of an exchange whichever side
  caused it; R-56's and R-57's verification rows), SPEC-004
  (`goad-check` and `goad-emit`), POL-001 (the command block's `deno check`
  paths), ADR-003 (the member list gains `goad-check`).

## Non-goals

- **Socket transport** (014), **SPEC-001 OQ-1** (capabilities) and **OQ-2** —
  each amends the kit when it lands. The kit documents the process transport
  only.
- **Brief §21 AC-14**, the interstitial-journal example. The scenario is to be
  replaced; its replacement is not this slice's.
- **An SDK.** Brief §15.2: the examples demonstrate that none is required.
- **Any change to the wire contract.** The kit describes it; it does not move it.

## Acceptance criteria

- [ ] AC-1 — A fresh agent in a walk capsule — the skill and the flake's
  exported goad packages, nothing else from this repository — writes a backend
  the checker accepts, with at least one view answered, and a person has run
  the host against it and seen the behaviour. Walked once with Claude Code and
  once with Codex, headless, from one fixed prompt. The capsule's negative
  control passes first: no goad source in its store, and no prior agent
  session in its home. A walk whose agent read goad's source fails this
  criterion and is re-run.
- [ ] AC-2 — Every wire example the reference ships is normalized by the gate;
  an example the host would refuse fails `just check`.
- [ ] AC-3 — The checker, run against a backend, reports each refusal with the
  side that was wrong, using the host's normalizer and failure taxonomy — not a
  reimplementation of either.
- [ ] AC-4 — SPEC-004 states the exit status of `goad-check` and of
  `goad-emit`.
- [ ] AC-5 — The skill ships new examples in more than one language, each a
  small, complete backend a person would want to see run; the checker accepts
  each in the gate. The host exercisers stay in the repo, and nothing presents
  them as the file a backend author copies.
- [ ] AC-6 — A person has run the checker against the shipped examples and a
  broken backend, and seen the report (`docs/AGENTS.md` §Tiers).
- [ ] AC-7 — Canon states that each refusal the checker reports — each
  failure, discarded instruction or cleanup failure the host reports on an
  exchange — names the side at fault and the requirement SPEC-001/R-59 assigns
  its kind, and a test holds it for every refusal kind the normalizer can
  produce.
- [ ] AC-8 — Token spend (input, output, cache), turns and wall time are
  recorded for every walk, first and re-walk. Indicative, not a benchmark: no
  threshold, but the re-walk does not regress.
- [ ] AC-9 — Every friction item from a walk — the agent's `ISSUES.md` and a
  read of its transcript — is dispositioned: fixed in the kit (and re-walked),
  or a follow-up. Any network fetch beyond the model API is a friction item.

## Governing canon

- SPEC-001 host/backend protocol — the contract the reference views and the
  checker judges against.
- SPEC-002 scheduling behaviour, SPEC-003 event ingress — the reference's other
  two sources.
- SPEC-004 process exit status — the checker enters its scope.
- ADR-001 one-way strata, ADR-003 workspace of strata — where the checker
  crate sits, and what it may name.
- POL-001 the phase gate — the fixture check joins it.
- ADR-002, ADR-004, ADR-005 — checked; to confirm in design.

## Open questions

- ~~OQ-1 — AC-14, the interstitial-journal example: in this slice, or its own?~~
  **Neither** — the journal scenario is to be replaced, and its replacement is
  scoped separately (`design-log.md`, 2026-09-26).
- ~~OQ-2 — Is the checker's report canon, or only its exit status (SPEC-004)?~~
  **Its exit status, and its claims:** each refusal names the side at fault and
  the requirement SPEC-001/R-59 assigns its kind — the one stating the rule the
  kind enforces, R-44 or R-45 where R-59 says (wording revised at design
  review, `design-log.md` 2026-09-30, U1, and reframed at round 3, `design-log.md` 2026-09-30, *R-59 reframed; PipeMissing; F-22; round 2's unbriefed repairs*). Its format is not canon (`design-log.md`,
  2026-09-26). Where that requirement lives is for design; where the
  requirement id comes from is for research.
- ~~OQ-3 — How AC-1 is walked: which agent, which jail, what it is given, what
  counts as passing.~~ **Both agents, headless, in a walk capsule (consumer
  jails at first, superseded 2026-09-30); checker pass plus a person running
  it; tokens and friction measured** — AC-1, AC-8, AC-9 (`design-log.md`,
  2026-09-26, 2026-09-30). The walk's task and the capsule's exact contents
  are the design's.
- ~~OQ-4 — Which requests the checker sends: evaluate only, or also response and
  event, and with what fabricated payloads.~~ **Evaluate with the three host
  kinds and one unrecognised one; author-supplied forwarded events; respond
  per view, R-57-typed; kind names and R-57 typing lifted into stratum 1**
  (`design-log.md`, 2026-09-26).
- ~~OQ-5 — Where the checker sits among the strata, and whether it is a new
  binary or a `goad` subcommand.~~ **`crates/goad-check`**, stratum 3, driving
  `goad_shell::host::Host`; renderer-free (`design-log.md`, 2026-09-26).
- ~~OQ-6 — Where the plugin lives in the tree, and how a consumer installs it.~~
  **`kit/`, with root marketplace manifests pointing at it; the flake exports
  `goad-kit` and `goad-check`; the walk capsule gets the store path through
  `goad-walk`'s tool set** (`design-log.md`, 2026-09-27, 2026-09-30).
- ~~OQ-7 — The walk's task and language — a behaviour none of the examples
  implement, in a language none of them use.~~ **An end-of-day wrap-up, in
  Ruby, stdlib only** (`design-log.md`, 2026-09-27).
- ~~OQ-8 — Where a refusal's requirement id and side at fault come from.~~
  **Total `requirement()` and `fault()` beside each error taxonomy in strata 1
  and 2, witnessed by the fixture corpus; sides are backend, host,
  configuration, environment, by where the cause lies; canon in SPEC-001**
  (`design-log.md`, 2026-09-26; sides defined at design review, U1, and R-59
  reframed at round 3, `design-log.md` 2026-09-30, *R-59 reframed; PipeMissing; F-22; round 2's unbriefed repairs*).

## Summary

## Follow-ups

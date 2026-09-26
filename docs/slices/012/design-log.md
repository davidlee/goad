# Design log — Slice 012

Append-only record of the design *conversation* — what was asked, what was
decided, in time order. It exists so that a compacted or interrupted session can
pick the thread back up. Never rewrite an entry; supersede it with a later one.

Only decisions live here. Adversarial review is owned end to end by its ledger
(`review-design.md`) — its brief, its findings, its synthesis. When a finding
prompts a decision from the user, that decision is recorded below like any
other, citing the finding id.

## Decisions

<!-- One entry per user decision, recorded immediately after the answer. -->

### YYYY-MM-DD — <question in one line>

- **Asked:** <the question and the options presented>
- **Recommended:** <agent's recommendation, if any>
- **Decided:** <the user's answer, verbatim where it matters>
- **Consequence:** <what changes in the design; D-ref if it became a §7 decision>

### 2026-09-26 — OQ-1: AC-14, the interstitial-journal example — in 012, or its own slice?

- **Asked:** fold brief §21 AC-14 into 012, or give it a slice sequenced after
  012 that uses the checker as its acceptance test.
- **Recommended:** its own slice, after 012.
- **Decided:** neither. *"I don't love the existing journal example, and I've
  been meaning to replace it with something better."*
- **Consequence:** AC-14 is out of 012. The journal scenario is not built by
  any slice as written; its replacement is the user's to scope. Parked in
  `notes.md` §Open. The kit's examples stay the moved toy backends.

### 2026-09-26 — OQ-2: is the checker's report canon, or only its exit status?

- **Asked:** (a) exit status only; (b) exit status plus the report's *claims* —
  each refusal names the side at fault and the SPEC-001 requirement it breaks —
  with the wording free; (c) a canonical, parseable report format.
- **Recommended:** (b), subject to research on whether the normalizer's
  refusals already carry a requirement id or the checker would have to map to
  one — a mapping is a second encoding of SPEC-001.
- **Decided:** (b). *"Agent sessions are short, and agents can adapt; errors
  here don't warrant the ceremony of another wire format."*
- **Consequence:** SPEC-004 names the checker's exit status. A requirement —
  home to settle in design (SPEC-004, or new canon) — states that each refusal
  names the side at fault and the violated requirement id. The report's format
  is not canon. Research item: where the requirement id comes from.

### 2026-09-26 — OQ-3: how AC-1 is walked — isolation

- **Asked:** (agent proposed `jcl`/`jcx`). User: the existing jails see the
  whole source tree; the walk must see only what a consumer with the skill and
  an installed goad sees. Options put by the user: (1) publish — crates.io, or
  the flake exporting binaries only — into a new jail; (2) new jails here that
  withhold what agents ought not to see; (3) a microVM.
- **Recommended:** (1) via the flake, not crates.io — publishing is
  irreversible and outward-facing, and a cargo consumer gets the crate source
  anyway. Two new jails defined in this flake, `jailed-consumer-{claude,codex}`:
  the exported goad packages plus the skill, none of the project's dev
  packages or the Slint mount, a fresh persisted home per walk (the shared
  `"agent"` home has held goad development sessions), launched from a scratch
  directory outside the checkout. A negative control first, inside the jail:
  reading the repo, the memory directory, and the store's `goad-source` each
  fail. (3) only if the store is not closure-scoped.
- **Decided:** agreed. Store paths outside the jail's closure are invisible
  (user, from knowledge of jail.nix), so (3) is not needed. Network may be
  disabled if necessary.
- **Consequence:** the consumer jails are in scope (flake). The negative
  control is a phase exit criterion, and still run — the user's statement
  about the store is the expectation it checks.

### 2026-09-26 — OQ-3: what the walk measures besides pass/fail

- **Asked:** (raised by the user) the walk should also measure token
  efficiency and the friction the agents meet.
- **Decided:** measure token spend per walk; each agent writes a post-run
  issue log of friction encountered; easy wins found there are addressed in
  the slice.
- **Consequence:** AC-8 and AC-9. Method in the next entry once agreed.

### 2026-09-26 — OQ-3: the walk's method, and network

- **Asked:** run each walk headless (`claude -p --output-format json`,
  `codex exec --json`; usage fields to confirm in research); one fixed prompt
  for both agents, ending in an instruction to write `ISSUES.md` (trying /
  expected / hit / did); the transcript read and tagged for friction as well,
  since self-report under-reports; an *easy win* is a kit-only change (skill,
  reference, checker wording — no canon, no wire) and is fixed in-slice; a
  fresh-agent re-walk after the fixes; no token threshold — no baseline, one
  run per agent, figures indicative. Network: left on — disabling it also cuts
  the model API.
- **Decided:** agreed. On network: *"Needing network access is a red flag for
  the usefulness of local guidance; better checked during review of
  transcripts than blocked."*
- **Consequence:** network stays on. Any network fetch in a walk's transcript
  beyond the model API is a finding against the kit. AC-1, AC-8, AC-9.

### 2026-09-26 — the examples: moved, or split by job

- **Asked:** the pre-scoping decision (roadmap §012) was that the examples
  move into the skill. Research showed `examples/shell/backend.sh` is a host
  exerciser — every R-16 kind, delay knobs, prompts on every evaluation — that
  `just demo` and `goad-shell`'s integration tests (`round_trip.rs`,
  `harness.rs`) run, and the gate typechecks `backend.ts`. Proposed: split by
  job — the host exerciser stays in the repo, renamed so it does not read as
  the file to copy; new examples ship in the skill, written to be both
  teaching examples and demo backends, two or three languages, each checker-
  green in the gate; the walk's task distinct from all of them.
- **Recommended:** the split.
- **Decided:** agreed — *"happy with the split"*. Supersedes the roadmap's
  "examples (moved, not copied)".
- **Consequence:** AC-5 rewritten. Which behaviours the new examples implement
  is the next question; the TypeScript example's fate (renamed exerciser, or
  retired) is for design.

### 2026-09-26 — which behaviours the new examples implement

- **Asked:** three candidates — A stand-up break (shell), B pre-push check-in
  (Python), C end-of-day shutdown (TypeScript). User reshaped A into a focus
  check and found B not obviously useful; agent proposed event-driven
  alternatives on the principle that a GUI prompt beats a CLI when the
  trigger has no terminal: B1 Downloads triage, B2 breadcrumbs on context
  switch, B3 long command finished.
- **Recommended:** B1; A moved from shell to Python (free text and JSON
  escaping are awkward in shell).
- **Decided:**
  - **A — focus check.** A choice view titled with the current focus;
    options *continue for 10 min* / *switch focus to ___* (a text field) /
    *take a quick break*. Notes tied to *continue*: a progress-log text field
    on that option, not repeated on all three.
  - **B1 — Downloads triage.** Kept: *"downloads watcher is a good idea, i
    like it."*
  - **B2 — breadcrumbs on context switch** replaces C.
- **Consequence:** three examples, A, B1, B2. Languages still to assign.
  Watch in the demo: fields hang off options (SPEC-001/R-15) and nothing marks
  a field optional — an empty `text` submits `""` — which is friction an
  author would meet too.

### 2026-09-26 — example languages, devshell dependencies, and the end of scoping

- **Asked:** A focus check in Python; B1 Downloads triage in shell + `jq`
  (every example must *read* its request, which the shell exerciser never
  does); B2 breadcrumbs in TypeScript (deno). The gate runs the checker
  against all three, so `python3`, `jq` and `inotify-tools` join the devshell
  (`projectPkgs`, `flake.nix`) — only deno is there now. The walk's language
  is one the examples do not use; picked in design. Scoping complete, OQ-4..6
  carried into design.
- **Decided:** *"confirm both."*
- **Consequence:** dependency addition endorsed. Stage → design.

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

### 2026-09-26 — OQ-5: where the checker sits

- **Asked:** a `goad check` subcommand (links Slint; SPEC-004's `goad`
  classes describe an event loop), or a new stratum-3 crate `crates/goad-check`
  driving `goad_shell::host::Host<ProcessBackend>` — renderer-free, sibling of
  `goad-emit` under ADR-003, reading the author's goad config via
  `Config::load`, with a `-- <argv>` convenience. Sub-question (canon):
  govern `goad-emit` in SPEC-004 at the same time as the checker, or only the
  checker (research.md R-b).
- **Recommended:** `crates/goad-check`; both binaries into SPEC-004.
- **Decided:** yes, to both, and to the name.
- **Consequence:** SPEC-004 gains the checker and `goad-emit` — a
  `canon-delta.md` entry. AC-4 widens to both binaries.

### 2026-09-26 — OQ-8: where a refusal's requirement id and side at fault come from

- **Asked:** (raised by research, R-a) nothing in code carries either today.
  Proposed: total `requirement()` and `fault()` beside each taxonomy —
  `ProtocolError`, `BoundsError`, `ScheduleError` (stratum 1); `BackendError`,
  `CleanupFailure`, `StateError` (stratum 2) — exhaustive, so the compiler
  holds completeness; the fixture corpus as independent witness (each refusal
  fixture's error names an id in its own `requirement` list), with the two
  R-17 `Json` fixtures' lists corrected. `Shape` cites R-44 only; recovering
  the misread requirement needs a serde path — follow-up. Sides: backend,
  host, configuration (`Spawn`), and "neither: observation"
  (`CleanupFailure`, SPEC-001/R-54). Canon in SPEC-001, which owns the
  taxonomy.
- **Decided:** all agreed. *"'environment' might be more consistent than
  'neither: observation'."*
- **Consequence:** the sides are **backend, host, configuration,
  environment**. Which variants are *environment* rather than *host* is
  design's to settle, variant by variant. Surfaces widen to `goad-semantics`
  and `goad-shell`. A `canon-delta.md` entry for SPEC-001. Follow-up
  candidate: a path for `Shape` refusals.

### 2026-09-26 — OQ-4: which requests the checker sends

- **Asked:** evaluate under `source: "host"` with `startup`, `requested`,
  `scheduled`, and one unrecognised host kind (making SPEC-001/R-56's
  tolerance testable; amends its §7 row, "review, not a test"); forwarded
  events supplied by the author (flag or file), never guessed; respond for
  each view seen, one option, values for exactly its fields (R-58), typed per
  R-57, through `Host::respond`. The checker's blame is honest only if its own
  values obey R-57, and the kind names and R-57 typing live in `goad`, which
  links Slint (research.md R-b, corrected). Options: lift both into stratum 1
  (`goad` delegates); depend on `goad` (links the renderer — the thing
  `goad-emit`'s crate edge prevents); restate R-57 (second encoding — ruled
  out). Examples take state and target directories from the environment; the
  gate points them at a temp directory (R-49 side effects).
- **Recommended:** the request set; lift both into stratum 1; environment-
  configured examples.
- **Decided:** yes.
- **Consequence:** `goad-semantics` gains the host kind names and a pure R-57
  value-per-kind; `goad`'s `Stimulus::kind` and `draft.rs::submitted`
  delegate — `goad` joins the surfaces. `canon-delta.md` entry: SPEC-001 §7,
  R-56's verification row. Event-driven examples ship their events beside the
  backend.

### 2026-09-27 — OQ-6: where the plugin lives, and how it is installed

- **Asked:** (research.md R-d) the kit in its own subdirectory `kit/` —
  Claude Code and Codex manifests over one `skills/goad-backend/` (SKILL.md,
  reference, examples and their event files, scripts) — with the marketplace
  manifests at the repo root, `source: ./kit`, because a root-source plugin
  copies the whole repo into the consumer's plugin cache. The flake exports
  `packages.goad-kit` (that tree only) and `packages.goad-check`. Consumers:
  `claude plugin marketplace add davidlee/goad` + `claude plugin install
  goad@goad`; `codex plugin marketplace add` + `codex plugin add`. Binaries
  from flake packages or `cargo install --git`; SKILL.md says how. Consumer
  jails load the store path (`claude --plugin-dir`, a local-path Codex
  marketplace). Caveat: `marketplace add owner/repo` clones the repo; the
  instructions recommend `--sparse kit`. `claude plugin validate kit/` as a
  phase exit check, not a gate step.
- **Recommended:** as asked.
- **Decided:** agree.
- **Consequence:** surfaces gain `kit/`, root marketplace manifests, and two
  flake packages.

### 2026-09-27 — OQ-7: the walk's task and language; `claude plugin eval`

- **Asked:** task — an end-of-day wrap-up: silent until a configured local
  hour; asks energy (number 1–5), anything left open (boolean), resume at
  (datetime); logs locally; `next_check` the next day's occurrence as an
  absolute instant with an offset (SPEC-001/R-22). Stated to the agent in
  plain language, no protocol words. It exercises what the examples leave
  unexercised, so it tests the reference rather than copying. Language — Ruby
  (stdlib JSON and time, no package fetch); Go the alternative, but `go run`
  compiles per spawn and risks the timeout. The runtime joins the consumer
  jail's closure explicitly. `claude plugin eval` (no-plugin baseline) not
  adopted for AC-1: Codex has no equivalent, and a baseline without the kit
  can learn the protocol only by fetching the repo; a follow-up as a
  regression harness for kit edits.
- **Decided:** *"yep. Ruby's slightly finicky under nixOS but that's only with
  bundler, the interpreter / stdlib is a single package.. sold"*
- **Consequence:** the walk prompt must not lead the agent toward bundler or
  gems — stdlib only; a gem install is friction (AC-9). `ruby` joins the
  consumer jails' closure. `plugin eval` → `notes.md` §Open.

### 2026-09-27 — OQ-9: how the kit joins the gate

- **Asked:** every kit check in `goad-check`'s own tests under the existing
  `cargo test --workspace` — reference response examples through
  `read_response`, event examples through `ingress::envelope::normalize`,
  request examples compared against `Request` serialization (no request
  reader exists; the host's writer is the authority), and the checker run
  against each shipped example with its event file and a temp state
  directory. POL-001 changes only its `deno check` paths (exerciser rename,
  new TypeScript example); no new gate command. AC-2 made non-vacuous: every
  reference example is a file or a fenced block with a tagged info-string
  (e.g. `json goad:response`), and the extractor fails on an untagged `json`
  block. The fixture corpus is not shipped whole; the reference cites a
  curated handful through the gate-checked path, and the walk's `ISSUES.md`
  decides whether a fuller catalogue is an easy win.
- **Decided:** agree.
- **Consequence:** `canon-delta.md` entry for POL-001's command block. Design
  questions all settled; next, `design.md`.

### 2026-09-29 — design.md §6: the nine questions raised in drafting

- **Asked:** design.md §6 OQ-1..OQ-9 (drafting's numbering, distinct from
  `slice-012.md`'s), each with a recommendation: (1) `PipeMissing` → R-37,
  host; `Io` → R-45, environment — the host obligation left undischarged,
  stated in the SPEC-001 canon entry; (2) lift `as_drawn` into stratum 1 so the
  checker submits what an untouched host form submits — orchestrator noted the
  cost: renderer defaults (the epoch, min-or-zero) move into stratum 1; (3) the
  checker answers the first option only, and names the rest; (4) no `scripts/`
  in the skill; (5) a cleanup failure alone exits 1; (6) `EmptyAlternatives`
  cites R-52, follow-up for a non-empty clause in R-16; (7) a launcher-made home
  bound over `$HOME` in this flake, follow-up for an upstream home-name
  parameter; (8) `goad` is in the consumer jail; (9) ADR-003's member list in
  `canon-delta.md`.
- **Recommended:** all nine as stated.
- **Decided:** *"accept all nine"*.
- **Consequence:** design.md §6 settled; its rows stand as drafted. Two
  follow-up candidates to `notes.md` §Open.

### 2026-09-30 — design.md presented section by section

- **Asked:** §1–§5.1 (framing, principles, system model); §5.2.1–5.2.3
  (checker CLI, run sequence, the requirement/side table — judgement calls:
  `Timeout` → backend, chain bound 8, `EnvelopeFault` without ids); §5.2.4–
  5.2.6 (lifts, report and statuses 0/1/2, kit tree, tagged fences, examples —
  drafting added: focus check asks for a focus when none is set; triage chains
  through a queue); §5.2.7–§5.5 (rename to `exercisers/`, flake, consumer
  jails, `just walk`, negative control, the walk prompt verbatim, recording,
  re-walk rule, state, edges).
- **Decided:** confirmed each (*"confirm"*, *"ok"*, *"ok"*, *"yep"*). With the
  last: `inotify-tools` **dropped** from the devshell (A-4: event files stand
  in for watchers in the gate; a person exercises `watch.sh` at audit),
  superseding that part of the 2026-09-26 dependency entry; the walk prompt
  accepted as worded; the untouched `number` value outside its own range
  (`max: -10`, no `min` → `0`) raised as a follow-up against the renderer.
- **Consequence:** design.md stands as presented. Next: §7–§10 review, then
  adversarial design review.

### 2026-09-30 — design.md §7–§10; R-59's reach

- **Asked:** the drafted R-59 ended "anything that reports such a refusal to a
  person MUST carry both" — binding `goad`'s diagnostics surface (SPEC-003/R-15)
  and its stderr, which no decision put in scope. (a) narrow: R-59 fixes what a
  refusal *names*; the checker's obligation to print both moves to SPEC-004
  R-12; the window follows in its own slice. (b) keep, and take the renderer's
  diagnostics into this slice.
- **Recommended:** (a). Also: spike R1 (Codex/Claude plugin load from a
  read-only store path) and R2 (`$HOME` bind hides the shared home) before the
  adversarial design review.
- **Decided:** *"yes, narrow and spike"*.
- **Consequence:** `canon-delta.md` R-59 narrowed; its §7 row drops the
  reporter tests, which move to SPEC-004 §7's R-11..R-13 row under R-12. A
  follow-up to `notes.md` §Open. Spike next, then the review.

### 2026-09-30 — where the walk runs

- **Asked:** the R1/R2 spike found a fresh walk home is logged out, and the
  shared home holding the logins is what the walk must hide. Options put: API
  keys through 1Password; copy credential files per walk; log in by hand.
- **User's alternative:** run the walks in an oubliette microVM capsule
  (`~/dev/oubliette`), driven over ssh with credentials as environment
  variables, home set-up automated — as doctrine slices are driven. No
  display is needed (checker and emitter are headless; OQ-8 already keeps the
  window out). The target is *"a flake that specifies the environment"*: a
  sibling repo, `~/dev/goad-walk`. The proxy refuses all but the model APIs
  and a short package-manager allowlist.
- **Also:** the kit-shape correction from the spike (`goad-kit` is a
  marketplace root; Claude loads `$KIT/kit`) — *"yep"*.
- **Decided:** capsule walks; `goad-walk` a sibling repo; fetch attempts are
  friction whether or not the proxy allowed them.
- **Consequence:** `design.md` §5.2.8–§5.2.9 rewritten (D21; D19 amended; R2
  retired; R7, R8 added; A-3 replaced; OQ-7 marked superseded).
  `slice-012.md`'s consumer jails become a walk capsule, and AC-1's negative
  control is restated for a capsule. The bwrap spike stays as evidence for
  the kit shape and the closure finding.

### 2026-09-30 — design review round 1: dispositions

- **Asked:** 33 findings (`review-design.md` F-1..F-33; F-33 from a Codex
  second witness, four of whose five findings corroborated the raiser's). A
  responder draft verified each against the source and grouped twelve into
  eight decisions:
  - U1 R-59's meaning: one reading ("the requirement under which the host
    refused", R-44 where nothing more specific states it); sides by where the
    cause lies; a declared imprecision (one kind, one side; the refusal carries
    the value that shows another cause); a closed scope excluding config-load
    and forwarded-envelope refusals; `Spawn` → R-44; `Io`/`PipeMissing` → R-45.
  - U2 `goad-check` statuses cut on whether a verdict was delivered (2 = none:
    pre-exchange, mid-run clock, unwritable report, checker defect); the chain
    bound is a report observation.
  - U3 no `--now`; the report says when no view was answered; AC-1 requires at
    least one view answered; `--now` a follow-up if walks ask.
  - U4 `goad-walk`'s input stays host-local `git+file:`; AC-1: a walk whose
    agent read goad's source fails and is re-run.
  - U5 respond fences checked by JSON type against `Submitted::as_drawn`'s,
    and `choice` membership; `datetime` spelling stated as unchecked.
  - U6 fix `goad-emit` to exit 2 on an unwritten answer; `crates/goad-emit`
    joins the surfaces.
  - U7 tolerate the membership witness; state its reach and name a mutation
    that reds.
  - U8 `EmptyAlternatives` → R-44 under U1 (orchestrator's recommendation,
    over the draft's R-16 amendment); the R-16 non-empty clause stays the
    OQ-6 follow-up.
  - The rest mechanical as drafted; F-26 a follow-up extending FU-7.
- **Decided:** *"yes"* — every recommendation, U8 the orchestrator's way.
- **Consequence:** dispositions written into the ledger; a fresh agent repairs
  `design.md`, `canon-delta.md` and `slice-012.md`; the kept raiser runs
  round 2.

### 2026-09-30 — design review round 2: dispositions

- **Asked:** round 2 (`review-design.md`, 74f7301) verified 32 of round 1's
  repairs, contested F-22, raised F-34..F-40. Four new findings are R-59's
  wording again. Proposed:
  - F-35: R-44 is named where **the kind cannot tell** which more specific
    rule an instance broke (the raiser's wording), not "where no more specific
    requirement states one".
  - F-34: `CleanupFailure` → **backend**, under the declared imprecision; the
    environment side loses "could not observe what it needed to".
  - F-38: **reverse U8** — add "and at least one" to R-16 in this slice's canon
    delta; `EmptyAlternatives` → R-16; the fixture's list gains R-16; the R-16
    follow-up in `notes.md` closes.
  - F-39: `PipeMissing` → **host**, **R-37** (the host asked for the pipe; only
    a host defect removes it). R-59's R-45 clause then covers `Io` alone.
  - F-22 (contested): the instance list is generated from the exhaustive
    match — each arm returns its own instances, `InapplicableKey` two.
  - F-36, F-37, F-40 mechanical: the checker spawns as the host does, from
    its working directory, and the gate and README start both from the example
    directory; re-walks bump `goad-walk`'s lock and `walks.md` records the
    goad revision; the `config::Command` note names its public fields.
  - Round 3 narrow (R-59 and F-22 only), then stop. If R-59 still leaks, the
    definition is the problem and gets reframed, not patched.
- **Decided:** *"sure"*. Also confirmed: the R-45 clause added in round 1's
  repair (now `Io` only).
- **Consequence:** round 2 repairs; the orchestration hands off to a fresh
  agent for round 3.

### 2026-09-30 — R-59 reframed; PipeMissing; F-22; round 2's unbriefed repairs

- **Asked:** round 2's repairs left R-59 failing its own letter (scope, via
  R-44's list, missed kinds each required by their own requirement;
  `PipeMissing` → R-37 was false; `Io` → R-45, `CleanupFailure::TimedOut` →
  backend and R-44 for `DuplicateKey`/`Spawn` strained). Four questions:
  1. **Reframe R-59** rather than patch it: (a) scope by channel — what the
     host reports on each channel of an exchange or answer: a failure, a
     discarded instruction, a cleanup failure, a refused answer; config-load
     and forwarded-envelope refusals excluded in terms; (b) the id, one clause
     per case — (i) the requirement stating the rule the kind enforces, (ii)
     R-44 where the kind cannot tell which more specific rule an instance
     broke, or where R-44's list is the only requirement that names the
     refusal, (iii) for a failure no requirement makes a refusal, R-45; (c)
     sides by where the cause lies — backend, "the cause lies in what the
     backend sent or did" — the declared imprecision kept. Orchestrator's
     check of every §5.2.3 row found three amendments: R-45's subject is
     "backend failure", which `PipeMissing` (host) and `Io` (environment) are
     not by its letter, so R-45 is reworded in the canon delta — "No failure
     of an exchange with the backend, whichever side caused it, may terminate
     the host…" — a wording fix, since the host already behaves so; (ii)'s
     second half names the refusal, not a rule (no rule says a command must
     be spawnable); R-59 names its subject once ("each report on those
     channels (below, a refusal)"), since a cleanup failure refuses nothing,
     and every restatement uses that noun.
  2. **`PipeMissing` → R-45, host**, under (b)(iii) — reversing round 2's
     R-37, recommended by the orchestrator in error.
  3. **F-22 (contested):** no hand-kept instance list can be forced complete
     in stable Rust; the proposed generate-from-match is circular. Options:
     state the limit in SPEC-003 §7 R-14's terms (compile gate plus review,
     not an assertion); add a variant-enumerating derive (`strum`); a test
     reading variant names from source text (precedent: `goad-boundary`'s
     `structure.rs`) — not recommended, it parses another crate's Rust by text.
  4. **Confirm round 2's repairer's two unbriefed changes:** `CleanupFailure`
     split by variant (`TimedOut` → backend; `CleanupFailure::Io` stays
     environment, a failed OS call and a failure to observe cleanup under
     R-48); `checking.md` names, per imprecise kind, where else a cause may lie.
- **Recommended:** 1 with all three amendments; 2; 3 state the limit; 4 confirm.
- **Decided:** *"accepted"* — all four as recommended.
- **Consequence:** `canon-delta.md` SPEC-001 R-59 rewritten on the reframe,
  and a new change rewording R-45; `design.md` §5.2.3 (*Meaning of the id*,
  table, rationale), §5.2.6 (coverage test states its limit; `checking.md`),
  §6 OQ-1, §10; `slice-012.md` AC-7 and OQ-2/OQ-8; `notes.md`. F-22 gets a
  round 3 Response. Round 3, narrow, follows the repair.

### 2026-09-30 — design review round 3: dispositions

- **Asked:** round 3 (`review-design.md`, 445699c) verified F-22 and
  F-34..F-40 and found no R-59 defect; it raised F-41..F-43, none a blocker.
  Proposed, each `doc-wrong`:
  - F-41: R-45's §7 row addition names the modes its survival witness runs,
    and says `Spawn`, `Io` and `PipeMissing` reach the caller through
    `Host::no_action` and that their survival is held by review — a spawn
    failure cannot be a mode of the one parameterized backend.
  - F-42: the row's counts are replaced by names — a view, then every mode in
    `PROTOCOL_MODES` and `TRANSPORT_MODES`, then the answer to that view. The
    orchestrator's check: the test runs nineteen exchanges (view, seventeen
    failures, answer); the row's "nineteen … then a successful one" counts
    twenty.
  - F-43: `CleanupFailure::Io` → R-48's rationale rests on R-48's obligation
    to initiate termination and wait, which a failed `start_kill`/`wait` call
    leaves failing through the operating system.
  - Then the raiser verifies the three mechanically and the ledger resolves;
    the design goes to plan.
- **Recommended:** as proposed.
- **Decided:** *"yes"*.
- **Consequence:** `canon-delta.md` SPEC-001 Change 7; `design.md` §5.2.3
  `CleanupFailure::Io` rationale; ledger Responses on F-41..F-43.

### 2026-09-30 — the design as a whole, after review

- **Asked:** the design review ledger resolved (`review-design.md`, State
  resolved at e4db7f5, Synthesis at 422aac8). The design changed through three
  rounds since it was approved section by section; `docs/AGENTS.md` asks for
  approval again. Summary put: R-59 reframed, four canon wording fixes (R-16,
  R-45 and its §7 row, `CleanupFailure` split, `PipeMissing` → R-45), verdict-cut
  statuses, the R-56 probe condition, the no-view line and AC-1's view, SPEC-004
  governing `goad-emit` with its code fix, the capsule walk; the stated limits
  (F-22, F-23, R-59's imprecision, R-45's reach) and what no round reached
  (manifest validation, the examples' correctness).
- **Decided:** *"yes. hand over for planning"* — `design.md` and
  `canon-delta.md` approved as they stand at 422aac8.
- **Consequence:** `slice-012.md` stage → plan. A fresh agent drafts `plan.md`.

### 2026-10-01 — two gaps the plan draft found (G1, G2)

- **Asked:** the planner (05e017c) stopped on two points the design left
  unsettled, both verified in code by the orchestrator:
  - **G1** — §5.2.4 says `goad` delegates to `Submitted::as_drawn(&FieldKind)`,
    but `view_model::as_drawn` takes a `DrawnKind` and returns an `Edited`
    whose `Adjusted` carries display text that `glass` draws through
    `view_model::untouched`. Delegating needs a `DrawnKind` → `FieldKind`
    rebuild and a `Submitted` → `Edited` conversion; without the second, the
    screen keeps a second untouched-value policy.
  - **G2** — §5.2.1 parses `--timeout` with `schedule::parse_span`, which
    accepts `0s` and `-1s`. The host's usable-timeout rule is `config.rs`'
    private `unsigned`; the checker would otherwise restate it.
- **Recommended:** G1 — `impl From<Submitted> for Edited` in `draft.rs`,
  spelling numbers through `adjusted`; `view_model::as_drawn` rebuilds the
  `FieldKind` and delegates; the identity test holds `Submitted` → `Edited` →
  `Submitted`. G2 — make `unsigned` public as `config::positive_duration`;
  its errors are status-2 usage errors.
- **Decided:** *"i'll take your recommendations"*.
- **Consequence:** `design.md` §5.2.1 (`--timeout`) and §5.2.4 (delegation)
  amended; `plan.md` PHASE-02/EN-2 and PHASE-04/EN-2 discharged by this
  entry.

### 2026-10-01 — plan review round 1: design-touching dispositions

- **Asked:** plan review round 1 (`review-plan.md`, b407b8b) raised five
  findings whose repair changes `design.md`, each verified in the tree by the
  orchestrator:
  - **F-11** — G1's conversion "in `draft.rs`, through `adjusted`" cannot be
    built: `adjusted` and `spelled` are private to `view_model.rs`, and
    `draft.rs` does not import `view_model`. Proposed: a private `as_edited`
    in `view_model.rs` beside `adjusted`, not a crate-wide `From` impl;
    `adjusted`'s doc stays true. Supersedes G1's placement only.
  - **F-18** — §5.2.6's reason for a second fence scanner ("one crate's test
    targets cannot reach another's helpers") is false: members share helpers
    from `tests/support/` through `#[path]` (memory
    `shared-test-helper-lives-at-workspace-root-via-path`). Proposed: the
    scanner lives in `tests/support/`, included by `round_trip.rs` and
    `goad-check`'s kit tests; `goad-check`'s binary tier includes the existing
    support files where they fit, and any copy the `dead_code` obstacle forces
    is named in FU-5's extension.
  - **F-6** — I-5 has no rule. Proposed: the test fails on a relative path in
    `kit/` that escapes `kit/`, and on a mention of `<name>/` for any
    top-level entry of the repository root other than `kit`, read at test
    time; with a negative control over an inline string.
  - **F-4** — `AtFault` has no printed form, so the checker would map sides
    itself. Proposed: `Display` on `AtFault` in stratum 1, printing
    `backend`, `host`, `configuration`, `environment`.
  - **F-13** — PHASE-04 is too large for a session. Proposed: split; the run
    itself (request plan, answering, chains, R-56 condition, refusal lines)
    becomes PHASE-12.
- **Recommended:** each as proposed.
- **Decided:** *"yeah go ahead"*.
- **Consequence:** `design.md` §5.2.3 (`AtFault`), §5.2.4 (the conversion's
  home), §5.2.6 (the scanner; I-5's rule), §5.5 I-5, §9 amended by the repair
  agent; `plan.md` repaired; round 2 verifies.

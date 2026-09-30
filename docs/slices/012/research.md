# Research — Slice 012

**Producers:** design agent (slice 012), by direct read of the cited sites and,
for R-e, by running the installed CLIs on a trivial prompt.
**As of:** 2026-09-26 · 5496856

Evidence artefact for design and plan. Later stages cite this instead of
re-deriving. Refresh in place when it drifts; do not append rounds.

## Verification legend

- ✓ — independently verified by the *consuming* agent (a read or grep of the
  cited site).
- unmarked — researcher claim: cited, not checked.
- *measured* — observed by running the thing, with the command stated.

Design and plan may only load-bear ✓ rows, or rows they verify at point of use.
Verify what you lean on, not everything.

## Citation forms

Canon claims cite the document id (`SPEC-003 §4`, `ADR-007`). Code claims cite
the file and symbol, never a line number (CLAUDE.md, *name, never count*).

## Thread 1 — governing canon

### Binding

- **SPEC-001 §1** already states the checker's purpose: *"a backend author can
  … know from the error they get back which side was wrong."* The kit is the
  first thing that delivers that sentence to someone outside the repo.
- **SPEC-001/R-44** is the failure taxonomy: a list of refusal classes that
  MUST each map to a distinct error. It is a host obligation. It names classes,
  not the requirement a backend broke.
- **SPEC-001/R-56** names the three host-originated kinds (`startup`,
  `requested`, `scheduled`), makes the set **open**, and puts a backend
  obligation in canon: *"a backend MUST tolerate a kind it does not
  recognise."* SPEC-001 §7's R-56 row says the tolerance clause is *"review,
  not a test"*, like SPEC-001's other backend-side obligations. A checker is
  the first thing that could test one.
- **SPEC-001/R-57, R-58** fix what a `respond` carries: one value per drawn
  field of the answered option, typed by `kind`. SPEC-001 §7's R-57 row: *"a
  mapping stated in two places is a mapping that can drift."* A checker that
  sends `respond` is bound by both requirements, just as the host is.
- **SPEC-001/R-32** — a `respond` needs the outstanding `view_id`, which the
  host minted (R-30). A checker that sends `respond` has to hold interaction
  state.
- **SPEC-001/R-49** — a failure is not a claim that nothing happened. A backend
  that the checker drives has real side effects (Downloads triage moves files).
- **SPEC-004 §2 Boundaries / OQ-2** — SPEC-004 already *owns* `goad-emit`'s
  exit status but does not *govern* it: §4 writes requirements for the host
  alone. It says admitting a second binary is *"an append to §4 and a column
  in §6"*. The checker would be the third binary. §3 P-A..P-D bind every
  binary, governed or not.
- **ADR-003 Decision** — stratum 3 holds entry points; *"a second binary is a
  second entry point"* is how `goad-emit` became its own member. The same rule
  admits a checker.
- **POL-001 §Compliance** — the gate's command block is canonical and names
  `deno check examples/typescript/backend.ts` by path, and says *"The gate is
  **six commands**"*. Renaming `examples/` amends POL-001. So does adding a
  command.

### Checked, not applicable

- **ADR-002** — superseded by ADR-003. It adds no constraint of its own.
- **ADR-004** (scheduled-firing spacing) — host timer behaviour. The checker
  runs no timer.
- **ADR-005** (the event envelope normalizes in stratum 2) — applies only if
  the reference ships SPEC-003 envelope examples. Those normalize through
  `goad_shell::ingress::envelope::normalize` (stratum 2), which fixes where
  their gate check can live (R-g).
- **SPEC-002** — the reference summarises it for authors; the checker does not
  exercise it.

### Amendment candidates

- **SPEC-004** — the checker's exit status (AC-4). Whether `goad-emit` is
  admitted at the same time is a question (see *Cross-thread*).
- **The AC-7 claim** — each refusal names the side at fault and the
  requirement it breaks. Its home is open: SPEC-004, SPEC-001 §6/§7, or new
  canon.
- **SPEC-001 §7, R-56 row** — *"review, not a test"* for backend tolerance
  would be stale if the checker tests it. Only if the design makes it test
  that.
- **POL-001 §Compliance** — the renamed `deno check` path, a `deno check` for
  the new TypeScript example, and any new gate command.

## Thread 2 — code map

### R-a — where a refusal's requirement id comes from

**Finding: no error type carries a requirement id, and nothing maps an error
to one.** Requirement ids exist in code only in the fixture envelope's
`requirement` array (`crates/goad-semantics/tests/protocol/runner.rs`,
`Envelope::requirement`, documented as *"The `R-N` ids this case verifies"*),
and in doc comments. The filename convention `R-16-a-text-field.json` repeats
the first of those ids.

The refusal kinds the host can produce, by taxonomy:

| taxonomy | where | variants |
|---|---|---|
| `ProtocolError` | `goad_semantics::error` | `Json`, `Shape`, `DuplicateKey`, `NestedHints`, `UnsupportedProtocolVersion`, `UnsupportedPrimitive`, `InapplicableKey`, `MissingField`, `EmptyOptions`, `DuplicateOptionId`, `DuplicateFieldId`, `DuplicateAlternativeId`, `EmptyAlternatives`, `Bounds`, `Schedule` |
| `BoundsError` | `goad_semantics::error` | `NotFinite` (unreachable from JSON, by the type's own doc), `Inverted` |
| `ScheduleError` — a **discard**, never an `Err` (`Normalized::discarded`, `Discarded::Schedule`) | `goad_semantics::error` | `NotAString`, `MissingOffset`, `TimeOfDay`, `CalendarUnit`, `OutOfRange`, `Unparseable` |
| `BackendError` | `goad_shell::error` | `Spawn`, `Timeout`, `ExitStatus`, `OutputTooLarge`, `PipeMissing`, `Io`, `Protocol(ProtocolError)` |
| `CleanupFailure` — a second channel, orthogonal to failure (R-54) | `goad_shell::error` | `TimedOut`, `Io` |
| `StateError` — refused before any backend is consulted | `goad_shell::error` | `NoOutstandingView`, `StaleViewId` |

`ProtocolError::Schedule` exists in the enum but never arrives as an `Err`.
`normalize.rs::every_reachable_error_in_the_taxonomy_is_named_by_a_fixture`
skips it for that reason.

**Error kind → requirement is many-to-many in the corpus.** This table was
made by tabulating every error fixture's `requirement` list against its
`expect.error` tag (`jq` over `tests/fixtures/protocol{,-text}/*.json`):

| error | fixtures' requirement lists | one id consistent with every fixture? |
|---|---|---|
| `Shape` | [R-11,R-44] [R-13,R-44] [R-15,R-44] [R-19,R-44] [R-3,R-44] [R-44,R-13] [R-52,R-44] | **R-44** — and only R-44. The requirement the backend actually misread (R-13 title, R-15 field keys, …) is not recoverable, because `Shape` wraps a serde error from derived deserialization and has no path. |
| `Json` | [R-17] [R-17] (NaN and infinity literals) | none shared with R-44 or R-38. Either the fixtures gain R-44/R-38, or `Json` cites R-17 — which is wrong for every other malformed document. |
| `DuplicateKey` | [R-44] [R-52,R-44] | R-44 |
| `MissingField` | [R-10] | R-10 (the `field` is `&'static str`; `view` is the only site found) |
| `UnsupportedPrimitive` | [R-12,…] ×4 | R-12 |
| `UnsupportedProtocolVersion` | [R-3] | R-3 |
| `InapplicableKey` | [R-50] ×3, [R-53] (`fields` on an alternative, raised by `normalize_alternative` through `inapplicable("fields", "choice", …)`) | **no single id.** R-50 or R-53 depends on the key and the site. |
| `EmptyOptions` | [R-13] ×2 | R-13 |
| `DuplicateOptionId` | [R-14,R-52] | R-14 or R-52 |
| `DuplicateFieldId` / `DuplicateAlternativeId` | [R-52] / [R-52,R-53] | R-52 |
| `EmptyAlternatives` | [R-52,R-53] | R-52 or R-53 |
| `NestedHints` | [R-18,R-47] | R-18 |
| `Bounds::Inverted` | [R-17] | R-17 |
| `Schedule` discards | [R-25,R-51] | R-25, with R-21/R-22/R-23 by variant |

What follows from the finding:

- **The checker would need a mapping, and none exists to reuse.** Putting it in
  the checker is the second encoding the slice exists to prevent.
- **The one place a single encoding can live** is beside each taxonomy: a total
  `requirement()` (and a side-at-fault) over each enum, in the stratum that owns
  the enum. That is stratum 1 for `ProtocolError`, `BoundsError` and
  `ScheduleError`, and stratum 2 for `BackendError`, `CleanupFailure` and
  `StateError`. The compiler then holds exhaustiveness. The corpus is an
  independent **witness**, not a second encoding: a test can assert that each
  error fixture's produced error names a requirement in that fixture's own
  list. That test fails today on `Json` (the R-17 fixtures) and on
  `InapplicableKey` if it cites one id for both R-50 and R-53.
- **`Shape` is coarse by construction.** An honest id for it is R-44 (*"a
  well-formed document of a shape the protocol refuses"*). Naming the
  requirement the backend misread would take a change to how normalization
  reports serde failures (a path, at least). That is a stratum-1 change of
  unknown size. It does not touch the wire.
- **Precedent for a "which rule" string:** `goad_shell::ingress::client::SendFault::NonConforming(&'static str)`,
  whose doc says *"The string says which rule"*. `goad-emit`'s `render::fault_line`
  splits its faults *"by which side was wrong"*. So side-at-fault already exists
  in the codebase as prose on each variant, and as nothing a program can read.

### R-b — what a headless checker can reuse

- `goad_shell::host::Host<B: Backend>` is the whole headless exchange: the
  transport, then `read_response`, then schedule resolution, then state
  update. It returns an `Outcome` carrying `view: Option<Presented>` (with the
  minted `view_id`), `discarded`, `stderr`, `failure: Option<Failure>`
  (`Backend(BackendError)` or `State(StateError)`) and `cleanup`. It has no
  renderer dependency. **A checker that drives `Host<ProcessBackend>` judges
  with the host's own normalizer, taxonomy and interaction identity by
  construction** — which is AC-3 met structurally, not by a parallel
  implementation.
- `Host::new(Config, B, now)` takes a whole `goad_shell::config::Config`
  (`backend.command`, `backend.timeout`, `schedule.default_poll`, optional
  `ingress`). `Config::load(path)` / `Config::parse(text)` build one, and so
  does `config::Command::new` plus a struct literal. **The checker can read the
  author's own goad configuration file**, and so check exactly the command and
  timeout the host will run.
- `goad_shell::backend::process::ProcessBackend` is async (tokio). The checker
  needs a tokio runtime. `goad-emit` has none, so it is not a precedent here.
  The workspace `tokio` features already include `rt` and `macros`.
- `goad_shell::clock::wall_clock` supplies `now`.
- **The host-originated kinds live in stratum 3's `goad` crate:**
  `crates/goad/src/wire.rs`, `Stimulus::kind` → `"startup" | "requested" |
  "scheduled"`. `crates/goad` **has** a lib target (`crates/goad/src/lib.rs`,
  one `pub mod` per module, `wire` among them), and `Stimulus` is `pub`. So a
  sibling crate *can* name it. The cost is what comes with it: `goad`'s
  `[dependencies]` carry `slint` unconditionally, and its `build.rs` compiles
  `ui/app.slint`. A checker depending on `goad` therefore builds and links the
  renderer. Slice 005/AC-7 made the opposite property the reason `goad-emit` is
  its own crate: *"links no renderer, and the crate edge is what"* holds it
  (`crates/goad-emit/Cargo.toml`'s comment: *"No `slint`, so a renderer type
  is `error[E0433]` here"*). The nix package would presumably also need
  `goad`'s GUI build inputs; that is not measured. So the case for lifting the
  three names into stratum 1 or 2 is **keeping a headless binary
  renderer-free**, not reachability.
- **R-57's value-per-kind lives in stratum 3 too:** `crates/goad/src/draft.rs::submitted`.
  This one is genuinely unreachable from outside the crate, because it is
  `pub(crate)`. Its argument type, `Edited`, is `pub`, but it is a widget-state
  type. Widening `submitted` would also buy the renderer dependency above.
  There is no stratum-1/2 function that says *"a value of R-57's type for this
  `FieldKind`"*.
- **Placement under the instruments:** `goad-boundary`'s manifest allowlist
  bills strata 1 and 2 only. Its module doc says *"a stratum-3 manifest is
  billed by nothing here"*. A stratum-3 checker crate adds no allowlist entry.
  The vocabulary scan reads every member's `src/` (`.rs`, `.slint`) and
  excludes `tests/`, so the checker's own source must be domain-free. Example
  backends outside `crates/` are not scanned.
- **`goad` subcommand vs new binary:** `goad`'s package is wrapped with
  `LD_LIBRARY_PATH`/`FONTCONFIG_FILE` and links Slint (`flake.nix`
  `goadPackages.goad`). SPEC-004 §4 governs `goad`'s statuses as a host (0
  answered, 1 stopped running, 2 never started). A checker subcommand would
  have to fit a cut about event loops, or split `goad`'s §4. `goad-emit` is the
  precedent for a separate member (ADR-003).

### R-c — the requests a checker could send

- `Request` is `Evaluate { now, event }` or `Respond { view_id, now, response }`
  (`goad_semantics::protocol::canonical`). `Event { source, kind, timestamp,
  data }` has public fields, so any source and kind can be fabricated.
  `Request` serializes; there is **no request deserializer** anywhere, because
  requests are outbound only.
- `Host::evaluate(now, event)` and `Host::respond(now, view_id, UserResponse)`
  are the two entry points. `respond` checks the id against host state
  **before** the transport (R-32), so the checker gets interaction identity for
  free if it drives `Host`. It only has to answer the `Presented.view_id` it
  last received.
- A `respond` must carry, for the chosen option, a value for **exactly** that
  option's fields (R-58), each typed by kind (R-57). The option is chosen from
  `Choice::options()`. Values: `text` → string, `boolean` → bool, `number` → a
  number inside `NumberRange` if bounded, `choice` → an `Alternative` id,
  `datetime` → RFC 3339 with offset. If the checker composes these with its own
  match, that match is a second R-57 encoding (R-b).
- **Forwarded events** carry the writer's `source` and `kind` (SPEC-001/R-56,
  SPEC-003/R-11). The checker cannot guess them. Downloads triage's watcher and
  breadcrumbs' `chpwd` hook have their own vocabularies. So forwarded events
  have to be **supplied by the author** — flags or a small file — and the
  checker only fabricates the host's own.
- **Backend-side obligations the checker could exercise** (none has a test
  today): tolerating an unrecognised host kind (R-56). A backend that fails on
  `{source:"host", kind:"<unknown>"}` has broken R-56, and the checker can say
  so.
- **Side effects are real.** The checker spawns the author's command with the
  inherited environment (`ProcessBackend::exchange` uses `tokio::process::Command`
  with no `env_clear`). A gate run over the shipped examples needs each example
  to take its state and target directories from the environment, and the test
  to set them to a temp dir. Downloads triage moves files, and every example
  owns state.

### R-d — plugin packaging (Claude Code 2.1.280, codex-cli 0.155.1; *measured* 2026-09-26)

- **Dual-agent precedent on this machine:** `~/.claude/plugins/marketplaces/diagram-design`
  (v2.6.33) ships one tree for both agents:
  - `.claude-plugin/plugin.json` — name, description, version, author, license
  - `.claude-plugin/marketplace.json` — `{"name", "owner", "plugins":[{"name","source":"./"}]}`
  - `.codex-plugin/plugin.json` — the same fields plus `"skills": "./skills/"`
    and an `interface` block (displayName, descriptions, capabilities,
    defaultPrompt)
  - `.agents/plugins/marketplace.json` — the Codex marketplace:
    `plugins:[{"name","source":{"source":"local","path":"./"},"policy":{…},"category"}]`
  - `skills/<name>/SKILL.md` plus `references/`, `scripts/`, `assets/`

  One skill directory serves both.
- **Install, Claude Code:** `claude plugin marketplace add <path | owner/repo |
  URL>` (supports `--sparse` for monorepos), then `claude plugin install
  <plugin>@<marketplace>`. There is a per-session form, `claude --plugin-dir
  <path>`, that needs no install and suits a jail. Also available:
  `claude plugin validate <path>` (manifest check), `claude plugin details`
  (reports the plugin's *projected token cost*), and `claude plugin eval`
  (runs eval cases against a plugin, with a no-plugin baseline arm).
- **Install, Codex:** `codex plugin marketplace add <local path | owner/repo[@ref]
  | git URL>` (supports `--sparse`), then `codex plugin add <plugin>@<marketplace>`.
  `~/.codex/skills/` exists on this machine as a user skills directory; whether
  Codex 0.155 also reads `~/.agents/skills` is unverified.
- **A root-level plugin copies the whole repo.** With `"source": "./"` the
  plugin *is* the repository, and the consumer's plugin cache receives crates,
  canon and slice docs — everything the walk must not see. A kit
  subdirectory as the plugin source, with the marketplace manifests at the root,
  avoids that. So does a flake package holding only the kit tree.
- **Binaries:** a skill cannot install binaries. A flake consumer gets them as
  flake packages (`packages.goad`, `packages.goad-emit` exist today, built by
  crane from `goad-source`; a checker would join them). A jail puts them in the
  closure. A non-nix consumer uses `cargo install --git`. In a walk, `nix run
  github:…` is a network fetch and so a friction item (AC-9).

### R-e — token usage fields (*measured* 2026-09-26, trivial prompts)

- **`claude -p --output-format json`** emits one `result` object with
  `num_turns`, `duration_ms`, `duration_api_ms`, `total_cost_usd`, `usage` and
  `modelUsage`. `usage` has `input_tokens`, `output_tokens`,
  `cache_read_input_tokens`, `cache_creation_input_tokens` (split into
  `ephemeral_5m`/`ephemeral_1h`), `output_tokens_details.thinking_tokens` and
  `server_tool_use.{web_search_requests, web_fetch_requests}`. `modelUsage` is
  keyed by model id, each with `inputTokens`, `outputTokens`,
  `cacheReadInputTokens`, `cacheCreationInputTokens`, `costUSD` and
  `thinkingTokens`. `subagent_stats` counts spawned subagents.
  - **Session totals, not last-turn:** a 3-turn run (two Bash calls) reported
    `usage` equal to `modelUsage`'s single entry, with cache reads about three
    times the single-turn figure. For multi-model sessions, sum `modelUsage`.
  - `server_tool_use` counts only the *built-in* web tools. A `curl` in Bash
    does not appear there, so the transcript read (AC-9) is still needed.
  - The transcript: `--output-format stream-json --verbose` streams every
    message, ending in the same result object (documented, not measured
    here). The session JSONL under the jail's `~/.claude/projects/` is the
    other copy.
- **`codex exec --json`** emits JSONL events: `thread.started`,
  `turn.started`, `item.started`/`item.completed` (`item.type` ∈
  `agent_message`, `command_execution`, …), and `turn.completed` with `usage:
  {input_tokens, cached_input_tokens, cache_write_input_tokens, output_tokens,
  reasoning_output_tokens}`.
  - **Per turn, cumulative within it:** a run with two shell commands reported
    one `turn.completed` whose input was about twice the single-call figure.
    `cached_input_tokens` is a **subset** of `input_tokens`
    (27392 < 27936), where Claude's cache fields are **disjoint** from its
    `input_tokens`. A table comparing the two has to normalise that.
  - No cost, no duration and no turn count: wall time must be measured
    outside the process, and "turns" is the count of `command_execution` /
    `agent_message` items. The JSONL stdout is itself the transcript.
    `--ephemeral` suppresses the on-disk session.

### R-f — what references `examples/` today (`git grep`, excluding `docs/slices/`)

| site | what it does |
|---|---|
| `justfile` `typecheck` | `deno check examples/typescript/backend.ts` — a gate command |
| `justfile` `demo` | `run "examples/demo.toml"` |
| **POL-001 §Compliance** | the canonical gate block names the same `deno check` path — **canon** |
| `examples/demo.toml` | `command = ["bash", "examples/shell/backend.sh"]` (cwd-relative) |
| `crates/goad-shell/tests/integration/harness.rs` | the deno example's argv (`../../examples/typescript/backend.ts`) |
| `crates/goad-shell/tests/integration/round_trip.rs` | `include_str!` of `examples/typescript/README.md`; runs `examples/shell/backend.sh`; a doc comment calls `backend.sh` *"the file a person copies to write their own"* — false after the split |
| `README.md` | `just demo` against `examples/demo.toml` and *"the ten-line shell backend"* |
| `.gitignore` | a comment on `examples/demo.toml`'s socket |
| `flake.nix` | a comment (`goad-shot … examples/demo.toml`) |
| `examples/shell/backend.sh`, `examples/typescript/{backend.ts,README.md}` | self-references |
| `docs/brief.md` | a tree listing `examples/`; §15.2 asks for small complete examples in several languages |
| `docs/roadmap.md` | `just demo … examples/shell/backend.sh` |
| `docs/memory/` | `a-backend-exchange-has-no-useful-duration`, `cite-requirements-not-finding-ids`, `deno-run-does-not-typecheck`, `path-flake-ref-breaks-on-demo-socket` |

`crates/goad-shell/src/config.rs` and `crates/goad-emit/src/main.rs` spell
`./backend.ts` in test literals. That is not a path into `examples/`.

### R-g — how the gate can hold the kit

- **The corpus runner** (`crates/goad-semantics/tests/protocol/runner.rs`):
  `Corpus { root, check }` over a flat directory of envelope files
  (`requirement`, `description`, `now`, `input`, `expect`), with a vacuity
  guard (`Fault::Vacuous`) and `deny_unknown_fields`. Roots are paths relative
  to the crate (`../../tests/fixtures/protocol`). Adding a root that points
  into the kit tree is one `Corpus` constant, and the runner already reads files
  in the stratum-1 test target. `check_protocol` runs `read_response` — the
  host's own door. `expect` is `accepted` (with the full canonical rendering
  and a required `discarded`) or `error` (the externally-tagged error).
- **The existing corpus is itself a catalogue a backend author could read:**
  64 files under `tests/fixtures/protocol{,-text}/`, each with requirement ids,
  a one-sentence protocol description, and accepted-or-refused. The gate already
  runs it.
- **Requests have no inbound reader**, so a request example in the reference
  cannot be *normalized*. The existing guard is `canonical.rs`'s serialization
  tests, which compare `serde_json::to_value(&Request)` against SPEC-001 §6.1's
  literals. A reference request example would be checked the same way: a
  canonical `Request` built in a test and compared as a JSON value.
- **SPEC-003 envelope examples** normalize in stratum 2
  (`goad_shell::ingress::envelope::normalize`, ADR-005). A check over them sits
  in `goad-shell`'s tests or above. It cannot sit in stratum 1.
- **One home for all of the kit's checks:** a stratum-3 checker crate may name
  both strata below it. Its test target can run `read_response`, envelope
  normalization, `Request` serialization, **and** the checker binary against
  each shipped example (`CARGO_BIN_EXE_…`, the shape of
  `crates/goad-emit/tests/binary/exchange.rs`), all under `cargo test
  --workspace`. That puts the kit in the gate **without a new POL-001
  command** — only the `deno check` path(s) change. The integration tier
  already spawns `deno` and `bash` examples (`harness.rs`), so a test that
  spawns interpreters has precedent. `python3`, `jq` and `inotify-tools` join
  `projectPkgs` (endorsed, `design-log.md` 2026-09-26). The crane packages
  build with `doCheck = false`, so the nix build is unaffected.
- **Markdown as the source of examples:** if the reference writes examples as
  fenced blocks, a test must extract them. The fence info-string (e.g.
  ` ```json response`, ` ```json refused R-13`) is where the annotation can
  live. An untagged `json` block would be unchecked, so the extractor needs to
  refuse untagged JSON blocks, or the vacuity hole returns.

## Cross-thread findings

1. **AC-3 is structural if the checker drives `Host`.** No reimplementation of
   the normalizer, the taxonomy or interaction identity. What the checker
   *adds* is choosing requests and reporting.
2. **AC-7 has no source today.** The requirement id and the side at fault
   are both prose on each variant. Making them data costs a total method per
   taxonomy enum in strata 1 and 2. That is new host code, not checker code,
   and it widens the slice's surfaces to `goad-semantics` and `goad-shell`.
   The corpus then witnesses it for free (a fixture test), and exhaustiveness
   is the compiler's.
3. **Three encodings the checker would otherwise duplicate** sit where a
   headless checker should not depend on them, or nowhere:
   - the host kinds (`Stimulus::kind`) are `pub` in `goad`'s lib, but that lib
     links Slint;
   - R-57's value typing (`draft.rs::submitted`) is in the same crate, and is
     also `pub(crate)`;
   - requirement ids exist nowhere.

   Each needs lifting or restating. Restating R-57 in the checker is
   the exact drift SPEC-001 §7's R-57 row warns about. And the checker's blame
   is only honest if its own `respond` obeys R-57/R-58. Otherwise a backend
   that rightly exits non-zero on a mistyped value is reported as the side at
   fault.
4. **SPEC-004 ordering.** Governing the checker while `goad-emit` stays
   ungoverned (SPEC-004 OQ-2) leaves the third binary governed and the second
   not.
5. **The walk's jail closure must hold the walk language's runtime.** Consumer
   jails exclude `projectPkgs` by decision, so whatever language OQ-7 picks is
   an explicit jail package.

## Design-input deltas

- The OQ-2 decision (report claims are canon, the format is not) stands. The
  new fact is that **meeting it costs host code in strata 1 and 2**, and that
  `Shape` can honestly name only R-44 unless normalization changes.
- The fixture corpus can serve the kit twice: as the AC-7 witness, and
  possibly as a shipped catalogue of accepted and refused examples.
- AC-2 is cheapest if the kit's checks live in the checker crate's tests under
  `cargo test --workspace`. POL-001 then changes only in its `deno check`
  path(s).
- Nothing here touches the wire contract.

## Spike: R1 and R2 (2026-09-30)

Run before the adversarial design review (`design-log.md` 2026-09-30). The
fixture is `spike/`: a stub kit whose skill carries a marker, and consumer
jails for Claude, Codex and a plain shell, built from the pinned `pub` jail
library with `design.md` §5.2.8's options. `spike/run.sh` reproduces every
result below. Claude Code 2.1.280, codex-cli 0.155.1.

- **R2 holds.** A launcher-made directory bound over `$HOME` after
  `persist-home` shadows the shared `agent` home. From inside: the checkout,
  `~/.claude`, `~/.codex`, the goad memory directory and every
  `*-goad-source` path are unreadable; outside, each is readable (the control).
  Writes to `$HOME` land in the walk home.
- **`set-env` does not put a store path in the jail.** With `GOAD_KIT` set
  only by `set-env`, the variable is present and the path is absent: bwrap
  binds the runtime closure of the jail's packages, and an environment string
  is not one. The kit must be a package dependency (`extraPkgs`). §5.2.8's
  claim that `set-env` puts it in the closure is wrong.
- **R1, Codex: the kit-only store path is refused.** `codex plugin
  marketplace add "$GOAD_KIT"` fails: *"marketplace root does not contain a
  supported manifest"*. Codex installs from a marketplace root, and §5.2.8's
  `goad-kit` is the plugin root (`./kit`). A store path shaped like the
  repository — `.agents/plugins/marketplace.json` beside `kit/` — adds,
  installs (`goad@goad`, *installed, enabled*), and Codex copies only `kit/`
  into `~/.codex/plugins/cache/`, so the consumer's cache still never holds
  more than the plugin. Read-only is no obstacle.
- **R1, Claude: both forms load from a store path.** `--plugin-dir
  "$GOAD_KIT"` (the plugin root) lists `goad-backend` in `claude plugin
  details`; so does a marketplace add of the repository-shaped path followed
  by `plugin install goad@goad`.
- **Not reached: the model seeing the skill.** A fresh home is logged out:
  Claude answers *"Not logged in · Please run /login"*, Codex `401
  Unauthorized`. Nothing in the design says how a walk authenticates, and the
  shared home — where the logins live — is exactly what R2 hides. Open.

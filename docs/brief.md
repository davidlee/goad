# Programmable Personal Intervention Shell

**Implementation brief — v0.1.0**  
**Status:** Ready for implementation  
**Working product name:** goad

## 1. Product purpose

Build a small native desktop application shell for personal software that is too specific to justify a conventional product.

The shell provides a consistent GUI, scheduling, external event ingress, backend invocation, response collection, and diagnostics. A user-owned backend supplies all domain meaning: when something is worth presenting, what should be asked, what should happen after the user responds, and how any domain state is stored.

A useful shorthand is:

> **The host understands interaction, not intent.**

The initial motivating use case resembles a habit or intervention application, but the product is deliberately not a habit tracker and must not grow a universal habit/rules engine.

Examples:

- Ask whether to fill in an interstitial journal; on acceptance, the backend launches `emacsclient` on today's file.
- Prompt after a user-owned script detects that Reddit has been opened under conditions the user considers interesting.
- Ask for a brief subjective rating after some activity and record arbitrary backend-owned state.
- Present several context-dependent next actions based on calculations, files, APIs, calendars, or other data known only to the backend.
- Suppress prompts entirely when the backend decides there is nothing useful to present.

The intended end-user development experience is also part of the product:

> Describe the bespoke behaviour you want to a coding agent. The agent reads the repository contract and writes or modifies a small backend in whatever language is convenient.

The shell should make this easier than building a one-off GUI application, without replacing programming with a configuration language.

---

## 2. Positioning

### 2.1 What it is

A **programmable personal intervention shell**:

- native desktop UI;
- one user-nominated backend per profile;
- language-agnostic JSON protocol;
- periodic and externally-triggered evaluation;
- semantic prompts and responses;
- backend-owned domain logic and persistence;
- designed to be modified primarily by coding agents.

### 2.2 What it is not

v0 must not become:

- a general habit-modelling engine;
- a workflow engine;
- an automation platform;
- a plugin marketplace;
- a multi-module runtime;
- a rules DSL;
- a data warehouse;
- an embedded scripting-language host;
- a security sandbox for untrusted code;
- a universal GUI-description protocol.

If a behaviour can live entirely in the backend, it should remain in the backend until demonstrated host-level value justifies otherwise.

---

## 3. Guiding principles

### 3.1 The host understands interaction, not intent

The host may understand:

- prompts;
- choices;
- fields;
- context;
- user responses;
- scheduling;
- opaque events;
- transport failures.

It should not understand concepts such as:

- habits;
- streaks;
- Reddit;
- exercise;
- journals;
- caffeine;
- work sessions;
- goals;
- compliance;
- reminders.

Those are backend concerns.

### 3.2 Programming is the rules engine

Do not model arbitrarily complex user behaviour in host configuration.

The backend is ordinary code. It may be Ruby, JavaScript, Python, Lua, a shell script, a compiled executable, or anything else capable of speaking the protocol.

This is a feature, not an implementation shortcut.

### 3.3 Permissive at the boundary; boring after normalization

The wire protocol should eventually accept convenient forms where they can be normalized safely.

Examples:

- ISO 8601 timestamp;
- relative duration such as `"45 minutes"`;
- natural-language time such as `"tomorrow morning"`;
- possibly numeric duration forms where context makes the unit explicit.

The Rust core should normalize accepted forms into strict canonical types immediately.

Permissiveness must not mean silent guessing. Ambiguous values should fail clearly rather than acquire invented semantics.

A useful rule:

> **Liberal grammar; strict canonical semantics.**

### 3.4 Small semantic vocabulary, generously parameterized

The protocol should describe semantic values, not widgets.

For example, a backend may eventually request a numeric value with bounds. The renderer decides whether that becomes a slider, spinner, text input, or another appropriate control.

Likewise, a choice may have no fields or may expose additional fields conditional on the selected option.

Avoid creating a proliferation of top-level prompt types.

### 3.5 Rich content is an escape hatch, not a growing ontology

The protocol should admit rich contextual content without teaching the host about every possible presentation.

Prefer a small content vocabulary such as:

- plain text;
- Markdown;
- HTML;
- URI.

Do not add first-class chart, table, image, Graphviz, Vega, Mermaid, diff, or similar protocol types merely because a backend wants to show one.

HTML/URI can carry arbitrary rich presentation later.

### 3.6 Backend ownership is broad

The backend owns:

- domain state;
- persistence;
- integrations;
- decision logic;
- side effects;
- interpretation of external events;
- reporting data preparation.

The host owns only its operational state, such as:

- current profile configuration;
- scheduling state;
- outstanding interaction identity;
- transport/diagnostic state;
- UI preferences where necessary.

### 3.7 Agent-modifiability is a product requirement

The repository should be unusually easy for a coding agent to understand and change correctly.

Documentation, examples, tests, and architectural invariants are part of the product.

The intended workflow is not “learn our DSL.” It is:

1. describe bespoke desired behaviour;
2. point an agent at the repository;
3. let the agent implement or modify the backend;
4. run protocol/acceptance tests;
5. use the result.

---

## 4. Technology stack

### 4.1 Host

- **Language:** Rust
- **GUI:** Slint
- **Configuration:** TOML
- **Primary platform for v0:** Linux
- **Async/runtime:** choose the smallest reasonable Rust async/event-loop integration required by the implementation
- **Wire format:** JSON; JSON Lines for persistent streams

Cross-platform architecture is desirable, but proving Windows/macOS support is not a v0 objective.

### 4.2 Backend

Backend technology is intentionally unspecified.

A valid backend may be:

- Ruby;
- JavaScript;
- Python;
- Lua;
- shell;
- Rust/Go/etc.;
- a daemon speaking JSONL over a Unix socket.

No embedded language runtime is required.

---

## 5. v0 profile model

v0 has **exactly one nominated backend per profile**.

There is no:

- backend discovery;
- module system;
- manifest registry;
- event subscription registry;
- event routing graph;
- multi-backend scheduling.

If users need multiple behaviours, the nominated backend may implement its own internal dispatch.

Illustrative configuration:

```toml
[backend]
command = ["ruby", "./backend.rb"]
socket = "/run/user/1000/intervention/backend.sock"
timeout = "5s"

[schedule]
default_poll = "30m"

[logging]
file = "~/.local/state/intervention/backend.log"
```

Exact names may change during implementation. The semantic intent should not.

Required:

- a backend script/command path or equivalent executable command.

Optional:

- backend Unix socket path;
- timeout;
- default poll interval;
- log file path.

A profile may later become a first-class selectable object, but v0 only needs enough configuration to run one profile cleanly.

---

## 6. Backend transport

v0 should support two backend transports behind one internal abstraction.

### 6.1 Preferred: persistent Unix socket + JSONL

If the configured backend socket exists and is usable:

- connect to it;
- exchange newline-delimited JSON;
- maintain an ordered request/response stream;
- allow only one outstanding request at a time in v0.

No request correlation or concurrent multiplexing is required.

### 6.2 Fallback: spawn-per-invocation + JSON

If no usable socket exists and a command is configured:

- spawn the backend command;
- send one JSON request on stdin;
- read one JSON response from stdout;
- enforce the configured/default timeout;
- collect stderr into diagnostics/logging as appropriate.

The semantic protocol should be identical across transports.

An internal boundary might resemble:

```rust
trait Backend {
    async fn exchange(&self, request: Request) -> Result<Response>;
}
```

The exact Rust interface is an implementation detail.

### 6.3 Transport selection

For v0:

1. if a configured backend socket is available, prefer it;
2. otherwise, if a command is configured, use spawn-per-invocation;
3. otherwise report a configuration error.

Do not make backend daemon management a host responsibility in v0.

---

## 7. External event ingress

External activity detection belongs outside the host in v0.

The host should **not** subscribe directly to raw window-manager, browser, shell, filesystem, or other noisy event streams.

Instead, arbitrary user-owned code determines when “something interesting happened” and emits a normalized opaque event to the host.

Example:

```text
raw sway/browser/etc. events
          |
          v
user-owned watcher/filter/debouncer
          |
          v
host event ingress
          |
          v
backend evaluation
```

The host may provide:

- a Unix domain socket for local event ingress;
- a small CLI convenience command such as `app emit ...`.

The event payload is opaque to the host except for a minimal envelope.

Illustrative event:

```json
{
  "source": "my-browser-watcher",
  "kind": "reddit-opened",
  "timestamp": "2026-08-22T17:10:00+10:00",
  "data": {
    "whatever": "the backend wants"
  }
}
```

The host forwards the event to the backend and renders a prompt only if the backend returns one.

v0 assumes filtering, classification, and debouncing have already happened externally.

---

## 8. Core protocol model

The semantic contract has three basic operations:

1. **evaluate** — given a stimulus, is there anything useful to show?
2. **respond** — the user supplied this response to an interaction.
3. **schedule** — do not ask again before this time.

A scheduled poll is simply another evaluation stimulus.

### 8.1 Evaluate request

Illustrative canonical request:

```json
{
  "protocol": 1,
  "type": "evaluate",
  "now": "2026-08-22T17:10:00+10:00",
  "event": {
    "source": "external",
    "kind": "reddit-opened",
    "timestamp": "2026-08-22T17:10:00+10:00",
    "data": {}
  }
}
```

A scheduled poll could use:

```json
{
  "protocol": 1,
  "type": "evaluate",
  "now": "2026-08-22T18:00:00+10:00",
  "event": {
    "source": "scheduler",
    "kind": "poll",
    "timestamp": "2026-08-22T18:00:00+10:00",
    "data": {}
  }
}
```

The exact shape may be tightened during implementation, but timer-driven and externally-driven evaluation should share the same semantic path.

### 8.2 Evaluate response

No interaction required:

```json
{
  "view": null,
  "next_check": "2026-08-22T18:00:00+10:00"
}
```

Interaction required:

```json
{
  "view": {
    "kind": "choice",
    "title": "Fill in your interstitial journal?",
    "body": "It has been a while since the last entry.",
    "options": [
      { "id": "yes", "label": "Yeah" },
      { "id": "no", "label": "Nah" }
    ]
  },
  "next_check": "45 minutes"
}
```

### 8.3 Response request

The host assigns an interaction identity when rendering a backend view.

Illustrative response:

```json
{
  "protocol": 1,
  "type": "respond",
  "view_id": "host-generated-id",
  "now": "2026-08-22T17:11:12+10:00",
  "response": {
    "option": "yes",
    "values": {}
  }
}
```

The backend may perform arbitrary side effects, record arbitrary state, and return another view and/or scheduling instruction.

### 8.4 Response to a response

Example:

```json
{
  "view": null,
  "next_check": "2026-08-22T18:00:00+10:00"
}
```

A backend may also immediately return another interaction if useful.

---

## 9. Scheduling semantics

`next_check` may be returned:

- from an evaluation;
- after a user response;
- from both.

The **latest valid scheduling instruction wins**.

`next_check` means:

> Do not perform the routine scheduled evaluation before this point.

It is not an exact real-time deadline. The host may wake slightly later.

If no new `next_check` is supplied:

- retain an existing valid scheduled check if one exists;
- otherwise use the configured default poll interval.

If parsing fails, report the backend protocol error and preserve a sensible existing/default schedule rather than disabling the application.

### 9.1 Accepted forms

v0 must at minimum support:

- ISO 8601 / RFC 3339 timestamps;
- simple relative durations if implementation cost is low enough.

The protocol should be designed so later versions can accept:

- `"45 minutes"`;
- `"tomorrow morning"`;
- other useful natural-language forms.

All accepted values must normalize to a canonical internal instant.

Do not make natural-language parsing a blocker for v0.

---

## 10. Semantic interaction model

### 10.1 v0 renderer requirement

v0 must support a basic choice interaction:

```json
{
  "kind": "choice",
  "title": "Question",
  "body": "Optional context",
  "options": [
    { "id": "a", "label": "First" },
    { "id": "b", "label": "Second" }
  ]
}
```

This is sufficient for the first end-to-end product.

### 10.2 Protocol direction

The data model should not assume that an option can only be a bare value.

A future-valid option should be able to carry fields:

```json
{
  "id": "great",
  "label": "Great",
  "fields": [
    {
      "id": "energy",
      "kind": "number",
      "label": "Energy",
      "min": 1,
      "max": 10
    },
    {
      "id": "notes",
      "kind": "text",
      "label": "Anything notable?",
      "multiline": true
    }
  ]
}
```

Another option in the same choice may have:

- different fields;
- a range;
- long-form text;
- no fields at all.

The v0 renderer does not need to implement these capabilities, but the protocol/types should avoid making them impossible.

Likely semantic field vocabulary over time:

- text;
- boolean;
- number;
- date/time;
- choice.

Likely presentation hints over time:

- multiline;
- range;
- min/max;
- units;
- placeholder;
- suggestions;
- optional.

The backend expresses semantics. The renderer chooses widgets.

---

## 11. Context and rich presentation

The protocol should distinguish the interaction itself from contextual material shown with it.

Longer-term content forms:

- plain text;
- Markdown;
- HTML;
- URI.

### 11.1 v0

Guarantee:

- plain text;
- Markdown if Slint integration is straightforward.

Admit but do not necessarily embed:

- HTML;
- URI.

For unsupported rich content, an acceptable v0 fallback is to open the URI/content externally where safe and meaningful.

### 11.2 Why HTML exists

HTML is the escape hatch for context richer than the semantic protocol should model.

A backend may eventually use it to present:

- images;
- tables;
- charts;
- syntax-highlighted code;
- diffs;
- custom explanatory layouts.

The host should not gain first-class knowledge of those concepts merely to display them.

Embedded WebView support is explicitly deferred unless it proves unexpectedly cheap and robust.

---

## 12. Interaction identity and ordering

The host must generate a `view_id` for rendered interactions.

Responses must identify the interaction they answer.

v0 should:

- serialize backend exchanges;
- allow only one active/outstanding interaction per profile;
- reject or ignore stale responses clearly;
- avoid introducing general concurrency semantics.

This intentionally reduces the state space.

---

## 13. Failure and diagnostic semantics

Backend code is user-owned and expected to fail sometimes.

A backend failure must not take down the host.

Handle at least:

- command not found;
- socket unavailable;
- connection loss;
- timeout;
- non-zero process exit;
- malformed JSON;
- protocol-invalid response;
- invalid scheduling value;
- unsupported required interaction primitive.

The host should surface an unobtrusive but discoverable diagnostic state and log enough information to debug the backend.

Unknown optional fields should generally be ignored.

Unknown required semantic primitives should fail clearly.

The protocol envelope is versioned from day one.

---

## 14. Security stance

Backends are **trusted user programs**, not sandboxed plugins.

They may intentionally:

- read/write arbitrary user files;
- launch applications;
- access the network;
- call local APIs;
- invoke `emacsclient`;
- maintain arbitrary databases;
- perform other actions with the user's normal authority.

The host must not imply that backend execution is isolated or safe for untrusted code.

Sandboxing may be explored later as an optional execution mode, but it is not part of the v0 security model.

---

## 15. Agent-first repository design

The repository should be designed for coding agents as first-class contributors.

Recommended structure:

```text
AGENTS.md
README.md

docs/
  PRODUCT.md
  ARCHITECTURE.md
  PROTOCOL.md
  BACKEND-GUIDE.md

examples/
  minimal-python/
  minimal-ruby/
  minimal-js/
  interstitial-journal/

src/
  ...

tests/
  protocol/
  integration/
```

### 15.1 AGENTS.md

`AGENTS.md` should remain short and act as a map plus invariant sheet, not a duplicate specification.

It should include guidance such as:

> This application intentionally does not understand the user's domain.

> Before adding host functionality, ask: “Could this behaviour live entirely in the backend?” If yes, it belongs in the backend.

> Protocol parsing is permissive. Internal representations are canonical.

> Do not narrow wire compatibility merely because the current renderer implements only a subset of admitted protocol capabilities.

It should point agents to the authoritative documents and required verification commands.

### 15.2 Backend guide

`BACKEND-GUIDE.md` should make the intended experience obvious:

> Tell your coding agent what personal behaviour you want. Have it read `PROTOCOL.md` and implement the backend.

Include small, complete examples in several languages.

The examples should demonstrate that no SDK is required.

### 15.3 Executable specification

Protocol fixtures and integration tests should carry as much of the contract as practical.

At minimum test:

- valid evaluate request/response;
- `view: null`;
- simple choices;
- response round-trip;
- scheduling replacement;
- process transport;
- persistent socket transport;
- timeout/failure;
- malformed backend output;
- external event ingress.

---

## 16. Optional Doctrine integration

Doctrine may be used as an optional development/governance layer.

It must not be:

- required to build the project;
- required to write a backend;
- visible in runtime protocol semantics;
- necessary to understand the basic starter experience.

The project is, however, unusually suitable as Doctrine dogfood because it has:

- a small comprehensible architecture;
- explicit product invariants;
- a versioned protocol;
- progressive requirements;
- clear verification obligations;
- an agent-first development model.

The useful comparison is:

```text
agent-ready repository
        +
optional Doctrine governance
        =
typed requirements, obligations,
planning, verification, provenance
```

If the distinction is useful, it should demonstrate its own value.

---

## 17. v0.1.0 feature scope

### Required

- Rust + Slint desktop application.
- Linux-first.
- TOML configuration.
- One backend per profile/configuration.
- Configurable backend command.
- Optional persistent backend Unix socket.
- Socket-first, process-fallback backend transport.
- JSON request/response for process invocation.
- JSONL ordered request/response for persistent socket.
- Default backend timeout.
- Default poll interval.
- Scheduled evaluation.
- `next_check` from evaluation and response; latest valid value wins.
- Local external event ingress.
- Opaque external event forwarding.
- Basic `choice` interaction.
- Host-generated interaction identity.
- User response forwarding.
- `view: null`.
- Plain-text contextual content.
- Markdown if straightforward.
- Backend diagnostics/logging.
- Protocol version field.
- Example backends.
- Agent-oriented repository documentation.
- Integration tests exercising complete round trips.

### Explicitly deferred

- Multiple backends/modules.
- Backend discovery.
- Plugin manifests.
- Host-level event routing/subscriptions.
- Raw window-manager/browser integrations.
- Host-level debouncing.
- Embedded scripting runtimes.
- Host-managed domain persistence.
- DuckDB integration.
- Charts/tables as semantic primitives.
- Embedded HTML/WebView rendering.
- Rich form renderer.
- Concurrent requests.
- Multiple simultaneous interactions.
- Backend daemon lifecycle management.
- Cross-platform certification.
- Backend sandboxing.
- Package/marketplace/distribution system for backends.

---

## 18. Representative v0 scenario

### Interstitial journal

A backend determines whether the user should be prompted to make an interstitial journal entry.

1. Scheduled poll fires.
2. Host sends `evaluate`.
3. Backend checks whatever state it owns.
4. Backend returns:

```json
{
  "view": {
    "kind": "choice",
    "title": "Fill in your interstitial journal?",
    "options": [
      { "id": "yes", "label": "Yeah" },
      { "id": "no", "label": "Nah" }
    ]
  },
  "next_check": "45 minutes"
}
```

5. Host assigns a `view_id` and renders the choice.
6. User selects `yes`.
7. Host sends `respond`.
8. Backend launches `emacsclient` with the file corresponding to today's date, records whatever state it wants, and returns:

```json
{
  "view": null,
  "next_check": "60 minutes"
}
```

The host never learns what an interstitial journal is.

---

## 19. Representative externally-triggered scenario

### User-defined Reddit intervention

1. A user-owned watcher observes browser/window-manager state.
2. The watcher performs its own filtering and debouncing.
3. When it considers an event interesting, it emits:

```json
{
  "source": "reddit-watcher",
  "kind": "reddit-opened",
  "timestamp": "2026-08-22T17:10:00+10:00",
  "data": {
    "count_last_hour": 4
  }
}
```

4. Host forwards the opaque event in an `evaluate` request.
5. Backend may return `view: null`.
6. Or it may return a choice prompt.
7. The host renders whatever valid semantic interaction it receives.

The host does not understand Reddit, browser windows, frequency limits, or the meaning of the supplied data.

---

## 20. Suggested implementation sequence

### Phase 1 — protocol core

Implement:

- canonical Rust request/response types;
- protocol versioning;
- strict internal normalization;
- JSON fixtures;
- simple choice/view model;
- scheduling model;
- backend error model.

Keep rendering out of this phase.

### Phase 2 — process backend

Implement:

- configured backend command;
- stdin JSON request;
- stdout JSON response;
- timeout;
- stderr/log capture;
- integration fixture backend.

Prove an end-to-end evaluate/respond cycle without a GUI.

### Phase 3 — minimal Slint shell

Implement:

- main window;
- title/body;
- choice buttons;
- empty/no-prompt state;
- visible diagnostic state;
- host-generated `view_id`.

Connect to the process backend.

### Phase 4 — scheduling

Implement:

- default poll interval;
- `next_check`;
- latest-valid-wins semantics;
- persistence of necessary operational schedule state if required.

### Phase 5 — external ingress

Implement:

- host Unix event socket;
- event envelope parsing;
- CLI/helper for sending an event;
- forwarding to backend evaluation.

Do not add event interpretation.

### Phase 6 — persistent backend transport

Implement:

- configured Unix socket;
- JSONL exchange;
- one in-flight request;
- process fallback;
- reconnect/failure behaviour.

### Phase 7 — agent-ready starter experience

Add:

- `AGENTS.md`;
- authoritative protocol guide;
- backend author guide;
- minimal backends in multiple scripting languages;
- interstitial-journal example;
- prompts/examples showing how to ask a coding agent to create a bespoke backend.

### Phase 8 — polish sufficient for v0.1.0

Add:

- configuration validation;
- logs/diagnostics;
- Markdown context if cheap;
- packaging/run instructions for Linux;
- complete acceptance suite.

---

## 21. Acceptance criteria

v0.1.0 is successful when all of the following are true:

1. A user can clone the repository and run the native Linux GUI.
2. Configuration can point at a trivial backend written in a scripting language.
3. The host periodically asks the backend whether anything should be shown.
4. The backend can return no view without error.
5. The backend can return a simple choice and have it rendered correctly.
6. Selecting an option produces a response event delivered back to the backend.
7. The backend may supply `next_check` during either evaluation or response.
8. A later valid `next_check` supersedes an earlier one.
9. An external user-owned script can send an opaque event to the host.
10. That event reaches the backend without host-level interpretation.
11. A backend may run as a persistent JSONL Unix-socket service.
12. If that service is unavailable, the host can fall back to configured process invocation.
13. Backend crashes, timeouts, invalid JSON, and protocol errors do not crash the GUI.
14. An example backend can implement the interstitial-journal scenario without requiring host changes.
15. An implementation agent can read `AGENTS.md` + linked docs and implement a new bespoke backend without needing to understand host internals.
16. No v0 implementation introduces domain concepts such as habits, streaks, journals, sites, or goals into the host model.

---

## 22. Architectural review questions

During implementation, use these as recurring checks:

1. **Could this behaviour live entirely in the backend?**  
   If yes, why is the host being changed?

2. **Is this protocol field semantic or presentational?**  
   Prefer semantic meaning plus renderer freedom.

3. **Are we narrowing the protocol to match the current v0 renderer?**  
   Avoid doing so where modest future flexibility is already understood.

4. **Are we inventing a generic engine to avoid letting users write code?**  
   Programming is deliberately part of the solution.

5. **Does the host now need to understand what the user is trying to accomplish?**  
   If yes, the abstraction boundary is probably being crossed.

6. **Can a coding agent infer the intended change and verify it from repository-local material?**  
   If no, improve the specification, tests, or repository guidance.

---

## 23. Product hypothesis

The underlying hypothesis is:

> Many personally valuable applications do not exist because each one's logic is too bespoke to justify a polished generic product, while building a complete GUI application for each is disproportionately expensive.

Coding agents alter that tradeoff.

If the reusable shell owns the mundane interaction mechanics and arbitrary user code owns meaning, a person can cheaply create software for problems with an addressable market of one.

v0.1.0 should prove that proposition with as little host intelligence as possible.

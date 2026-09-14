# Research brief — Slice 007

Hand this to a fresh agent. Delete it once `research.md` lands, or keep it as
the statement of what was asked; it is not canon either way.

---

You are the research agent for **slice 007, "the renderer grows a form"**, in
the goad repository (branch `main`, `ab9ba3a` at the time of writing — check for
drift). It is at `/home/david/dev/goad` unjailed and at `/workspace/goad`
inside the bwrap jail; `git rev-parse --show-toplevel` settles it. You produce
**one artefact**:
`docs/slices/007/research.md`, filled in from the template already sitting at
that path. You write nothing else and you change no code.

## Read first, in this order

1. `CLAUDE.md`, then `docs/AGENTS.md`. The methodology and the five invariants.
   Not optional. Note in particular what a research artefact is for: later
   stages **cite it instead of re-deriving**, so an uncited claim is worthless
   and a wrong one is worse than absent.
2. `docs/slices/007/slice-007.md` — the scope, tier, AC-1..9 and OQ-1..6. OQ-1
   is **already answered**: the controller owns the draft. The rest are open and
   are what design will decide.
3. `docs/slices/007/canon-delta.md` — CD-1, the proposed SPEC-001/R-57. This is
   the slice's working authority for the wire contract, and it is a **draft**:
   nothing outside slice 007 may cite it, and your job includes attacking it.
4. `docs/slices/007/design-log.md` — the two decisions already taken.
5. `docs/specs/001-host-backend-protocol.md`. All of it. R-8, R-9, R-15, R-16,
   R-18, R-35, R-50, R-52, R-53, R-55, §6.2 and OQ-2 are the load-bearing parts.

## The two threads

Fill both sections of the template. Cite canon by document id and section
(`SPEC-001 §6.2`, `ADR-001`); cite code by `path:line`. Mark nothing ✓ — that
column is for the *consuming* agent, not for you.

### Thread 1 — governing canon

Binding constraints, the checked-and-not-applicable list with a reason each, and
amendment candidates. Specific questions, each of which must be answered from
the documents rather than from reasoning about them:

- **Is CD-1's gap real?** Search SPEC-001 exhaustively for any statement of what
  JSON type a submitted field value takes. The slice claims there is exactly one
  example (`"minutes": 20`, §6.2) and no rule. Confirm or refute. If any rule,
  fixture, or verification row already implies a type for any kind, that changes
  the tier argument and you must say so loudly.
- **Does R-9's opacity reach the host writing a value, or only reading one?**
  Quote the text. §6.3 and §7's R-9 verification row are relevant.
- **R-55 and a partial submission.** A renderer that draws four kinds and not
  the fifth submits an answer missing a key. Does any requirement address a
  response that omits a field the view declared? R-8's wording is the place to
  start; R-35 bounds what the host may refuse.
- **What does canon say about an unanswered field versus a false one?** The
  slice asserts the host cannot distinguish them and must not try. Find whatever
  supports or contradicts that, including SPEC-001/OQ-2's own wording.
- **ADR-001 and a draft in the controller.** State exactly what stratum
  `crates/goad/src/{controller,view_model,glass}.rs` sit in, what each may name,
  and which of the four ADR-001 instruments would catch a violation. Read
  `docs/policy/001-the-phase-gate.md` §Verification for what each instrument
  holds *and what it does not reach* — the residue matters here.
- **SPEC-002/OQ-4**, quoted in full, plus SPEC-002's principles about what the
  host may and may not judge. The slice argues that "a person is mid-answer" is
  interaction state rather than domain meaning. Find the text that settles or
  fails to settle that.
- **The domain-vocabulary invariant.** `group` as a hint key is fine. Establish
  what the scan actually checks and what it does not
  (`crates/goad-boundary/tests/checks/vocabulary.rs`, `src/scan.rs`), so design
  knows which host-side names are safe and which are merely unscanned.

### Thread 2 — code map

Hotspots, cited facts, precedents. Specific questions:

- **The draft's home.** `Controller` (`crates/goad/src/controller.rs`) and
  `Prepared` (`src/reception.rs`). Establish: every field, who writes it, and
  the exact lifetime of `shown` across `absorb`'s three `Shift` arms. The
  decision is that the draft lives *inside* `Prepared`; verify nothing in the
  existing fold makes that impossible or ugly, and name what it costs.
- **The present path, end to end.** `serve`'s two `glass.present` call sites,
  `Controller::frame`, `Frame`'s lifetime, `SlintGlass::present`, and the
  `VecModel<OptionRow>` it holds. AC-5 turns on this: state precisely when
  `present` runs and what it overwrites.
- **The command path.** `crate::wire::{Command, Stimulus}`, the mpsc channel,
  its capacity, `dispatch`, and what happens to a command that resolves to no
  exchange. An edit command will travel this road; say what it costs and whether
  the channel can drop one.
- **The answer path.** `Controller::answer` (`controller.rs:200`) — the two
  refusals it already makes, and where `values: BTreeMap::new()` is constructed
  and consumed all the way to the wire.
- **The mapper.** `present()` and `Undrawn` in `src/view_model.rs`. What
  `Undrawn::OptionFields` currently carries, every test that asserts on it, and
  every path that consumes it (`diagnostics.rs`, `reception.rs`). Narrowing that
  variant is a breaking change to those tests: enumerate them.
- **The canonical field types.** `Field`, `FieldKind`, `Fields`, `Hints`,
  `Alternative` in `crates/goad-semantics/src/protocol/canonical.rs`. Confirm
  the accessors a renderer needs already exist and are `pub`. The slice's scope
  says semantics is untouched — verify that is true, and if a missing accessor
  makes it false, that is a finding.
- **The pinned Slint widget set.** For each of `boolean`, `text`, `number`,
  `choice`, `datetime`: which widget, its exact in/out/in-out properties and
  callbacks, in **each** style that `with_style` can select — a widget that
  exists in `material` and not in the current default is a trap.
  `slint-build-1.17.1/lib.rs:153` for `with_style`.

  Read **pinned** sources only. Two routes, and the second is better:
  `~/.cargo/registry/src/*/i-slint-compiler-1.17.1/widgets/` unjailed, or —
  in the jail, where the registry holds no Slint at all — the checkout bound at
  `/workspace/slint` (`~/.local/src/slint` on the host), which is a full clone
  carrying the **`v1.17.1` tag**. `git show v1.17.1:<path>` reads the tagged
  release. Its *working tree* is `88c5e6a32` on `origin/pre-release/1.18` and
  **has drifted** — good for layout ideas, never for API facts.
- **The headless test tier.** `crates/goad/tests/renderer/` — `harness.rs`,
  `tree.rs`, `table.rs`, `wiring.rs`. Establish how a test drives a widget and
  reads its state (`i-slint-backend-testing-1.17.1/search_api.rs` has
  `accessible_checked`, `invoke_accessible_default_action`,
  `set_accessible_value` and friends). AC-1 to AC-6 must be expressible here;
  for each AC say which existing test file is its precedent, or say that none is.
- **The example backends.** `examples/` and `just demo`. What a form-sending
  example would have to look like, and whether the existing example is extended
  or joined by a second.

## Cross-thread findings, and design-input deltas

These two sections are the point of the exercise, not a summary. Specifically:

- Anything that makes an AC unachievable as written, or achievable only by
  asserting a proxy for the behaviour rather than the behaviour. (The project
  has been bitten: four green tests in slice 004 each asserted something the
  regression they guarded would have survived.)
- Anything that changes the size of CD-1 — a fifth row, a row that turns out to
  be unnecessary, or a reason R-57 should be two requirements.
- Anything that makes OQ-2's recommendation (draw four kinds, leave `datetime`
  undrawn) wrong in either direction.
- Anything that makes the controller-owned draft more expensive than the
  decision assumed. That decision is taken, and you are not re-opening it; you
  are telling design what it will have to pay.

## Rules

- **Read the artefact, not your memory of it.** Quote and cite. Where a claim in
  `slice-007.md` turns out to be wrong, say so plainly in *Design-input deltas*
  and cite the site — do not edit the slice card, and do not soften it.
- Verify before you assert. An unverified claim goes in unmarked with the words
  that make it a claim.
- You change no code, run no formatter, and touch no file but `research.md`.
- `just check` is not yours to run. If you believe something is broken, that is
  a finding.
- If the work is heading past roughly 200k tokens, write what you have into
  `research.md`, mark the unfinished threads **PARTIAL** with what remains, and
  stop. A second agent continues from the file. A truncated final report is not
  a substitute for a complete artefact on disk.
- Set the **Producers** and **As of** header lines (date, commit) before you
  finish.
- If the environment contradicts this brief — a path that does not exist, a tool
  that is absent — say so in the artefact's header and work around it. Do not
  treat the brief as more authoritative than what you can see.

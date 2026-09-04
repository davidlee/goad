# Design log — Slice 002

Append-only record of the design *conversation* — what was asked, what was
decided, in time order. It exists so that a compacted or interrupted session can
pick the thread back up. Never rewrite an entry; supersede it with a later one.

Only decisions live here. Adversarial review is owned end to end by its ledger
(`review-design.md`) — its brief, its findings, its synthesis. When a finding
prompts a decision from the user, that decision is recorded below like any
other, citing the finding id.

## Decisions

<!-- One entry per user decision, recorded immediately after the answer. -->

### 2026-09-04 — How the slice is driven, and where it stops

- **Asked:** what to set running now, given more tokens than attention.
  Options: a research spike alone; research then a drafted slice and design;
  or a standing loop through to an accepted plan.
- **Recommended:** the research spike alone, on the grounds that slice 002's
  cost estimate turns on one unknown (whether a Slint UI can be driven and
  asserted headlessly) and everything after that is cheaper to decide once it
  is answered.
- **Decided:** research, then draft `slice-002.md` and `design.md`. Stop before
  plan.
- **Consequence:** this slice runs research → slice → design → adversarial
  design review, and halts with the design presented for acceptance. `plan.md`
  is not begun.

### 2026-09-04 — Gate autonomy: an explicit deviation from `docs/AGENTS.md`

- **Asked:** how much autonomy at the gates the methodology reserves for the
  user — hold every gate as written; batch the small dispositions; or decide
  everything except canon.
- **Recommended:** hold every gate.
- **Decided:** **decide everything except canon.**
- **Consequence:** a standing deviation, recorded here because
  `docs/AGENTS.md` requires explicit user instruction to depart from the
  workflow and this is that instruction. For slice 002 only:
  - Design decisions that the methodology would put to the user one question at
    a time are taken by the agent and recorded in this log with the reasoning
    and the alternatives rejected, in the same shape as a decision the user
    took. The record is not weakened by the deviation; only the decider changes.
  - Adversarial-review findings are dispositioned by the agent without
    per-finding confirmation. The ledger still owns them end to end, ids stay
    immutable, and a disposition still states its reason.
  - **Canon still stops and waits.** Nothing in `docs/specs/`, `docs/policy/`
    or `docs/adr/` is written, amended or promoted without explicit user
    endorsement. A slice-002 `draft-spec.md` or `canon-delta.md` may be
    drafted — that is the mechanism the methodology provides for exactly this
    — but it is not promoted.
  - **ADR-002's T1 is a canon event, not a design decision.** Slint fires it;
    splitting the crate supersedes ADR-002 and therefore stops and waits.
    Research may dry-run the split for evidence; it may not land it.
  - STOP conditions in `docs/AGENTS.md` that are not decisions — an
    unanticipated obstacle that invalidates the design, a dependency the design
    did not admit — still stop, because they are not the agent's to take.


### 2026-09-04 — Scope extended: through plan, to autonomous execution

**Supersedes the first entry above.** That entry stopped the slice before
`plan.md`; this one does not.

- **Asked:** unprompted instruction, given mid-run.
- **Decided:** verbatim — "run design autonomously; work through it at a high
  level, walk me through it, ask any critical questions. Then thrash it after I
  approve the direction with codex to adversarial review out any implementation
  wrinkles." And: "from there the slice should be autonomously executable."
- **Consequence:** the slice now runs
  research → design → **walkthrough and user approval of the direction** →
  codex adversarial review → repair → plan → autonomous phase execution.

  Three things follow that the first entry did not imply:

  1. **There is one human gate, and it is the design direction.** Not the
     methodology's per-question interview — a single walkthrough at the level of
     the shape, with the questions that genuinely need the user attached to it.
     Everything before it and after it runs unattended.
  2. **The adversarial design review runs against codex** (`gpt-5.6-sol`, per
     `~/.claude/CLAUDE.md`), not only against fresh Claude agents, and it is
     briefed for *implementation wrinkles* rather than for intent. Findings land
     in `review-design.md` as the ledger requires; the deviation is only in who
     reviews and what they are pointed at.
  3. **`plan.md` acquires a hard requirement the methodology only implies:
     every phase must be executable by an agent with no user present.** Entry
     and exit criteria must be machine-checkable — `just check` exits 0, this
     test exists and fails first, this file contains this symbol — not
     judgements. STOP conditions must be stated as conditions an agent can
     recognise, not as advice. A phase that cannot be written that way is a
     phase whose design is not settled, and that is a finding against the
     design, not a licence to improvise.

  Canon is unchanged by this entry: it still stops and waits, and ADR-002's T1
  split still stops and waits.

### 2026-09-05 — ADR-002 T1: the crate splits, and it splits first

- **Asked:** T1 has fired. Split into a workspace; stay single-crate behind a
  lint quarantine; or split but fold it into the renderer work.
- **Recommended:** split, first, before Slint lands.
- **Decided:** split, first, before Slint lands.
- **Consequence:** `research.md` Thread 6 is the evidence — 111 renames, 91
  byte-identical, one substantive file change, ~6 minutes to a green gate, and
  the error-taxonomy split ADR-002 flagged as a cost was two lines. ADR-001's
  direction discipline held in production code; it slipped in two places in the
  test layout, and the split is what found them.

  Shape, as measured: `crates/goad-semantics` + `crates/goad-shell`, with the
  name `goad` reserved for stratum 3; `[workspace.dependencies]`,
  `[workspace.lints]`, one root `clippy.toml`; fixtures at `tests/fixtures/`.

  **This is the canon endorsement `docs/AGENTS.md` requires**, and it obliges
  four canon movements, all drafted in `canon-delta.md` during the slice and
  promoted at audit, none written into `docs/` mid-slice:
  1. A new ADR **superseding ADR-002**. ADR-002 says it is superseded, never
     amended, when the split happens.
  2. ADR-002's *stated reason* for expecting T1 is measurably false — an
     optional build-dependency plus `#[cfg(feature = "ui")]` gates cleanly, to
     one node. T1 fires on the un-gateable **dev**-dependency (16 → 223 crates
     in ADR-001's own `--no-default-features` column) and on the lint
     collision. The superseding ADR states the real ground.
  3. **SPEC-001 §7 names the fixture directory normatively**, so moving it is a
     canon change, not a file move.
  4. `docs/slices/001/design.md` §9 is the gate's canonical source and this
     slice changes the gate. Promoting §9 into canon is a canon creation.

### 2026-09-05 — The empty state is a tray icon, not a window

- **Asked:** what goad is when it has nothing to show — tray icon plus a prompt
  window; a hidden window with no tray; or an always-visible window.
- **Recommended:** tray plus prompt window.
- **Decided:** tray plus prompt window.
- **Consequence:** `SystemTrayIcon` is the only affordance Slint offers that
  answers brief §13's "unobtrusive but discoverable" ✓, `system-tray` is a
  default feature so it adds nothing to the 411 crates, and a visible tray keeps
  the event loop alive with no window at all — which is goad's steady state.

  Three mechanical facts from `research.md` Thread 5 that the design must state,
  each verified:
  - The tray is a separate top-level component and does **not** share globals
    with the `Window`; each gets its own copy.
  - `hide()` panics ("Constant property being changed") unless `visible` carries
    a binding.
  - The tray shows nothing until a non-empty icon image is assigned.

  Consequences beyond the tray: `run_event_loop_until_quit()` rather than
  `ComponentHandle::run()`; `slint::set_xdg_app_id` called before first show,
  since the app icon comes from the app id and the `icon` property is silently
  dropped; and the prompt window is destroyed and recreated per prompt, because
  Wayland has no unmap — so the view model lives in Rust, never in Slint
  properties that die with the window.

  What this does **not** buy: goad still cannot place a window, raise it, or
  steal focus. All three are verified no-ops on Wayland. Getting a prompt in
  front of the user is compositor policy, addressed by setting a stable xdg app
  id and documenting a window rule — not by host code.

### 2026-09-05 — The devshell supplies a font

- **Asked:** the headless test tier panics inside `fontique` with an empty
  fontconfig, and the face that makes it pass comes from the NixOS system
  profile rather than `flake.nix`. Add a font to the devshell; document the
  dependency; or measure it in a container first.
- **Recommended:** add a font package to `flake.nix`.
- **Decided:** add a font package to `flake.nix`.
- **Consequence:** slice 001's PHASE-09 verified `just check` from a clean clone
  under `nix develop`, and that guarantee becomes false the day the renderer
  lands unless the devshell supplies a font. This is the dependency addition
  `CLAUDE.md` requires be asked about rather than assumed, and it is now
  granted — scoped to a font package for the devshell, nothing else.

# Handover — the follow-ups from SATAN's walks

Written 2026-10-19, at `04147ca`. For the next agent: shape these follow-ups
into work. Read `CLAUDE.md` and `docs/AGENTS.md` first. **No code without an
accepted plan.** This document is a starting brief, not a plan.

## Where things stand

The person's aim for slice 012's kit is SATAN, their assistant agent (pi.dev,
`~/dev/satan`, jailed), writing its own goad backends. Two informal walks
happened on 2026-10-19:

- **W-0:** SATAN wrote `unwatched`, a three-question backend.
- **W-1:** SATAN wrote a router, which goad's live unit now runs. goad runs
  one backend, so the router fronts every backend SATAN writes.

Both walks are in `docs/slices/012/walks.md`, with a disposition per friction
item. SATAN's verbatim handbacks are `walks/satan-0-ISSUES.md` and
`satan-1-ISSUES.md`, and `plan-log.md` (2026-10-19) records the decisions.
Three items were kit fixes and are done: `fe6804d` and `4473c7d`. The rest
are follow-ups, held in `notes.md` §Open under *W-0's checker follow-ups*,
*W-1's follow-ups*, and *A config cannot travel with its backend*. That's
what this handover covers.

**Slice 012 is still open**:

- PHASE-09, PHASE-10 and PHASE-11 are pending, waiting on oubliette.
- Audit has not run.
- `canon-delta.md` is not yet promoted.

That constrains everything below:

- **Formally**, follow-ups reach `docs/follow-ups.md` at 012's close
  (`docs/AGENTS.md` §Close). Work on them now is a new slice drawing on
  `notes.md` §Open, or a 012 amendment the person endorses. Ask which.
- **The checker's canon is unpromoted.** SPEC-004's `goad-emit` and
  `goad-check` requirements (`canon-delta.md`, SPEC-004 R-8 onward) exist only
  as drafts. A change to either binary's command line, run or statuses edits
  those drafts or waits for promotion. Settle the order with the person
  before writing a design.
- **Surfaces collide.** PHASE-10 may edit `crates/goad-check/src` (report
  wording) and `kit/`. One writer per worktree; a parallel effort takes its own
  worktree, or waits for PHASE-10.

## The follow-ups, grouped by what they touch

Ids are `walks.md` rows. Each group's tier is a guess for the slice to confirm.

### A. `goad-check`: what a check exercises (code + SPEC-004 drafts)

- **W0-1 / W1-2 — set `now`.** Time-gated views are never asked, so
  `accepted` means "nothing refused", not "behaviour checked". `checking.md`
  states the limit. `design.md` §5.2.1 settles the command line.
- **W0-2 — per-view coverage.** Today the report warns only when no view came
  back at all. W-0's adjudication view was never reached, and nothing said so.
- **W0-3 / W1-3 — answer every option, not only the first at its drawn
  defaults.** Declines are never exercised. `design.md` §5.2.2 settles the
  run, and the chain bound of 8 interacts with this.
- **W0-7 / W1-4 — scratch state by default.** A check runs the backend
  against real state unless `HOME`/`XDG_*` are redirected. Each backend
  invents its own variable, so a router's check must know every child's.
  Candidates:
  - scratch `HOME`/`XDG_*` by default, with an opt-out;
  - `goad-check --env K=V`;
  - a protocol-level state directory, which is a SPEC-001 change and the
    heaviest of the three.

These four share the checker's run, so they probably belong in one slice.

### B. `goad-emit --config` (code; SPEC-004 drafts)

- **W1-8.** Without `--socket`, `goad-emit` reads only the default config's
  ingress path. An event meant for one instance can land in another's socket
  without warning. Neighbours: `docs/follow-ups.md` FU-42 (`goad-emit`'s
  statuses), and `canon-delta.md` SPEC-004 R-8 to R-10.

Small. It could ride with A.

### C. Backends of backends (canon; design questions first)

Each item here needs the CLAUDE.md test before any design: *could this live
entirely in the backend?* The router itself passes it, so it is a backend and
the host is unchanged. These items ask whether goad should make routers
easier. That is a protocol question, not a defect.

- **W1-6 — the span grammar is reimplemented.** A router merging children's
  `next_check`s must parse spans (`scheduling.md`), so there are now two
  implementations of one grammar. The brief (§15.2) says no SDK is required.
  Candidates:
  - a `goad-check` subcommand that resolves a `next_check` to an instant;
  - a kit example router that carries the parser;
  - nothing.
- **W1-7 — events have no routing field.** `kind` doubles as the child's
  name. An additive `target` key would be a SPEC-001 wire change. Counter-case:
  `kind` and `data` already suffice, and the router is the only consumer that
  cares.
- **W1-9 — no room for backend-owned keys in goad's config.** goad refuses
  unknown keys (`goad_shell::config`, `deny_unknown_fields`, deliberately), so
  the router's table lives in a second file. Opening a namespace is a
  config-format change.
- **W0-6 — a config cannot travel with its backend.** `command` resolves
  against goad's working directory (SPEC-001/R-36), so every SATAN config uses
  absolute paths. Resolving against the config file's directory would be a
  SPEC-001 change to R-36, and the change most likely to pay for itself.
- **A kit example router** is a natural vehicle for W1-6. It's not a row; it's
  this handover's suggestion.

### D. Not follow-ups, but keep in view

- **W1-5 — routers rely on SPEC-001/R-31.** One view is outstanding at a time,
  so a router need only remember which child it last showed. `protocol.md`'s
  *Asking a sequence of questions* (`fe6804d`) states the reliance. **SPEC-002
  OQ-4** (`docs/roadmap.md` §Open decisions; FU-31) asks whether a scheduled
  firing may supersede a view a person is mid-answering. Any answer that
  relaxes R-31 breaks every router silently. Cite this in OQ-4's arguments.

## Suggested first move

Talk it through with the person before offering options (their standing
preference):

1. A new slice or a 012 amendment, and before or after 012's audit, given the
   unpromoted SPEC-004 drafts.
2. Which groups. A and B together look like one tier-2 slice on the binaries.
   C is protocol design and likely its own slice, possibly starting with
   W0-6 alone.
3. For C, whether goad should help routers at all, or leave composition
   entirely to backends.

Then the normal lifecycle: `slice-nnn.md` from `docs/templates/slice`, design,
plan.

## Live context (outside this repository)

- SATAN's backends are in `~/satan-corpus/goad/`: `router/` (the live one;
  `children.toml` is its table), `unwatched/`, and the older checklist
  `backend.py`.
- `~/.config/goad/config.toml` links to `router/config.toml`, and the old file
  is kept as `config.toml.checklist`. The ingress socket is
  `/run/user/1000/satan/goad.sock`.
- SATAN loads the kit through `~/notes/flake.nix`: the goad input, `jpi
  --skill`. Its lock pins `3814306`, so SATAN has not seen `fe6804d` or
  `4473c7d`. That changes once goad's `main` is pushed and `nix flake update
  goad` is run in `~/notes`.
- malebolge, SATAN's durable daemon, has no skill loading yet. Its own
  handover is `~/dev/malebolge/docs/handover-skills-extension.md`.
- To reproduce a router check without touching real state, run from
  `~/satan-corpus/goad/router/` with `ROUTER_STATE`, `UNWATCHED_STATE`,
  `GOAD_DATA`, `GOAD_SATAN_QUEUE`, `HOME` and `XDG_STATE_HOME` pointing into a
  scratch directory:
  `goad-check --config config.toml --event events/unwatched.json --event events/unknown-child.json`.
  W1-4 is exactly why that list is so long.

# Handover — slice 015, orchestrated execution

Written 2026-10-19, on `main`, by the orchestrator that ran scoping, design
and plan. For the next orchestrator: run execution, audit and close. Read
`CLAUDE.md` and `docs/AGENTS.md` first. This document is a brief, not
authority: the plan, the design and canon govern, and where this disagrees
with them it is wrong.

## Where things stand

```
scoping ✔ ─► design ✔ (4 review rounds) ─► plan ✔ (2 rounds, accepted) ─► execute ─► audit ─► close
                                                                          ▲ you start here
```

- **The slice:** a config travels with its backend. A backend runs in its
  config file's real directory (every link resolved); a relative
  `ingress.path` resolves there too; argv is never rewritten
  (`slice-015.md`, Purpose).
- **Stage:** `planned`. Every phase is `pending` (`notes.md` §Status). No
  code has been written. No worktree exists yet.
- **Canon:** `canon-delta.md` holds SPEC-001 R-60 (and §6.4, §7 row) and
  SPEC-003 R-17 (and §6.1, §7 row). The person endorsed every word of it,
  including each amendment from review. **Nothing executes against canon
  until audit promotes it.** A phase never edits `canon-delta.md` or
  `design.md`; a renamed test is recorded in the phase sheet and audit
  re-points the citation (`plan.md` §Owed; `review-plan.md` F-3).
- **Ledgers:** `review-design.md` and `review-plan.md` are both resolved.
  `review-plan.md` F-15 is `settle-in-code`: PHASE-02/VT-10 with mutation
  M14. If that case survives M14, the finding returns `contested`.
- **Decisions:** `design-log.md` and `plan-log.md` hold every decision the
  person made. Read both before any judgement call.

## The phases (`plan.md` owns the detail)

| phase | what becomes true | riskiest part |
|---|---|---|
| PHASE-01 — the anchor | a loaded config carries its canonical directory; a relative `ingress.path` names the same socket for goad and `goad-emit` | the parse-signature change reaches three test targets; the socket-length pin fails, by design, under a `TMPDIR` too long for it |
| PHASE-02 — the spawn | the backend runs in its config's directory with `PWD` set; a spawn failure names the executable and the directory | **the riskiest phase**: most new cases and mutations, `BackendError::Spawn`'s shape reaches three crates, two cases depend on the environment (a copied `sh`, `python3`). The plan names a PARTIAL checkpoint after EX-3 |
| PHASE-03 — checker pins, kit prose | `goad-check`'s `PATH` and argv-form pins; `running.md`, `checking.md`, `events.md`, the example configs and READMEs state the new rule | `git merge main` first; collides with 012's PHASE-10 |
| PHASE-04 — exercisers, repo, the person's run | `demo.toml`, `justfile`, `.gitignore`, the memory note; then VH-1, the person's run (AC-4, and AC-5 via `just emit`) | VH-1 is a person, not an agent |

## How to run it

The pattern that held for slices 003, 005 and 010
(`docs/memory/` and the auto-memory *slice-orchestration-pattern*):

1. **Create the worktree** from the main checkout, once:
   `git worktree add .claude/worktrees/slice-015 -b slice-015`. Every 015
   commit from here to close is made there (`plan.md` §Branch and worktree).
   This handover is the last 015 commit on `main` before the merge.
2. **Per phase, you write the phase sheet**, in the worktree's `notes.md`
   §Phase sheets, immediately before the phase — never ahead. The sheet is
   the plan's second reading, so its writer must not be its executor
   (slice 010). Carry into it:
   - the Surfaces as a **closed list**: anything outside it is a STOP;
   - assumptions split into *verified at plan* and *first tested here*;
   - the plan's notes for the implementer, lifted in, not left to skim;
   - the mutation table **pre-filled** for that phase (edit, cases it must
     red), so the executor fills in *compiled?* and *redded*;
   - the natural-red rule: each case red today is seen red, with the
     assertion it failed on recorded; a red for another reason than
     `design.md` §9's does not count (`plan.md` §Sequencing).
   - Quote a plan criterion when the sheet restates it. If the sheet narrows
     one, say so and why.
   - If expanding the sheet shows the plan is wrong, go back to plan; do not
     repair it in the sheet.
3. **Entry checks you run before handing over** a phase: the previous
   phase's exit criteria hold; and, for PHASE-01, PHASE-02 and PHASE-03,
   slice 012's `notes.md` §Status on `main` does not show PHASE-10
   `in progress`. If it does, STOP and consult the person.
4. **One executor per phase**, a fresh agent, in the worktree, alone. Its
   brief: the sheet, `git -C .claude/worktrees/slice-015 log -1 --oneline`
   first, red/green/refactor, `just check` green at the end, lint as it goes,
   commit only its Surfaces plus `notes.md`, a hard checkpoint at ~200k
   tokens (commit PARTIAL with *where I stopped*). Earlier slices used Sonnet
   for execution and no phase needed escalation; PHASE-02 may merit Opus.
5. **Re-measure at the phase commit, yourself:** `just check` in the
   worktree, and at least one of the phase's mutations re-run. Read the
   files, not the executor's report.
6. **Then the next phase.** Set the §Status row; commit in the worktree.

**Audit** after PHASE-04 (`docs/AGENTS.md` §Audit & reconcile), fresh
agent, in the worktree. Budget it as two sessions: review rounds on the
repairs cost as much as the first round. Code review is full strength and
unbounded (`review-code.md`). Then close and the merge, per `plan.md` §Owed
and §Branch.

## Gates that belong to the person

- Any STOP from a sheet or an executor: a design question, a Surface
  outside the closed list, a 012 collision.
- **VH-1** (PHASE-04): the person runs goad from a directory other than the
  config's, and through a link, and sees a view. Hand them the commands
  pasted in full, in order; their shell is **nushell**, so no bash syntax.
  A GUI launch from an agent needs `run_in_background`, not `&`.
- Each code-review disposition at audit, before acting on it.
- Canon promotion at audit. The wording is endorsed; promotion itself still
  needs explicit endorsement. Prefer promoting after 012 (R-60 follows 012's
  R-59); if 015 closes first, SPEC-001 carries R-60 with no R-59 until 012
  promotes (`canon-delta.md`, Status).
- The merge, if a 012 walk (PHASE-09 or PHASE-10) is in progress.

## Things that will bite

- **Never `git stash` or `git checkout`.** Mutations: copy the file to the
  scratchpad, edit, run, copy back, `git status` clean.
- **`flake.lock` in the main checkout is modified and not 015's.** Never
  stage it. The worktree does not have the change.
- **One writer per worktree.** Do not commit in the worktree while an
  executor runs; `git commit` takes the whole index.
- **Stale root demo sockets.** `goad-demo.sock` and `goad-demo.sock.lock`
  at the repo root of each checkout go once PHASE-04 moves the socket.
  `notes.md` §Owed has the box.
- **A test asserting a proxy** passes when the regression it guards would
  too. Every mutation must compile before its red counts.
- **A long `TMPDIR`** (a jail) turns the socket-length pin red by design.
  Run the gate where `std::env::temp_dir()` leaves room for the 107-byte
  control.
- **Cite by symbol, never count**, in code docs and notes too.

## Context outside this repository

- Slice 012 is open: PHASE-09..PHASE-11 wait on oubliette; its audit has
  not run. Its `plan.md` *Owed to audit and close* carries a row for this
  slice's rule change, recorded there as design drift.
- SATAN's configs (`~/satan-corpus/goad/router/`, `unwatched/`) use absolute
  paths and are unaffected. Once 015 ships they may drop them; that is their
  owner's edit, not this slice's. `~/.config/goad/config.toml` links to
  `router/config.toml`, which is AC-1's live case.

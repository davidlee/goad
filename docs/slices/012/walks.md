# Slice 012 — walks

One section per walk. PHASE-09 and PHASE-10 add the measured walks, with the
columns `design.md` §5.2.9 lists. Each friction item is a row with a
disposition: **kit fix** (skill, reference, example or checker wording; no
canon, no wire), or **follow-up** with its reason.

## W-0 — SATAN, informal (2026-10-19)

**Not an AC-1 walk.** It ran outside a capsule, with no negative control, on
neither Claude Code nor Codex, and from the person's own prompt rather than
the fixed one (`plan-log.md` 2026-10-19, *W-0: SATAN writes a backend*). It
counts as a friction source only.

| column | value |
|---|---|
| agent | SATAN: pi 0.87.1, as `jailed-pi` (bwrap) from `~/notes`, the kit loaded by `--skill` |
| model | `deepseek-v4.1-flash`, pi's configured default (not read from the transcript) |
| goad revision | `3814306`, `~/notes`' `flake.lock` |
| turns, wall time, tokens, cost | not recorded |
| fetch attempts | not read; no transcript read was made |
| backend | `~/satan-corpus/goad/unwatched/`: `backend.py`, `test_backend.py`, `config.toml`, `justfile`, `README.md` |
| verdict | `goad-check --config config.toml`, run on the host by the orchestrator from the backend's directory with a scratch `XDG_STATE_HOME`: **accepted, status 0**, three views answered in one chain (startup, scheduled and the unrecognised kind returned no view) |
| person-run | pending. The first attempt ran a stale `~/.cargo/bin/goad`, which has no Wayland libraries and shadows the nix-built one; that is the person's machine, not the kit |

SATAN's handback, verbatim: `walks/satan-0-ISSUES.md`.

### Friction

| # | item | kit before | disposition |
|---|---|---|---|
| W0-1 | `goad-check` cannot set `now`, so a time-gated view (here the wager's adjudication) is never asked, and the verdict is still accepted | `checking.md` states it | **follow-up**: a checker option to set `now`. It changes the checker's command line, which `design.md` §5.2.1 settles |
| W0-2 | the report says nothing when *some* views were never reached: it warns only when no exchange returned a view | — | **follow-up**: per-view coverage in the report. More than report wording |
| W0-3 | the checker answers only each view's first option, at its drawn defaults, so a decline is never exercised | `checking.md` and `protocol.md` R-58 state it | **follow-up**: answer every option. It changes the run `design.md` §5.2.2 settles |
| W0-4 | an untouched number arrives as `0.0`; a backend reading only integers would refuse it | `protocol.md` R-57 said only "a JSON number", and its respond example showed `20`, which goad never sends (`Submitted::to_json` writes every number as an `f64`) | **kit fix**: `protocol.md` R-57 says a number is always written with a fraction, and to read it as a number; the example says `20.0`. Commit `fe6804d` |
| W0-5 | a chained question has to carry itself, because a respond names its view only by the opaque `view_id` | the facts were in `protocol.md` R-8, R-30, R-31 and R-32; the pattern was not | **kit fix**: `protocol.md` *Asking a sequence of questions*, with both routes (option ids, or state), and why the state route is safe. Commit `fe6804d` |
| W0-6 | a config cannot sit beside its backend portably: `command` resolves against goad's working directory, so SATAN used absolute paths | `running.md` and `checking.md` state it (R-36) | **follow-up**: resolving against the config's directory is a SPEC-001 change, its own slice |
| W0-7 | `goad-check` runs the backend against real state unless `HOME`/`XDG_*` are redirected | `checking.md` states it | **follow-up**: scratch `HOME`/`XDG_*` by default, with an opt-out. It changes the checker's run. SATAN's `justfile` already redirects it |
| W0-8 | no `systemctl` in the jail, so the home-manager unit was not tried | — | **none**: the jail, not the kit |

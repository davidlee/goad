# W-0 — SATAN's handback

SATAN's own report at the end of the walk, as pasted by the person
(2026-10-19). Verbatim; the analogue of a walk agent's `ISSUES.md`.

```text
 Built at ~/satan-corpus/goad/unwatched/ — backend.py, test_backend.py,
 config.toml, justfile, README.md.

 The three questions

 1. What didn't the screen see? — the hours with no window are the ones
    that explain the day, and I have no other route to them.
 2. What are you carrying that isn't written down? — my memory is built
    from artifacts, so an unwritten commitment is invisible by
    construction.
 3. What will be true this time tomorrow? — one claim, a confidence 0–100.
    It becomes a wager, due in 20h, and I ask again: held / partly /
    failed / stopped mattering, with a note. That is the only falsifiable
    evidence about your self-model this system can get.

 A sitting walks q1→q2→q3, one view at a time, at most weekly or on "Check
 now" (not within an hour of the last). later closes the whole sitting.
 Nothing appears at startup. Answers append to
 ~/.local/state/unwatched/answers.jsonl — that's the file I'd read back;
 wager lines carry their own claim and confidence so an outcome stands
 alone.

 Verification

 - 26 unit tests: sequence, resume-after-interrupt, weekly gap, "Check
   now" rate limit, empty stake, adjudication, later deferral, corrupt
   state, unknown host kind, view-shape invariants. All pass.
 - goad-check --config config.toml (scratch XDG_STATE_HOME): verdict:
   accepted, no exchange refused.
 - Wager loop driven by hand through the real process: stake → state → 20h
   → adjudication view → recorded outcome.

 Friction, first trial

 - goad-check cannot set now. The wager was never asked during the check,
   yet the verdict was accepted. Any time-gated view is invisible to it;
   unit tests are the only proof.
 - It only ever picks the first option, with defaults in every field. All
   three questions were answered; none of the declines (Not now, "nothing
   to say") were ever exercised — the honest half of a low-friction
   questionnaire.
 - An untouched number arrives as a float (confidence=0.0 for min: 0).
   Accepting int only would silently accept nothing.
 - The option id is the only return channel — the host mints view_id and
   hands back no view, so chained questions must encode themselves in the
   option id (q2:named, w:held). Workable, but every sequential backend
   rediscovers it.
 - Configs cannot travel: command resolves against goad's cwd, not the
   config's directory, so an absolute path is mandatory. A config checked
   in beside its backend is machine-specific.
 - goad-check writes real state unless XDG_STATE_HOME is redirected; the
   just recipes do it so checks can't pollute the answers.
 - Ran inside a bwrap jail: systemctl and /run/current-system absent, so
   the systemd unit path was not exercised. goad itself was invoked from
   PATH.
```

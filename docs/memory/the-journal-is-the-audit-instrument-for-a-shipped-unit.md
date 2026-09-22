# When a slice ships a systemd unit, the journal is an audit instrument nothing else replaces

Slice 006's audit found its only finding here, after three code-review rounds
found none.

## The fact

A slice that ships a unit ships *behaviour over time* — restart policy, exit
codes, start ordering — and none of it is reachable by the gate, by a test, or
by an adversarial review of the diff. All three read the artefact. The journal
is the only record of the artefact **running**, and by audit it holds days of it
rather than the single start the executing phase observed.

Two things fell out of one reading in slice 006:

- **A claim built on a sample of one.** PHASE-05 recorded that the packaged
  binary had repaired a tray-icon failure, from one clean start. Eight starts
  later the failure was on five of them. The observation was true; the inference
  was not.
- **A restart directive that suppresses the one restart that works.**
  `RestartPreventExitStatus=2` was argued from three `StartupError` variants and
  applied to all ten. The omitted one, `Platform`, is raised by
  `run_event_loop_until_quit` — a compositor going away under a host that has
  run for hours — and it was the only exit-2 that had ever occurred. Twice the
  host stayed down for about two hours.

## How to apply

At audit, for any slice that touches a unit:

```
systemctl --user show <unit> -p ActiveState -p ExecStart -p Restart \
  -p RestartPreventExitStatus -p MainPID
journalctl --user -u <unit> --since <slice start> -o short-iso
```

Read **every** start and every exit, not the most recent. Ask what each non-zero
exit actually was, and whether anything restarted afterwards — and how long it
took, because `RestartSec` distinguishes systemd retrying from a person or a
session doing it.

A phase's observation is a sample. An audit has a population.

Related: `getting-eyes-on-the-running-host.md`,
`verify-the-enumeration-not-the-conclusion.md`.

# Losing only the host's display connection, on the running host

Proven at slice 010's audit (AC-9), 2026-09-23, on the daily-driver machine.
`docs/slices/010/audit.md` §Evidence *AC-9 — on the running host* is the
record, journal lines included.

## The fact

The production failure is the host's own Wayland connection breaking with the
compositor still up — journald shows `Io error: Broken pipe (os error 32)` from
the host, three times, then its exit. **Shutting down the host's end of its
Wayland socket from outside reproduces exactly that**: the same three lines,
then `goad: the host was running and stopped: …` and status 1.

The routes that look like it and are not:

- **Killing the compositor** — the unit is `PartOf` the graphical session, so
  systemd stops the host with the session: a different end, and it takes the
  desktop with it.
- **A compositor close binding** — winit turns it into `CloseRequested`, a close
  *request*, which the host treats as asked for: status 0, no line. A broken
  connection sends no message at all.
- **Waiting** — it does happen on its own (six times in two days before slice
  010), but not on demand.

## How to do it

Commands for nu. `ptrace_scope` is 1 on this machine, so the attach needs
`sudo`; `gdb` is not installed, so build it from nixpkgs.

```nu
systemctl --user show goad -p MainPID --value
ss -xpn | lines | where {|l| $l | str contains "wayland-0"}
ss -xpn | lines | where {|l| $l | str contains "goad"}
```

Find the fd **from the compositor's side**: its row on
`/run/user/1000/wayland-0` names a peer inode, and the `goad` row whose own
inode is that peer carries `fd=<FD>`. The host holds more than one unix socket
(its ingress listener is another), so the `goad` rows alone do not say which is
the display's.

```nu
let gdb = (nix build nixpkgs#gdb --no-link --print-out-paths | lines | first)
sudo $"($gdb)/bin/gdb" -p <PID> -batch -ex 'call (int)shutdown(<FD>, 2)'
```

Then read `journalctl --user -u goad --since "10 min ago" -o short-iso` and
`systemctl --user show goad -p NRestarts -p ExecMainStatus -p ActiveState`.

## How to apply

Use it whenever a claim is about what the host does when its display goes —
exit status, the stderr line, anything written after it, the supervisor's
restart. No test tier reaches this: the binary tier is headless by
construction, and nothing in the gate runs an event loop that can fail.
`shutdown` is what was run and seen to match production; `close` from outside
was not tried, and would free the fd number for reuse inside the host.

Related: `the-journal-is-the-audit-instrument-for-a-shipped-unit.md`,
`getting-eyes-on-the-running-host.md`.

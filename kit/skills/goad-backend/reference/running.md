# Running: the process, and the config file

This file says how the host runs your backend, and how a person tells goad
which backend to run. The **host** is goad, or `goad-check` standing in for it;
the **backend** is your program. What the request and response hold is in
[protocol.md](protocol.md). Each rule is headed by its id as `goad-check`
prints it.

## Name your command

### SPEC-001/R-36 — the command is an argument vector, with no shell

The config file's `command` is a list: the program, then its arguments, for
example `["python3", "backend.py"]`. No shell runs it, so pipes, globs, `~`
and `$VARIABLES` mean nothing in it. An empty list, or an empty program, is
refused when the config file is read.

`command` is not resolved against the config file's directory. A relative
program or argument is relative to the directory goad, or `goad-check`, is
started in. For a host started from anywhere else — a desktop launcher, a
service — make every path in it absolute.

A program the host cannot start (not found, not executable) is reported
under `SPEC-001/R-44` — explained in [protocol.md](protocol.md) — on the
configuration's side, not your backend's.

## Read one request, write one response

The host starts a new process for every exchange. Nothing carries over from
one to the next: keep any state you need in a file.

### SPEC-001/R-37 — one request on stdin, then stdin is closed

Read stdin to its end, then parse it.

### SPEC-001/R-38 — exactly one JSON document on stdout

Anything after the document, other than whitespace, is an error. Write
diagnostics to stderr, never to stdout.

## Use stderr

### SPEC-001/R-42 — stderr is captured and reported with every outcome

What you write to stderr is shown with every exchange, accepted or refused,
including one that timed out. It is the place for diagnostics.

### SPEC-001/R-39 — both streams are drained at once

You will not block on a full stderr pipe while the host waits for stdout.

### SPEC-001/R-43 — both streams are bounded

Stdout past its bound is a failure, and the host stops reading it. Stderr past
its bound is not a failure: the host keeps the first part, marks it truncated,
and reads the rest to the end.

## Finish in time

### SPEC-001/R-41 — `timeout` bounds how long you have to answer

The config file's `[backend] timeout`. A backend that has not answered by then
is terminated, and the exchange fails under this id.

### SPEC-001/R-48 — the host terminates your process after every exchange

Every exchange ends with your process terminated and reaped. Do not start
background work from a backend that must outlive the exchange; run it as a
separate program. A child process that keeps your stderr open, or survives
termination, can keep the host from confirming the process was disposed of.
That is reported under this id, apart from whether your response was
accepted.

## Exit

### SPEC-001/R-40 — exit 0; any other status is a failure

A non-zero exit fails the exchange, and its stdout is discarded even when it
was a valid response.

### SPEC-001/R-49 — a failure means the host took no action

It does not mean your backend did nothing. Anything your backend did before it
failed stands, and the next exchange may ask the same thing again: make what
your backend does safe to repeat.

### SPEC-001/R-45 — a backend failure never takes the host down

Whatever your backend does, the host goes on, and runs it again at the next
check. An input or output failure between the host and your process is
reported under this id, on the host's or the environment's side.

## Write the config file

goad reads one TOML file. `goad config.toml` names it; plain `goad` reads
`$XDG_CONFIG_HOME/goad/config.toml`, or `~/.config/goad/config.toml` when that
variable is unset, empty or not absolute. `goad-check --config config.toml`
reads the same file.

```toml goad:config
[backend]
command = ["python3", "backend.py"]
timeout = "30s"

[schedule]
default_poll = "30m"

[ingress]
path = "./goad.sock"
```

| key | required | what it is |
|---|---|---|
| `backend.command` | yes | the argument vector above |
| `backend.timeout` | yes | how long one exchange may take, a positive span |
| `schedule.default_poll` | yes | how long until the next check when your backend names none, a positive span ([scheduling.md](scheduling.md)) |
| `ingress.path` | no | where goad listens for forwarded events ([events.md](events.md)); relative to the directory goad is started in |

A span is written as `next_check`'s relative form is, such as `"30s"`,
`"45 minutes"` or `"2h"`. A key goad does not read is an error, not ignored.

### SPEC-004/R-3 — goad exits 2 when it cannot start

A config file that is missing, unreadable or invalid — a key it does not read,
an empty `command`, a span it cannot parse, a socket it cannot bind — stops
goad before it starts, with exit status 2.

### SPEC-004/R-4 — its last line on stderr says why

It is the last line goad writes there. It begins `goad:` and says what went
wrong.

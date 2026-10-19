# Events: telling the host something happened

This file says how a program other than your backend — a **watcher**, such as
a shell hook or a file monitor — tells goad that something happened, and what
your backend then receives. The **host** is goad; the **backend** is your
program. A watcher writes an **envelope**, a small JSON object, to a socket
the host listens on; the host answers it, and asks your backend to evaluate
it. Each rule is headed by its id.

## Forward an event

### SPEC-003/R-1 — the host listens only when the config file names a socket

Add `[ingress] path` to the config file (see [running.md](running.md)). With
no path, goad opens no socket and takes no events. The socket is readable and
writable by its owner only. A socket left at the path by a goad that has
exited is replaced when goad starts again; a path held by a running goad, or
occupied by anything but a socket, stops the second goad from starting.

### SPEC-003/R-9 — an envelope has exactly `source`, `kind`, `timestamp` and `data`

```json goad:envelope
{"source": "downloads-triage", "kind": "file-arrived", "timestamp": "2026-10-17T09:00:00+10:00", "data": {"name": "invoice.pdf"}}
```

- `source` names your watcher, a non-empty string;
- `kind` names what happened, a non-empty string in your own vocabulary;
- `timestamp` is when it happened (`SPEC-003/R-10`);
- `data` is any JSON value, `null` included: anything else you want to say
  goes here.

A missing key, an empty `source` or `kind`, a key of the wrong type, a key
repeated, or any fifth key is refused, naming the key. There is no version
key.

### SPEC-003/R-10 — `timestamp` carries its UTC offset

RFC 3339, with `Z` or `+10:00`, as `next_check`'s instants do
([scheduling.md](scheduling.md)). One without an offset is refused.

### SPEC-003/R-13 — `source` may not be `"host"`

That source is reserved to the evaluations goad asks for itself, and your
backend may trust it (`SPEC-001/R-56`, in [protocol.md](protocol.md)). An
envelope claiming it is refused.

### SPEC-003/R-6 — one envelope per connection

Connect, write one envelope, then end it with a newline or by closing your
write side. The host reads no further on that connection. An envelope larger
than 64 KiB is refused, and so is a connection that sends no complete
envelope within half a second.

## Send it with `goad-emit`

`goad-emit` writes one envelope, waits for the reply, and exits. It fills in
`timestamp` from the clock:

```sh
goad-emit --source downloads-triage --kind file-arrived --data '{"name":"invoice.pdf"}'
goad-emit --socket ./goad.sock --source cron --kind woke
```

`--data` defaults to `null`. Without `--socket` it reads `[ingress] path`
from goad's default config file (`$XDG_CONFIG_HOME/goad/config.toml`, or
`~/.config/goad/config.toml`) and resolves a relative one against that file's
directory, as goad does, so the two agree wherever each is started. For a
goad started on another config file, pass `--socket`: its path is used as
given, so a relative one is relative to the directory `goad-emit` runs in.
It sets no deadline of its own: wrap it in `timeout` if you need one. Any
other program that writes one line to the socket and reads one line back
works as well.

### SPEC-004/R-8 — `goad-emit` exits 0 when the host accepted the envelope

Also when `--help` or `--version` was answered. It says nothing on success.

### SPEC-004/R-9 — `goad-emit` exits 1 when the host refused the envelope

Its last line on stderr starts `goad-emit: refused:`, then the `reason`, then
`retry_after_ms=N` when the host sent it, then the detail in parentheses. A
wrapper branches on the reason, the first word after `refused:`.

### SPEC-004/R-10 — `goad-emit` exits 2 when no host judged the envelope

A bad argument, no socket configured, nothing listening at the socket, a
reply that is not one the host may send: whatever the cause, it is 2, and the
last line on stderr starts `goad-emit:` and says which. Status 2 says nothing
about whether a goad is running.

## What the host replies

### SPEC-003/R-8 — every envelope gets exactly one reply

One JSON object on one line, then the host closes the connection:

```text
{"protocol":1,"accepted":true}
{"protocol":1,"accepted":false,"reason":"too_soon","retry_after_ms":1800,"detail":"an event-triggered evaluation began 1.2s ago; the minimum spacing is 3s"}
```

`goad-emit` prints nothing for an acceptance, and the line `SPEC-004/R-9`
describes for a refusal.

### SPEC-003/R-14 — branch on `reason`, never on `detail`

`reason` is present exactly when `accepted` is `false`, and is one of:

| `reason` | what happened | what to change |
|---|---|---|
| `malformed` | the bytes were not one JSON document | your serializer |
| `invalid_envelope` | not an object, or a key missing, empty, of the wrong type, repeated or unknown, or a `timestamp` the host could not read | the envelope; `detail` names the key |
| `reserved_source` | `source` was `"host"` | choose another source |
| `too_large` | over 64 KiB | send less |
| `timed_out` | no complete envelope in time | end it with a newline, or close your write side |
| `engaged` | an exchange with your backend was already running | retry, or drop it |
| `too_soon` | inside the spacing below | wait `retry_after_ms`, or combine events in your watcher |
| `unavailable` | the host cannot act now: it is stopping, or cannot read its clock, or the connection failed | wait and retry |

The set is closed. `detail` is prose for a person, and its wording may change.
`retry_after_ms`, on `too_soon` only, is whole milliseconds, rounded up: wait
that long and the spacing has passed. It is advice, not a reservation.

### SPEC-003/R-12 — an event during an exchange, or too soon after the last, is refused, not queued

goad runs one exchange with your backend at a time, and begins an
event-triggered evaluation at most once every 3 seconds. An envelope that
arrives during an exchange is refused `engaged`; one inside the 3 seconds
is refused `too_soon`. The host holds no queue: an event it refuses is gone
unless your watcher sends it again. A watcher that may fire in bursts should
combine its events itself.

## What your backend receives

### SPEC-003/R-11 — the event reaches your backend as written

An accepted envelope becomes the `event` of an evaluate (`SPEC-001/R-7`, in
[protocol.md](protocol.md)), carrying the watcher's `source` and `kind`
verbatim:

```json goad:request
{
  "protocol": 1,
  "type": "evaluate",
  "now": "2026-10-16T23:00:01Z",
  "event": {"source": "downloads-triage", "kind": "file-arrived", "timestamp": "2026-10-16T23:00:00Z", "data": {"name": "invoice.pdf"}}
}
```

goad never reads `data` or `kind`. It keeps their values, not their spelling:
the `timestamp` arrives in UTC, an object's keys may be reordered, and a
number past a double's precision loses it. `now` is the host's instant, not
the watcher's. Answer this evaluate like any other: a view, or `"view": null`.

## Check it

`goad-check --event FILE` sends a file's envelope to your backend after its
own requests, so you can see your backend answer it without a watcher or a
running goad. The file holds one envelope, exactly as a watcher would write
it; `source: "host"` and every other refusal above stops the run. See
[checking.md](checking.md).

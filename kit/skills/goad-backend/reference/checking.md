# Checking: `goad-check`

This file says how to run `goad-check` against your backend and how to read
what it prints. The **backend** is your program; the **host** is goad.
`goad-check` runs your backend exactly as goad does, through goad's own code,
sends it a fixed set of requests, and reports every **refusal**: anything the
host would have reported about an exchange — a failure, a discarded
`next_check`, or a process it could not dispose of. Each rule is headed by
its id, in the form the report prints.

## Run it

```sh
goad-check --config config.toml
goad-check -- python3 backend.py
goad-check --timeout 30s --event events/file-arrived.json -- ./backend.sh
```

- `--config PATH` reads goad's config file and runs its `[backend] command`
  with its `timeout`, exactly as goad will. With neither `--config` nor `--`,
  it reads goad's default config file (`$XDG_CONFIG_HOME/goad/config.toml`,
  or `~/.config/goad/config.toml`). `[ingress]` is ignored: no socket is
  opened.
- `-- PROGRAM [ARG]...` runs that command instead of a config file's. An empty
  program is refused, as goad refuses an empty `command`.
- `--timeout SPAN`, only with `--`, is how long one exchange may take; the
  default is `5s`. A config file states its own, so `--timeout` without `--`
  is refused, and so are `--config` and `--` together.
- `--event FILE`, repeatable, sends one forwarded event after the host's own
  requests (below). Each file holds one envelope, as a watcher writes it
  ([events.md](events.md)); one goad would refuse stops the run.
- `--help` and `--version` answer and exit 0.

A relative program or argument in `command` resolves against the directory
`goad-check` was started in, as it does for goad — never against the config
file's directory. So a config that passes the check from your project
directory can still fail for a goad started from anywhere else: make its paths
absolute (see *Name your command* in [running.md](running.md)).

### A check runs your backend for real

Every exchange runs your backend, which may write its state, move files or
send messages, as it would under goad. To keep a check off your own state, run
it with `HOME` and the `XDG_*` variables (`XDG_STATE_HOME`, `XDG_CONFIG_HOME`,
`XDG_DATA_HOME`, `XDG_CACHE_HOME`) pointing into a scratch directory, and have
your backend keep its state under them, as the examples do with
`$XDG_STATE_HOME`.

## What it sends

In order, each an evaluate with `now` and the event's `timestamp` read from
the clock and `data` `null`:

1. `source: "host"`, `kind: "startup"`;
2. `kind: "requested"`;
3. `kind: "scheduled"`;
4. `source: "host"`, `kind: "goad-check-unrecognised"` — a kind goad does not
   send, to see your backend tolerate one (`SPEC-001/R-56`, below);
5. each `--event` envelope, in the order given.

When a response carries a view, `goad-check` answers it: the first option,
with each of its fields at the value it was drawn showing — `false`, `""`,
the `min` (or `0`), the first alternative, the Unix epoch
(`SPEC-001/R-58`, in [protocol.md](protocol.md)). If the answer brings
another view it answers that too, up to 8 answers per request.

There is no way to set `now`. A backend that shows views only at some hours is
checked at the hour you run it; if no exchange returned a view, the report says
so, and the answer half of your backend was not exercised.

## Read the report

The report is on stdout, one block per request:

```text
evaluate startup
  view 2026-10-18T04:42:52.076388428Z#0: "Still on it?" · next check 2026-10-18T05:12:52.076100465Z
  respond yes with note=""
  view: null · next check 2026-10-18T05:12:52.076100465Z
evaluate requested
  view: null · next check 2026-10-18T05:12:52.076100465Z
  REFUSED  backend  SPEC-001/R-21  next_check discarded: schedule is a time of day, which is neither an instant nor a span: 18:00
evaluate goad-check-unrecognised
  view: null · next check 2026-10-18T05:12:52.076100465Z
  REFUSED  backend  SPEC-001/R-40  backend exited with status 1
  stderr: unknown kind
  REFUSED  backend  SPEC-001/R-56  a backend MUST tolerate a kind it does not recognise
verdict: refused, 2 exchange(s) refused
```

- `evaluate <kind>` begins a request.
- `view <view_id>: "<title>" · next check <instant>`, or `view: null · next
  check <instant>`, is what the host holds **after** the exchange: the view it
  would show, and the next check it resolved
  ([scheduling.md](scheduling.md)). It is not what your backend sent. After a
  failed exchange it is always `view: null`, and the next check is the one the
  host kept (`SPEC-001/R-29`); the `REFUSED` line below it is the verdict on
  that exchange.
- `respond <option> with <field>=<value> …` (or `with no values`) is the
  answer `goad-check` sent, followed by the outcome of that exchange. It says
  what was sent, not what your backend did with it.
- `REFUSED  <side>  <id>  <what>` is a refusal: the side at fault
  (`SPEC-001/R-59`, below), the id of the rule, explained in this reference
  under a heading of its own, and what happened.
- `stderr: <line>` is your backend's own stderr, shown for every exchange.
  It is never a refusal, however alarming it reads; only a `REFUSED` line
  is. `stderr truncated: …` says the host kept only its first part.
- `chain bound of 8 responds reached: …` says a view was left unanswered.
  `no exchange returned a view, so respond was not exercised` says what it
  says. Neither is a refusal.
- `verdict: accepted, no exchange refused`, or `verdict: refused, N
  exchange(s) refused`, is the last line.

### SPEC-001/R-59 — every refusal names one side and one rule

The **side** is where the refusal's cause lies:

- **backend** — in what your backend sent or did;
- **configuration** — in the config file, which named something the host
  could not use, such as a program that cannot be started;
- **host** — on goad's side;
- **environment** — in the operating system, which failed the host.

Each kind of refusal names one side, always the same one. Where a kind's
cause can lie elsewhere, the report carries what lets you see it:

- a timeout (`SPEC-001/R-41`, backend): check the configured `timeout`, which
  may be too short for honest work;
- a backend that exited with no status (`SPEC-001/R-40`, backend): it was
  killed by a signal, which may have come from outside your backend;
- a command that could not be started (`SPEC-001/R-44`, configuration): read
  the operating system's error, which may be a lack of resources rather than
  a wrong path;
- a disposal that did not finish (`SPEC-001/R-48`, backend): look at your
  backend's own child processes, which may hold its stderr open; the host
  does not say what state the process was in.

Each id is explained in [protocol.md](protocol.md), [running.md](running.md)
or [scheduling.md](scheduling.md).

### When the R-56 claim is made

The `goad-check-unrecognised` evaluate tests `SPEC-001/R-56` (in
[protocol.md](protocol.md)): answer a kind you do not know. If that evaluate
fails on the backend's side while at least one of `startup`, `requested` and
`scheduled` did not fail, the report adds a second `REFUSED` line under
`SPEC-001/R-56`. A backend that fails alike on every kind is not charged with
it, nor is a failure on another side; that exchange's own refusal is still
reported.

## Read the exit status

### SPEC-004/R-11 — 0: the host reported nothing

A verdict was delivered and no exchange was refused; or `--help` or
`--version` was answered. It does not mean your backend is correct: only that
goad would have reported nothing for the requests `goad-check` sent. It says
nothing about requests it did not send, a view the chain bound left
unanswered, or answers when no view was returned; the report says when either
happened.

### SPEC-004/R-12 — 1: at least one refusal

A verdict was delivered and the host reported at least one refusal, on
whichever side, including a process it could not dispose of. The last line
on stderr begins `goad-check:` and counts them. It does not mean your backend
is at fault: the side of each refusal is in the report.

### SPEC-004/R-13 — 2: no verdict

The run did not finish: a bad argument, a config file it cannot find or read,
an event file goad would refuse, a clock it cannot read, a report it could
not write, or a fault in `goad-check` itself. Whatever the cause, it is 2,
and the last line on stderr begins `goad-check:` and says which. Exchanges may
have run before it stopped; what they reported is not a verdict, and says
nothing about your backend.

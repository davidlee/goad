# Scheduling: when the host asks again

This file says how your backend tells the host when to ask again, and what
the host does with that. The **host** is goad, or `goad-check` standing in
for it; the **backend** is your program. A **check** is an `evaluate` the
host sends on its own account with kind `"scheduled"` (see
[protocol.md](protocol.md)). Each rule is headed by its id as `goad-check`
prints it.

## Say when to check again

Any response, to an evaluate or a respond, may carry `next_check`: a string
saying when the host should next check. Leave it out and the host decides
(`SPEC-001/R-26`, below).

### SPEC-001/R-21 — `next_check` is an instant or a span, and never a time of day

Two forms. An **instant** is an RFC 3339 date-time:

```json goad:response
{"view": null, "next_check": "2026-08-23T18:00:00+10:00"}
```

A **span** is a length of time from the request's `now`, such as `"30s"`,
`"45 minutes"`, `"2h"` or `"1d 2h"`:

```json goad:response
{"view": null, "next_check": "45 minutes"}
```

Whitespace around either is ignored. A time of day, such as `"18:00"` or
`"T18:00:00"`, is neither form: it is discarded, never read as eighteen
hours. To say *six this evening*, send the instant with its offset.

```json goad:response discarded R-21
{"view": null, "next_check": "18:00"}
```

A unitless number in a string (`"18"`) is not a span either; it is discarded
under `SPEC-001/R-25`.

### SPEC-001/R-22 — an instant carries its UTC offset

`Z` or `+10:00`. An instant without one is discarded, with its own error
rather than a general parse failure:

```json goad:response discarded R-22
{"view": null, "next_check": "2026-08-23T18:00:00"}
```

### SPEC-001/R-23 — a span has no months or years

Their length depends on a calendar, and the host resolves spans without one.
A span in either is discarded:

```json goad:response discarded R-23
{"view": null, "next_check": "1 month"}
```

### SPEC-001/R-24 — a day is 24 hours, a week 168

Exactly, whatever the clocks do in between: `"1d"` is never 23 or 25 hours.

### SPEC-001/R-28 — a past instant is kept as you sent it

The host does not correct an instant already past, or a negative span such
as `"-45 minutes"`. It is due at once, and fires once (`SPEC-002/R-3`).

## When the value is wrong

### SPEC-001/R-25 — an invalid `next_check` is discarded and reported; the rest is accepted

The view in the same response is still shown. The report names which way the
value failed, each its own error: not a string (naming the JSON type sent); an
instant without an offset (`SPEC-001/R-22`); a time of day (`SPEC-001/R-21`);
a calendar unit (`SPEC-001/R-23`); a span too large for the host to add to
`now`; or unparseable.

```json goad:response discarded R-25
{"view": null, "next_check": 45}
```

```json goad:response discarded R-25
{"view": null, "next_check": "tomorrow morning"}
```

`"next_check": null` is not invalid: it means you left it out
(`SPEC-001/R-51`, in [protocol.md](protocol.md)).

## What the host does with it

### SPEC-001/R-26 — the latest valid instruction wins; otherwise the default poll

After each exchange the host holds exactly one instant, the next check:

1. the `next_check` your response just sent, if it was valid — earlier or
   later than the one held, it replaces it;
2. otherwise, the instant it already held, if that is still ahead;
3. otherwise, now plus `default_poll`, the config file's
   `[schedule] default_poll` (see [running.md](running.md)).

So a backend that never sends `next_check` is asked every `default_poll`, and
one that sends it once, then stops, is asked once at that instant and then on
the default cadence. An instant that has fired is not kept.

### SPEC-001/R-29 — a failed exchange changes nothing

When an exchange fails — refused, timed out, a non-zero exit — any
`next_check` in it is not read. The host keeps the instant it held if that is
still ahead; if it had already come due, the next check is now plus
`default_poll`.

### SPEC-002/R-1 — a due check is asked without a person asking

When the next check comes due, the host sends an evaluate with kind
`"scheduled"`, whether or not anyone is at the screen.

### SPEC-002/R-7 — a new instruction shortens or lengthens the wait

The pending wait is replaced either way: an earlier instant brings the check
forward, a later one pushes it back.

### SPEC-002/R-3 — an instant fires at most once

An instant already past fires as soon as it may, then is spent. Sending a past
instant on every answer does not make the host ask in a loop.

### SPEC-002/R-4 — scheduled checks are at least 3 seconds apart

However soon you ask to be checked again, the next scheduled check begins at
least 3 seconds after the one before. It is not configurable. It never delays
a person's request or the startup evaluation, and the instant the host holds
is still the one you sent.

### SPEC-002/R-11 — missed checks are not caught up

If the computer slept through several intervals, one check is due, and one
evaluation discharges it. Do not count evaluations to measure time: read
`now`.

Only one exchange runs at a time, so a check that comes due during another
exchange waits for it to finish.

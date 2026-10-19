# The protocol: requests, responses, views

This file says what your backend reads and what it writes. A **backend** is
your program. The **host** is goad, the desktop app that runs it — or
`goad-check`, which runs it the same way and reports what it finds. One
**exchange** is one request the host writes and the one response your backend
writes back.

Every rule here is headed by its id as `goad-check` prints it in a report, for
example `SPEC-001/R-13`. The specifications those ids come from are not
shipped: each id a report can print is explained in this reference, under a
heading of its own. How the request reaches your process, and what happens to
the process, is in [running.md](running.md); `next_check` is in
[scheduling.md](scheduling.md).

## Read the request

The request is one JSON object on stdin.

### SPEC-001/R-6 — two kinds of request, told apart by `type`

`"type": "evaluate"` asks whether you have anything to show now.
`"type": "respond"` carries a person's answer to a view you showed; it is
described under [The answer](#the-answer).

### SPEC-001/R-1 — every request carries `"protocol": 1`

Read it if you want to refuse a protocol you were not written for. Today it is
always `1`.

### SPEC-001/R-7 — an evaluate carries `now` and an event

`now` is the host's current instant. `event` has a `source`, a `kind`, a
`timestamp` and a `data` payload:

```json goad:request
{
  "protocol": 1,
  "type": "evaluate",
  "now": "2026-08-23T04:12:00Z",
  "event": {"source": "host", "kind": "scheduled", "timestamp": "2026-08-23T04:12:00Z", "data": null}
}
```

Every instant the host sends is RFC 3339 with an explicit offset. This host
writes UTC, with `Z`; parse any offset, since nothing promises which one you
receive.

### SPEC-001/R-56 — why the host is asking: `source` and `kind`

When the host asks on its own account, `source` is `"host"` and `kind` is one
of:

- `"startup"` — once, when the host starts;
- `"requested"` — a person asked for a check;
- `"scheduled"` — the next check you asked for came due.

You may branch on these three, under `source: "host"` only. The set is open:
treat any other kind as an evaluation whose reason you do not know, and answer
it. Never fail on it. `goad-check` sends a kind no host originates, and reports
your backend under this id if that evaluation fails while at least one of the
host's own kinds succeeds.

An event forwarded from outside the host carries its writer's own `source`
and `kind`, verbatim (see [events.md](events.md)). No forwarded event can
claim `source: "host"`.

### SPEC-001/R-9 — the host carries data verbatim

The host never reads an event's `data` or a submitted value. What a forwarder
wrote in `data` reaches you as written. For the host's own kinds `data` means
nothing.

## Write the response

The response is one JSON object on stdout. Its only required key is `view`.

### SPEC-001/R-10 — a response carries `view`

A response without `view` is refused, naming the missing field.

### SPEC-001/R-11 — `"view": null` means there is nothing to show

This is an ordinary answer, not a failure:

```json goad:response
{"view": null}
```

### SPEC-001/R-51 — `null` means omitted, except for `view`

Anywhere else, a key set to `null` is read as if you left it out, and is not
reported. So a serializer that writes `null` for an absent optional is fine.
A value of the wrong type is not `null`: `"next_check": 45` is discarded and
reported.

### SPEC-001/R-2 — `protocol` may be omitted

A response without it is read as protocol 1.

### SPEC-001/R-3 — any `protocol` but 1 is refused

The refusal names the version you sent:

```json goad:response refused R-3
{"protocol": 2, "view": null}
```

### SPEC-001/R-4 — keys the host does not know are ignored

You may add keys of your own anywhere; the host ignores them. The exceptions
are keys the protocol does name, used where they mean nothing: those are
refused (`SPEC-001/R-18`, `SPEC-001/R-50`, `SPEC-001/R-53` below).

### SPEC-001/R-44 — stdout is one JSON document of a shape the protocol admits

Each of these is refused under this id, and each is reported as its own error:

- bytes that are not one JSON document (a serializer fault);
- a document of the wrong shape: a required key missing, a value of the wrong
  type, an array where an object belongs (a misreading of this file);
- a key repeated in one object, at any depth.

```json goad:response refused R-44
{"view": null, "view": null}
```

A command the host cannot start is reported under the same id, on the
configuration's side, not yours: see *Name your command* in
[running.md](running.md).

`next_check`, the optional key that says when to ask again, is explained in
[scheduling.md](scheduling.md). An invalid one is discarded and reported, and
the rest of the response is still accepted (`SPEC-001/R-25`, there):

```json goad:response discarded R-25
{"view": null, "next_check": 45}
```

### SPEC-001/R-20 — nothing you send is dropped silently

Apart from `next_check`, a part of a response the host cannot read refuses the
whole response. It is never dropped while the rest is accepted.

## Show a choice

A **view** is what the host shows a person. Its one kind is `choice`: a title,
an optional body, and the **options** a person picks between. An option may
carry **fields**, values the person fills in before picking it.

```json goad:response
{
  "view": {
    "kind": "choice",
    "title": "Take a break?",
    "body": {"kind": "markdown", "value": "You have been at it for **2h**."},
    "options": [
      {"id": "now", "label": "Now"},
      {"id": "later", "label": "In a bit", "fields": [
        {"id": "minutes", "kind": "number", "label": "Minutes", "min": 5, "max": 120, "units": "min"},
        {"id": "note", "kind": "text", "label": "Anything to remember?", "multiline": true},
        {"id": "done", "kind": "boolean", "label": "Finished the task?"},
        {"id": "resume", "kind": "datetime", "label": "Resume at"},
        {"id": "mood", "kind": "choice", "label": "Mood",
         "options": [{"id": "up", "label": "Good"}, {"id": "down", "label": "Tired"}]}
      ]}
    ]
  },
  "next_check": "45 minutes"
}
```

goad may not draw every content form or field kind. What it cannot draw it
leaves out and reports; it still shows the rest of the view.

### SPEC-001/R-13 — a choice has a title and at least one option

`body` is optional. A choice with no options is refused:

```json goad:response refused R-13
{"view": {"kind": "choice", "title": "Nothing to pick", "options": []}}
```

### SPEC-001/R-12 — an unknown `kind` refuses the whole response

This holds at any depth: the view's `kind`, a field's, a body's. The refusal
quotes the string and the path where it appeared.

```json goad:response refused R-12
{"view": {"kind": "slider", "title": "How much?", "options": [{"id": "ok", "label": "OK"}]}}
```

### SPEC-001/R-14 — option ids are unique within a view

A response selects an option by its id, so two options with one id are
refused.

### SPEC-001/R-52 — field ids are unique within an option, and alternative ids within a field

An answer is a map keyed by field id, and a `choice` field's answer is one of
its alternative ids, so a duplicate of either is refused, naming the id and
the path.

### SPEC-001/R-19 — a body is a string, or an object with `kind` and `value`

A bare string is `text`. The other forms are objects:
`{"kind": "markdown", "value": "…"}`, and likewise `text`, `html` and `uri`.
Anything else — an array, a number — is refused under `SPEC-001/R-44`. The
host never fetches a `uri`.

## Ask for values

### SPEC-001/R-15 — a field has an id, a kind and a label; other keys are hints

A **hint** is a key the protocol does not name, such as `units` or
`multiline` above. Hints are for the renderer only: one may change how a field
looks, and goad may ignore any of them. Never put in a hint something the
answer depends on.

### SPEC-001/R-16 — five field kinds

| `kind` | its own keys |
|---|---|
| `text` | — |
| `boolean` | — |
| `datetime` | — |
| `number` | `min` and `max`, both optional |
| `choice` | `options`, required, at least one |

### SPEC-001/R-17 — a number's bounds are finite, and `min` is not above `max`

Either breach refuses the response.

### SPEC-001/R-50 — a kind's own key on another kind is refused

`min` on a `text` field is not a hint; the refusal names the key and the kind.

```json goad:response refused R-50
{"view": {"kind": "choice", "title": "Note?", "options": [
  {"id": "save", "label": "Save", "fields": [{"id": "note", "kind": "text", "label": "Note", "min": 1}]}
]}}
```

### SPEC-001/R-53 — a choice field's options are an id and a label only

Their ids are **alternative** ids, not option ids: an option is what a person
picks for the whole view, an alternative is a value one field submits. An
alternative carrying `fields` is refused.

### SPEC-001/R-18 — hints sit flat on the field; a `hints` object is refused

Write `"units": "min"` on the field itself. `"hints": {"units": "min"}` is
refused, naming the path, rather than read as one hint called `hints`.
`"hints": null` is omission.

## The answer

When a person picks an option, the host sends a respond.

### SPEC-001/R-8 — a respond carries the `view_id`, `now`, the option and the values

This answers the view under *Show a choice*:

```json goad:request
{
  "protocol": 1,
  "type": "respond",
  "now": "2026-08-23T04:14:31Z",
  "view_id": "2026-08-23T04:12:00Z#3",
  "response": {
    "option": "later",
    "values": {"minutes": 20.0, "note": "back after coffee", "done": false,
               "resume": "2026-08-23T05:00:00+10:00", "mood": "up"}
  }
}
```

Your response to a respond is an ordinary response. `"view": null` closes the
question; a new view asks the next one.

### SPEC-001/R-57 — a value's JSON type is fixed by its field's kind

| `kind` | value |
|---|---|
| `boolean` | a JSON boolean |
| `text` | a JSON string |
| `number` | a JSON number, always written with a fraction: twenty is `20.0` |
| `choice` | the chosen alternative's id, a JSON string |
| `datetime` | an RFC 3339 date-time string with an offset |

Read a number as a number, never only as an integer: a parser that accepts
only integers refuses every number goad sends, untouched `0.0` included.

### SPEC-001/R-58 — values for exactly the drawn fields of the chosen option

Every field the host drew of the option picked has a value, and no other field
does. A field goad could not draw is absent. A field nobody touched submits
what it was drawn showing: `false`, `""`, the `min` (or `0` with none), the
first alternative, the Unix epoch. That is what this host sends, not a
promise; do not read any of them as *untouched*. If you need to tell an
unanswered field from `false` or `""`, do not send that field: there is no
way to ask for it yet.

### SPEC-001/R-35 — the host does not judge an answer

Whether an answer is acceptable is yours to decide. A number may be outside
its bounds: a field with only a `max` submits `0` untouched.

### SPEC-001/R-30 — the host mints every `view_id`

Treat it as an opaque string. A `view_id` in your response is ignored.

### SPEC-001/R-31 — one question is outstanding at a time

The **outstanding** view is the last one shown and not yet closed.

### SPEC-001/R-33 — a new view replaces the outstanding one

The replaced view's `view_id` is stale at once. `"view": null` in answer to an
evaluate leaves the outstanding question as it was; in answer to a respond it
closes it.

### SPEC-001/R-32 — an answer to a stale view never reaches you

The host refuses it, and your backend is not run.

### Asking a sequence of questions

A respond names the view it answers only by its `view_id`, which the host
minted and you first see in that respond (`SPEC-001/R-30`). It carries no copy
of the view. To know which question an answer belongs to, do one of two
things:

- **put the question in the option ids**: `q2:named` and `q2:skip`, so the
  option alone says which question was answered;
- **keep the current question in your own state**, and move it on with each
  view you send. Every respond answers the last view you sent, because only
  one question is outstanding (`SPEC-001/R-31`) and an answer to a replaced
  one never reaches you (`SPEC-001/R-32`). The Downloads triage example
  works this way, with its queue.

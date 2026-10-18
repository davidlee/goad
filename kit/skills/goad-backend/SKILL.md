---
name: goad-backend
description: Use when writing or fixing a backend program for goad, the desktop intervention shell.
---

# Writing a goad backend

Use this skill to write, fix or check a **backend**: the program goad runs to
decide what to ask a person, and when. goad, the **host**, owns the window;
your backend owns every meaning. It reads one JSON request on stdin and
writes one JSON response on stdout, in any language.

## The loop

1. Write the backend ([reference/protocol.md](reference/protocol.md)).
2. Run `goad-check` against it, and read its report
   ([reference/checking.md](reference/checking.md)).
3. Fix every `REFUSED` line; each names a rule this reference explains.
4. Write goad's config file ([reference/running.md](reference/running.md)).
5. A person runs `goad`. You cannot see its window; `goad-check` is how you
   see what it would do.

## Get the binaries

`goad-check` checks a backend; `goad-emit` forwards an event to a running
goad. Install both with Nix:

```sh
nix profile install github:davidlee/goad#goad-check github:davidlee/goad#goad-emit
```

or with Cargo:

```sh
cargo install --git https://github.com/davidlee/goad goad-check goad-emit
```

## The reference

- [reference/protocol.md](reference/protocol.md) — what does a request hold,
  and what may a response say: views, options, fields, answers?
- [reference/scheduling.md](reference/scheduling.md) — how do I say when goad
  should ask again, and what does it do with that?
- [reference/events.md](reference/events.md) — how does another program tell
  goad something happened, and what does my backend receive?
- [reference/running.md](reference/running.md) — how is my process run, how
  long may it take, where do diagnostics go, and what goes in the config file?
- [reference/checking.md](reference/checking.md) — how do I run `goad-check`,
  and what do its report and exit status mean?

Every id a `goad-check` report prints, such as `SPEC-001/R-13`, is explained
in the reference under a heading of its own.

## The examples

Each runs as it stands; its README says how.

- [examples/focus-check/](examples/focus-check/) — Python: a view with fields,
  state kept between exchanges, and `next_check` as an instant.
- [examples/downloads-triage/](examples/downloads-triage/) — Bash and `jq`: a
  watcher forwarding events with `goad-emit`, and one answer chained to the
  next question.
- [examples/breadcrumbs/](examples/breadcrumbs/) — TypeScript for Deno: a
  shell hook forwarding events, and a backend that answers them.

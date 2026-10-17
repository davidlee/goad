# breadcrumbs

Leaves a note about the directory you are leaving, and shows the one you left
about the directory you enter. TypeScript, for `deno`, with no imports.

## See it in under a minute

From this directory, with `goad-check` and `deno` on `PATH`:

```sh
goad-check --config config.toml --event events/changed-directory.json
```

This is the event the checker sends, a change from `/home/me/work` to
`/home/me/play`:

```json goad:envelope
{"source":"breadcrumbs","kind":"changed-directory","timestamp":"2026-10-17T09:00:00+10:00","data":{"from":"/home/me/work","to":"/home/me/play"}}
```

The report ends `verdict: accepted, no exchange refused` and shows a
`respond` line for the question *Where were you in /home/me/work?*

To see the window, run `goad config.toml` from this directory, then from a
second shell in this directory emit the same event. `goad-emit` takes the
envelope's fields as arguments; its timestamp is its own:

```sh
goad-emit --socket ./goad.sock --source breadcrumbs --kind changed-directory --data '{"from":"/home/me/work","to":"/home/me/play"}'
```

To have your shell report every directory change, source `chpwd.zsh` from
`~/.zshrc`; it reaches a host started on the default configuration, and says
where to add `--socket` for any other.

## Run it from anywhere else

`command` and `[ingress] path` are relative to the directory goad is started
in. Copy this directory out and make both absolute:

```toml goad:config
# `command` and the socket are relative to this directory: run goad and
# goad-check from here.
[backend]
command = ["deno", "run", "--allow-read", "--allow-write", "--allow-env=HOME,XDG_STATE_HOME", "backend.ts"]
timeout = "30s"

[schedule]
default_poll = "30m"

# Where a running goad listens for events, for chpwd.zsh and goad-emit.
[ingress]
path = "./goad.sock"
```

becomes, in your copy, `"/absolute/path/to/backend.ts"` as the last element
of `command` and `path = "/absolute/path/to/goad.sock"`. Pass the same
`--socket` to `goad-emit`. A running host leaves `goad.sock` and
`goad.sock.lock` in the directory it was started in.

## Behaviour

- A forwarded `changed-directory` event from `breadcrumbs` asks *Where were
  you in FROM?* with *Leave a note* (a text field) and *Nothing to note*.
- Either answer then looks up TO. If it has a note, the view shows it with
  *Thanks* (keep it) and *Clear* (delete it). Otherwise nothing more is shown.
- An untouched text field arrives as `""` and stores no note.
- Host events and other forwarded events: silent.

## State and environment

State is under `$XDG_STATE_HOME/breadcrumbs/` (default
`~/.local/state/breadcrumbs/`):

- `notes.json`: the note for each directory.
- `pending.json`: what the outstanding view asks, because an answer carries
  no context of its own.

The config grants `deno` read and write access and the two environment
variables it reads, `HOME` and `XDG_STATE_HOME`; it needs no network.

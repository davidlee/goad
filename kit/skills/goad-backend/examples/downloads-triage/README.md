# downloads-triage

Asks where each new download should go. Bash and `jq`. A file arriving in
the downloads directory is queued; the backend asks about the first in the
queue, and each answer is followed by the next question until the queue is
empty.

## See it in under a minute

From this directory, with `goad-check`, `bash` and `jq` on `PATH`:

```sh
goad-check --config config.toml --event events/file-arrived.json
```

This is the event the checker sends, the file `invoice.pdf` arriving:

```json goad:envelope
{"source":"downloads-triage","kind":"file-arrived","timestamp":"2026-10-17T09:00:00+10:00","data":{"name":"invoice.pdf"}}
```

The report ends `verdict: accepted, no exchange refused`. The file is not
there, so the backend says so on stderr, drops it from the queue and asks
nothing more: that is the real case of a download moved before you answered.

To see the window, run `goad config.toml` from this directory, then from a
second shell in this directory emit the same event. `goad-emit` takes the
envelope's fields as arguments; its timestamp is its own:

```sh
touch ~/Downloads/invoice.pdf
goad-emit --socket ./goad.sock --source downloads-triage --kind file-arrived --data '{"name":"invoice.pdf"}'
```

To have downloads reported as they arrive, run this beside the host. It needs
`inotifywait` (the inotify-tools package):

```sh
bash watch.sh --socket ./goad.sock
```

`watch.sh` passes its arguments on to `goad-emit`, and emits the file's name,
not its path.

## Run your own copy

`command` and `[ingress] path` are relative to the config file's directory,
so goad and `goad-check` find both wherever you start them. A running host
writes `goad.sock` and `goad.sock.lock` beside the config file, and an
installed kit is read-only, so copy this directory out and run your copy. Its
config needs no edit:

```toml goad:config
# `command` and the socket are relative to this file's directory, wherever
# goad or goad-check is started.
[backend]
command = ["bash", "backend.sh"]
timeout = "30s"

[schedule]
default_poll = "30m"

# Where a running goad listens for events, for watch.sh and goad-emit.
[ingress]
path = "./goad.sock"
```

To reach your copy's goad, pass `goad-emit` and `watch.sh` the path to your
copy's `goad.sock` with `--socket`.

## Behaviour

- A forwarded `file-arrived` event from `downloads-triage` queues
  `data.name`, a file name under the downloads directory. Names containing
  `/` are ignored.
- The view is *Where does NAME go?*: *Move*, with a `destination` choice
  (Documents first, then Pictures, Music, Videos), and *Leave it*.
- Each answer handles the first file in the queue, then shows the next one's
  view, or nothing when the queue is empty.
- A file no longer in the downloads directory is dropped with a line on
  stderr. A name already present at the destination is not overwritten: the
  file stays where it is, with a line on stderr.
- Host events and other forwarded events queue nothing. If the queue is not
  empty, they show its first file again.

## State and environment

State is under `$XDG_STATE_HOME/downloads-triage/` (default
`~/.local/state/downloads-triage/`): `queue.json`.

The directories are read from the environment, never from
`~/.config/user-dirs.dirs`, and default to `~/Name`: `XDG_DOWNLOAD_DIR`
(`~/Downloads`), `XDG_DOCUMENTS_DIR` (`~/Documents`), `XDG_PICTURES_DIR`,
`XDG_MUSIC_DIR`, `XDG_VIDEOS_DIR`. A desktop that keeps its directories only
in `user-dirs.dirs` does not export them: export them to use yours.

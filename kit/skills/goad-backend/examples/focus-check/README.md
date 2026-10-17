# focus-check

A time-boxed focus, in Python's standard library (3.10 or later). It shows
what you are focusing on and asks whether to continue, switch or take a
break; with no focus set it asks for one. It speaks when you press **Check
now**, and again when the block has ended. Otherwise it answers
`"view": null`.

## See it in under a minute

From this directory, with `goad-check` and `python3` on `PATH`:

```sh
goad-check --config config.toml
```

The report ends `verdict: accepted, no exchange refused` and shows a
`respond` line: the checker answered the first option of the view the
backend asked.

To see the window, run `goad config.toml` from this directory and press
**Check now**.

## Run it from anywhere else

`command` is relative to the directory goad is started in. Copy this
directory out and make `command` absolute:

```toml goad:config
# `command` is relative to this directory: run goad and goad-check from here.
[backend]
command = ["python3", "backend.py"]
timeout = "30s"

[schedule]
default_poll = "30m"
```

becomes, in your copy, `command = ["python3", "/absolute/path/to/backend.py"]`.

## Behaviour

- `requested`: with a focus set, a choice titled with it: *Continue for 10
  min* (with a progress note), *Switch focus to* (with a new focus), *Take a
  quick break*. With none, it asks for one.
- `scheduled`, or any host kind it does not know: the same choice once the
  block has ended, otherwise silent. While a block runs it asks to be checked
  again at the block's end, as an absolute instant.
- `startup`, and any forwarded event: silent.
- A new block lasts 25 minutes.
- An untouched text field arrives as `""` and counts as no answer: an empty
  focus sets none, an empty progress note is not logged.

## State and environment

State is under `$XDG_STATE_HOME/focus-check/` (default
`~/.local/state/focus-check/`):

- `state.json`: the current focus and when its block ends.
- `log.jsonl`: one line per decision.

Diagnostics go to stderr.

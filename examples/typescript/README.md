# A minimal goad backend

`backend.ts` is a complete backend in one file. It needs deno and nothing else —
no build step, no lockfile, no `node_modules`.

Point a config at it:

```toml
[backend]
command = ["deno", "run", "-A", "./examples/typescript/backend.ts"]
timeout = "5s"

[schedule]
default_poll = "30m"
```

`command` is an argument vector, never a shell string: goad does not interpose a
shell, so there are no quoting rules and no injection surface. A relative path is
resolved against goad's working directory.

## What it does

It prompts for an interstitial journal entry, and it decides for itself when to.
On `evaluate` it answers `view: null` — nothing to show — until the event says
enough time has passed, and then it returns a choice. On `respond` it does its
own work and answers `view: null` again.

goad never learns what an interstitial journal is. Replace every decision in the
file and the host behaves identically.

## Trust

`-A` grants the script the full authority of the user running goad. That is what
a backend is: the user's own program, run on their behalf. deno's default-deny
permissions are not a security boundary here and `-A` is not a loophole in one —
there is nothing to contain. Read a backend before you run it, the same way you
would read a shell script.

## Types

`deno run` does not typecheck. Run `deno check backend.ts` when you edit it —
goad's own `just check` does exactly that for this file.

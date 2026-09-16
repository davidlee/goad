# A `path:` flake ref breaks once `just demo` has run

Learned at slice 007 (cited by `plan.md` and all six phase briefs before it was
written; the citation is older than the file).

## The fact

`just demo` runs goad on `examples/demo.toml`, whose event-ingress socket is a
**relative** path resolved against the repository root — so a unix socket
appears in the checkout and stays there. A `path:` flake reference copies the
whole working tree into the nix store, and copying a unix socket fails. Once the
demo has run even once, `nix develop path:.` and friends stop working, with an
error about the socket rather than about anything you changed.

Use the bare git-input form instead: `nix develop .` — it takes the tracked
tree, and the socket is untracked.

## Why

`examples/demo.toml`'s comment says the `--socket` argument is not optional
because the demo runs on an explicit configuration path rather than the default
one. The consequence nobody wrote down is where the socket then lives: in the
working directory, which is the repository root, because `just` sets it there.

The failure is confusing out of proportion to its cause. It appears after a
successful demo run, in a command that has nothing to do with the demo, and
names a file the person did not create.

## How to apply

- Reach for `nix develop .` by default. Keep `path:` for a tree you know has no
  socket in it.
- If a `path:` ref must be used, remove the socket first; it is regenerated on
  the next run and holds no state worth keeping.
- The general form is worth more than the instance: **a relative socket path in
  a config resolves against the working directory, and the working directory is
  usually the repository.** Any future demo or fixture that opens a socket,
  named pipe, or device node in the checkout breaks store copying the same way.

Related: `wayland-window-placement-is-the-compositors.md`, the other thing
about running this product that is the environment's and not the code's.

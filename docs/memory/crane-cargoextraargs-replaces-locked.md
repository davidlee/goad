# Setting crane's `cargoExtraArgs` replaces its default, which is `--locked`

Learned at slice 006, PHASE-01. Both the spike for this slice and
`~/dev/doctrine`'s flake get it wrong.

## The fact

`cargoExtraArgs` is not appended to crane's defaults — it **replaces** them, and
the default is `--locked`. So adding a package selector:

```nix
cargoExtraArgs = "-p goad";        # --locked is now gone
cargoExtraArgs = "--locked -p goad";  # write it back
```

A build that silently drops `--locked` will happily resolve a dependency the
lockfile does not pin, which is the whole property a lockfile exists to give
you, and nothing in the build output says it stopped holding.

## The neighbouring trap in the same option

crane's `checkPhaseCargoCommand` also inherits `cargoExtraArgs`. With
`doCheck = true` and a `--bin` selector, the check phase reports green having
run no tests at all. This slice answered that by setting `doCheck = false`
throughout and keeping the tests on the phase gate, where a person can see them.

## How to apply

Any time you set `cargoExtraArgs`, write `--locked` back in unless you have a
stated reason not to. Treat "does `--locked` survive this?" as a checked
property of a flake, not an assumption — `nix derivation show` prints the
resolved command.

Related: `path-flake-ref-breaks-on-demo-socket.md`.

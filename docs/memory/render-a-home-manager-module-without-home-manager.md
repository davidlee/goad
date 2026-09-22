# A home-manager module can be rendered, and read, with no home-manager input

Built at slice 006, PHASE-02, to verify a generated systemd unit.

## The fact

`lib.evalModules` over three modules prints the attrset a home-manager module
would produce, without taking home-manager as a flake input:

1. a **stub** declaring only the options the module under test *sets*
   (`home.packages`, `systemd.user.services`),
2. the module itself,
3. a fragment that enables it and supplies its required options.

`lib` comes from the flake's own locked nixpkgs — `flake.inputs.nixpkgs.lib` —
and `derivation { name; system; builder; }` is a fake package that satisfies
`types.package` without building anything.

## What it does not hold

The stub's option types are **permissive**. This renders the module's output; it
is not home-manager's acceptance of that output. A type home-manager would
reject — `attrsOf anything` admitting a nested attrset that merges into the
wrong block, say — passes this harness and fails in the consumer's tree.

State that boundary wherever the harness is cited. A check that proves less than
it appears to is worse than none.

## Also worth knowing

`nix flake show` prints `homeManagerModules: unknown` and does not descend — it
has no type for a module output, so it proves the attribute exists and nothing
more. `builtins.isFunction` on the export is the cheap check that it evaluates.

Related: `a-check-that-compares-two-derived-things-holds-nothing.md`.

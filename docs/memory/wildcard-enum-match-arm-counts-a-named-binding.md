# `clippy::wildcard_enum_match_arm` counts a named binding as a wildcard, not only `_`

Measured at slice 006, PHASE-04, in `crates/goad/src/main.rs`.

## The fact

Under that file's crate-root `deny`, an arm spelled `other => …` in a match on
an enum fails the build with *wildcard match will also match any future added
variants*. The lint does not care that the binding has a name; it catches any
arm that is irrefutable over the remaining variants.

The consequence is concrete and not a style preference:
`map_err(|fault| match fault { … })` is **not available in that file at all**,
because the closure's match is over the error enum and any catch-all arm is a
wildcard.

## How to apply

Match on the enclosing `Result` instead:

```rust
let config = match Config::load(path) {
  Err(ConfigError::Read(fault)) => return Err(StartupError::ConfigUnreadable { … }),
  Err(fault)                    => return Err(StartupError::ConfigUnparseable { … }),
  Ok(config)                    => config,
};
```

`Err(fault)` is a tuple-struct pattern at the arm's top level and the binding
sits *inside* it, which the lint does not reach. `goad-emit`'s `socket_path`
already makes this cut; it is the workspace's shape for the problem, not an
invention.

So: in a file with this deny, "split one error arm into two" is a decision about
**which value you match on**, made before you start writing, not a refactor you
can reach for afterwards.

## The cost: the same move takes the match out of the lint's reach

Measured at slice 010, PHASE-01, by negative control with the mutated build seen
to compile. `exit::status` matches on `&Result<Ended, StartupError>`; a `_` arm
added beneath `Ok(Ended::AsAsked)` fires **neither** `wildcard_enum_match_arm`
nor `match_wildcard_for_single_variants`. The same wildcard in a match over
`&Ended` fires the latter.

So matching on the enclosing `Result` is both the remedy above and the way a
match leaves the deny altogether. Where a function is written that way, its
exhaustiveness is held by its **cases** — one per shape, each seen to fail
under a mutation — and not by the crate-root deny. Do not write, in a design or
a doc comment, that the deny holds such a match; check with a compiled negative
control (`negative-control-must-compile.md`).

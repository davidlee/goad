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

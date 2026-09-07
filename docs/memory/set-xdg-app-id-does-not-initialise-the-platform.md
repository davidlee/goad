# `set_xdg_app_id` does not initialise the Slint platform — a component constructor does

Measured 2026-09-08 against slint 1.17.1, on the slice 002 startup sequence
(`crates/goad/src/main.rs` step 3).

## The fact

`slint::set_xdg_app_id` is not a lazy initialiser. Called before any component
exists, it returns

```
No default Slint platform was selected, and no Slint platform was initialized
```

which `main.rs` reports as `StartupError::Platform` and exits 2. What selects
the backend is constructing a component — `PromptWindow::new()` succeeds where
`set_xdg_app_id` immediately before it fails, in goad and in an eighty-line
scratch crate with nothing but `slint = "=1.17.1"`.

`design.md` §5.4 (slice 002) and `plan.md:1487` state the app id is set *before
any component is constructed*. The constraint that actually holds is weaker:
before anything is **shown**. Construction is not showing, so the correct order
is `PromptWindow::new()`, then `set_xdg_app_id`, then everything else.

## Why it matters

The gate did not see it. Every renderer and event-loop test installs
`i-slint-backend-testing` itself, so no target in `just check` constructs the
real platform: the gate was green for three slices while
`./target/debug/goad <config>` could not start on any machine.

## How to apply

- Any Slint call that configures the platform — app id, and anything else
  reached through the global context — goes *after* the first component
  constructor and *before* the first `show()`.
- A test suite that installs its own backend proves nothing about the real one.
  Launching the binary is its own check, and nothing in the gate substitutes
  for it (see `docs/memory/slint-testing-backend-initialises-once-per-process.md`).
  `just demo` is that check, and it is a person's to run.

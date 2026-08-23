# Slint build mechanics under the nix dev shell

Established by a throwaway spike, `99404f8`, which the next commit deleted. Read
that commit for the code; this note is the durable part.

**Version at the time:** slint 1.17.1 (latest published, 2026-08-23).

## Wiring

Three pieces, all required:

- `Cargo.toml` — `slint` as a dependency, `slint-build` as a
  **build-dependency**. Same version.
- `build.rs` — `slint_build::compile("ui/main.slint")`.
- Rust — `slint::include_modules!()` at crate root, which brings in a generated
  struct per exported `.slint` component.

Generated API, for a component `MainWindow`: `MainWindow::new()` returns
`Result<_, slint::PlatformError>`; `.run()` drives the event loop; a
`callback foo(string)` becomes `on_foo(impl Fn(SharedString))`; an
`in-out property <string> bar` becomes `get_bar()` / `set_bar()`. `as_weak()`
plus `upgrade()` is how a callback closure reaches back into the window without
a reference cycle.

## The gotcha

Stock widgets need an explicit import inside the `.slint` file:

```slint
import { Button } from "std-widgets.slint";
```

Without it the failure is `error: Unknown element 'Button'` reported by the
**build script**, surfacing as `failed to run custom build command for …` with a
`Result::unwrap()` panic in `build.rs`. It is not a rustc error and does not
point at the Rust source. Worth knowing before losing time to it.

## Cost

411 unique dependencies. ~19s clean debug build of a hello-world binary on this
machine (`user 2m38s` across cores), plus the `build.rs` codegen step on every
build. This is the number behind ADR-002: it is what a headless protocol test
would pay if it shared a crate with the renderer.

## Nix

No changes to `flake.nix` were needed. The runtime shared libraries Slint
`dlopen()`s are already assembled as `guiLibs` and exported on
`LD_LIBRARY_PATH` for both the dev shell and the agent jail — wayland,
libxkbcommon, libglvnd, fontconfig, and the gcc runtime. They are deliberately
*not* build inputs; the comment in `flake.nix` explains that the window never
opens without them on the library path.

Verified present in the live shell: `WAYLAND_DISPLAY=wayland-1`, `DISPLAY=:0`,
`XDG_RUNTIME_DIR=/run/user/1000`.

## What this does not establish

- Nothing about Markdown rendering, rich content, or any widget beyond `Button`
  and the layout primitives.
- Nothing about running Slint headless, so nothing about GUI tests in CI.
- Slint owns its own event loop. Since the host uses tokio (slice 001), the
  integration is the runtime on its own thread with
  `slint::invoke_from_event_loop` marshalling back into the UI. Known pattern,
  not free, and unproven here — it is slice 002's work.

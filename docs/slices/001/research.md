# Research — Slice 001

**Producers:** main agent (repository survey, Slint toolchain spike)
**As of:** 2026-08-23 · 70f282a (working tree: flake.nix, flake.lock, README.md staged)

Evidence artefact for design and plan. Later stages cite this instead of
re-deriving. Refresh in place when it drifts; do not append rounds.

## Verification legend

- ✓ — independently verified by the *consuming* agent (a read or grep of the
  cited site).
- unmarked — researcher claim: cited, not checked.

Design and plan may only load-bear ✓ rows, or rows they verify at point of use.
Verify what you lean on, not everything.

## Citation forms

Canon claims cite the document id (`SPEC-003 §4`, `ADR-007`). Code claims cite
`path:line`. An uncited claim is unverifiable by definition.

## Thread 1 — governing canon

### Binding

None. `docs/specs/`, `docs/policy/` and `docs/adr/` all exist and are all
empty ✓ (`find docs -type f`, 2026-08-23 — only `brief.md`, `AGENTS.md` and the
slice templates are present).

`docs/AGENTS.md` is the methodology this slice follows. By its own definition
canon is `specs`, `policy` and `adr`; the methodology document is not itself in
that set.

### Checked, not applicable

- `docs/specs/*` — empty directory, nothing to apply.
- `docs/policy/*` — empty directory, nothing to apply.
- `docs/adr/*` — empty directory, nothing to apply.
- `docs/memory/*` — empty directory, no prior facts to carry.
- Root `AGENTS.md` / `CLAUDE.md` — `CLAUDE.md` is a symlink to `AGENTS.md` ✓.
  The file was zero bytes when this research ran and is no longer: as of
  2026-08-23 it carries a pointer to `docs/AGENTS.md`, the canon rule, dev-shell
  facts, and four working principles ✓. AC-10 is therefore additive. Still
  absent, and still required by brief §15.1: the
  host-does-not-understand-the-domain invariant, the permissive-wire /
  canonical-internal rule, the warning against narrowing the protocol to the
  current renderer, and the verification commands.

### Amendment candidates

None: nothing exists to amend. The inverse applies — this slice is the first
opportunity to *create* canon, which is OQ-1.

## Thread 2 — code map

### Hotspots

There is no Rust code. `src/` and `tests/` exist and are empty ✓; there is no
`Cargo.toml` ✓. Every file this slice touches is a new file.

### Cited facts

Toolchain, from the nix dev shell, 2026-08-23:

- `cargo 1.99.0-beta.1 (eb98b54bc 2026-08-11)` ✓, from
  `rust-bin.beta.latest.default` (`flake.nix:41`).
- `pkg-config` on PATH ✓ (`flake.nix:43`).
- Slint's runtime shared libraries are supplied as `guiLibs` and exported on
  `LD_LIBRARY_PATH` in the dev shell and inside the agent jail
  (`flake.nix:29-36`, `flake.nix:104`, `flake.nix:87`). Verified present in the
  live shell ✓ — wayland, libxkbcommon, libglvnd, fontconfig, gcc lib.
- `WAYLAND_DISPLAY=wayland-1`, `DISPLAY=:0`, `XDG_RUNTIME_DIR=/run/user/1000` ✓.
- crates.io reachable ✓.

Slint spike (throwaway, built under
`$SCRATCH/slint-spike/`, not in the repository):

- Latest published `slint` is **1.17.1** ✓ (crates.io versions API).
- `slint = "1.17"` plus `slint-build = "1.17"` as a build dependency compiles
  clean; no additional system dependencies beyond `guiLibs` were needed ✓.
- The generated binary opens a window and runs its event loop without error ✓
  (`timeout 6 ./target/debug/slint-spike` exited 124, i.e. still alive at kill;
  window visually confirmed by the user).
- Mechanics: `build.rs` calls `slint_build::compile("ui/main.slint")`; the Rust
  side uses `slint::include_modules!()`; a `.slint` file must
  `import { Button } from "std-widgets.slint";` before using stock widgets.
  Omitting the import fails in the **build script**, not in rustc, so the error
  surfaces as `failed to run custom build command` ✓.

Async runtime sizing, measured 2026-08-23 by resolving throwaway manifests in
the scratchpad (`cargo tree --prefix none`, unique `name version` pairs):

- `tokio` with `["process", "net", "time", "rt", "macros", "io-util"]` — **14**
  unique dependencies ✓.
- `async-process` 2 + `async-net` 2 + `async-io` 2 + `futures-lite` 2 — **31**
  unique dependencies ✓.

Tokio is one large crate with few dependencies; the smol ecosystem is many small
crates. So brief §4.1's "smallest reasonable" favours tokio on dependency count
and maintenance surface, which is the opposite of the intuitive reading.

Slint's own tree, for comparison: **411** unique dependencies, 19s clean debug
build of the spike (warm registry, `user 2m38s` across cores) ✓. This is the
number the single-crate-versus-workspace question turns on — a headless protocol
test in the same crate as a Slint binary pays that build.

### Precedents

No Rust precedents exist in-repo. Structural precedent to follow is
documentary: `docs/AGENTS.md` (methodology), the slice templates under
`docs/templates/slice/`, and the repository layout recommended in brief §15 —
`AGENTS.md`, `docs/`, `examples/`, `src/`, `tests/protocol/`,
`tests/integration/`.

`flake.nix` establishes one relevant house style: comments explain *why* a
non-obvious thing is done, at length, rather than restating what the code does.

## Cross-thread findings

The absence of canon and the absence of code coincide, which makes this slice
unusually load-bearing: whatever it establishes becomes precedent by default
rather than by decision. The two open questions with the longest reach are
OQ-1 (does the protocol become canon now) and OQ-2 (single crate or workspace),
because later slices inherit both without a natural opportunity to revisit.

The GUI spike also settles a sequencing argument in the brief's favour. Brief
§20 puts rendering in phase 3 and keeps it out of phase 1; the usual reason to
front-load a GUI is to retire integration risk, and that risk is now retired
outside the slice system for the cost of one scratch build.

## Design-input deltas

- Slint version and build mechanics are known, so slice 002 needs no
  investigation phase for them; they belong in `docs/memory/` at close.
- No async runtime is present in the dependency tree, so OQ-3 is a genuinely
  open choice rather than a fait accompli.
- Brief §15's recommended layout splits `tests/protocol/` from
  `tests/integration/`, which maps onto this slice's fixture corpus (AC-9) and
  round-trip test (AC-7) respectively.

# Spike — slice 006, the crane package

**What this is:** the measurement apparatus for `research.md` Thread 3, exactly
as it was built and run against `c658b1a` on 2026-09-20, then reverted out of
the working tree. It is kept so that a reader can check Thread 3's claims
against something runnable rather than against prose, and so the plan's first
phase starts from a thing that built rather than from a description of one.

**What this is not:** a design, a proposal, or the shape the slice will land.
It is deliberately *wider* than the deliverable — it carries five throwaway
packages whose only job was to fail, or to be timed:

| package | what it was for | what it showed |
|---|---|---|
| `goad` | the arm that works | builds, wraps, runs from an empty environment |
| `goad-emit` | the second binary | builds in 10s on warm artifacts, needs no wrapper |
| `goad-narrow` | negative control | crane's stock filter drops `ui/app.slint`; `build.rs` fails in 7s |
| `goad-slint-only` | negative control | `.slint` admitted, `.ttf` stripped: the Slint compiler cannot resolve the two font imports |
| `goad-checked-nofonts` | `doCheck` arm B1 | six `instant.rs` cases fail — no tzdb in the sandbox |
| `goad-checked-fonts` | `doCheck` arm B2 | `TZDIR` + `FONTCONFIG_FILE` get past the lib tier; `event_loop_schedule` still fails |
| `goad-remeasure` | timing | 22s for the final layer against warm `cargoArtifacts` |

**Known to be wrong for the deliverable**, and listed here so nobody copies it:

- `workspaceVersion` is the literal `"0.1.0"`. The spike proved *why* it cannot
  be read with `builtins.fromTOML` (Thread 3 S-1); where the real version and
  revision come from is OQ-4 and is not answered here.
- `goad-emit` is given `guiLibs` as `buildInputs`. It has no renderer and needs
  none.
- the `.ttf` filter admits every face in `assets/`, including the untracked
  ones the build does not use (Thread 3 S-2's stated residue).
- no `homeManagerModules` output at all — OQ-1 is untouched by this spike.
- `cargoExtraArgs` on the working `goad` arm drops crane's default `--locked`.

**It does not build as it stands**, and that is not a defect to repair here:
`flake.lock` has no `crane` entry at this commit, and `Cargo.toml`'s `tokio`
entry is still two lines. Both are Thread 3 findings, and the plan's first
phase is where they land.

**At close:** this directory is a record of a measurement, like a ledger. It is
superseded by whatever `flake.nix` the slice actually lands, and `audit.md`
says so rather than this being quietly deleted.

# Slint spike

Throwaway. Built 2026-08-23 to answer one question before slice 001 was scoped:
does a Slint binary build and open a window in this repository's nix dev shell?

It does. Findings are recorded in `docs/memory/` and in
`docs/slices/001/research.md`; this directory is deleted in the commit
immediately following the one that added it. It is kept in history only so the
code behind those findings can be read rather than taken on trust.

Not host code. It predates the host's first `Cargo.toml` and obeys none of the
project's architectural canon.

Run: `cargo run` from this directory, inside `nix develop`.

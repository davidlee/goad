# goad — task runner.
#
# `just` is in `flake.nix` `devToolPkgs`, so these recipes work from a clean
# clone in the dev shell (AC-1). A shell entered before that landed will resolve
# `just` from the user's nix profile instead; reload it.
#
# `docs/slices/002/draft-policy.md`'s command block is canonical — the slice's
# working authority until it is promoted at audit (`docs/AGENTS.md:36`, `:38`) —
# and `docs/slices/002/design.md` §5.6 is where it is derived. Every recipe here
# runs the same command with the same arguments as a line of it, in the same
# order — not the same characters: a fenced block carries neither comments nor
# recipe line wrapping. Change the block first, then mirror. `just -n check`
# prints the sequence for comparison (PHASE-01/VA-3).

# Run the whole phase gate — six commands, in order.
default: check

# The phase gate. A phase is not green until this exits 0.
check: build test test-stratum1 typecheck lint fmt-check

build:
  cargo build --workspace

test:
  cargo test --workspace

# The only command in the gate that builds and runs stratum 1 with exactly the
# features its own manifest asks for. `cargo test --workspace` unifies features
# across members, so stratum 1 is compiled there with whatever stratum 2 and 3
# switch on in a shared dependency. It is **not** a purity check — it rejects
# nothing; it makes the other instruments check a configuration that stands on
# its own (§5.6).
test-stratum1:
  cargo test -p goad-semantics

# The example backend is documentation agents edit (brief §3.7), and `deno run`
# does not typecheck it — measured at slice 001 PHASE-08, a type error runs to
# exit 0. So the gate does. deno is in `flake.nix` `projectPkgs`, so AC-1's clean
# clone in the dev shell still holds.
typecheck:
  deno check examples/typescript/backend.ts

# One column, not two. `tokio` and `toml` are unconditional dependencies of
# stratum 2 now and absent from stratum 1, so the `shell` feature has nothing
# left to gate and `--no-default-features` is no longer a distinct column; the
# second clippy line and its `-A dead_code -A unreachable_pub` carve-out go with
# it (§5.6). `--workspace` lints every member, the renderer included, so nothing
# is added back.
lint:
  cargo clippy --workspace --all-targets -- -D warnings

fmt-check:
  cargo fmt --all --check

# Not in the gate: it writes. Here so the gate's failure has an obvious answer.
fmt:
  cargo fmt --all

# Not in the gate: these start a GUI, and the gate stays headless. `cargo run`
# so that a stale binary is rebuilt rather than launched.

# Start goad with your own configuration file.
run config:
  cargo run -p goad --bin goad -- {{config}}

# The example backend that always prompts, so there is a window to look at.
demo: (run "examples/demo.toml")

# The `--socket` is what earns this recipe its place: emit discovers the host's
# *default* configuration path only, and `demo` runs on an explicit one, so a
# bare `goad-emit --source … --kind …` looks at the wrong file (005/F-6).

# Prompt an evaluation in a running `just demo`, from a second shell.
emit source kind:
  cargo run -p goad-emit -- --socket ./goad-demo.sock --source {{source}} --kind {{kind}}

# Not in the gate, and deliberately: POL-001's six commands are the gate and
# nothing here joins them (D8). The residue that buys is stated in the design —
# a `flake.nix` that stops building is green at the gate and breaks in the
# consuming flake. This recipe is where that is found, by someone running it.
#
# The flake reference is the **bare git form** — `.#goad`, never `path:.`. A
# `path:` reference copies the working directory into the store, and this one
# holds `goad-demo.sock`, a unix socket nix refuses outright, and
# `.claude/worktrees/`, six gitignored worktrees it would copy anyway. The bare
# form reads the git tree instead, which has its own edge: **an untracked file
# is invisible to it.** `git add` a new file before building, or the build is of
# a tree that does not contain it.
#
# `--no-link` so that no `result` symlink lands in the checkout; the two store
# paths are printed instead.

# Build both binaries with nix. Not part of `check`.
package:
  nix build --no-link --print-out-paths .#goad .#goad-emit

# Not in the gate: it installs outside the repository.
#
# `cargo install --path .` cannot work here — the workspace root is a virtual
# manifest with no `[package]`. Each binary's own member is the target.
#
# The env file is half of the install, not a convenience. The GUI libraries are
# dlopen'd rather than linked, so `ldd` resolves clean and the binary still
# opens no window without `LD_LIBRARY_PATH`; fontconfig finds fonts through a
# config file, so `FONTCONFIG_FILE` is the second half and its absence is a
# window that draws no text. A binary installed without both is broken in a way
# nothing reports until a window next happens to draw.
#
# That coupling is what `just package` retires — on the nix path only. A
# wrapped binary carries both variables itself, and the home-manager unit names
# no `EnvironmentFile` at all. This recipe is the *cargo* path, which goad must
# keep working on machines that are not NixOS, so here the pair survives and the
# env file with it (OQ-6, D7). What changed is its reader: after slice 006
# nothing automatic reads it, only a person. The store paths it names go stale
# like any others, and nothing reports that they have; the repair is to run this
# recipe again (R2).
#
# `${VAR:?...}` rather than a bare expansion: run outside the dev shell both are
# empty, and an env file naming two empty values is the same silent breakage
# written down.

# Install both binaries into $CARGO_HOME/bin, with the environment they need.
install:
  : "${LD_LIBRARY_PATH:?run this in the dev shell: both vars come from flake.nix}" "${FONTCONFIG_FILE:?}"
  cargo install --path crates/goad --locked
  cargo install --path crates/goad-emit --locked
  mkdir -p ${XDG_CONFIG_HOME:-$HOME/.config}/goad
  printf 'LD_LIBRARY_PATH=%s\nFONTCONFIG_FILE=%s\n' "$LD_LIBRARY_PATH" "$FONTCONFIG_FILE" \
    > ${XDG_CONFIG_HOME:-$HOME/.config}/goad/env

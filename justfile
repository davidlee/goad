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

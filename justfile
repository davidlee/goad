# goad — task runner.
#
# `just` is in `flake.nix` `devToolPkgs`, so these recipes work from a clean
# clone in the dev shell (AC-1). A shell entered before that landed will resolve
# `just` from the user's nix profile instead; reload it.
#
# `docs/slices/001/design.md` §9's command block is canonical. Every recipe here
# runs the same command with the same arguments as a line of it, in the same
# order — not the same characters: §9 carries inline comments and wraps the
# second clippy line, and neither survives into a recipe. Change §9 first, then
# mirror. `just -n check` prints the sequence for comparison (PHASE-01/VA-3).

# Run the whole phase gate — §9's six commands, in §9's order.
default: check

# The phase gate. A phase is not green until this exits 0.
check: build test test-stratum1 lint fmt-check

build:
  cargo build

test:
  cargo test

# The mechanical half of ADR-001's dependency rule (D49, F-51): fails to compile
# if anything under `semantics/` acquires a runtime dependency.
# `cargo tree --no-default-features` is the diagnostic when it does.

# Stratum 1 alone.
test-stratum1:
  cargo test --no-default-features

# `lint`, in full — the doc comment just above the recipe is the short form.
#
# Both feature columns, per `design.md` §9 (AC-1).
#
# Two columns because `--all-targets` alone lints only the default feature set,
# so every `#[cfg(not(feature = "shell"))]` path — and stratum 1 compiled
# without the shell above it — would go unlinted and "zero warnings" would be a
# claim about one column of two.
#
# The two `-A`s on the second column are load-bearing, not noise-suppression.
# There `shell` is gone, so any `semantics/` item whose only caller lives in
# stratum 2 is genuinely unused — ADR-001's feature gate working, not a defect.
# The first column stays strict, so dead code still fails the gate. Change these
# in §9 first; this recipe mirrors it.

# Lint both feature columns at zero warnings.
lint:
  cargo clippy --all-targets -- -D warnings
  cargo clippy --all-targets --no-default-features -- -D warnings -A dead_code -A unreachable_pub

fmt-check:
  cargo fmt --check

# Not in the gate: it writes. Here so the gate's failure has an obvious answer.
fmt:
  cargo fmt

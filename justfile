# goad — task runner.
#
# `just` comes from the user's nix profile, not `flake.nix` devToolPkgs. Add it
# there if these recipes become load-bearing for anyone else.

default: lint

# `lint`, in full — the doc comment just above the recipe is the short form.
#
# Both feature columns, per `docs/slices/001/design.md` §9 (AC-1).
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

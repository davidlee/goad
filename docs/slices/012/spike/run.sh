#!/usr/bin/env bash
# 012 spike: R1 (plugin load from a read-only store path) and R2 (the walk-home
# bind shadows the shared jail home). Throwaway; results in research.md
# §"Spike: R1 and R2". Builds with a path: ref — the checkout's git: form
# resolves the enclosing repo, and a copy outside it avoids that.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
tmp=$(mktemp -d); cp -r "$here"/. "$tmp"; cd "$tmp"
nix build "path:$tmp#shell" -o shell
jail=$tmp/shell/bin/jailed-shell
fresh() { rm -rf "$tmp/w"; mkdir -p "$tmp/w/home" "$tmp/w/work"; cd "$tmp/w/work"; }

echo "== R2: what the walk home hides"; fresh
GOAD_WALK_HOME=$tmp/w/home "$jail" bash -c '
  probe(){ if ls "$1" >/dev/null 2>&1; then echo "READABLE $1"; else echo "hidden   $1"; fi; }
  probe /home/david/dev/goad/CLAUDE.md
  probe "$HOME/.claude"; probe "$HOME/.codex"
  probe /home/david/.claude/projects/-home-david-dev-goad/memory
  echo "goad-source paths: $(ls -d /nix/store/*-goad-source 2>/dev/null | wc -l)"
  ls -A "$GOAD_KIT" "$GOAD_KIT_REPO"'

echo "== R1 Codex"; fresh
GOAD_WALK_HOME=$tmp/w/home "$jail" bash -c '
  codex plugin marketplace add "$GOAD_KIT" || echo "kit-only refused (expected)"
  codex plugin marketplace add "$GOAD_KIT_REPO" && codex plugin add goad@goad && codex plugin list'

echo "== R1 Claude"; fresh
GOAD_WALK_HOME=$tmp/w/home "$jail" bash -c '
  claude --plugin-dir "$GOAD_KIT" plugin details goad | head -6
  claude plugin marketplace add "$GOAD_KIT_REPO" && claude plugin install goad@goad && claude plugin details goad@goad | head -6
  claude -p hi --plugin-dir "$GOAD_KIT" || true   # fresh home: "Not logged in"'

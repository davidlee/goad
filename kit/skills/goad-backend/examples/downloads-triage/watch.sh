# shellcheck shell=bash
# Emit a `file-arrived` event for each file that appears in the downloads
# directory (XDG_DOWNLOAD_DIR, default ~/Downloads). Needs `inotifywait`
# (inotify-tools), `goad-emit` and `jq` on PATH.
#
# Arguments are passed through to goad-emit, so a host started on this
# example's config.toml is reached by passing the goad.sock beside it; from
# this directory:
#
#   bash watch.sh --socket ./goad.sock
#
# The event's data is the file's name, not its path: the backend resolves it
# under the same directory.

set -euo pipefail

downloads=${XDG_DOWNLOAD_DIR:-$HOME/Downloads}

inotifywait --monitor --quiet --event close_write --event moved_to \
  --format '%f' "$downloads" |
  while IFS= read -r name; do
    goad-emit "$@" --source downloads-triage --kind file-arrived \
      --data "$(jq -cn --arg name "$name" '{name: $name}')"
  done

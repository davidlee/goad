# shellcheck shell=bash
# Downloads triage: a goad backend in bash and jq.
#
# A file arriving in the downloads directory reaches this backend as a
# forwarded `file-arrived` event (see watch.sh), carrying the file's name. The
# name is queued; the backend asks where the head of the queue should go. Each
# respond handles the head and returns the next head's view while the queue is
# not empty (SPEC-001/R-33), so a burst of arrivals is triaged one question at
# a time.
#
# One process per exchange: one JSON request on stdin, one JSON response on
# stdout, nothing else on stdout. Diagnostics go to stderr.
#
# Environment, standard names only:
#   XDG_STATE_HOME     state lives in $XDG_STATE_HOME/downloads-triage/
#                      (default ~/.local/state); queue.json is the queue
#   XDG_DOWNLOAD_DIR   where arrivals are looked for (default ~/Downloads)
#   XDG_DOCUMENTS_DIR, XDG_PICTURES_DIR, XDG_MUSIC_DIR, XDG_VIDEOS_DIR
#                      the destinations (defaults ~/Documents, ~/Pictures,
#                      ~/Music, ~/Videos). Documents is the first alternative.
#
# A file that is no longer in the downloads directory when its turn comes is
# dropped from the queue with a line on stderr; a person can move or delete a
# download before answering. A name already present at the destination is not
# overwritten: the file stays in the downloads directory, with a line on stderr.

set -euo pipefail

example='downloads-triage'
state_dir=${XDG_STATE_HOME:-$HOME/.local/state}/$example
queue=$state_dir/queue.json
downloads=${XDG_DOWNLOAD_DIR:-$HOME/Downloads}

# Values from the request are read one jq call each, never by matching text
# in the request: `data` is a writer's, and must not reach the control flow.
request=$(cat)
field() { jq -r "$1" <<<"$request"; }

diagnostic() { printf '%s: %s\n' "$example" "$*" >&2; }

# The destination directory of an alternative id, from its standard variable.
destination() {
  case $1 in
    documents) printf '%s' "${XDG_DOCUMENTS_DIR:-$HOME/Documents}" ;;
    pictures) printf '%s' "${XDG_PICTURES_DIR:-$HOME/Pictures}" ;;
    music) printf '%s' "${XDG_MUSIC_DIR:-$HOME/Music}" ;;
    videos) printf '%s' "${XDG_VIDEOS_DIR:-$HOME/Videos}" ;;
    *) return 1 ;;
  esac
}

queue_head() { jq -r '.[0] // empty' "$queue" 2>/dev/null || true; }

# Replace the queue with the output of a jq filter over it.
rewrite_queue() {
  mkdir -p "$state_dir"
  [ -f "$queue" ] || printf '[]' >"$queue"
  jq -c "$@" "$queue" >"$queue.new"
  mv "$queue.new" "$queue"
}

# The answer: the view for the head of the queue, or none.
answer() {
  local name
  name=$(queue_head)
  if [ -z "$name" ]; then
    printf '{"view":null}\n'
    return
  fi
  jq -cn --arg name "$name" '{view: {
    kind: "choice",
    title: "Where does \($name) go?",
    options: [
      {id: "move", label: "Move", fields: [
        {id: "destination", kind: "choice", label: "Destination", options: [
          {id: "documents", label: "Documents"},
          {id: "pictures", label: "Pictures"},
          {id: "music", label: "Music"},
          {id: "videos", label: "Videos"}
        ]}
      ]},
      {id: "leave", label: "Leave it"}
    ]
  }}'
}

# Move the head of the queue to `destination`. The head is dropped whatever
# happens, so a file that cannot be moved is not asked about forever.
move_head() {
  local name target
  name=$(queue_head)
  if [ -z "$name" ]; then
    return
  elif [ ! -e "$downloads/$name" ]; then
    diagnostic "$name is no longer in $downloads"
  elif ! target=$(destination "$1"); then
    diagnostic "no such destination: $1"
  elif [ -e "$target/$name" ]; then
    diagnostic "$target/$name exists; leaving $name where it is"
  else
    mkdir -p "$target"
    mv -- "$downloads/$name" "$target/$name"
  fi
}

# A name from an event is a file name, never a path.
arrival() {
  local name
  # shellcheck disable=SC2016 # `$name` is a jq variable, not a shell one.
  local enqueue='if index($name) then . else . + [$name] end'
  name=$(field '.event.data.name | select(type == "string") // empty')
  case $name in
    '' | */* | . | ..) diagnostic "ignoring an arrival with no usable name" ;;
    *) rewrite_queue --arg name "$name" "$enqueue" ;;
  esac
}

case $(field '.type // empty') in
  respond)
    if [ "$(field '.response.option // empty')" = move ]; then
      move_head "$(field '.response.values.destination // empty')"
    fi
    rewrite_queue '.[1:]'
    ;;
  *)
    if [ "$(field '.event.source // empty')" = "$example" ] \
      && [ "$(field '.event.kind // empty')" = file-arrived ]; then
      arrival
    fi
    ;;
esac

answer

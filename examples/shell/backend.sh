# A goad backend in ten lines of shell, and the one `just demo` runs.
#
# The protocol is one JSON document in on stdin, one JSON document out on
# stdout, per process. Nothing here needs a runtime, a package manager, or a
# shebang: the config names `["bash", "examples/shell/backend.sh"]`, so bash is
# the program and this file is its argument. It is bash rather than `sh`
# deliberately — `${value//from/to}` below is a bash expansion.
#
# Unlike `examples/typescript/backend.ts`, this one decides nothing. It prompts
# every time it is asked to evaluate, which is what makes it a demo: run it and
# a window is there. A real backend reads its own state and answers `view: null`
# far more often than not.

request=$(cat)

# Substring matching on JSON is not something to imitate — it is here so that
# this file stays readable without a parser. `${request#*'"type":"'}` then
# `%%'"'*` reads a value with no parser, and the **first** occurrence is the
# right one for each of the three read here: `type` is the request's own second
# key and `source` and `kind` are the event's first two (the host serialises
# compactly, in field order), so all three precede any `data` a watcher wrote.
#
# **Every branch below reads one of these values, and none matches against
# `$request`.** `case $request in *'"type":"respond"'*)` matches that substring
# anywhere — including inside `data`, which is opaque and carries whatever a
# watcher put there — so a whole-request match hands a watcher the choice of
# which prompt this file shows, or whether it shows one at all. That is one
# rule, and it applies to every branch: a value the host carries opaquely must
# not reach this file's control flow.
type=${request#*'"type":"'}
type=${type%%'"'*}
source=${request#*'"source":"'}
source=${source%%'"'*}
kind=${request#*'"kind":"'}
kind=${kind%%'"'*}

# `source` and `kind` are the watcher's own words, and the host carries them
# through unexamined — so they can contain `"` or `\`, either of which would
# break the JSON below if interpolated raw. Escaping both is what a backend
# owes its own output, and in bash it is two expansions.
escaped() {
  local value=$1
  value=${value//\\/\\\\}
  value=${value//\"/\\\"}
  printf '%s' "$value"
}

case $type in
  respond)
    printf '{"view":null,"next_check":"45 minutes"}\n'
    ;;
  *)
    case $source in
      host)
        printf '%s\n' '{
          "view": {
            "kind": "choice",
            "title": "Fill in your interstitial journal?",
            "body": "The last entry was a while ago.",
            "options": [
              { "id": "yes", "label": "Yeah" },
              { "id": "no",  "label": "Nah" }
            ]
          },
          "next_check": "45 minutes"
        }'
        ;;
      *)
        printf '%s\n' '{
          "view": {
            "kind": "choice",
            "title": "An event arrived: '"$(escaped "$source")"' / '"$(escaped "$kind")"'",
            "body": "This view came from the ingested envelope, not the fixed prompt.",
            "options": [
              { "id": "ok", "label": "OK" }
            ]
          },
          "next_check": "45 minutes"
        }'
        ;;
    esac
    ;;
esac

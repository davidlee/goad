# A goad backend in ten lines of shell, and the one `just demo` runs.
#
# The protocol is one JSON document in on stdin, one JSON document out on
# stdout, per process. Nothing here needs a runtime, a package manager, or a
# shebang: the config names `["bash", "examples/shell/backend.sh"]`, so bash is
# the program and this file is its argument.
#
# Unlike `examples/typescript/backend.ts`, this one decides nothing. It prompts
# every time it is asked to evaluate, which is what makes it a demo: run it and
# a window is there. A real backend reads its own state and answers `view: null`
# far more often than not.

request=$(cat)

# Substring matching on JSON is not something to imitate — it is here so that
# this file stays readable without a parser. It also lets this file answer an
# `evaluate` without parsing its `event`: `source` and `kind` are the request's
# own first two keys (Event's field order, and the host serialises compactly),
# so `${request#*'"source":"'}` then `%%'"'*` reads a value with no parser —
# a match on `event.source` rather than on a `"source"` key `data` might carry.
case $request in
  *'"type":"respond"'*)
    printf '{"view":null,"next_check":"45 minutes"}\n'
    ;;
  *'"source":"host"'*)
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
    source=${request#*'"source":"'}
    source=${source%%'"'*}
    kind=${request#*'"kind":"'}
    kind=${kind%%'"'*}
    printf '%s\n' '{
      "view": {
        "kind": "choice",
        "title": "An event arrived: '"$source"' / '"$kind"'",
        "body": "This view came from the ingested envelope, not the fixed prompt.",
        "options": [
          { "id": "ok", "label": "OK" }
        ]
      },
      "next_check": "45 minutes"
    }'
    ;;
esac

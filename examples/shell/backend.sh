# A goad backend in one file of shell, and the one `just demo` runs.
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
#
# The form it sends is **protocol-shaped, not renderer-shaped**. It carries a
# field of a kind this renderer does not draw, on purpose: what a backend author
# copies from here is the contract, and a renderer that draws a subset of it
# reports the rest rather than narrowing what may be sent (SPEC-001/R-55). A
# form trimmed to what today's window happens to draw would teach the opposite.

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
# not reach this file's control flow. The `respond` arm reads three more values
# and **branches on none of them**; its own note says why the first occurrence
# is again the right one there.
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
    # The record: what the person answered, from this one exchange.
    #
    # `values` is one key per field the host drew of the option that was
    # pressed, and no key for any other field (SPEC-001/R-58). The host carries
    # it opaquely — writing it down is the whole of what a backend is for, and
    # a real one would put it in its own store rather than in a log line.
    #
    # Three more parserless reads, and the **first** occurrence is again the
    # right one for each: a `respond` carries no `event` and no `data`, its keys
    # are `protocol`, `type`, `now`, `view_id`, `response`, and `response`'s are
    # `option` then `values`. `values` is an object rather than a string, so it
    # ends where the response and the envelope do — `}}` — rather than at a
    # quote.
    view_id=${request#*'"view_id":"'}
    view_id=${view_id%%'"'*}
    option=${request#*'"option":"'}
    option=${option%%'"'*}
    values=${request#*'"values":'}
    values=${values%\}\}}

    # Stderr, because stdout is the single JSON document the protocol allows
    # and nothing else. The host captures stderr whatever the outcome (R-42)
    # and shows it on its diagnostic surface, so this line is readable from
    # inside the running product without leaving it.
    #
    # Neither `escaped` nor the control-flow rule is in play here: this line is
    # plain text rather than JSON, and none of the three values reaches a
    # branch — they are read, printed, and dropped.
    printf 'answered %s: option %s, values %s\n' "$view_id" "$option" "$values" >&2

    printf '{"view":null,"next_check":"45 minutes"}\n'
    ;;
  *)
    case $source in
      host)
        # `group` is a hint, carried flat as one of the field object's own
        # remaining keys (R-18) — as `multiline` is on the text field below.
        # A heading is drawn wherever the value changes, in declared order:
        # `started` carries none and is drawn under no heading at all, then two
        # runs of two. Nothing here is sorted and nothing is merged; the host
        # takes no position on what a backend groups by.
        #
        # `note` is a `text` field. This renderer draws `boolean` and reports
        # every other kind undrawn, so it will not appear in the window — it
        # will appear on the diagnostic surface, and its id will be absent from
        # the `values` recorded above. That is R-55 working, and it is the one
        # thing in this file that only matters because it is *not* drawn.
        printf '%s\n' '{
          "view": {
            "kind": "choice",
            "title": "Fill in your interstitial journal?",
            "body": "The last entry was a while ago.",
            "options": [
              { "id": "yes", "label": "Yeah",
                "fields": [
                  { "id": "started", "kind": "boolean", "label": "Made a start" },
                  { "id": "desk",    "kind": "boolean", "label": "At the desk",       "group": "Where you were" },
                  { "id": "outside", "kind": "boolean", "label": "Out of the house",  "group": "Where you were" },
                  { "id": "focused", "kind": "boolean", "label": "Focused",           "group": "How it went" },
                  { "id": "tired",   "kind": "boolean", "label": "Tired",             "group": "How it went" },
                  { "id": "note",    "kind": "text",    "label": "Anything notable?", "group": "How it went", "multiline": true }
                ] },
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

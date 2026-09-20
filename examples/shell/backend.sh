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
# The form it sends is **protocol-shaped, not renderer-shaped**, and that is the
# rule rather than an artefact of what the window happens to draw: what a
# backend author copies from here is the contract, and a renderer that draws a
# subset of it reports the rest rather than narrowing what may be sent
# (SPEC-001/R-55). It carries **one field of every kind `R-16` admits**, so
# `just demo` shows the whole of what a backend may ask for and a person can see
# every control the renderer draws.
#
# **Two knobs, and neither is a protocol feature.** A real backend has no such
# thing; these are here because the behaviours worth watching in this host are
# things it does *during* and *across* exchanges, and a demo that answers
# instantly every forty-five minutes shows neither.
#
#   GOAD_DEMO_DELAY=<seconds>   hold the reply. Unset, this file answers in
#                               milliseconds — a window too short to type into
#                               is a window in which nothing can be observed to
#                               survive. `3` is a window to work in; `6` is past
#                               `demo.toml`'s 5s timeout, so the exchange is
#                               refused, the refusal reaches the diagnostic
#                               surface, and the host carries on.
#
#   GOAD_DEMO_PULSE=1           draw the form **once** and then answer
#                               `"view": null` to every scheduled check, asking
#                               for the next one in a second. The host keeps the
#                               interaction outstanding on a null view, so each
#                               check re-presents *the same view* — which is the
#                               only condition under which *a present disturbs
#                               nothing* means anything. Presents then land
#                               about every three seconds (SPEC-002/R-4 floors
#                               the spacing), which is faster than a person can
#                               finish typing a word.

request=$(cat)

# Before anything is written, and deliberately **after** the read: the host
# bounds how long it waits for a reply, and `examples/demo.toml` sets that
# bound to **5s** — so `GOAD_DEMO_DELAY=3` is a window to work in and
# `GOAD_DEMO_DELAY=6` is the *other* thing this knob is good for: watching a
# backend get timed out, the refusal land on the diagnostic surface, and the
# host carry on and check again.
if [ -n "${GOAD_DEMO_DELAY:-}" ]; then
  sleep "$GOAD_DEMO_DELAY"
fi

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
        # **Branching on `kind` here, and only here, is safe.** This file's rule
        # is that a value the host carries opaquely must not reach its control
        # flow — and `source: "host"` is reserved to the host (R-56), so inside
        # this arm `kind` is the host's own word (`startup`, `requested`,
        # `scheduled`) and not a watcher's. The `*)` arm below, where `source`
        # is a watcher's, still branches on neither.
        if [ -n "${GOAD_DEMO_PULSE:-}" ] && [ "$kind" = "scheduled" ]; then
          # Nothing to show, and come back in a second. The host leaves the
          # drawn view outstanding rather than closing it, so this is what makes
          # every later check a *re-present of the same view* instead of a new
          # form.
          printf '{"view":null,"next_check":"1 second"}\n'
          exit 0
        fi
        if [ -n "${GOAD_DEMO_PULSE:-}" ]; then
          next_check="1 second"
        else
          next_check="45 minutes"
        fi
        # `group` is a hint, carried flat as one of the field object's own
        # remaining keys (R-18) — as `multiline` is on the text field below.
        # A heading is drawn wherever the value changes, in declared order:
        # `started` carries none and is drawn under no heading at all, then two
        # runs of two. Nothing here is sorted and nothing is merged; the host
        # takes no position on what a backend groups by.
        #
        # **One field of every kind, and the two `number` controls both drawn.**
        # Which control a `number` gets is the host's decision and not the
        # backend's: a range the host can prove operable draws a slider, and
        # every other `number` — `pages` here, with no bounds at all — draws a
        # text field. A backend states the range it means and says nothing about
        # controls (SPEC-001/R-17, R-35).
        #
        # `group` is a hint, carried flat as one of the field object's own
        # remaining keys (R-18), as `multiline` is on the text field. A heading
        # is drawn wherever the value changes, in declared order: `started`
        # carries none and is drawn under no heading at all, then the runs
        # below. Nothing here is sorted and nothing is merged; the host takes no
        # position on what a backend groups by.
        #
        # **What each field submits is fixed by its kind and nothing else**
        # (R-57), and the `answered` line on stderr is where to read it back —
        # visible on the host's own diagnostic surface without leaving the app.
        # Two are worth watching: an untouched `when` submits the Unix epoch
        # while the button reads *not set*, and `mood` submits the **alternative**
        # id (`ok`), never the option id.
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
                  { "id": "energy",  "kind": "number",  "label": "Energy",            "group": "How it went", "min": 0, "max": 10 },
                  { "id": "pages",   "kind": "number",  "label": "Pages written",     "group": "How it went" },
                  { "id": "mood",    "kind": "choice",  "label": "Mood",              "group": "How it went",
                    "options": [
                      { "id": "good", "label": "Good" },
                      { "id": "ok",   "label": "Fine" },
                      { "id": "bad",  "label": "Rough" }
                    ] },
                  { "id": "when",    "kind": "datetime", "label": "When",             "group": "How it went" },
                  { "id": "note",    "kind": "text",    "label": "Anything notable?", "group": "How it went", "multiline": true }
                ] },
              { "id": "no",  "label": "Nah" }
            ]
          },
          "next_check": "'"$next_check"'"
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

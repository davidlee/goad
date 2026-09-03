# A well-behaved backend that carries a whole round trip, in bash (AC-12, EX-3).
#
# It exists to distinguish a transport that works for any configured command
# from one that works for deno. No shebang and no executable bit: `command` is
# an argument vector and `bash` is argv[0] (R-36).
#
# Two things it does that the deno example deliberately does not.
#
# It echoes the request back on stderr, so a test can assert the exact bytes
# arrived — which is how R-35's claim is checked: an answer naming an option no
# view offered must reach the backend unchanged.
#
# And it appends one line per invocation to the file named by argv[2] — argv[1]
# is this script, since `bash` is argv[0]. That is the witness for "the backend
# was not spawned", a question the host cannot be asked to answer about itself,
# and argv is how it is parameterized: the command is an argument vector
# (R-36), so a caller needs neither an environment variable — process-wide, and
# racy under `cargo test` — nor a JSON parser bash does not have.
#
# It selects its answer by matching the raw request text, because bash has no
# JSON parser and the dev shell declares none. That is sound here and would not
# be in the example: the test controls the request, and the two values below are
# the two `harness` sends. Anything else is a broken fixture, and says so.
request="$(cat)"
printf '%s' "$request" >&2
printf '%s\n' "$request" >>"${1:?the invocation log path must be argv[2]}"

case "$request" in
*'"type":"respond"'*)
  printf '{"view":null,"next_check":"60 minutes"}\n'
  ;;
*'"minutes_since_entry":0'*)
  printf '{"view":null,"next_check":"45 minutes"}\n'
  ;;
*'"minutes_since_entry":90'*)
  printf '{"view":{"kind":"choice","title":"Log the last hour?","options":[{"id":"log","label":"Log it"},{"id":"skip","label":"Skip"}]},"next_check":"45 minutes"}\n'
  ;;
*)
  printf 'this script has no answer for that request\n' >&2
  exit 3
  ;;
esac

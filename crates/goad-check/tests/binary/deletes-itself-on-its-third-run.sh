#!/bin/sh
# Answers each evaluate conformingly and, on its third run, deletes itself, so
# the fourth spawn — the R-56 probe's — fails before any process starts:
# `BackendError::Spawn`, the configuration's side. The one way a cooperating
# backend fails the probe alone on a side other than its own.
#
# Run as the program itself, never through `bash`, because `bash` on a missing
# script spawns and exits 127, the backend's side. So it carries a shebang, and
# `a_probe_failure_on_another_side_is_not_charged_with_r56` copies it to a temp
# path and makes the copy executable; the committed file is never deleted.
#
# argv[1] is the invocation log, one line per run.
log="${1:?the invocation log path must be argv[1]}"
cat >/dev/null
printf 'invoked\n' >>"$log"
if [ "$(wc -l <"$log")" -ge 3 ]; then
  rm -f -- "$0"
fi
printf '{"view":null,"next_check":"45 minutes"}\n'

# Explains itself, then never answers (F-3, R-42). The assertion is that the
# explanation survives the timeout path, which is the failure mode with the
# least obvious cause and so the one that most needs a diagnostic.
#
# `exec`, not a bare `sleep`: bash running a script **file** forks, and a forked
# sleep is a grandchild holding both pipes — a different case, and PHASE-06's.
# See `hangs-past-the-timeout.sh` for the whole of that measurement.
echo "backend about to hang" >&2
exec sleep 30

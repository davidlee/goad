# The same backend, one redirection different: the grandchild inherits
# **stdout** as well, so stdout never reaches EOF (F-63). The response below is
# written and never read — the body cannot complete, the exchange pays the
# timeout, and disposal then pays the cleanup budget. Both dimensions fail, and
# this is the only case observed to do so.
#
# The host cannot distinguish "the backend is still writing" from "the backend
# exited and something else holds the pipe": both are a pipe with no EOF on it.
# The configured timeout is the only answer available.
cat >/dev/null
echo "$$" >&2
sleep 2 &
printf '{"view":null}\n'

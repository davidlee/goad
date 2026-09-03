# Never answers, and stays `bash` while doing it — no `exec`.
#
# That is the whole difference from `hangs-past-the-timeout.sh`, and it exists
# for one case: cancellation has to observe the child *while* the exchange is in
# flight, and an `exec`'d backend's `/proc/<pid>/cmdline` no longer names the
# script it came from. Bash running a script file forks its last command anyway,
# so the sleep below is a grandchild holding both pipes; this shell stays,
# holding nothing but its own copies of them.
#
# Five seconds, not thirty: the case aborts long before, and the grandchild
# outlives the kill either way.
cat >/dev/null
echo "$$" >&2
sleep 5

# Answers correctly and leaves a grandchild holding **stderr** (F-48, F-53,
# F-63). The response is delivered and the child is reaped; only the drain is
# stuck, so `result` is `Ok` and `cleanup` is `TimedOut` — the case the variant
# is named for, and the reason it is not called `Orphaned`.
#
# The bare `sleep` is the whole fixture, and it is the inverse of the `exec` the
# two hanging scripts carry: bash running a script *file* forks, so `sleep 2 &`
# is a grandchild that outlives the kill. `exec` here would destroy the case.
# Its stdout goes to /dev/null, so stdout still reaches EOF when this shell
# exits and the body completes — that is what separates this case from
# `leaves-a-grandchild-holding-stdout-too.sh`.
#
# Two seconds, not thirty: it need only outlive the 500 ms cleanup budget.
cat >/dev/null
echo "$$" >&2
sleep 2 >/dev/null &
printf '{"view":null}\n'

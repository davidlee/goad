# Floods stdout past the 8 MiB bound (R-43, F-2). The exchange fails with
# `OutputTooLarge`; this script's claim is the other half — that the cap path
# costs no part of the cleanup budget, because nothing outlives the kill.
#
# `exec` is load-bearing, and here it is load-bearing in the direction opposite
# to `hangs-past-the-timeout.sh`. Bash running a script *file* forks, so a bare
# `yes` on the last line would be a **grandchild** holding stderr: the drain
# would never reach EOF, and `cleanup` would come back `TimedOut` rather than
# `None`. Measured: 6 ms and `None` with `exec`, 507 ms and `TimedOut` without.
# The flood case would have quietly become a grandchild case, which is F-63's
# defect wearing new clothes.
#
# `$$` before the exec is the pid the host kills — `exec` replaces this shell,
# so the pid does not change.
cat >/dev/null
echo "$$" >&2
exec yes xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx

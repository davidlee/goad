# Never answers. The pid is written to stderr as bookkeeping, not as a
# diagnostic — R-41 asks for the process to be confirmed gone afterwards, and a
# pid the test can signal is the only confirmation independent of the host's own
# report.
#
# `exec` is load-bearing, and this is the one place the transport probe misleads:
# it drove its backends with `bash -c`, which execs the last command of the
# script it is given, so a bare `sleep 30` there *became* the child. Bash
# running a script **file** forks instead, so a bare `sleep 30` here is the
# child's own child — it outlives the kill, holds both pipes open, and turns
# this into the grandchild case, which is PHASE-06's and not this one. Measured:
# without `exec`, `cleanup` is `TimedOut` rather than `None`.
#
# With `exec`, the pid above is also the process the host kills, which is what
# makes R-41's confirmation independent of the host's own report.
echo "$$" >&2
exec sleep 30

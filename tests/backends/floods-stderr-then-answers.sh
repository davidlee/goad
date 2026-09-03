# Floods stderr past the 256 KiB bound and then answers correctly (D34, F-25,
# R-43). The exchange **succeeds** with `truncated` set: stderr is diagnostic, so
# the bound stops the host storing and never stops it reading.
#
# The size is load-bearing, and it is not the flood that has to exceed the 64 KiB
# pipe buffer — it is the part left **after** the bound. 400 KB is 256 KiB kept
# plus ~144 KB the host must go on reading. Measured at 300 KB the remainder was
# ~37 KB, which fits the buffer, and the case then passed against a host that
# stopped reading at the bound: right outcome, wrong reason.
#
# The last write is what makes this discriminating at all, and it took a
# break-and-revert to find. "Truncated, and the exchange succeeded" is true of a
# host that stops *reading* too, because a bounded reader that drops its handle
# closes the pipe, the flooder dies of `EPIPE` rather than blocking, and the
# answer still arrives. So the script asks the pipe a question the host cannot
# answer for it: one more line to stderr, after the bound, whose success decides
# which body is written.
#
#   host keeps draining  → the write succeeds → `{"view":null}`
#   host closed at bound → the write fails    → the other body, or a SIGPIPE death
#   host stopped, holding the pipe open → this blocks forever → timeout
cat >/dev/null
echo "$$" >&2
yes "stderr flood line pad pad pad pad pad pad pad pad pad" | head -c 400000 >&2
if echo "the host is still reading" >&2; then
  printf '{"view":null}\n'
else
  printf '{"stderr":"closed at the bound"}\n'
fi

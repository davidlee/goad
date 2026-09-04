# Answers without reading a byte of its request, then exits 0. Nothing in the
# protocol obliges a backend to read what it is sent (R-37 obliges the host to
# write and close, no more), and AC-12 names exactly this backend as sufficient.
#
# Exiting closes the read end of the host's stdin pipe. Whether the host's
# write then fails with EPIPE depends on timing alone — a write that lands
# before this script has finished starting succeeds, one after it fails — and
# a request past the pipe buffer (64 KiB on Linux) makes the failure certain,
# since the write cannot complete until something reads. The test that uses
# this script sends such a request so the case is deterministic, and asserts
# that the answer below is what the host reports either way (F-24).
printf '{"view":null}'
exit 0

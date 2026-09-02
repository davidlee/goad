# Exits before reading a byte of the request, which closes the read end of the
# host's stdin pipe.
#
# That alone is not enough to fail a write: a request smaller than the pipe
# buffer (64 KiB on Linux) is accepted by the kernel and sits there whether or
# not anyone will ever read it — measured, 20/20. `BackendError::Io` needs the
# write to be *in progress* when the reader goes, which is why the test that
# uses this script sends a padded request.
exit 0

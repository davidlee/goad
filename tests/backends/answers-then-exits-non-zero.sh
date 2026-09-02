# A valid response the exit status then disclaims (D15, R-40, F-59). The body
# is well-formed and must be discarded anyway; the stderr must not be.
cat >/dev/null
printf '{"view":null}\n'
echo "that answer is not to be trusted" >&2
exit 1

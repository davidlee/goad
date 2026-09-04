# Writes more than a pipe buffer to stdout *before* reading its request, then
# reads it, then answers. A host that writes the request before it starts
# reading stdout deadlocks here: the backend blocks on a full stdout pipe, the
# host blocks on a full stdin pipe, and only the timeout ends it (F-10, F-23).
#
# The flood is whitespace, which serde reads past, so what the host receives
# is one well-formed document with a very long prefix rather than garbage.
head -c 200000 /dev/zero | tr '\0' ' '
cat > /dev/null
printf '{"view":null}'

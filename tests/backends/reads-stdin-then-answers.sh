# A correct backend, and the case that proves the host closes stdin: `cat`
# reads to EOF, which is the obvious way to write one, and hangs forever if the
# host holds the pipe open (R-37, AC-12).
#
# The request is echoed back on stderr so the test can assert the exact bytes
# arrived, rather than only that something did. No shebang and no executable
# bit: `command` is an argument vector and `bash` is argv[0] (R-36).
request="$(cat)"
printf '%s' "$request" >&2
printf '{"view":null}\n'

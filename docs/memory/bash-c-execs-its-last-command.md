# `bash -c` execs its last command; a script file forks it

Measured at slice 001, PHASE-05 and PHASE-06 (`notes.md`), and it invalidated
a transport probe's grandchild measurement.

## The fact

`bash -c 'sleep 10; echo done'` runs `echo` as a child of bash, but
`bash -c 'echo start; sleep 10'` **execs** `sleep` as its last simple command —
bash is gone, the process the host spawned *is* `sleep`. The same two lines in
a script **file** run under `bash script.sh` fork every command, so `sleep` is
a **grandchild** of the process the host spawned, and killing the child does
not touch it.

## Why it matters here

The process transport reaps and kills the child it spawned, not the child's
children. A backend that leaves a grandchild holding stdout or stderr is the
case that makes the cleanup budget elapse (SPEC-001 R-41, R-48). A probe driven
with `bash -c` never produced that case, because its "grandchild" was the
child. The integration suite's misbehaving backends are script files under
`tests/backends/` for this reason, and two of them (`hangs-without-exec.sh`,
`leaves-a-grandchild-holding-*.sh`) exist to make the difference visible.

The `/proc` walk that checks for orphans has a positive control
(`transport.rs::a_backend_that_is_running_is_seen_as_a_child`) because a filter
blind to a backend that `exec`s passes by seeing nothing.

## How to apply

- Writing a fixture backend that must *be* the child: `exec` explicitly, or use
  `bash -c` and know the last command is the one that gets it.
- Writing one that must leave a grandchild: a script file, and put the
  long-running command somewhere other than last, or background it.
- Measuring process-tree behaviour: check with `ps -o pid,ppid,comm` before
  believing the number.

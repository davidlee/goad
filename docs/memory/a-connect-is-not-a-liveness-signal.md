# A successful `connect` is not a liveness signal in a process that forks

Learned at slice 004 (`review-code.md` F-18, F-19), where it was shipping code
for the whole slice and surfaced as a 1-in-35 test flake.

## The fact

*Something is listening at this path* and *a live host holds this path* are
different propositions, and `fork` separates them. A `fork` duplicates the
listening socket's file descriptor into the child, so the socket stays bound
and connectable after its owner has closed its own descriptor — for as long as
any child holds the copy, which is until it `exec`s. A process that forks
therefore reads its own stale sockets as live.

This is not `connect` being unreliable. It answers its own question correctly;
it is the wrong question.

Measured across three implementations: 2 failures in 200 sequential unloaded
runs of the affected case. It does not reproduce under parallel load, which is
where a flake chase looks first.

## The rule

Ask liveness of something a dead process **cannot still hold**. An exclusive
advisory lock (`flock`) on a sidecar file, taken before the bind and held for
the process's lifetime, is one such thing.

## Why that works — the obvious reason is wrong

It is **not** that the child's copy is `CLOEXEC` and gone at `exec`.
Inheritance is not the difference: a `flock` belongs to the **open file
description**, so a `fork` duplicates it by the very mechanism that duplicated
the listening descriptor above. This entry's own lesson applies to its own
remedy, which is why the wrong reason is worth writing down.

What makes the lock sound is narrower: **it is never released while the holder
lives**, so there is no release for a probe to race. The old failure needed a
release followed by a probe of the same path; the lock has no release until the
process ends.

The residue that leaves is real: an un-exec'd child of a host that has *just
died* holds the lock until it execs. That window is microseconds wide,
self-clearing, and reachable only at the instant of a death — against a
`connect` probe's window, which recurred on every fork for the whole of a
process's life.

## How to apply

- Any "is someone else already running here?" check on a filesystem path: use a
  lock whose lifetime is the process, not a probe of the artefact.
- **Making the fork window deterministic, for a test:** hand the descriptor to
  a child as its **stdin**. `dup2` onto fd 0 clears `CLOEXEC`, so the window
  that is microseconds wide in a real spawn becomes the child's whole life, and
  a race becomes a test. Reusable for anything that needs a fork window held
  open on purpose.
- Related: `a-green-test-can-assert-a-proxy.md` — the case that caught this had
  been green for the whole slice.

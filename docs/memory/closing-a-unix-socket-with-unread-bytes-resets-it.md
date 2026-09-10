# Closing an `AF_UNIX SOCK_STREAM` socket with the writer's bytes still unread resets the connection

Learned at slice 004, PHASE-03/VT-13, by measurement — the case failed with
exactly this error before a drain existed.

## The fact

If the host stops reading before the peer has necessarily finished writing, and
then drops the connection, the close is a **reset**. The peer gets `ECONNRESET`
on its read of the reply this host already wrote — the reply is taken down with
the connection.

`too_large` is the guaranteed case: the read stops at the byte cap, but the
writer may have sent, or still be sending, more.

## The fix, and the wrong fix

The wrong fix is cheap to reach for: wrap the drain in
`tokio::time::timeout(DEADLINE, …)`, the same shape the bounded read itself
uses. Measured, that **doubles** the time a silently stalled writer waits for
its refusal — ~502 ms to ~1.0025 s — because a writer with nothing queued gives
the drain nothing to do *but* wait out its own copy of the deadline.

The fix that costs nothing in the common case is **non-blocking**:
`UnixStream::try_read` in a bytes-bounded loop, stopping the instant nothing is
immediately readable, never waiting for more to arrive.

## The rule

Any refusal path that stops reading before a stream's peer has necessarily
finished writing needs a non-blocking drain before the connection closes.

**And the bounded-*wait* shape that is right for the read is the wrong shape to
reuse for the cleanup.** The read's bound exists to end a wait; the cleanup
step's job is to end instantly when there is nothing left. Two different jobs
that look like one.

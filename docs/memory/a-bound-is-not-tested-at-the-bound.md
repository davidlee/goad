# A bound is not tested by asserting the outcome at the bound

Learned at slice 001, PHASE-06 and the code review (`review-code.md` F-9,
F-43): the first test for "the stdout reader stops at the cap" passed against
a reader that borrowed the handle and one that owned it, and the first test for
"the read takes exactly one byte past the bound" passed against `take` and
against a reader that drained to EOF.

## Why the obvious test proves nothing

A reader that stops at the bound and a reader that keeps draining produce the
**same outcome** — `OutputTooLarge`, same limit, same status — when the backend
is a finite flood. The outcome is the thing both implementations agree on. What
they disagree on is *when the pipe closes* and *how many bytes were pulled*,
and neither is in the outcome.

## The tests that hold

Each case has to pose a **question the host cannot answer without the property**:

- **Ownership at the bound** — the backend reports whether it saw the pipe
  close *before* it exited (`floods-stdout-and-reports-the-broken-pipe.sh`), so
  a reader that drops the handle at the return rather than at the bound fails.
- **Exactness** — a `Counting` reader that fills every buffer it is offered
  and counts what was pulled; the assertion is `limit + 1` bytes exactly
  (`process.rs::a_refused_read_takes_exactly_one_byte_past_the_bound`). A
  reader that only *eventually* stops reads more.
- **The other bound's difference** — the stderr bound truncates and keeps
  draining; the test writes past one pipe buffer *before* the response and
  would deadlock, not fail, under a reader that stopped
  (`transport.rs::a_stderr_flood_is_truncated_and_the_exchange_still_succeeds`).

And each mechanism needs a **positive control** — a case showing the probe
would have seen the thing it is looking for (`a_backend_that_is_running_is_seen_as_a_child`
for the `/proc` walk; the vacuity guards on every scan).

## How to apply

Before writing a test for a limit, name the two implementations it must tell
apart and check that its assertion differs between them. If it does not,
the assertion is about the outcome, not the bound; find the observable that
differs — a count, a timing, a side effect the other party reports — and
assert that.

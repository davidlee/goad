# A log line a backend writes before it reads its request is not evidence the exchange finished

Learned at slice 003's code review, round 3 (`review-code.md` F-22, the slice's
only blocker), and swept as a class rather than fixed at the one site.

## The fact

`tests/backends/logs-the-request-then-answers.sh` appends its log line **before**
it reads the request from stdin. A test that waits for the line and then stops
the host has synchronised on the *start* of the exchange, not its end: the cancel
arm drops the in-flight call, `absorb` never runs, and the state the assertion
reads is `None`.

It reproduced at 3 red in 5 full renderer runs and 12 of 12 green when the test
ran alone. That asymmetry is the signature — a race that only loses when the
machine is busy looks like flakiness and gets blamed on the machine.

## The rule

**Synchronise on an observable the work itself produces, downstream of the state
you are about to assert on.** In this tree that is the rendered next-check line:
`glass.present` writes it at the top of the iteration *after* `absorb`, so it
cannot be read early. `scheduling.rs::absorbed_line(rfc3339)` names it, and six
sites wait on it.

Five of those six previously had a fixed 20 ms or 500 ms sleep standing between
a log line and an assertion about what the exchange resolved. A delay used as
synchronisation is the same defect with a longer fuse.

## How to apply

- Before waiting on a log line, read the script that writes it and ask what has
  and has not happened by the time it lands.
- Prefer an observable the host renders after the state transition you care
  about. A sleep between the two is not a fix.
- The exception is real and worth keeping visible: a test that needs the exchange
  **still in flight** when a stop lands wants the early line precisely because it
  is early. Slice 003 left two `@hang` cases on the log line for that reason, with
  the reason written down. Say which requirement you have before choosing.

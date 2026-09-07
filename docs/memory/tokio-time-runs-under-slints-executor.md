# A `tokio::time::Sleep` polled by Slint's executor under an `EnterGuard` keeps time

Measured at slice 003 by spike S-1, `research.md` Thread 5 (*Spike S-1 result*),
on 2026-09-07. Slint 1.17.1. The probe itself was
`docs/slices/003/timer-probe.local.rs`, gitignored and preserved for re-running.

## The fact

The host's production arrangement is Slint's own executor driving a
`slint::spawn_local` future, with a multi-thread tokio runtime's `EnterGuard`
held for the life of the event loop. tokio's timer works there, unmodified:

| what was put in the `spawn_local` future | measured |
|---|---|
| `tokio::time::sleep(300 ms)` | 301.5 ms |
| a pinned `Sleep`, `reset()` to 200 ms, in a `biased` `select!` | fired via the timer arm at 201.3 ms |
| the same `Sleep`, `reset()` **after it had already fired**, to 150 ms | fired via the timer arm at 150.4 ms |
| a real `Host::evaluate` over a real child process | 2.9 ms, `next_check` resolved |
| `sleep_until` an instant 10 s in the **past** | 2.1 µs, no spin, no underflow |

The whole probe cost 656 ms.

## Why it matters

There is no second time source to add. `slint::Timer` does **not** run under
`init_no_event_loop()`, which is the harness the cheap renderer tier uses, so
reaching for it would mean two timer facilities and two sets of tests. tokio is
already a dependency of stratum 3 and already what the transport's own budgets
use, so choosing it cost no manifest change.

The `reset()`-after-firing result is the load-bearing one: it is exactly the
re-arm a scheduling loop performs on every exchange, and a facility that could
not be re-armed after firing would have forced a different loop shape.

## How to apply

- Do not add a second timer facility, and do not assume a future polled by
  Slint's executor needs one. Measure before believing an async primitive is
  unavailable under that executor.
- An already-elapsed `sleep_until` deadline completes immediately. It does not
  hang and it does not underflow, so a past instant needs no special case in the
  loop — only in the arithmetic that produces the duration.
- Incidental noise, not a failure: constructing a `Tray` under the headless
  testing backend logs `Slint: Failed to create system tray icon: Failed to
  create a rgba8 buffer from an icon image`. Existing tests build a `Tray` the
  same way and pass.

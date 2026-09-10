# `invocations(&log) >= n` proves an exchange began, not that it was absorbed

Learned at slice 004, PHASE-04 and PHASE-05, by reproduction rather than
inference.

## The fact

Waiting on the backend's invocation count tells you an exchange **started**. It
says nothing about whether the loop has absorbed that exchange's result. Under
`just check`'s concurrent load the gap between the two is wide enough to flip
an assertion: a second envelope sent right after `invocations(&log) >= 2`
landed *before* the second exchange's `absorb`, so the loop was still engaged
and the reply was `engaged` where the case expected `too_soon`.

Reproduced twice, back to back, in two separate cases. Under 48-way CPU
oversubscription the pre-fix binary failed 13 of 25 runs; with the fix, 40 of
40 passed under heavier load still.

## The rule

Never depend on state a prior exchange's `absorb` set after waiting only on an
invocation count.

## How to apply

Wait for the exchange's **own rendered result** to appear instead — for this
codebase, `window.get_next_check() == next_check_line(...)` for that exchange's
own instruction. `scheduling.rs`'s `absorbed_line` doc comment names this race
for the scheduled-firing case; it applies identically to an ingested one.

The tell that a case is exposed: it sends a second thing, or reads a value the
loop writes, immediately after an invocation-count wait.

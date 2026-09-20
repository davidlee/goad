# The audit stage needs its own budget, and the repairs are half of it

Confirmed four times, most sharply in slice 009: planned as four sessions, it
took **seven**, and every one of the extra three went into the review loop.

## The fact

`docs/AGENTS.md` treats audit as one stage after the last phase. It is not one
stage. It is a loop, and the loop's cost is dominated by something the plan
never sizes: **reviewing the repairs**.

Slice 009's shape:

| round | what it reviewed | found |
|---|---|---|
| 1 | the implementation | a blocker and six majors of live defect |
| 2 | round 1's repairs | two majors, both *about what holds a repair* |
| 3 | round 2's twelve repairs | one live defect, one coverage gap, four false claims |
| 4 | round 3's seven repairs | no defect; seven false claims |

Rounds 2 and 3 each found a **major in a repair**, and twice a repair
re-created the very defect its own commit was closing. So the repairs are not
the cheap tail of the audit — they are where the remaining defects live, and
they cost about half the total.

## Why the estimate is always wrong

The plan sizes the audit against the code, because that is what exists when the
plan is written. What actually drives the cost is the number of *findings*,
which nobody knows yet, times the rounds needed to confirm their repairs, which
depends on how many repairs are themselves wrong.

Slice 009's round 1 nearly doubled mid-round — from fourteen findings to
twenty-one — when two areas a reviewer had briefed and never reached were run
as a fresh agent rather than read as a clean surface.

## How to apply

- **Budget the audit as at least two units of work: the review, and the review
  of the repairs.** If the slice is large, assume more.
- **Re-budget after round 1 lands**, when the finding count is real. Say the new
  number out loud rather than absorbing it.
- **Checkpoint rather than push through.** Wrap an agent session at roughly
  200–250k tokens with a PARTIAL section stating what is done, what is
  outstanding *in the order it should be taken*, and what not to rediscover.
  Slice 009's checkpoints are what made seven sessions survivable.
- **Do not let the harvest be the tail of the last session.** Lifting durable
  facts into `docs/memory/` is its own pass; done tired, it is done badly, and
  it is the one part of the audit that pays forward into every later slice.
- **Watch for the exit.** When a round returns findings about the *record*
  rather than about behaviour, the next step is mechanical verification, not
  another round — see `review-rounds-stop-on-a-measured-trend.md`.

Related: `review-rounds-stop-on-a-measured-trend.md`,
`writing-the-repair-down-is-the-review.md`,
`a-repair-can-be-wrong-about-what-it-holds.md`.

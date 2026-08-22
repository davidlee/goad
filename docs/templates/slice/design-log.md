# Design log — Slice NNN

Append-only working record for the design stage. Survives compaction and
interruption; the design document itself stays clean. Never rewrite an entry —
supersede it with a later one.

## Decisions

<!-- One entry per user decision, recorded immediately after the answer. -->

### YYYY-MM-DD — <question in one line>

- **Asked:** <the question and the options presented>
- **Recommended:** <agent's recommendation, if any>
- **Decided:** <the user's answer, verbatim where it matters>
- **Consequence:** <what changes in the design; D-ref if it became a §7 decision>

## Adversarial review

<!-- One block per review round. Findings are append-only and keep their ids
     across rounds. -->

### Round 1 — <reviewer> — YYYY-MM-DD

**Brief given:** <what the reviewer was asked to attack>

| id | severity | finding | disposition | resolution |
|----|----------|---------|-------------|------------|
| F-1 | blocker / major / minor / question | | accepted / rejected / tolerated / deferred | |

<!-- severity — blocker: cannot proceed. major: design is wrong or unsound.
       minor: real but survivable. question: reviewer needs an answer.
     disposition — accepted: the finding stands, design changes.
       rejected: with evidence, not assertion. tolerated: real, accepted
       knowingly, with the rationale. deferred: becomes a follow-up.
     Confirm each disposition with the user before integrating. Fix the class,
     not the instance; do not introduce new flaws repairing old ones.
     Repeat rounds until repairs are themselves reviewed and nothing serious
     remains. -->

**Synthesis:** <what the round changed, and the standing risks it leaves.>

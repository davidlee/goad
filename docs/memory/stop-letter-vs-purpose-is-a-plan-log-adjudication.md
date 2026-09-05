# A STOP whose letter fires but whose purpose doesn't is adjudicated in the plan log, with the hunks pasted

Learned at slice 002, PL-13, PL-14, PL-17 (`plan-log.md`).

## The pattern

A plan's exit criteria and STOP conditions are guards against a specific
failure (usually: the phase turned into a redesign, or a verification item
was discharged dishonestly). Real execution sometimes trips a STOP's
**letter** — the literal condition as written — while its **purpose**
plainly is not engaged: a criterion asserting an equality that a cleaner
mechanical result exceeds (PL-13), a comment-only hunk in a production file
that PS-1 exists to catch redesigns in (PL-13), a verification finding that
a criterion's original wording under-specifies what "asserted" must mean
(PL-17).

The adjudication is never made silently or by the executor's own
initiative. It is written into `plan-log.md` as a numbered decision:
**Asked** (what the letter vs. purpose tension actually was), **Decided**
(the amended criterion or accepted judgement), **Why** (the purpose that
survives), **Rejected** (the alternative of just halting, and why it was
worse), **Consequence** (what downstream phases now inherit). The actual
diff hunks or evidence are pasted into the entry, not summarized, so a
later reader can check the judgement rather than trust it.

PL-14 generalizes this into a standing rule for an unattended run: an
executor that hits a STOP records what happened and returns a stop status;
only the orchestrating session may judge a purpose-not-engaged continuation,
and it must record that judgement the same way. Every other STOP halts for
the user.

## How to apply

- Hitting a STOP or a falsified criterion during autonomous execution: do
  not silently reinterpret it. Write the tension into the plan log with the
  Asked/Decided/Why/Rejected/Consequence shape, paste the actual evidence,
  and only continue past it if the log entry's own reasoning would survive
  a skeptical reader.
- A criterion's *letter* failing is not the same question as its *purpose*
  being engaged — always ask both, and answer both in writing before
  deciding whether to continue.
- This is for the orchestrating session's judgement, not an individual
  executor's — a subagent doing one phase's work hits a STOP and stops; it
  does not adjudicate past it.

# The false statements in a review are strengthenings of true ones, not careless copies

Learned in slice 007's code review. Across five rounds, **eight findings were
prose in the ledger or in a doc comment asserting something untrue of the tree**
— F-7, F-8, F-12, F-14, F-18, F-19, F-20, F-23. Rounds 2, 4 and 5 each caught
the responder doing it, in repairs made carefully, immediately after reading the
standard they failed to meet.

The useful part is *how* they were wrong. None was a typo or a careless
transcription. Each was a small edit that made a true sentence stronger:

- **A scope widened.** F-11 wrote "every marker name in the `renderer` binary
  arrives through `logging_backend`'s prefix" — true. F-21's Response widened it
  to "the two claiming binaries" — false, because one call site mints a bare
  marker directly (F-23). Nothing else changed.
- **A class replaced by a list.** "held wherever such a path is minted" became
  "the three `socket_path` helpers", enumerated by path. Twice short afterwards
  (F-17, F-22).
- **A number carried across the edit that moved it.** F-18's Outcome cited the
  two cases at the line numbers the *raiser* had measured, in a round whose own
  repairs had already shifted them — wrong before the round closed, for the
  reason that same finding diagnoses one section above (F-24).
- **A count inherited from a document rather than the code.** "the thirty-three
  call sites" came from the finding's text; the tree has 35, and 33 was a
  different file's row count (F-18).

**Why it matters.** A reviewer looking for carelessness will not find these: the
sentences read *better* than what they replaced, which is why they survived the
edit. The check that catches them is not re-reading — it is re-measuring. F-19's
false count could have been disproved by arithmetic alone, in less time than it
took to write.

**How to apply.** Any sentence in a repair that quantifies — *every*, *all
four*, *the two*, *only*, a line number, a count — is a claim, and writing it is
not evidence for it. Run the grep before writing the number, and again after the
repair moves the lines. Prefer the formulation that cannot drift: name the
symbol, not the line; state the fact, not the tally. When correcting such an
error, show the original rather than editing it away — a ledger that edits away
its own wrong record is worth less than one that shows it.

See also [[a-count-in-a-comment-is-a-claim-nothing-checks]],
[[enumerate-the-class-not-the-instances]], and
[[read-the-artefact-not-the-agent-report]].

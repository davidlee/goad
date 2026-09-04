# Code cites requirement ids, not review-finding ids

Settled at slice 001's audit (`review-code.md` F-13, F-44), and in force from
slice 002.

## The rule

A comment in `src/`, `tests/` or `examples/` that needs to say *why* cites a
spec requirement (`SPEC-001/R-26`), a spec section (`SPEC-001 §6.4`), an ADR, or
the brief. It does not cite a slice-local review finding (`F-N`) or design
decision (`D-N`).

## Why

`F-N` and `D-N` are unique only inside one slice folder — slice 001 alone has
three ledgers whose ids overlap (`review-design.md` F-53 and `review-code.md`
F-53 are different findings). A reader of the code has to know which ledger the
author meant, and a future slice's ledger will reuse the number again. `R-N` ids
are immutable, append-only, and qualified by spec id when there is more than one
spec; they survive promotion, and every one has a §7 row naming the test that
holds it.

Slice 001's code keeps its `F-N` citations (tolerated by user decision: they grep
to the slice's own ledgers, and a sweep would trade a pointer for a paraphrase).
Do not extend the practice.

## How to apply

- Reaching for `F-N` in a comment: find the requirement the finding produced
  and cite that. If no requirement states it, that is a gap in the spec, not a
  reason to cite the ledger.
- A rationale that is not a requirement — a measurement, a runtime quirk —
  belongs in `docs/memory/` and is cited by file name.

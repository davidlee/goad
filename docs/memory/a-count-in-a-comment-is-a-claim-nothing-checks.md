# A count written in a comment is a claim, and nothing in this repository checks it

Learned across slice 007, PHASE-01 through PHASE-06 — four phases running, four
different mechanisms, one instrument.

## The fact

A comment that counts something — "nine modules after 005", "in ten lines of
shell", "thirty-three rows", "the four callers" — is an assertion about the file
it sits in. No compiler, no lint, no gate step and no `grep` for the new name
reaches it. It goes stale the moment the thing it counts changes, and it goes
stale **silently**, which a wrong statement in prose does not.

The four in one slice:

- `lib.rs`'s module count, stale the moment `draft` landed (PHASE-03 F-4).
- `backend.sh:1`'s *"in ten lines of shell"*, on a file past thirty (PHASE-06
  F-3).
- PHASE-01 F-5 and PHASE-02 T-11, the same shape in two more files.

## Why

Every other kind of stale comment is *wrong* and reads as wrong. A count reads
as precise — it looks like it was measured, so a reader trusts it instead of
checking, and a reader who trusts it is the one the comment costs. That is the
same asymmetry as `a-green-test-can-assert-a-proxy.md`: the artefact asserts
more confidence than it holds.

It is not that agents are careless with counts. It is that a count is the one
kind of claim in a comment with **no** feedback path at all. A stale
`path:line` at least breaks when someone follows it.

## How to apply

- **Prefer not writing the count.** "The modules `lib.rs` declares" beats "the
  nine modules `lib.rs` declares" and costs the reader nothing. This is the only
  fix that scales.
- Where a count genuinely carries meaning — *"thirty-three rows, and a row
  without a name is reported by index"* — put it where something checks it: an
  assertion over the array's length, not a sentence above it.
- When you change a file, **read the file**, comments included. That is the only
  instrument for this class and it is not automatable at reasonable cost: a
  checker would have to understand what each count counts.
- Reconcile a count to the thing it describes, never to another count. Slice
  007's PHASE-02 T-11 got a total right by summing greps and still described the
  wrong list.

Related: `cite-requirements-not-finding-ids.md` and
`a-repair-sweep-misses-the-binding-site.md` — both about a comment saying
something that used to be true.

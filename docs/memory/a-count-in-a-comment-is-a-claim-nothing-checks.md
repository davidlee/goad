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
kind of claim in a comment with **no** feedback path at all.

**Amended after slice 009, which falsified the sentence that used to end this
paragraph.** It read: *"A stale `path:line` at least breaks when someone follows
it."* It does not. A stale line number resolves silently to whatever is now at
that line — usually a comment — and nobody follows it, so it rots exactly as a
count does and is read as a link rather than as a claim. Measured: 53 in-repo
line citations, 27 landing on a comment or a blank line. See
`cite-by-symbol-not-line-number.md`.

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

## This memory existing was not enough

Slice 009 broke the *"prefer not writing the count"* rule twice more — `F-C4`
(a schedule header naming a `STEP` the file no longer had), `F-D1` (*"All six
`init` handlers"* when a repair had made it seven) — and `F-D1`'s miscount was
introduced by the commit that closed `F-C6`, whose own Response and commit
message both called the new handler *"the sixth"*.

So the operative lesson is stronger than the one first written here: **a rule
recorded in `docs/memory/` does not stop the next agent writing the count.**
What stopped it, eventually, was repairing the two findings by **deleting the
number** rather than correcting it — *"Every `init` handler in the markup is
`root.inits += 1`"*, *"the same reason as the earlier reopen"*. There is then
nothing left to go stale, and no third round.

Correcting a count is repairing the instance. Removing it is repairing the class.

## Slice 006 put it in canon

It happened twice more in one slice — `StartupError`'s doc said *eight* with
nine in the enum, and the repair for that missed a second count eleven lines
above it in the same file, in the enum the slice had itself grown from two
variants to three by **hand-incrementing the number**. Three separate
enumerations of the class, each setting out to be exhaustive, passed over it.

So the rule is now in `CLAUDE.md` §Working here rather than only here, stated as
one rule with two halves and one reason: **name, never count — and cite by
symbol, never by line number.** A named thing survives an edit above it; a
number does not. It binds canon explicitly, because that is where it rots worst:
nothing re-reads a spec on the commit that moves the line it cites or adds the
variant it counts. Slice 006's audit found three wrong line citations in
SPEC-003, **two of which had been wrong before that slice opened**.

The exemption, stated once so it does not have to be re-argued: a count of
something that **cannot grow** — a closed list a test holds, or a statement
about a finished sequence.

Related: `cite-by-symbol-not-line-number.md`,
`cite-requirements-not-finding-ids.md` and
`a-repair-sweep-misses-the-binding-site.md` — all about a comment saying
something that used to be true.

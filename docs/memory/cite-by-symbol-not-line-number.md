# Cite by symbol, never by line number

Learned across slice 009's audit, where the same class was raised, repaired, and
raised again in the commit that repaired it — three rounds running.

## The fact

A `file.rs:120` written in a doc comment is wrong the next time anything above
line 120 moves. It does not break, it does not warn, and **nobody follows it**,
so it rots in place and is read as precise.

Measured at the end of slice 009, over `crates/goad/src` and
`crates/goad/tests`:

| | |
|---|---|
| in-repo `file.rs:NNN` citations | **53** |
| landing on a comment or a blank line | **27** |
| of those, simply wrong | **~24** |
| accounted for by **one** file's drift (`main.rs`) | **13** |

Nine of the twenty-four were the *same* citation — `main.rs:86`, cited across
seven files as *"the command channel holds one"*, for a channel that had moved
to `main.rs:98`.

The history is the argument. **F-T4** raised three citations pointing at a
comment instead of the guard they named. **F-C2** repaired them and called it
*"a class repair"*. **F-D3** found that the same commit had **re-created** the
defect — nineteen new lines of doc comment moved a guard nineteen lines down,
under a citation round 3 had just verified — and that three more had been wrong
all along.

## Why

A count at least looks like a count. A line number looks like a *link*, and the
reader's model of a link is that following it either works or visibly fails.
This one silently resolves to the wrong place, and the wrongness is invisible
from the citing site — you have to open the other file to see it.

It is worse than the count class (`a-count-in-a-comment-is-a-claim-nothing-checks.md`,
which claims a stale `path:line` "at least breaks when someone follows it" —
slice 009 measured that it does not) because the cost is *concentrated*: one
edit to a popular file invalidates every citation of it at once, across files
nobody is looking at.

## How to apply

- **Name the symbol.** A function, method, type, constant or test:
  `` `SlintGlass::present` ``, `` `Debounce::hold` ``, `` `MINIMUM_SPACING` ``.
  A symbol moves with its definition.
- **Where no symbol exists** — a local binding inside a function is the common
  case — cite **the enclosing function plus a quoted fragment of the line**:
  ``the capacity-1 channel `main` creates (`mpsc::channel::<Command>(1)`)``.
  A quoted fragment is greppable and cannot rot. Slice 009's one surviving
  mutation citation survived a re-numbering for exactly this reason: it quoted
  the guard's source text beside the wrong line number.
- **Vendored citations keep their line numbers.** `i-slint-core-1.17.1/model.rs:211`
  is pinned to an exact version and cannot move; a symbol there is less useful,
  not more. Carve this out explicitly or the next agent converts those too.
- **A discipline is not an instrument.** F-C2 repaired the class with a rule and
  nothing enforced it, which is why F-D3 exists. If the class matters, the check
  is about twenty lines: resolve each citation and print the line that is
  actually there. Slice 009 ran one from a scratchpad and declined to land it;
  landing it is still owed.

The rule is now in `CLAUDE.md` §Working here.

Related: `a-count-in-a-comment-is-a-claim-nothing-checks.md`,
`cite-requirements-not-finding-ids.md`, `enumerate-the-class-not-the-instances.md`.

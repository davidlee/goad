# A `contains` scan and a word-boundary scan are different instruments, and neither substitutes for the other

Learned at slice 003, PHASE-04 (`design.md` FD-2, D-16, D-23) and hardened at its
code review (F-1, F-17, F-18).

## The fact

`crates/goad-boundary` carries two matchers and they answer different questions:

- **`str::contains`**, over a path token such as `schedule::resolve`. Safe for a
  path, because no identifier word can contain `::`.
- **`scan::mentions`**, which splits words on non-alphanumeric bytes and at case
  boundaries. This is what "no production line names the identifier `resolve`"
  requires — a `contains` scan would also catch `resolve` inside
  `resolve_from`, `unresolved` and every other longer identifier, and would
  assert something nobody meant.

`structure.rs` now carries one of each, named separately.

## Three ways such a scan passes while holding nothing

Each of these was live in this tree and each is now closed:

1. **A path substring misses a brace-grouped `use`.** `use schedule::{resolve};`
   followed by a bare `resolve(…)` call defeats a scan looking for the path.
   The repair was to match the resolving **call** — `resolve(` with no identifier
   byte before it — plus a second assertion catching the path taken as a value
   without being called (`let _f = …::resolve;`).
2. **A literal desynchronises the walk.** Counting braces over raw text lets one
   unbalanced `{` inside a test module's string literal blind the scan to the
   rest of the file. `scan::code_without_literals` is `code_of`'s sibling over
   the same state machine, replacing each literal with one space.
3. **A `#[cfg(test)]` cut at the first occurrence hides production code after
   it.** The walk must skip each test **item** and resume, not truncate the file.
   The fixture `tests/fixtures/structure/production_after_tests.rs` carries the
   shape no real subject file has.

## How to apply

- Say which question the scan answers — a *path* or an *identifier* — and pick
  the matcher to match. Do not reuse one instrument's helper for the other's job.
- Every scan needs a vacuity guard that fails when it inspected nothing: a
  non-zero file count, every file yielding at least one production line, and a
  directory-level line-count floor. A scan over a directory that was renamed away
  reports no violations and has stopped testing anything.
- Prefer matching the **call** over matching the name. A reworded diagnostic or a
  renamed private helper should not red the suite, and a real new call site
  should.
- Keep an on-disk fixture for each defeat you close, and check it fails before
  the repair. See also `docs/memory/grammar-seams-are-pinned-as-fixtures.md`.

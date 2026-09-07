# A test helper shared by two workspace members lives at the workspace root, pulled in by `#[path]`

Learned at slice 002, PHASE-01 (the `driving.rs` cut) and PL-4
(`plan-log.md:85-110`).

## The pattern

A test helper needed by more than one test target, in more than one
workspace member, is not its own crate and is not duplicated per member. It
lives once at the workspace root (`tests/support/driving.rs`) and each
including target pulls it in with a literal path attribute:

```rust
#[path = "../../../../tests/support/driving.rs"]
mod driving;
```

## The rule that goes with it (PL-4)

Every symbol in the shared file must be called by **every** including
target, not just some of them. When PHASE-06 needed `describe_outcome` in
only one of the two includers, it was moved out to that target's own
`harness.rs` rather than left in the shared file — a shared helper carrying
a symbol only one caller uses is a coupling nobody asked for, and the next
reader has no way to tell which caller a given function is really for.

## The rule's sharper edge (slice 003, FD-3 / D-18)

"Every symbol must be used by every includer" is not only about tidiness — the
gate makes it a compile error, and the error appears **only at the new
includer's own build**. `dead_code` is `warn` and `-D warnings` promotes it, so
adding a third target that includes a shared file fails on every `pub(crate)`
symbol that target does not use.

Slice 003 added a third includer (an event-loop test target for one topology)
and found thirteen of nineteen symbols unreachable from it. The answer was to
**split the file** — the scripted-backend helpers moved out to their own shared
file, unchanged, so the new target could include only what it uses — not to
suppress the lint. A module-wide `#[expect(dead_code)]` over a hand-written
helper is not the site-local exception
`docs/policy/001-the-phase-gate.md` §Compliance authorises.

Note the asymmetry: a new includer that uses only *part* of a shared file's
surface compiles fine. It is the file's *unused* surface, seen from the new
includer, that fails. So the cost of adding an includer scales with what the
file exports, not with what the includer needs.

## How to apply

- Before adding something to a workspace-root shared test helper, check it
  is genuinely needed by every target that includes the file. If not, it
  belongs in the one target that needs it.
- When a later change makes a shared symbol unused by one includer, move it
  out immediately rather than leaving dead weight in the shared file — this
  re-settlement is expected maintenance, not a sign the split was wrong.
- Do not reach for a new crate to share test-only code across members; the
  `#[path]` pattern above is the lighter, already-established tool.
- Before adding a third includer, read the shared file's whole exported
  surface, not just the part you want. If most of it is unreachable from the
  new target, split the file rather than suppressing `dead_code` — and bill the
  orphaned imports the split creates
  (`docs/memory/a-bounded-surface-is-billed-from-the-compiler.md`).

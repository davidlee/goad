# `git stash` is forbidden here; an autonomous session recovers read-only

Learned at slice 002, two separate executor incidents this slice, both
recovered and byte-verified.

## The rule

`CLAUDE.md` forbids `git stash` (create or pop) without explicit user
agreement. This held even under PL-14's autonomy grant for the run's
unattended phases — a STOP-worthy action is not something an executor
takes on its own initiative just because no user is present to ask.

## What to do when a stash already exists

Read it without touching it: `git show stash@{N}:<path>` recovers a file's
content at a given stash entry with no `pop`, `apply`, or `drop` — the
stash stays exactly as it was, and nothing in the working tree changes.

This slice hit the situation twice: once from a stray stash left over from
an earlier session, once from an executor incident mid-run. In both cases
the recovery was read-only, and the recovered content was verified
byte-for-byte against what it should have been before anything else
proceeded. Dropping a stash once it is confirmed redundant with the working
tree is still the user's call, not a phase's or an autonomous session's.

## How to apply

- Never run `git stash` (push, pop, apply, or drop) without the user
  explicitly agreeing to it in that moment — a prior grant of autonomy for
  other actions does not extend to this one.
- If a stash exists and you need to know what it holds, use
  `git show stash@{N}:<path>` (or `git stash show -p stash@{N}`) — never
  `pop` or `apply` to "just look".
- Verify recovered content byte-for-byte (`diff`, not eyeballing) before
  relying on it for anything downstream.
- Leave the decision to drop a stash to the user, even once you've
  confirmed it's redundant.
